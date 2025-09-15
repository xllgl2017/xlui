use crate::error::UiResult;
use crate::frame::App;
use crate::map::Map;
use crate::window::event::WindowEvent;
use crate::window::ime::IME;
#[cfg(target_os = "windows")]
use crate::window::win32::Win32Window;
use crate::window::wino::{EventLoopHandle, LoopWindow};
#[cfg(target_os = "linux")]
use crate::window::x11::ime::flag::Capabilities;
#[cfg(target_os = "linux")]
use crate::window::x11::X11Window;
use crate::window::{WindowId, WindowType};
use std::process::exit;
use std::sync::Arc;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GWLP_USERDATA};

pub struct Application {
    #[cfg(target_os = "linux")]
    native_window: X11Window,
    #[cfg(target_os = "windows")]
    native_window: Win32Window,
    loop_windows: Map<WindowId, LoopWindow>,
}

impl Application {
    pub fn new<A: App>(app: A) -> UiResult<Self> {
        #[cfg(target_os = "linux")]
        let ime = Arc::new(IME::new_x11("xlui ime"));
        #[cfg(target_os = "linux")]
        ime.set_capabilities(Capabilities::PreeditText | Capabilities::Focus);
        #[cfg(target_os = "linux")]
        let ii = ime.clone();
        #[cfg(target_os = "linux")]
        ime.create_binding(ii);
        #[cfg(target_os = "windows")]
        let ime = Arc::new(IME::new_win32());
        let mut attr = app.window_attributes();
        #[cfg(target_os = "linux")]
        let native_window = X11Window::new(&mut attr, ime.clone())?;
        #[cfg(target_os = "windows")]
        let native_window = Win32Window::new(&mut attr, ime).unwrap();
        let window_type = native_window.last_window();
        let wid = window_type.id;
        let app = Box::new(app);
        let mut loop_window = pollster::block_on(async { LoopWindow::create_window(app, window_type, attr).await });
        loop_window.event(WindowEvent::Redraw);
        let mut loop_windows = Map::new();
        loop_windows.insert(wid, loop_window);
        Ok(Application {
            native_window,
            loop_windows,
        })
    }

    pub fn run(mut self) -> UiResult<()> {
        #[cfg(target_os = "windows")]
        unsafe { SetWindowLongPtrW(self.native_window.last_window().win32().hwnd, GWLP_USERDATA, &self as *const _ as isize); }
        loop {
            let (wid, event) = self.native_window.run();
            if let WindowEvent::ReqClose = event {
                let window = self.loop_windows.remove(&wid);
                if let Some(window) = window { if window.app_ctx.context.window.type_ == WindowType::ROOT { exit(0); } }
                continue;
            }
            if let Some(window) = self.loop_windows.get_mut(&wid) {
                if let WindowEvent::CreateChild = event {
                    let (app, attr) = window.app_ctx.context.new_window.take().unwrap();
                    let window_type = self.native_window.create_child_window(&window.app_ctx.context.window, &attr)?;
                    let wid = window_type.id();
                    let loop_window = pollster::block_on(async { LoopWindow::create_window(app, window_type, attr).await });
                    self.loop_windows.insert(wid, loop_window);
                    continue;
                }
                window.event(event);
            }
        }
    }
}

