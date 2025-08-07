use crate::render::RoundedBorderRenderer;
use glyphon::{Cache, Resolution, Viewport};
use std::sync::Arc;
use wgpu::{LoadOp, Operations, RenderPassDescriptor};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};
use xlui::font::Font;
use xlui::frame::context::{Context, Render};
use xlui::size::Size;
use xlui::{Device, DeviceInput};
use xlui::map::Map;

mod render;

struct State {
    device: Device,
    context: Context,
    rounded_renderer: RoundedBorderRenderer,
}

impl State {
    async fn new(window: Arc<Window>) -> State {
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
        };
        // let text_render = TextRender::new(&device);
        let context = Context {
            size: Size { width: window.inner_size().width, height: window.inner_size().height },
            font: font.clone(),
            viewport,
            window,
            surface,
            resize: false,
            render: Render::new(&device),
            updates: Map::new(),
        };
        let rounded_renderer = RoundedBorderRenderer::new(&device.device, cap.formats[0]);

        let mut state = State {
            device,
            context,
            rounded_renderer,
        };

        // Configure surface for the first time
        state.configure_surface();

        state
    }

    fn configure_surface(&mut self) {
        // self.buttons.iter_mut().for_each(|button| button.need_repaint = true);
        self.context.surface.configure(&self.device.device, &self.device.surface_config);
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.context.size.width = new_size.width;
        self.context.size.height = new_size.height;
        self.device.surface_config.width = new_size.width;
        self.device.surface_config.height = new_size.height;
        // reconfigure the surface
        self.configure_surface();
    }

    fn render(&mut self) {
        // Create texture view
        self.context.viewport.update(&self.device.queue, Resolution {
            width: self.context.size.width,
            height: self.context.size.height,
        });

        // Create the renderpass which will clear the screen.

        let surface_texture = match self.context.surface.get_current_texture() {
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
        self.rounded_renderer.draw(&mut renderpass);
        drop(renderpass);

        // Submit the command in the queue to execute
        self.device.queue.submit([encoder.finish()]);
        // self.window.pre_present_notify();
        surface_texture.present();
    }
}

#[derive(Default)]
struct App {
    state: Option<State>,
}


impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Create window object
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let state = pollster::block_on(State::new(window.clone()));
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
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut app = App::default();
    event_loop.run_app(&mut app).unwrap();
}
