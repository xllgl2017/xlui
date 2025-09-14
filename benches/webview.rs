#![windows_subsystem = "windows"]

use std::sync::Arc;
use std::{ptr, sync::mpsc};
use windows::{
    core::*,
    Win32::{
        Foundation::{E_POINTER, HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM}
        ,
        System::{Com::*, LibraryLoader, Threading},
        UI::WindowsAndMessaging::{self, MSG, WNDCLASSW},
    },
};

use webview2_com::{Microsoft::Web::WebView2::Win32::*, *};
use windows::Win32::UI::WindowsAndMessaging::{WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use xlui::{Size, UiResult};

fn main() -> UiResult<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
    }
    // set_process_dpi_awareness()?;
    let frame = FrameWindow::new();
    let mut webview = WebView::create(frame, true)?;
    webview.navigate("https://www.baidu.com/")?;
    webview.run()
}

pub struct FrameWindow {
    window: HWND,
    size: Size,
}

impl FrameWindow {
    fn new() -> Self {
        let hwnd = {
            let window_class = WNDCLASSW {
                lpfnWndProc: Some(window_proc),
                lpszClassName: w!("WebView"),
                ..Default::default()
            };

            unsafe {
                WindowsAndMessaging::RegisterClassW(&window_class);

                WindowsAndMessaging::CreateWindowExW(
                    Default::default(),
                    w!("WebView"),
                    w!("WebView"),
                    WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                    WindowsAndMessaging::CW_USEDEFAULT,
                    WindowsAndMessaging::CW_USEDEFAULT,
                    800,
                    600,
                    None,
                    None,
                    LibraryLoader::GetModuleHandleW(None).ok().map(|h| HINSTANCE(h.0)),
                    None,
                )
            }
        }.unwrap();

        FrameWindow {
            window: hwnd,
            size: Size { width: 800, height: 600 },
        }
    }
}

type WebViewSender = mpsc::Sender<Box<dyn FnOnce(WebView) + Send>>;
type WebViewReceiver = mpsc::Receiver<Box<dyn FnOnce(WebView) + Send>>;

pub struct WebView {
    controller: ICoreWebView2Controller,
    webview: ICoreWebView2,
    thread_id: u32,
    frame: Arc<FrameWindow>,
    url: String,
}

impl Drop for WebView {
    fn drop(&mut self) {
        unsafe { self.controller.Close().unwrap() }
    }
}

impl WebView {
    pub fn create(parent: FrameWindow, debug: bool) -> UiResult<WebView> {
        let hwnd = parent.window;
        let environment = {
            let (tx, rx) = mpsc::channel();

            CreateCoreWebView2EnvironmentCompletedHandler::wait_for_async_operation(
                Box::new(|environmentcreatedhandler| unsafe {
                    CreateCoreWebView2Environment(&environmentcreatedhandler)
                        .map_err(webview2_com::Error::WindowsError)
                }),
                Box::new(move |error_code, environment| {
                    error_code?;
                    tx.send(environment.ok_or_else(|| windows::core::Error::from(E_POINTER)))
                        .expect("send over mpsc channel");
                    Ok(())
                }),
            )?;

            rx.recv().unwrap()
        }?;

        let controller = {
            let (tx, rx) = mpsc::channel();

            CreateCoreWebView2ControllerCompletedHandler::wait_for_async_operation(
                Box::new(move |handler| unsafe {
                    environment
                        .CreateCoreWebView2Controller(hwnd, &handler)
                        .map_err(webview2_com::Error::WindowsError)
                }),
                Box::new(move |error_code, controller| {
                    error_code?;
                    tx.send(controller.ok_or_else(|| windows::core::Error::from(E_POINTER)))
                        .expect("send over mpsc channel");
                    Ok(())
                }),
            )?;

            rx.recv().unwrap()
        }?;

        let mut client_rect = RECT::default();
        unsafe {
            let _ = WindowsAndMessaging::GetClientRect(hwnd, &mut client_rect);
            controller.SetBounds(RECT {
                left: 0,
                top: 0,
                right: parent.size.width as i32,
                bottom: parent.size.height as i32,
            })?;
            controller.SetIsVisible(true)?;
        }

        let webview = unsafe { controller.CoreWebView2()? };

        if !debug {
            unsafe {
                let settings = webview.Settings()?;
                settings.SetAreDefaultContextMenusEnabled(false)?;
                settings.SetAreDevToolsEnabled(false)?;
            }
        }


        let thread_id = unsafe { Threading::GetCurrentThreadId() };

        let webview = WebView {
            controller,
            webview,
            thread_id,
            frame: Arc::new(parent),
            url: String::new(),
        };

        // webview.init(r#"window.external = { invoke: s => window.chrome.webview.postMessage(s) };"#)?;
        unsafe {
            let mut _token = 0;
            webview.webview.add_WebMessageReceived(
                &WebMessageReceivedEventHandler::create(Box::new(move |_webview, args| {
                    if let Some(args) = args {
                        let mut message = PWSTR(ptr::null_mut());
                        if args.WebMessageAsJson(&mut message).is_ok() {
                            let message = CoTaskMemPWSTR::from(message).to_string();
                            println!("{}", message);
                        }
                    }
                    Ok(())
                })),
                &mut _token,
            )?;
        }

        Ok(webview)
    }

