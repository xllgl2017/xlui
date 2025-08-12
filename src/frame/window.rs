use crate::font::Font;
use crate::frame::context::{Context, Render};
use crate::frame::App;
use crate::map::Map;
use crate::size::Size;
use crate::ui::AppContext;
use crate::{Device, DeviceInput};
use glyphon::{Cache, Resolution, Viewport};
use std::sync::Arc;

pub(crate) struct Window<A> {
    pub(crate) app_ctx: AppContext,
    pub(crate) app: A,
}

impl<A: App> Window<A> {
    pub(crate) async fn new(window: Arc<winit::window::Window>, mut app: A) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();
        let cache = Cache::new(&device);
        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let size = window.inner_size();
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: cap.formats[0],
            // Request compatibility with the sRGB-format texture view we‘re going to create later.
            view_formats: vec![cap.formats[0].add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };

        let font = Arc::new(Font::new());
        let viewport = Viewport::new(&device, &cache);
        let device = Device {
            device,
            queue,
            cache,
            texture_format: cap.formats[0],
            surface_config,

            device_input: DeviceInput::new(),
        };
        let context = Context {
            size: Size {
                width: size.width,
                height: size.height,
            },
            font: font.clone(),
            viewport,
            window,
            surface,
            resize: false,
            render: Render::new(&device),
            updates: Map::new(),
        };
        device.device.on_uncaptured_error(Box::new(|err| {
            println!("Error: {:?}", err);
        }));
        let mut app_ctx = AppContext::new(device, context);
        app_ctx.draw(&mut app);


        let mut state = Window {
            app_ctx,
            app,
        };
        state.configure_surface();

        state
    }

    pub fn get_window(&self) -> &winit::window::Window {
        &self.app_ctx.context.window
    }

    fn configure_surface(&mut self) {
        self.app_ctx.context.surface.configure(&self.app_ctx.device.device, &self.app_ctx.device.surface_config);
    }


    pub(crate) fn render(&mut self) {
        // self.app_ctx.device.device.poll(wgpu::MaintainBase::Poll).unwrap();
        // Create texture view
        self.app_ctx.context.viewport.update(&self.app_ctx.device.queue, Resolution {
            width: self.app_ctx.context.size.width,
            height: self.app_ctx.context.size.height,
        });
        self.app_ctx.redraw(&mut self.app)
    }
}


//设备输入
impl<A: App> Window<A> {
    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.app_ctx.context.resize = true;
        self.app_ctx.context.size.width = new_size.width;
        self.app_ctx.context.size.height = new_size.height;
        self.app_ctx.device.surface_config.width = new_size.width;
        self.app_ctx.device.surface_config.height = new_size.height;
        // reconfigure the surface
        self.configure_surface();
        // self.ui.resize();
    }
}
