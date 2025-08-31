use std::any::Any;
#[cfg(feature = "winit")]
use winit::event_loop::{ControlFlow, EventLoop};
#[cfg(feature = "winit")]
use crate::window::winit_app::WInitApplication;
use crate::ui::Ui;
use crate::window::application::Application;
use crate::WindowAttribute;

pub mod context;


pub trait App: Any + 'static {
    fn draw(&mut self, ui: &mut Ui);
    fn update(&mut self, _: &mut Ui) {}
    fn redraw(&mut self, _: &mut Ui) {}

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute::default()
    }

    fn run(self)
    where
        Self: Sized,
    {
        //wasm-pack build --target web
        #[cfg(feature = "winit")]
        {
            let event_loop = EventLoop::with_user_event().build().unwrap();
            let proxy_event = event_loop.create_proxy();
            event_loop.set_control_flow(ControlFlow::Wait);
            let mut application = WInitApplication::new();
            application.set_app(Some(self));
            application.set_proxy_event(Some(proxy_event));
            event_loop.run_app(&mut application).unwrap()
        }
        #[cfg(not(feature = "winit"))]
        {
            let mut app = Application::new();
            app.create_window(self);
            app.run();
        }
    }
}