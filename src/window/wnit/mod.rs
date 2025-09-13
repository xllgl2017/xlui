pub mod handle;

use crate::frame::context::{Context, Render, UpdateType};
use crate::frame::App;
use crate::map::Map;
use crate::ui::AppContext;
use crate::window::{UserEvent, WindowType};
use crate::{Device, DeviceInput, Size, WindowAttribute};
use glyphon::{Cache, Resolution, Viewport};
use std::error::Error;
use std::sync::Arc;

pub(crate) struct Window {
    pub(crate) app_ctx: AppContext,
    pub(crate) app: Box<dyn App>,
}

impl Window {
    #[cfg(feature = "winit")]
    pub(crate) async fn new_winit(window: Arc<WindowType>, mut app: Box<dyn App>, attr: WindowAttribute) -> Result<Self, Box<dyn Error>> {
        let w = window.clone();
        let device = Self::rebuild_device(&window, |device| {
            device.on_uncaptured_error(Box::new(move |err| {
                w.request_update_event(UserEvent::ReInit);
                println!("Error: {:?}", err);
            }));
        }).await?;
        let viewport = Viewport::new(&device.device, &device.cache);
        let context = Context {
            font: attr.font.clone(),
            user_update: (window.id, UpdateType::None),
            viewport,
            window,
            render: Render::new(&device),
            updates: Map::new(),
            new_window: None,
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

    pub(crate) async fn rebuild_device(window: &Arc<WindowType>, listen: impl FnOnce(&wgpu::Device)) -> Result<Device, Box<dyn Error>> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await?;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await?;
        let cache = Cache::new(&device);
        let surface = instance.create_surface(window.clone())?;
        let cap = surface.get_capabilities(&adapter);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: cap.formats[0],
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![cap.formats[0].add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: window.winit().inner_size().width,
            height: window.winit().inner_size().height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        listen(&device);
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
            width: self.app_ctx.device.surface_config.width,
            height: self.app_ctx.device.surface_config.height,
        });
        self.app_ctx.redraw(&mut self.app);
    }

    pub(crate) fn resize(&mut self, new_size: Size) {
        self.app_ctx.device.surface_config.width = new_size.width;
        self.app_ctx.device.surface_config.height = new_size.height;
        self.configure_surface();
    }
}