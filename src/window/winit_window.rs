use std::error::Error;
use std::sync::Arc;
use glyphon::{Cache, Resolution, Viewport};
use winit::event_loop::EventLoopProxy;
use crate::frame::App;
use crate::frame::context::{Context, Render, UpdateType};
use crate::ui::AppContext;
use crate::window::{WindowId, WindowKind};
use crate::{Device, DeviceInput, Size, WindowAttribute};
use crate::map::Map;

pub(crate) struct Window {
    pub(crate) app_ctx: AppContext,
    pub(crate) app: Box<dyn App>,
}

impl Window {
    #[cfg(feature = "winit")]
    pub(crate) async fn new_winit(window: Arc<WindowKind>, mut app: Box<dyn App>, attr: WindowAttribute, event: EventLoopProxy<(WindowId, UpdateType)>) -> Result<Self, Box<dyn Error>> {
        let e = event.clone();
        let wid = window.id();
        let device = Self::rebuild_device(&window, |device| {
            device.on_uncaptured_error(Box::new(move |err| {
                e.send_event((wid, UpdateType::ReInit)).unwrap();
                println!("Error: {:?}", err);
            }));
        }).await?;
        let viewport = Viewport::new(&device.device, &device.cache);
        let context = Context {
            size: Size {
                width: window.size().width,
                height: window.size().height,
            },
            font: attr.font.clone(),
            viewport,
            window,
            resize: false,
            render: Render::new(&device),
            updates: Map::new(),
            event,
        };
        let mut app_ctx = AppContext::new(device, context);
        app_ctx.draw(&mut app);
        let mut state = Window {
            app_ctx,
            app,
        };
        state.configure_surface();

        Ok(state)
    }

    pub(crate) async fn rebuild_device(window: &Arc<WindowKind>, listen: impl FnOnce(&wgpu::Device)) -> Result<Device, Box<dyn Error>> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await?;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await?;
        let cache = Cache::new(&device);
        // let target = window.surface_window().into();
        let surface = instance.create_surface(window.clone())?;
        let cap = surface.get_capabilities(&adapter);
        // let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: cap.formats[0],
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![cap.formats[0].add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: window.size().width,
            height: window.size().height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        listen(&device);
        // let wid = window.id();
        // let e = event.clone();
        // device.on_uncaptured_error(Box::new(move |err| {
        //     e.send_event((wid, UpdateType::ReInit)).unwrap();
        //     println!("Error: {:?}", err);
        // }));
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

    pub fn configure_surface(&mut self) {
        self.app_ctx.device.surface.configure(&self.app_ctx.device.device, &self.app_ctx.device.surface_config);
    }


    pub(crate) fn render(&mut self) {
        // Create texture view
        self.app_ctx.context.viewport.update(&self.app_ctx.device.queue, Resolution {
            width: self.app_ctx.context.size.width,
            height: self.app_ctx.context.size.height,
        });
        println!("44444444444444444444");
        self.app_ctx.redraw(&mut self.app);
        println!("44444444444444444444");
    }

    pub(crate) fn resize(&mut self, new_size: Size) {
        self.app_ctx.context.resize = true;
        self.app_ctx.context.size.width = new_size.width;
        self.app_ctx.context.size.height = new_size.height;
        self.app_ctx.device.surface_config.width = new_size.width;
        self.app_ctx.device.surface_config.height = new_size.height;
        // self.configure_surface();
    }
}