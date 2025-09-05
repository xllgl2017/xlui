use crate::frame::App;
use crate::map::Map;
use crate::window::event::WindowEvent;
use crate::window::ime::IME;
use crate::window::wino::{EventLoopHandle, LoopWindow};
use crate::window::x11::ime::flag::Capabilities;
use crate::window::x11::X11Window;
use crate::window::{WindowId, WindowType};
use crate::WindowAttribute;
use std::process::exit;
use std::sync::Arc;

pub struct Application {
    native_window: X11Window,
    loop_windows: Map<WindowId, LoopWindow>,
}

impl Application {
    pub fn new<A: App>(app: A) -> Self {
        let ime = Arc::new(IME::new_x11("xlui ime").enable());
        ime.set_capabilities(Capabilities::PreeditText | Capabilities::Focus);
        let ii = ime.clone();
        ime.create_binding(ii);
        let attr = app.window_attributes();
        let native_window = X11Window::new(&attr, ime.clone()).unwrap();
        let window_type = native_window.last_window();
        let wid = window_type.id;
        let app = Box::new(app);
        let loop_window = pollster::block_on(async { LoopWindow::create_window(app, window_type, &attr).await });
        let mut loop_windows = Map::new();
        loop_windows.insert(wid, loop_window);
        Application {
            native_window,
            loop_windows,
        }
    }

    pub fn run(mut self) {
        loop {
            let (wid, event) = self.native_window.run();
            if let WindowEvent::ReqClose = event {
                let window = self.loop_windows.remove(&wid);
                if let Some(window) = window { if window.app_ctx.context.window.type_ == WindowType::ROOT { exit(0); } }
                continue;
            }
            if let Some(window) = self.loop_windows.get_mut(&wid) {
                if let WindowEvent::CreateChild = event {
                    let window_type = self.native_window.create_child_window(&window.app_ctx.context.window, &WindowAttribute::default());
                    let (app, attr) = window.app_ctx.context.new_window.take().unwrap();
                    let wid = window_type.id();
                    let loop_window = pollster::block_on(async { LoopWindow::create_window(app, window_type, &attr).await });
                    self.loop_windows.insert(wid, loop_window);
                    continue;
                }
                window.event(event);
            }
        }
    }
}

