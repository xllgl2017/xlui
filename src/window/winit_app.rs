use crate::frame::context::{Render, UpdateType};
use crate::frame::App;
use crate::{Pos, Size};
use crate::window::wino::{LoopWindow, WindowKind};
use crate::window::{Window, WindowId};
use glyphon::Viewport;
use std::collections::HashMap;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::window::ImePurpose;

pub struct WInitApplication<A> {
    windows: HashMap<WindowId, Window>,
    app: Option<A>,
    proxy_event: Option<EventLoopProxy<(WindowId, UpdateType)>>,
    rebuilding: bool,
}

impl<A> WInitApplication<A> {
    pub fn new() -> Self {
        WInitApplication {
            windows: HashMap::new(),
            app: None,
            proxy_event: None,
            rebuilding: false,
        }
    }

    pub fn set_app(&mut self, app: Option<A>) {
        self.app = app;
    }

    pub fn set_proxy_event(&mut self, proxy_event: Option<EventLoopProxy<(WindowId, UpdateType)>>) {
        self.proxy_event = proxy_event;
    }
}

impl<A: App + 'static> ApplicationHandler<(WindowId, UpdateType)> for WInitApplication<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("111111111111111111111111111111");
        let app = self.app.take().unwrap();
        let event = self.proxy_event.take().unwrap();
        let attr = app.window_attributes();
        let winit_window = event_loop.create_window(attr.as_winit_attributes()).unwrap();
        println!("{}", format!("winit id: {:?}", winit_window.id()));
        winit_window.set_ime_allowed(true);
        winit_window.set_ime_cursor_area(PhysicalPosition::new(400, 300), PhysicalSize::new(100, 100));
        winit_window.set_ime_purpose(ImePurpose::Normal);
        let loop_window = Arc::new(WindowKind::WInit(winit_window));
        let window = pollster::block_on(Window::new_winit(loop_window.clone(), Box::new(app), attr, event)).unwrap();
        self.windows.insert(loop_window.id(), window);
        loop_window.request_redraw();
    }

    fn user_event(&mut self, _: &ActiveEventLoop, (wid, t): (WindowId, UpdateType)) {
        if self.rebuilding {
            println!("Rebuilding");
            return;
        }
        if let UpdateType::ReInit = t {
            self.rebuilding = true;
            println!("sleep start");
            println!("sleep end");
            let window = self.windows.get_mut(&wid).unwrap();
            println!("recv {:?}", window.app_ctx.context.window.size());
            let event = window.app_ctx.context.event.clone();
            let wid = window.app_ctx.context.window.id();
            window.app_ctx.device = pollster::block_on(async {
                Window::rebuild_device(&window.app_ctx.context.window, |device| {
                    device.on_uncaptured_error(Box::new(move |err| {
                        println!("Error: {:#?}", err);
                        event.send_event((wid, UpdateType::ReInit));
                    }))
                }).await
            }).unwrap();
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

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: winit::window::WindowId, event: WindowEvent) {
        let wid = WindowId::from_winit_id(id);
        let window = self.windows.get_mut(&wid);
        if window.is_none() { return; }
        let window = window.unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                // let pos = self.windows.iter().position(|x| x.get_window().id() == id).unwrap();
                self.windows.remove(&wid);
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
                window.resize(Size { width: size.width, height: size.height });
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
                window.app_ctx.device.device_input.mouse.update(Pos { x: position.x as f32, y: position.y as f32 });
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

pub struct WInitWindow {
    event: EventLoopProxy<(WindowId, UpdateType)>,
    window: winit::window::Window,
}


impl WInitWindow {}