    pub fn run(self) -> UiResult<()> {
        let webview = &self.webview;
        let url = self.url.as_str();
        let (tx, rx) = mpsc::channel();

        if !url.is_empty() {
            let handler =
                NavigationCompletedEventHandler::create(Box::new(move |_sender, _args| {
                    tx.send(()).expect("send over mpsc channel");
                    Ok(())
                }));
            let mut token = 0;
            unsafe {
                webview.add_NavigationCompleted(&handler, &mut token)?;
                let url = CoTaskMemPWSTR::from(url);
                webview.Navigate(*url.as_ref().as_pcwstr())?;
                let result = webview2_com::wait_with_pump(rx);
                webview.remove_NavigationCompleted(token)?;
                result?;
            }
        }

        let mut msg = MSG::default();

        loop {
            unsafe {
                let result = WindowsAndMessaging::GetMessageW(&mut msg, None, 0, 0).0;

                match result {
                    -1 => break Err(windows::core::Error::from_win32().into()),
                    0 => break Ok(()),
                    _ => match msg.message {
                        WindowsAndMessaging::WM_APP => (),
                        _ => {
                            let _ = WindowsAndMessaging::TranslateMessage(&msg);
                            WindowsAndMessaging::DispatchMessageW(&msg);
                        }
                    },
                }
            }
        }
    }

    pub fn navigate(&mut self, url: &str) -> UiResult<&Self> {
        self.url = url.to_string();
        Ok(self)
    }

    pub fn init(&self, js: &str) -> UiResult<&Self> {
        let webview = self.webview.clone();
        let js = String::from(js);
        AddScriptToExecuteOnDocumentCreatedCompletedHandler::wait_for_async_operation(
            Box::new(move |handler| unsafe {
                let js = CoTaskMemPWSTR::from(js.as_str());
                webview
                    .AddScriptToExecuteOnDocumentCreated(*js.as_ref().as_pcwstr(), &handler)
                    .map_err(webview2_com::Error::WindowsError)
            }),
            Box::new(|error_code, _id| error_code),
        )?;
        Ok(self)
    }

    pub fn eval(&self, js: &str) -> UiResult<&Self> {
        let webview = self.webview.clone();
        let js = String::from(js);
        ExecuteScriptCompletedHandler::wait_for_async_operation(
            Box::new(move |handler| unsafe {
                let js = CoTaskMemPWSTR::from(js.as_str());
                webview.ExecuteScript(*js.as_ref().as_pcwstr(), &handler).map_err(webview2_com::Error::WindowsError)
            }),
            Box::new(|error_code, _result| error_code),
        )?;
        Ok(self)
    }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    match msg {
        WindowsAndMessaging::WM_SIZE => {
            LRESULT::default()
        }

        WindowsAndMessaging::WM_CLOSE => {
            unsafe {
                let _ = WindowsAndMessaging::DestroyWindow(hwnd);
            }
            LRESULT::default()
        }

        WindowsAndMessaging::WM_DESTROY => {
            LRESULT::default()
        }

        _ => unsafe { WindowsAndMessaging::DefWindowProcW(hwnd, msg, w_param, l_param) },
    }
}
