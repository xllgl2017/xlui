use crate::font::Font;
use crate::frame::context::{Context, Render};
use crate::frame::App;
use crate::size::Size;
use crate::style::Style;
use crate::ui::Ui;
use crate::{Device, DeviceInput};
use glyphon::{Cache, Resolution, Viewport};
use std::sync::Arc;
use crate::map::Map;

pub(crate) struct Window<A> {
    pub(crate) ui: Ui,
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

        let font = Arc::new(Font::new_from_file("/home/xl/RustroverProjects/xrs/target/res/font/simfang.ttf"));
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
            popup: Map::new(),
        };
        let mut ui = Ui::new(device, context, Style::light_style());
        app.draw(&mut ui);

        let mut state = Window {
            app,
            ui,
        };
        state.configure_surface();

        state
    }

    pub fn get_window(&self) -> &winit::window::Window {
        &self.ui.ui_manage.context.window
    }

    fn configure_surface(&mut self) {
        self.ui.ui_manage.context.surface.configure(&self.ui.device.device, &self.ui.device.surface_config);
    }


    pub(crate) fn render(&mut self) {
        // Create texture view
        self.ui.ui_manage.context.viewport.update(&self.ui.device.queue, Resolution {
            width: self.ui.ui_manage.context.size.width,
            height: self.ui.ui_manage.context.size.height,
        });
        self.ui.draw(&mut self.app);
    }
}


//设备输入
impl<A: App> Window<A> {
    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.ui.ui_manage.context.resize = true;
        self.ui.ui_manage.context.size.width = new_size.width;
        self.ui.ui_manage.context.size.height = new_size.height;
        self.ui.device.surface_config.width = new_size.width;
        self.ui.device.surface_config.height = new_size.height;
        // reconfigure the surface
        self.configure_surface();
        self.ui.resize();
    }
}
