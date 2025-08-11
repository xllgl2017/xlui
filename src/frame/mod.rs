use crate::frame::window::Window;
use crate::size::Size;
use crate::ui::Ui;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event::{ElementState, MouseButton, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Icon, WindowId, WindowLevel};
use crate::size::pos::Pos;

mod window;
pub mod context;

pub struct WindowAttribute {
    pub inner_size: Size,
    pub min_inner_size: Size,
    pub max_inner_size: Size,
    pub position: [i32; 2],
    pub resizable: bool,
    pub title: String,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub blur: bool,
    pub decorations: bool,
    pub window_icon: Arc<Vec<u8>>,
    pub window_level: WindowLevel,
}


impl WindowAttribute {
    fn as_winit_attributes(&self) -> winit::window::WindowAttributes {
        let attr = winit::window::WindowAttributes::default();
        let img = image::load_from_memory(self.window_icon.as_ref()).unwrap();
        let rgb8 = img.to_rgba8();
        let icon = Icon::from_rgba(rgb8.to_vec(), img.width(), img.height()).unwrap();
        attr.with_inner_size(self.inner_size.as_physical_size())
            .with_min_inner_size(self.min_inner_size.as_physical_size())
            .with_max_inner_size(self.max_inner_size.as_physical_size())
            .with_position(winit::dpi::Position::Physical(winit::dpi::PhysicalPosition::new(self.position[0], self.position[1])))
            .with_resizable(self.resizable)
            .with_title(self.title.as_str())
            .with_maximized(self.maximized)
            .with_visible(self.visible)
            .with_transparent(self.transparent)
            .with_blur(self.blur)
            .with_decorations(self.decorations)
            .with_window_icon(Some(icon))
            .with_window_level(self.window_level)
    }
}

impl Default for WindowAttribute {
    fn default() -> WindowAttribute {
        WindowAttribute {
            inner_size: Size { width: 800, height: 600 },
            min_inner_size: Size { width: 0, height: 0 },
            max_inner_size: Size { width: 2560, height: 1440 },
            position: [100, 100],
            resizable: true,
            title: "xlui".to_string(),
            maximized: false,
            visible: true, //是否隐藏窗口
            transparent: false, //窗口透明，配合LoadOp::Clear
            blur: true, //未知
            decorations: true, //标题栏
            window_icon: Arc::new(include_bytes!("../../logo.jpg").to_vec()),
            window_level: WindowLevel::Normal,
        }
    }
}


struct Application<A> {
    windows: Vec<Window<A>>,
    attribute: WindowAttribute,
    app: Option<A>,
}

impl<A> Application<A> {
    fn new() -> Self {
        Application {
            windows: vec![],
            attribute: WindowAttribute::default(),
            app: None,
        }
    }

    fn with_attrs(mut self, attrs: WindowAttribute) -> Self {
        self.attribute = attrs;
        self
    }
}

impl<A: App + 'static> ApplicationHandler for Application<A> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("111111111111111111111111111111");
        let app = self.app.take().unwrap();
        let attr = self.attribute.as_winit_attributes();
        let winit_window = Arc::new(event_loop.create_window(attr).unwrap());
        let window = pollster::block_on(Window::new(winit_window.clone(), app));
        self.windows.push(window);
        winit_window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        let window = self.windows.iter_mut().find(|x| x.get_window().id() == id);
        if window.is_none() { return; }
        let window = window.unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                let pos = self.windows.iter().position(|x| x.get_window().id() == id).unwrap();
                self.windows.remove(pos);
                if self.windows.len() == 0 { event_loop.exit(); }
            }
            WindowEvent::RedrawRequested => {
                println!("11");
                window.render();
                window.app_ctx.context.resize = false;
            }
            WindowEvent::Resized(size) => {
                window.resize(size);
            }
            WindowEvent::MouseInput { state, button, .. } => {
                // println!("{:?}", state);
                match (state, button) {
                    (ElementState::Pressed, MouseButton::Left) => {
                        window.app_ctx.device.device_input.mouse.previous = window.app_ctx.device.device_input.mouse.lastest.clone();
                        window.app_ctx.device.device_input.mouse.pressed_pos = window.app_ctx.device.device_input.mouse.lastest.clone();
                        window.app_ctx.device.device_input.mouse.pressed = true;
                        window.app_ctx.update(&mut window.app)
                    }
                    (ElementState::Released, MouseButton::Left) => {
                        window.app_ctx.device.device_input.mouse.clicked = true;
                        window.app_ctx.update(&mut window.app);
                        window.app_ctx.device.device_input.mouse.clicked = false;
                        window.app_ctx.device.device_input.mouse.pressed_pos = Pos::new();
                        window.app_ctx.device.device_input.mouse.pressed = false;
                    }
                    (_, _) => {}
                }
            }
            WindowEvent::MouseWheel { delta, .. } => {
                match delta {
                    MouseScrollDelta::LineDelta(x, y) => {
                        window.app_ctx.device.device_input.mouse.delta = (x, y);
                        window.app_ctx.update(&mut window.app);
                        window.app_ctx.device.device_input.mouse.delta = (0.0, 0.0);
                    }
                    _ => {}
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                window.app_ctx.device.device_input.mouse.update(position);
                window.app_ctx.update(&mut window.app);
            }
            WindowEvent::KeyboardInput { device_id: _device_id, event, .. } => {
                if !event.state.is_pressed() { return; }
                window.app_ctx.key_input(event.logical_key, &mut window.app);
                window.app_ctx.context.window.request_redraw();
            }
            _ => (),
        }
    }
}


pub trait App: Sized + 'static {
    fn draw(&mut self, ui: &mut Ui);
    fn update(&mut self, ui: &mut Ui);
    fn redraw(&mut self, ui: &mut Ui);

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute::default()
    }

    fn run(self) {
        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Wait);
        let mut application = Application::new().with_attrs(self.window_attributes());
        application.app = Some(self);
        event_loop.run_app(&mut application).unwrap()
    }
}