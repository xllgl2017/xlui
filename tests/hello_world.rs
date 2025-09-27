// use crate::render::RoundedBorderRenderer;
// use glyphon::{Cache, Resolution, Viewport};
// use std::sync::Arc;
// use wgpu::{LoadOp, Operations, RenderPassDescriptor};
// use winit::event_loop::EventLoopProxy;
// use winit::{
//     application::ApplicationHandler,
//     event::WindowEvent,
//     event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
//     window::{Window, WindowId},
// };
// use xlui::map::Map;
// use xlui::{Device, DeviceInput, Font};
//
// mod render;
//
// struct State {
//     device: Device,
//     context: Context,
//     rounded_renderer: RoundedBorderRenderer,
// }
//
// impl State {
//     async fn new(window: Arc<Window>, event: EventLoopProxy<(WindowId, UpdateType)>) -> State {
//         let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
//         let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await.unwrap();
//         let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await.unwrap();
//         let cache = Cache::new(&device);
//         let surface = instance.create_surface(window.clone()).unwrap();
//         let cap = surface.get_capabilities(&adapter);
//         let size = window.inner_size();
//         let surface_config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: cap.formats[0],
//             view_formats: vec![cap.formats[0].add_srgb_suffix()],
//             alpha_mode: wgpu::CompositeAlphaMode::Auto,
//             width: size.width,
//             height: size.height,
//             desired_maximum_frame_latency: 2,
//             present_mode: wgpu::PresentMode::AutoVsync,
//         };
//         let font = Arc::new(Font::from_file("/home/xl/RustroverProjects/xrs/target/res/font/simfang.ttf"));
//         let viewport = Viewport::new(&device, &cache);
//
//         let device = Device {
//             device,
//             queue,
//             cache,
//             texture_format: cap.formats[0],
//             surface_config,
//             device_input: DeviceInput::new(),
//             surface,
//         };
//         // let text_render = TextRender::new(&device);
//         let context = Context {
//             size: xlui::Size { width: window.inner_size().width, height: window.inner_size().height },
//             font: font.clone(),
//             viewport,
//             resize: false,
//             render: Render::new(&device),
//             updates: Map::new(),
//             user_update: (, UpdateType::None),
//             new_window: None,
//             window: Arc::new(()),
//         };
//         let rounded_renderer = RoundedBorderRenderer::new(&device.device, cap.formats[0]);
//         let mut state = State {
//             device,
//             context,
//             rounded_renderer,
//         };
//
//         // Configure surface for the first time
//         state.configure_surface();
//
//         state
//     }
//
//     fn configure_surface(&mut self) {
//         self.device.surface.configure(&self.device.device, &self.device.surface_config);
//     }
//
//     fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
//         self.context.size.width = new_size.width;
//         self.context.size.height = new_size.height;
//         self.device.surface_config.width = new_size.width;
//         self.device.surface_config.height = new_size.height;
//         self.configure_surface();
//     }
//
//     fn render(&mut self) {
//         // Create texture view
//         self.context.viewport.update(&self.device.queue, Resolution {
//             width: self.context.size.width,
//             height: self.context.size.height,
//         });
//
//         // Create the renderpass which will clear the screen.
//
//         let surface_texture = match self.device.surface.get_current_texture() {
//             Ok(texture) => texture,
//             Err(_) => {
//                 println!("failed to acquire next swapchain texture");
//                 return;
//             }
//         };
//         let texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor {                // Without add_srgb_suffix() the image we will be working with
//             // might not be "gamma correct".
//             format: Some(self.device.texture_format.add_srgb_suffix()),
//             ..Default::default()
//         });
//
//         // Renders a GREEN screen
//         let mut encoder = self.device.device.create_command_encoder(&Default::default());
//         let mut renderpass = encoder.begin_render_pass(&RenderPassDescriptor {
//             label: None,
//             color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                 view: &texture_view,
//                 resolve_target: None,
//                 ops: Operations {
//                     load: LoadOp::Clear(wgpu::Color { r: 0.97254902, g: 0.97254902, b: 0.97254902, a: 1.0 }),
//                     store: wgpu::StoreOp::Store,
//                 },
//             })],
//             depth_stencil_attachment: None,
//             timestamp_writes: None,
//             occlusion_query_set: None,
//         });
//         self.rounded_renderer.draw(&mut renderpass);
//         drop(renderpass);
//
//         // Submit the command in the queue to execute
//         self.device.queue.submit([encoder.finish()]);
//         // self.window.pre_present_notify();
//         surface_texture.present();
//     }
// }
//
// struct App {
//     state: Option<State>,
//     event: EventLoopProxy<(WindowId, UpdateType)>,
// }
//
//
// impl ApplicationHandler<(WindowId, UpdateType)> for App {
//     fn resumed(&mut self, event_loop: &ActiveEventLoop) {
//         // Create window object
//         let window = Arc::new(
//             event_loop
//                 .create_window(Window::default_attributes())
//                 .unwrap(),
//         );
//         let event = self.event.clone();
//         let state = pollster::block_on(State::new(window.clone(), event));
//         self.state = Some(state);
//
//         window.request_redraw();
//     }
//
//     fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
//         let state = self.state.as_mut().unwrap();
//         match event {
//             WindowEvent::CloseRequested => {
//                 println!("The close button was pressed; stopping");
//                 event_loop.exit();
//             }
//             WindowEvent::RedrawRequested => {
//                 state.render();
//                 // Emits a new redraw requested event.
//                 // state.get_window().request_redraw();
//             }
//             WindowEvent::Resized(size) => {
//                 // Reconfigures the size of the surface. We do not re-render
//                 // here as this event is always followed up by redraw request.
//                 state.resize(size);
//             }
//             WindowEvent::MouseInput { state, button, .. } => {
//
//                 // println!("{:?} {:?}", state, button);
//             }
//             WindowEvent::CursorMoved { position, .. } => {
//                 // println!("{:?}", position);
//             }
//             WindowEvent::MouseWheel { delta, .. } => {
//                 // println!("{:?}", delta);
//             }
//             _ => (),
//         }
//     }
// }
//
// fn main() {
//     let event_loop = EventLoop::with_user_event().build().unwrap();
//     let proxy_event = event_loop.create_proxy();
//     event_loop.set_control_flow(ControlFlow::Wait);
//     let mut app = App {
//         state: None,
//         event: proxy_event,
//     };
//
//     event_loop.run_app(&mut app).unwrap();
// }

