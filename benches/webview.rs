#![windows_subsystem = "windows"]

use std::sync::Arc;
use std::{ptr, sync::mpsc};
use webview2_com::{CoTaskMemPWSTR, CreateCoreWebView2ControllerCompletedHandler, CreateCoreWebView2EnvironmentCompletedHandler, NavigationCompletedEventHandler, WebMessageReceivedEventHandler};
use webview2_com::Microsoft::Web::WebView2::Win32::{CreateCoreWebView2Environment, ICoreWebView2, ICoreWebView2Controller, ICoreWebView2Environment, ICoreWebView2WebMessageReceivedEventHandler};
use windows::core::{w, PWSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::System::Com::{CoInitializeEx, COINIT_APARTMENTTHREADED};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging;
use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWLP_USERDATA, MSG, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE};
use xlui::{Size, UiError, UiResult};

fn main() -> UiResult<()> {
    // unsafe {
    //     CoInitializeEx(None, COINIT_APARTMENTTHREADED).ok()?;
    // }
    let frame = FrameWindow::new();
    let mut webview = WebView::create(frame, true)?;
    webview.url = "https://www.baidu.com/".to_string();
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
                let hinstance = GetModuleHandleW(None).unwrap();
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
                    Some(HINSTANCE::from(hinstance)),
                    None,
                )
            }
        }.unwrap();

        FrameWindow {
            window: hwnd,
            size: Size { width: 800.0, height: 600.0 },
        }
    }
}

pub struct WebView {
    controller: ICoreWebView2Controller,
    webview: ICoreWebView2,
    frame: Arc<FrameWindow>,
    url: String,
}

impl Drop for WebView {
    fn drop(&mut self) {
        unsafe { self.controller.Close().unwrap() }
    }
}

impl WebView {
    fn create_environment() -> UiResult<ICoreWebView2Environment> {
        let (tx, rx) = mpsc::channel();
        CreateCoreWebView2EnvironmentCompletedHandler::wait_for_async_operation(
            Box::new(|handler| unsafe {
                CreateCoreWebView2Environment(&handler).map_err(webview2_com::Error::WindowsError)
            }),
            Box::new(move |error_code, environment| {
                error_code?;
                tx.send(environment).or(Err(UiError::SendErr))?;
                Ok(())
            }),
        )?;
        rx.recv()?.ok_or(UiError::NullPtr)
    }

    fn create_controller(hwnd: HWND, environment: ICoreWebView2Environment) -> UiResult<ICoreWebView2Controller> {
        let (tx, rx) = mpsc::channel();

        CreateCoreWebView2ControllerCompletedHandler::wait_for_async_operation(
            Box::new(move |handler| unsafe {
                environment.CreateCoreWebView2Controller(hwnd, &handler).map_err(webview2_com::Error::WindowsError)
            }),
            Box::new(move |error_code, controller| {
                error_code?;
                tx.send(controller).or(Err(UiError::SendErr))?;
                Ok(())
            }),
        )?;

        rx.recv()?.ok_or(UiError::NullPtr)
    }

    fn create_message_receiver() -> UiResult<ICoreWebView2WebMessageReceivedEventHandler> {
        Ok(WebMessageReceivedEventHandler::create(Box::new(move |_webview, args| {
            let args = args.ok_or(UiError::NullPtr)?;
            let mut message = PWSTR(ptr::null_mut());
            unsafe { args.WebMessageAsJson(&mut message)?; }
            let message_str = CoTaskMemPWSTR::from(message).to_string();
            println!("message: {}", message_str);
            Ok(())
        })))
    }

    pub fn create(parent: FrameWindow, debug: bool) -> UiResult<WebView> {
        let environment = WebView::create_environment()?;
        let controller = WebView::create_controller(parent.window, environment)?;
        unsafe {
            controller.SetBounds(RECT {
                left: 0,
                top: 0,
                right: parent.size.width as i32,
                bottom: parent.size.height as i32,
            })?;
            controller.SetIsVisible(true)?;
            let webview = controller.CoreWebView2()?;
            if !debug {
                let settings = webview.Settings()?;
                settings.SetAreDefaultContextMenusEnabled(false)?;
                settings.SetAreDevToolsEnabled(false)?;
            }
            webview.add_WebMessageReceived(&WebView::create_message_receiver()?, &mut 0)?;
            Ok(WebView {
                controller,
                webview,
                frame: Arc::new(parent),
                url: String::new(),
            })
        }
    }

    pub fn run(mut self) -> UiResult<()> {
        let webview = &self.webview;
        if !self.url.is_empty() {
            let handler = NavigationCompletedEventHandler::create(Box::new(move |_sender, _args| { Ok(()) }));
            let mut token = 0;
            unsafe {
                webview.add_NavigationCompleted(&handler, &mut token)?;
                let url = CoTaskMemPWSTR::from(self.url.as_str());
                webview.Navigate(*url.as_ref().as_pcwstr())?;
                webview.remove_NavigationCompleted(token)?;
            }
        }
        unsafe { SetWindowLongPtrW(self.frame.window, GWLP_USERDATA, &mut self as *mut _ as isize); }

        let mut msg = MSG::default();

        loop {
            unsafe {
                while WindowsAndMessaging::GetMessageW(&mut msg, None, 0, 0).into() {
                    let _ = WindowsAndMessaging::TranslateMessage(&msg);
                    WindowsAndMessaging::DispatchMessageW(&msg);
                }
            }
        }
    }


    // pub fn init(&self, js: &str) -> UiResult<&Self> {
    //     let webview = self.webview.clone();
    //     let js = String::from(js);
    //     AddScriptToExecuteOnDocumentCreatedCompletedHandler::wait_for_async_operation(
    //         Box::new(move |handler| unsafe {
    //             let js = CoTaskMemPWSTR::from(js.as_str());
    //             webview
    //                 .AddScriptToExecuteOnDocumentCreated(*js.as_ref().as_pcwstr(), &handler)
    //                 .map_err(webview2_com::Error::WindowsError)
    //         }),
    //         Box::new(|error_code, _id| error_code),
    //     )?;
    //     Ok(self)
    // }

    // pub fn eval(&self, js: &str) -> UiResult<&Self> {
    //     let webview = self.webview.clone();
    //     let js = String::from(js);
    //     ExecuteScriptCompletedHandler::wait_for_async_operation(
    //         Box::new(move |handler| unsafe {
    //             let js = CoTaskMemPWSTR::from(js.as_str());
    //             webview.ExecuteScript(*js.as_ref().as_pcwstr(), &handler).map_err(webview2_com::Error::WindowsError)
    //         }),
    //         Box::new(|error_code, _result| error_code),
    //     )?;
    //     Ok(self)
    // }
}

extern "system" fn window_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WindowsAndMessaging::WM_SIZE => {
            let window = unsafe { (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut WebView).as_mut() };
            if let Some(window) = window {
                let width = (lparam.0 as u32 & 0xffff) as f32;
                let height = ((lparam.0 as u32 >> 16) & 0xffff) as f32;
                println!("resize: width: {}; height: {}", width, height);
                unsafe {
                    window.controller.SetBounds(RECT {
                        left: 0,
                        top: 0,
                        right: width as i32,
                        bottom: height as i32,
                    }).unwrap();
                }
            }
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

        _ => unsafe { WindowsAndMessaging::DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}
