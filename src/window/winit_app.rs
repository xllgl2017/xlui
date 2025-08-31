use crate::frame::context::{Render, UpdateType};
use crate::frame::App;
use crate::{Pos, Size};
use glyphon::Viewport;
use std::collections::HashMap;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, EventLoopProxy};
use winit::keyboard::{Key, NamedKey};
use winit::window::{ImePurpose, WindowId};
use crate::window::WindowKind;
use crate::window::winit_window::Window;

pub struct WInitApplication<A> {
    windows: HashMap<super::WindowId, Window>,
    app: Option<A>,
    proxy_event: Option<EventLoopProxy<(super::WindowId, UpdateType)>>,
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

    pub fn set_proxy_event(&mut self, proxy_event: Option<EventLoopProxy<(super::WindowId, UpdateType)>>) {
        self.proxy_event = proxy_event;
    }
}

impl<A: App + 'static> ApplicationHandler<(super::WindowId, UpdateType)> for WInitApplication<A> {
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

    fn user_event(&mut self, _: &ActiveEventLoop, (wid, t): (super::WindowId, UpdateType)) {
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
                        let _ = event.send_event((wid, UpdateType::ReInit));
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

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let wid = super::WindowId::from_winit_id(id);
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
                let key = match event.logical_key {
                    Key::Named(name) => {
                        match name {
                            NamedKey::Enter => crate::key::Key::Enter,
                            NamedKey::Space => crate::key::Key::Space,
                            NamedKey::ArrowDown => crate::key::Key::DownArrow,
                            NamedKey::ArrowLeft => crate::key::Key::LeftArrow,
                            NamedKey::ArrowRight => crate::key::Key::RightArrow,
                            NamedKey::ArrowUp => crate::key::Key::UpArrow,
                            NamedKey::End => crate::key::Key::End,
                            NamedKey::Home => crate::key::Key::Home,
                            NamedKey::Backspace => crate::key::Key::Backspace,
                            NamedKey::Delete => crate::key::Key::Delete,
                            _ => return,
                        }
                    }
                    Key::Character(c) => crate::key::Key::Char(c.as_str().chars().next().unwrap()),
                    Key::Unidentified(_) => return,
                    Key::Dead(_) => return,
                };
                window.app_ctx.key_input(UpdateType::KeyRelease(Some(key)), &mut window.app);
                window.app_ctx.context.window.request_redraw();
            }
            WindowEvent::Ime(ime) => {
                println!("{:?}", ime);
            }
            _ => (),
        }
    }
}