use crate::frame::context::{Render, UpdateType};
use crate::ui::Ui;
use crate::window::attribute::WindowAttribute;
use crate::window::Window;
use glyphon::Viewport;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy};
use winit::window::{ImePurpose, WindowId};

pub mod context;

struct Application<A> {
    windows: HashMap<WindowId, Window>,
    app: Option<A>,
    proxy_event: Option<EventLoopProxy<(WindowId, UpdateType)>>,
    rebuilding: bool,
}

impl<A> Application<A> {
    fn new() -> Self {
        Application {
            windows: HashMap::new(),
            app: None,
            proxy_event: None,
            rebuilding: false,
        }
    }
}

impl<A: App + 'static> ApplicationHandler<(WindowId, UpdateType)> for Application<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("111111111111111111111111111111");
        let app = self.app.take().unwrap();
        let event = self.proxy_event.take().unwrap();
        let attr = app.window_attributes();
        let winit_window = Arc::new(event_loop.create_window(attr.as_winit_attributes()).unwrap());
        winit_window.set_ime_allowed(true);
        winit_window.set_ime_cursor_area(PhysicalPosition::new(400, 300), PhysicalSize::new(100, 100));
        winit_window.set_ime_purpose(ImePurpose::Normal);
        let window = pollster::block_on(Window::new(winit_window.clone(), Box::new(app), attr, event)).unwrap();
        self.windows.insert(winit_window.id(), window);
        winit_window.request_redraw();
    }

    fn user_event(&mut self, _: &ActiveEventLoop, (wid, t): (WindowId, UpdateType)) {
        if self.rebuilding {
            println!("Rebuilding");
            return;
        }
        if let UpdateType::ReInit = t {
            self.rebuilding = true;
            println!("sleep start");
            std::thread::sleep(std::time::Duration::from_secs(30));
            println!("sleep end");
            let window = self.windows.get_mut(&wid).unwrap();
            println!("recv {:?}", window.app_ctx.context.window.inner_size());
            window.app_ctx.device = pollster::block_on(async { Window::rebuild_device(&window.app_ctx.context.window, window.app_ctx.context.event.clone()).await }).unwrap();
            println!("1");
            window.app_ctx.context.viewport = Viewport::new(&window.app_ctx.device.device, &window.app_ctx.device.cache);
            println!("2");
            window.app_ctx.context.render = Render::new(&window.app_ctx.device);
            println!("3");
            window.configure_surface();
            println!("4");
            window.app_ctx.update(UpdateType::ReInit, &mut window.app);
            println!("5");
            self.rebuilding = false;
            println!("re init finished");
        } else {
            println!("recv event");
            let window = self.windows.get_mut(&wid).unwrap();
            window.app_ctx.update(t, &mut window.app)
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let window = self.windows.get_mut(&id);
        if window.is_none() { return; }
        let window = window.unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                // let pos = self.windows.iter().position(|x| x.get_window().id() == id).unwrap();
                self.windows.remove(&id);
                if self.windows.len() == 0 { event_loop.exit(); }
            }
            WindowEvent::RedrawRequested => {
                if self.rebuilding {
                    println!("rebuilding window");
                    return;
                }
                println!("11");
                window.render();
                window.app_ctx.context.resize = false;
            }
            WindowEvent::Resized(size) => {
                window.resize(size);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                match (state, button) {
                    (ElementState::Pressed, MouseButton::Left) => {
                        window.app_ctx.device.device_input.mouse.mouse_press();
                        window.app_ctx.update(UpdateType::MousePress, &mut window.app);
                    }
                    (ElementState::Released, MouseButton::Left) => {
                        window.app_ctx.device.device_input.mouse.mouse_release();
                        window.app_ctx.update(UpdateType::MouseRelease, &mut window.app);
                        window.app_ctx.device.device_input.mouse.a = 0.0;
                    }
                    (_, _) => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        window.app_ctx.device.device_input.mouse.delta = (x, y);
                        window.app_ctx.update(UpdateType::MouseWheel, &mut window.app);
                        window.app_ctx.device.device_input.mouse.delta = (0.0, 0.0);
                    }
                    _ => {}
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                window.app_ctx.device.device_input.mouse.update(position);
                window.app_ctx.update(UpdateType::MouseMove, &mut window.app);
            }
            WindowEvent::KeyboardInput { device_id: _device_id, event, .. } => {
                if !event.state.is_pressed() { return; }
                window.app_ctx.key_input(UpdateType::KeyRelease(Some(event.logical_key)), &mut window.app);
                window.app_ctx.context.window.request_redraw();
            }
            WindowEvent::Ime(ime) => {
                println!("{:?}", ime);
            }
            _ => (),
        }
    }
}


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
        let event_loop = EventLoop::with_user_event().build().unwrap();
        let proxy_event = event_loop.create_proxy();
        event_loop.set_control_flow(ControlFlow::Wait);
        let mut application = Application::new();
        application.app = Some(self);
        application.proxy_event = Some(proxy_event);
        event_loop.run_app(&mut application).unwrap()
    }
}