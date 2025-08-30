use std::error::Error;
use std::process::exit;
use std::sync::Arc;
use std::sync::mpsc::SyncSender;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use glyphon::{Cache, Resolution, Viewport};
use crate::window::event::WindowEvent;
use crate::window::x11::X11Window;
use crate::{Device, DeviceInput, Pos, Size};
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};
use x11::xlib;
use crate::frame::App;
use crate::frame::context::{Context, Render, UpdateType};
use crate::map::Map;
use crate::ui::AppContext;
use crate::window::{Window, WindowId};
use crate::window::x11_app::X11Application;

pub enum WindowKind {
    #[cfg(feature = "winit")]
    WInit(winit::window::Window),
    Xlib(X11Window),
}

impl WindowKind {
    pub fn x11(&self) -> &X11Window {
        match self {
            #[cfg(feature = "winit")]
            WindowKind::WInit(_) => panic!("only not winit"),
            WindowKind::Xlib(v) => v
        }
    }

    pub fn size(&self) -> Size {
        match self {
            #[cfg(feature = "winit")]
            WindowKind::WInit(v) => {
                let inner_size = v.inner_size();
                Size {
                    width: inner_size.width,
                    height: inner_size.height,
                }
            }
            WindowKind::Xlib(v) => v.size()
        }
    }
    pub fn request_redraw(&self) {
        match self {
            #[cfg(feature = "winit")]
            WindowKind::WInit(v) => v.request_redraw(),
            WindowKind::Xlib(v) => v.request_redraw()
        }
    }

    pub fn id(&self) -> WindowId {
        match self {
            #[cfg(feature = "winit")]
            WindowKind::WInit(v) => WindowId::from_winit_id(v.id()),
            WindowKind::Xlib(v) => v.id(),
        }
    }
}

impl HasWindowHandle for WindowKind {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self {
            #[cfg(feature = "winit")]
            WindowKind::WInit(v) => v.window_handle(),
            WindowKind::Xlib(v) => Ok(v.window_handle())
        }
    }
}

impl HasDisplayHandle for WindowKind {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        match self {
            #[cfg(feature = "winit")]
            WindowKind::WInit(v) => v.display_handle(),
            WindowKind::Xlib(v) => Ok(v.display_handle())
        }
    }
}

pub trait EventLoopHandle {
    fn event(&mut self, id: WindowId, event: WindowEvent);
}

pub struct LoopWindow {
    pub(crate) app_ctx: AppContext,
    pub(crate) app: Box<dyn App>,
}

impl LoopWindow {
    #[cfg(not(feature = "winit"))]
    pub async fn create_window<A: App>(mut app: A, sender: SyncSender<(WindowId, WindowEvent)>) -> LoopWindow {
        let attr = app.window_attributes();
        let x11_window = X11Window::new(attr.inner_size, &attr.title, sender.clone()).unwrap();
        let platform_window = Arc::new(WindowKind::Xlib(x11_window));
        let device = Self::rebuild_device(&platform_window, sender.clone()).await.unwrap();
        device.surface.configure(&device.device, &device.surface_config);
        let viewport = Viewport::new(&device.device, &device.cache);
        let context = Context {
            size: platform_window.size(),
            font: attr.font.clone(),
            viewport,
            window: platform_window.clone(),
            resize: false,
            render: Render::new(&device),
            updates: Map::new(),
            event: sender,
        };
        let mut app_ctx = AppContext::new(device, context);
        let mut app: Box<dyn App> = Box::new(app);
        app_ctx.draw(&mut app);
        LoopWindow {
            app_ctx,
            app,
        }
    }

    pub fn run(&mut self) {
        let window = self.app_ctx.context.window.clone();
        self.event(self.app_ctx.context.window.id(), WindowEvent::Redraw);
        loop {
            let event = window.x11().run();
            self.event(self.app_ctx.context.window.id(), event);
        }
    }

    pub(crate) async fn rebuild_device(window: &Arc<WindowKind>, sender: SyncSender<(WindowId, WindowEvent)>) -> Result<Device, Box<dyn Error>> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await?;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await?;
        let cache = Cache::new(&device);
        let surface = instance.create_surface(window.clone())?;
        let cap = surface.get_capabilities(&adapter);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: cap.formats[0],
            view_formats: vec![cap.formats[0].add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: window.size().width,
            height: window.size().height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        let wid = window.id();

        device.on_uncaptured_error(Box::new(move |err| {
            sender.send((wid, WindowEvent::Reinit)).unwrap();
            println!("Error: {:#?}", err);
            println!("{}", err.to_string());
        }));
        Ok(Device {
            device,
            queue,
            cache,
            surface,
            texture_format: cap.formats[0],
            surface_config,
            device_input: DeviceInput::new(),
        })
    }
}

impl EventLoopHandle for LoopWindow {
    fn event(&mut self, id: WindowId, event: WindowEvent) {
        if self.app_ctx.context.window.id() != id { return; }
        println!("{:?}", event);
        match event {
            WindowEvent::None => {}
            WindowEvent::KeyPress => {}
            WindowEvent::KeyRelease => {}
            WindowEvent::MouseMove(pos) => {
                self.app_ctx.device.device_input.mouse.update(pos);
                self.app_ctx.update(UpdateType::MouseMove, &mut self.app);
            }
            WindowEvent::MouseWheel => {}
            WindowEvent::MousePress(_) => {
                self.app_ctx.device.device_input.mouse.mouse_press();
                self.app_ctx.update(UpdateType::MousePress, &mut self.app);
            }
            WindowEvent::MouseRelease(_) => {
                self.app_ctx.device.device_input.mouse.mouse_release();
                self.app_ctx.update(UpdateType::MouseRelease, &mut self.app);
                self.app_ctx.device.device_input.mouse.a = 0.0;
            }
            WindowEvent::Redraw => {
                self.app_ctx.context.viewport.update(&self.app_ctx.device.queue, Resolution {
                    width: self.app_ctx.device.surface_config.width,
                    height: self.app_ctx.device.surface_config.height,
                });
                self.app_ctx.redraw(&mut self.app)
            }
            WindowEvent::Reinit => {}
            WindowEvent::Resize(size) => {
                self.app_ctx.context.size = size;
                self.app_ctx.device.surface_config.width = size.width;
                self.app_ctx.device.surface_config.height = size.height;
                let device = &self.app_ctx.device.device;
                let config = &self.app_ctx.device.surface_config;
                println!("3333333333333333");
                // window.app_ctx.device.device.poll(wgpu::PollType::Wait).unwrap();
                println!("3434564545454545");
                self.app_ctx.device.surface.configure(device, config);
                println!("444444444444444");
                // // window.resize(size);
                // window.render();
                // window.app_ctx.redraw(&mut window.app)
                // window.app_ctx.context.window.request_redraw();
            }
            WindowEvent::ReqClose => self.app_ctx.context.event.send((self.app_ctx.context.window.id(), WindowEvent::ReqClose)).unwrap(),
            WindowEvent::Update(_) => {}
        }
    }
}