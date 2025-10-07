use crate::error::UiResult;
use crate::ui::Ui;
#[cfg(not(feature = "winit"))]
use crate::window::win32::Win32Window;
#[cfg(feature = "winit")]
use crate::window::winit_app::WInitApplication;
#[cfg(not(feature = "winit"))]
use crate::window::wino::EventLoopHandle;
use crate::WindowAttribute;
use std::any::Any;
use std::thread::{sleep, spawn};
use std::time::Duration;
use windows::Win32::Graphics::Gdi::InvalidateRect;
#[cfg(not(feature = "winit"))]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GWLP_USERDATA};
#[cfg(feature = "winit")]
use winit::event_loop::{ControlFlow, EventLoop};

pub mod context;


pub trait App: Any + 'static {
    fn draw(&mut self, ui: &mut Ui);
    fn update(&mut self, _: &mut Ui) {}

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute::default()
    }

    fn run(self) -> UiResult<()>
    where
        Self: Sized,
    {
        //wasm-pack build --target web
        #[cfg(feature = "winit")]
        return start_winit_app(self);
        #[cfg(all(windows, not(feature = "winit")))]
        return start_win32_app(self);
    }
}


#[cfg(feature = "winit")]
fn start_winit_app<A: App>(app: A) -> UiResult<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let proxy_event = event_loop.create_proxy();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut application = WInitApplication::new();
    application.set_app(Some(app));
    application.set_proxy_event(Some(proxy_event));
    event_loop.run_app(&mut application)?;
    Ok(())
}

#[cfg(all(windows, not(feature = "winit")))]
fn start_win32_app<A: App>(app: A) -> UiResult<()> {
    let mut win32 = Win32Window::new(app)?;
    let window = win32.get_window_by_index(0);
    let handle = window.handle().clone();
    unsafe { SetWindowLongPtrW(window.handle().win32().hwnd, GWLP_USERDATA, &mut win32 as *mut _ as isize); }
    handle.request_redraw();
    win32.run()?;
    Ok(())
}