// 完整可运行 demo：Rust + winit + wgpu
// 功能：
// - 渲染一个可填充的矩形（四边独立宽度、四角独立圆角）
// - 支持软阴影（可设置偏移、模糊半径、颜色、强度）
// - 使用 full-screen triangle + Fragment SDF 方案
//
// Cargo.toml 依赖：
// [dependencies]
// wgpu = "0.15"
// winit = "0.28"
// pollster = "0.2"
// bytemuck = { version = "1.9", features = ["derive"] }
// env_logger = "0.10"

// use std::sync::Arc;
// use std::time::Instant;
// use wgpu::util::DeviceExt;
// use winit::{event::*, event_loop::EventLoop};
// use bytemuck::{Pod, Zeroable};
// use wgpu::{include_wgsl, StoreOp, TextureFormat};
// use winit::event_loop::ControlFlow;
// use winit::window::WindowAttributes;
//
// #[repr(C)]
// #[derive(Clone, Copy, Pod, Zeroable, Debug)]
// struct RectUniforms {
//     center_position: [f32; 2],    //⬅️ 中心(x,y)
//     radius: [f32; 2],             //⬅️ 半径(w/2,h/2)
//     corner_radii: [f32; 4],       //⬅️ 圆角(左上、右上、右下、左下)
//     border_widths: [f32; 4],      //⬅️ 边框(左、右、上、下)
//     fill_color: [f32; 4],         //⬅️ 填充颜色(rgba)
//     border_color: [f32; 4],       //⬅️ 边框颜色(rgba)
//     screen: [f32; 4],             //⬅️ 总大小（宽、高）,缩放比例、填充
//     shadow_params: [f32; 4],      //⬅️ 阴影(x,y)、模糊半径、强度
//     shadow_color: [f32; 4],       //⬅️ 阴影颜色
// }
//
// impl Default for RectUniforms {
//     fn default() -> Self {
//         Self {
//             center_position: [200.0, 200.0],
//             radius: [100.0, 100.0],
//             corner_radii: [3.0, 3.0, 3.0, 3.0],
//             border_widths: [1.5, 1.5, 1.5,1.5],
//             fill_color: [240.0 / 255.0, 240.0 / 255.0, 240.0 / 255.0, 1.0],
//             border_color: [0.0, 0.0, 0.0, 1.0],
//             screen: [800.0, 600.0, 1.0, 0.0],
//             shadow_params: [5.0, 5.0, 10.0, 0.0],
//             shadow_color: [0.0, 0.0, 0.0, 0.45],
//
//         }
//     }
// }
//
//
// struct State {
//     surface: wgpu::Surface<'static>,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     config: wgpu::SurfaceConfiguration,
//     size: winit::dpi::PhysicalSize<u32>,
//     pipeline: wgpu::RenderPipeline,
//     uniform_buffer: wgpu::Buffer,
//     uniform_bind_group: wgpu::BindGroup,
//     start: Instant,
//     uniforms: RectUniforms,
//     format: TextureFormat,
// }
//
// impl State {
//     async fn new(window: Arc<winit::window::Window>) -> Self {
//         let size = window.inner_size();
//
//         let instance = wgpu::Instance::default();
//         let surface = unsafe { instance.create_surface(window) }.unwrap();
//         let adapter = instance.request_adapter(
//             &wgpu::RequestAdapterOptions {
//                 power_preference: wgpu::PowerPreference::HighPerformance,
//                 compatible_surface: Some(&surface),
//                 force_fallback_adapter: false,
//             },
//         ).await.expect("Failed to find an appropriate adapter");
//
//         let (device, queue) = adapter.request_device(
//             &wgpu::DeviceDescriptor {
//                 label: None,
//                 required_features: Default::default(),
//                 required_limits: Default::default(),
//                 memory_hints: Default::default(),
//                 trace: Default::default(),
//             },
//         ).await.expect("Failed to create device");
//
//         let surface_caps = surface.get_capabilities(&adapter);
//         let surface_format = surface_caps.formats[0];
//
//         let config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: surface_format,
//             width: size.width,
//             height: size.height,
//             present_mode: wgpu::PresentMode::Fifo,
//             desired_maximum_frame_latency: 0,
//             alpha_mode: surface_caps.alpha_modes[0],
//             view_formats: vec![],
//         };
//         surface.configure(&device, &config);
//
//         // shaders
//         // let vs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         //     label: Some("vs"),
//         //     source: wgpu::ShaderSource::Wgsl(VERT_SHADER.into()),
//         // });
//         // let fs_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         //     label: Some("fs"),
//         //     source: wgpu::ShaderSource::Wgsl(FRAG_SHADER.into()),
//         // });
//         let shader = device.create_shader_module(include_wgsl!("1.wgsl"));
//
//         // uniform buffer
//         let mut uniforms = RectUniforms::default();
//         uniforms.screen[0] = size.width as f32;
//         uniforms.screen[1] = size.height as f32;
//         uniforms.screen[2] = 1.0; // pixel scale if needed
//
//         let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("uniform_buffer"),
//             contents: bytemuck::bytes_of(&uniforms),
//             usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//         });
//
//         let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             entries: &[wgpu::BindGroupLayoutEntry {
//                 binding: 0,
//                 visibility: wgpu::ShaderStages::FRAGMENT,
//                 ty: wgpu::BindingType::Buffer { ty: wgpu::BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
//                 count: None,
//             }],
//             label: Some("uniform_bind_group_layout"),
//         });
//
//         let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//             layout: &bind_group_layout,
//             entries: &[wgpu::BindGroupEntry { binding: 0, resource: uniform_buffer.as_entire_binding() }],
//             label: Some("uniform_bind_group"),
//         });
//
//         // pipeline
//         let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: Some("pipeline_layout"),
//             bind_group_layouts: &[&bind_group_layout],
//             push_constant_ranges: &[],
//         });
//
//         let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: Some("rect_pipeline"),
//             layout: Some(&pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module: &shader,
//                 entry_point: Some("vs_main"),
//                 compilation_options: Default::default(),
//                 buffers: &[],
//             },
//             primitive: wgpu::PrimitiveState::default(),
//             depth_stencil: None,
//             multisample: wgpu::MultisampleState {
//                 count: 4,
//                 mask: !0,
//                 alpha_to_coverage_enabled: false,
//             },
//             fragment: Some(wgpu::FragmentState {
//                 module: &shader,
//                 entry_point: Some("fs_main"),
//                 compilation_options: Default::default(),
//                 targets: &[Some(wgpu::ColorTargetState {
//                     format: config.format,
//                     blend: Some(wgpu::BlendState::ALPHA_BLENDING),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 })],
//             }),
//             multiview: None,
//             cache: None,
//         });
//
//         Self {
//             surface,
//             device,
//             queue,
//             config,
//             size,
//             pipeline,
//             uniform_buffer,
//             uniform_bind_group,
//             start: Instant::now(),
//             uniforms,
//             format: surface_format,
//         }
//     }
//
//     fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
//         if new_size.width > 0 && new_size.height > 0 {
//             self.size = new_size;
//             self.config.width = new_size.width;
//             self.config.height = new_size.height;
//             self.surface.configure(&self.device, &self.config);
//         }
//     }
//
//     fn input(&mut self, _event: &WindowEvent) -> bool {
//         false
//     }
//
//     // fn update(&mut self) {
//     //     // animate shadow slightly for demo
//     //     let t = self.start.elapsed().as_secs_f32();
//     //     let wobble = (t * 1.5).sin() * 6.0;
//     //     self.uniforms.shadow_params[0] = 12.0 + wobble; // offset x
//     //     self.uniforms.shadow_params[1] = 14.0 + wobble * 0.5; // offset y
//     //
//     //     self.uniforms.screen[0] = self.size.width as f32;
//     //     self.uniforms.screen[1] = self.size.height as f32;
//     //
//     //     // upload
//     //     self.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&self.uniforms));
//     // }
//
//     fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
//         let output = self.surface.get_current_texture()?;
//         let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
//         let msaa_texture = self.device.create_texture(&wgpu::TextureDescriptor {
//             label: None,
//             size: wgpu::Extent3d {
//                 width: self.size.width,
//                 height: self.size.height,
//                 depth_or_array_layers: 1,
//             },
//             mip_level_count: 1,
//             sample_count: 4,
//             dimension: wgpu::TextureDimension::D2,
//             format: self.format,
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             view_formats: &[],
//         });
//         let msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
//         let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("encoder") });
//
//         {
//             let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                 label: Some("render_pass"),
//                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                     view: &msaa_view,
//                     resolve_target: Some(&view),
//                     ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::WHITE), store: StoreOp::Store },
//                 })],
//                 depth_stencil_attachment: None,
//                 timestamp_writes: None,
//                 occlusion_query_set: None,
//             });
//
//             rpass.set_pipeline(&self.pipeline);
//             rpass.set_bind_group(0, &self.uniform_bind_group, &[]);
//             // draw full-screen triangle
//             rpass.draw(0..3, 0..1);
//         }
//
//         self.queue.submit(Some(encoder.finish()));
//         output.present();
//         Ok(())
//     }
// }
//
// fn main() {
//     let event_loop = EventLoop::new().unwrap();
//
//     let window = event_loop.create_window(WindowAttributes::default()).unwrap();
//     let window = Arc::new(window);
//
//     let mut state = pollster::block_on(State::new(window.clone()));
//     event_loop.set_control_flow(ControlFlow::Wait);
//     event_loop.run(move |event, control_flow| {
//         match event {
//             Event::WindowEvent { ref event, window_id } if window_id == window.id() => {
//                 if !state.input(event) {
//                     match event {
//                         WindowEvent::RedrawRequested => {
//                             // state.update();
//                             match state.render() {
//                                 Ok(_) => {}
//                                 Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
//                                 Err(e) => eprintln!("render error: {:?}", e),
//                             }
//                         }
//                         // WindowEvent::KeyboardInput { input, .. } => {
//                         //     if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
//                         //         *control_flow = winit::event_loop::ControlFlow::Exit;
//                         //     }
//                         // }
//                         WindowEvent::Resized(physical_size) => {
//                             state.resize(*physical_size);
//                         }
//                         // WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
//                         //     state.resize(new_inner_size);
//                         // }
//                         _ => {}
//                     }
//                 }
//             }
//             _ => {}
//         }
//     }).unwrap();
// }
fn main() {

}