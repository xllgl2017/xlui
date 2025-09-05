use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};
use crate::error::UiResult;
use crate::window::{UserEvent, WindowId};
use winit::dpi::PhysicalSize;

pub struct WInitWindowHandle {
    window: winit::window::Window,
    user_event: winit::event_loop::EventLoopProxy<(WindowId, UserEvent)>,
}

impl WInitWindowHandle {
    pub fn new(window: winit::window::Window, event: winit::event_loop::EventLoopProxy<(WindowId, UserEvent)>) -> Self {
        WInitWindowHandle {
            window,
            user_event: event,
        }
    }

    pub fn send_user_event(&self, wid: WindowId, event: UserEvent) -> UiResult<()> {
        self.user_event.send_event((wid, event))?;
        Ok(())
    }

    pub fn inner_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }

    pub fn set_ime_position(&self, x: f32, y: f32) {
        let pos = winit::dpi::LogicalPosition::new(x, y);
        let size = winit::dpi::LogicalSize::new(100.0, 100.0);
        self.window.set_ime_cursor_area(pos, size);
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        self.window.window_handle()
    }

    pub fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        self.window.display_handle()
    }
}