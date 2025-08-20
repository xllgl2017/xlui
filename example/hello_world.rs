mod tr;

use crate::render::RoundedBorderRenderer;
use glyphon::{Cache, Resolution, Viewport};
use std::sync::Arc;
use wgpu::{LoadOp, Operations, RenderPassDescriptor};
use winit::event_loop::EventLoopProxy;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use xlui::font::Font;
use xlui::frame::context::{Context, Render, UpdateType};
use xlui::map::Map;
use xlui::size::Size;
use xlui::{Device, DeviceInput};
use crate::tr::TriangleRender;

mod render;

struct State {
    device: Device,
    context: Context,
    rounded_renderer: RoundedBorderRenderer,
    triangle_render: TriangleRender,
}

impl State {
    async fn new(window: Arc<Window>, event: EventLoopProxy<(WindowId, UpdateType)>) -> State {
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
            surface,
        };
        // let text_render = TextRender::new(&device);
        let context = Context {
            size: Size { width: window.inner_size().width, height: window.inner_size().height },
            font: font.clone(),
            viewport,
            window,
            resize: false,
            render: Render::new(&device),
            updates: Map::new(),
            event,
        };
        let rounded_renderer = RoundedBorderRenderer::new(&device.device, cap.formats[0]);
        let triangle_render = TriangleRender::new(&device.device, cap.formats[0]);
        let mut state = State {
            device,
            context,
            rounded_renderer,
            triangle_render,
        };

        // Configure surface for the first time
        state.configure_surface();

        state
    }

    fn configure_surface(&mut self) {
        self.device.surface.configure(&self.device.device, &self.device.surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.context.size.width = new_size.width;
        self.context.size.height = new_size.height;
        self.device.surface_config.width = new_size.width;
        self.device.surface_config.height = new_size.height;
        self.configure_surface();
    }

    fn render(&mut self) {
        // Create texture view
        self.context.viewport.update(&self.device.queue, Resolution {
            width: self.context.size.width,
            height: self.context.size.height,
        });

        // Create the renderpass which will clear the screen.

        let surface_texture = match self.device.surface.get_current_texture() {
            Ok(texture) => texture,
            Err(_) => {
                println!("failed to acquire next swapchain texture");
                return;
            }
        };
        let texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {                // Without add_srgb_suffix() the image we will be working with
            // might not be "gamma correct".
            format: Some(self.device.texture_format.add_srgb_suffix()),
            ..Default::default()
        });

        // Renders a GREEN screen
        let mut encoder = self.device.device.create_command_encoder(&Default::default());
        let mut renderpass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(wgpu::Color { r: 0.97254902, g: 0.97254902, b: 0.97254902, a: 1.0 }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        // self.rounded_renderer.draw(&mut renderpass);
        self.triangle_render.render(&mut renderpass);
        drop(renderpass);

        // Submit the command in the queue to execute
        self.device.queue.submit([encoder.finish()]);
        // self.window.pre_present_notify();
        surface_texture.present();
    }
}

struct App {
    state: Option<State>,
    event: EventLoopProxy<(WindowId, UpdateType)>,
}


impl ApplicationHandler<(WindowId, UpdateType)> for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let event = self.event.clone();
        let state = pollster::block_on(State::new(window.clone(), event));
        self.state = Some(state);

        window.request_redraw();
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let state = self.state.as_mut().unwrap();
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                state.render();
                // Emits a new redraw requested event.
                // state.get_window().request_redraw();
            }
            WindowEvent::Resized(size) => {
                // Reconfigures the size of the surface. We do not re-render
                // here as this event is always followed up by redraw request.
                state.resize(size);
            }
            WindowEvent::MouseInput { state, button, .. } => {

                // println!("{:?} {:?}", state, button);
            }
            WindowEvent::CursorMoved { position, .. } => {
                // println!("{:?}", position);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                // println!("{:?}", delta);
            }
            _ => (),
        }
    }
}

fn main() {
    let event_loop = EventLoop::with_user_event().build().unwrap();
    let proxy_event = event_loop.create_proxy();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App {
        state: None,
        event: proxy_event,
    };

    event_loop.run_app(&mut app).unwrap();
}
