// use std::sync::Arc;
// use wgpu::util::DeviceExt;
// use winit::{
//     event::*,
//     event_loop::{ControlFlow, EventLoop},
// };
// use winit::window::WindowAttributes;
//
// // 顶点结构
// #[repr(C)]
// #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// struct Vertex {
//     pos: [f32; 2],
//     uv: [f32; 2],
// }
//
// impl Vertex {
//     fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
//         wgpu::VertexBufferLayout {
//             array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &[
//                 wgpu::VertexAttribute {
//                     offset: 0,
//                     shader_location: 0,
//                     format: wgpu::VertexFormat::Float32x2,
//                 },
//                 wgpu::VertexAttribute {
//                     offset: 8,
//                     shader_location: 1,
//                     format: wgpu::VertexFormat::Float32x2,
//                 },
//             ],
//         }
//     }
// }
//
// // 矩形顶点
// fn rect_vertices(min: [f32; 2], max: [f32; 2]) -> (Vec<Vertex>, Vec<u16>) {
//     let vertices = vec![
//         Vertex { pos: [min[0], min[1]], uv: [0.0, 0.0] }, // 左下
//         Vertex { pos: [max[0], min[1]], uv: [1.0, 0.0] }, // 右下
//         Vertex { pos: [max[0], max[1]], uv: [1.0, 1.0] }, // 右上
//         Vertex { pos: [min[0], max[1]], uv: [0.0, 1.0] }, // 左上
//     ];
//     let indices = vec![0, 1, 2, 0, 2, 3];
//     (vertices, indices)
// }
//
// // Uniforms
// #[repr(C)]
// #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
// struct Uniforms {
//     rect_min: [f32; 2],
//     _padding0: [f32; 2],    // 8 -> 对齐 rect_max
//     rect_max: [f32; 2],
//     radius: f32,
//     _padding1: [f32; 3],    // 12 -> 对齐到16
//     color: [f32; 4],
//     resolution: [f32; 2],
//     _padding2: [f32; 2],    // 8 -> 对齐到16
// }
//
// fn main() {
//     let event_loop = EventLoop::new().unwrap();
//     let window = Arc::new(event_loop.create_window(WindowAttributes::default()).unwrap());
//
//     let size = window.inner_size();
//
//     let instance = wgpu::Instance::default();
//     let surface = instance.create_surface(window.clone()).unwrap();
//     let adapter = pollster::block_on(async {
//         instance
//             .request_adapter(&wgpu::RequestAdapterOptions {
//                 power_preference: wgpu::PowerPreference::HighPerformance,
//                 compatible_surface: Some(&surface),
//                 force_fallback_adapter: false,
//             })
//             .await
//             .unwrap()
//     });
//
//     let (device, queue) = pollster::block_on(async {
//         adapter
//             .request_device(&wgpu::DeviceDescriptor::default())
//             .await
//             .unwrap()
//     });
//
//     let surface_caps = surface.get_capabilities(&adapter);
//     let surface_format = surface_caps.formats[0];
//     let mut config = wgpu::SurfaceConfiguration {
//         usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//         format: surface_format,
//         width: size.width,
//         height: size.height,
//         present_mode: wgpu::PresentMode::Fifo,
//         desired_maximum_frame_latency: 0,
//         alpha_mode: surface_caps.alpha_modes[0],
//         view_formats: vec![],
//     };
//     surface.configure(&device, &config);
//
//     // === 顶点和索引 ===
//     let (vertices, indices) = rect_vertices([0.0, 0.0], [100.0, 100.0]);
//     let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: Some("Vertex Buffer"),
//         contents: bytemuck::cast_slice(&vertices),
//         usage: wgpu::BufferUsages::VERTEX,
//     });
//     let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: Some("Index Buffer"),
//         contents: bytemuck::cast_slice(&indices),
//         usage: wgpu::BufferUsages::INDEX,
//     });
//
//     // === Uniforms ===
//     let uniforms = Uniforms {
//         rect_min: [0.0, 0.0],
//         rect_max: [100.0, 100.0],
//         radius: 50.0,
//         color: [0.2, 0.6, 0.9, 1.0],
//         resolution: [size.width as f32, size.height as f32],
//         _padding0: [0.0;2],
//         _padding1: [0.0;3],
//         _padding2: [0.0;2],
//     };
//     let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//         label: Some("Uniform Buffer"),
//         contents: bytemuck::bytes_of(&uniforms),
//         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
//     });
//     let uniform_bind_group_layout =
//         device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             label: Some("Uniform Bind Group Layout"),
//             entries: &[wgpu::BindGroupLayoutEntry {
//                 binding: 0,
//                 visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
//                 ty: wgpu::BindingType::Buffer {
//                     ty: wgpu::BufferBindingType::Uniform,
//                     has_dynamic_offset: false,
//                     min_binding_size: None,
//                 },
//                 count: None,
//             }],
//         });
//     let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
//         label: Some("Uniform Bind Group"),
//         layout: &uniform_bind_group_layout,
//         entries: &[wgpu::BindGroupEntry {
//             binding: 0,
//             resource: uniform_buffer.as_entire_binding(),
//         }],
//     });
//
//     // === Shader ===
//     let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
//         label: Some("Rounded Rect Shader"),
//         source: wgpu::ShaderSource::Wgsl(include_str!("1.wgsl").into()),
//     });
//
//     // === Pipeline ===
//     let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//         label: Some("Pipeline Layout"),
//         bind_group_layouts: &[&uniform_bind_group_layout],
//         push_constant_ranges: &[],
//     });
//     let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//         label: Some("Render Pipeline"),
//         layout: Some(&pipeline_layout),
//         vertex: wgpu::VertexState {
//             module: &shader,
//             entry_point: Some("vs_main"),
//             compilation_options: Default::default(),
//             buffers: &[Vertex::desc()],
//         },
//         fragment: Some(wgpu::FragmentState {
//             module: &shader,
//             entry_point: Some("fs_main"),
//             compilation_options: Default::default(),
//             targets: &[Some(wgpu::ColorTargetState {
//                 format: config.format,
//                 blend: Some(wgpu::BlendState::ALPHA_BLENDING),
//                 write_mask: wgpu::ColorWrites::ALL,
//             })],
//         }),
//         primitive: wgpu::PrimitiveState::default(),
//         depth_stencil: None,
//         multisample: wgpu::MultisampleState::default(),
//         multiview: None,
//         cache: None,
//     });
//     event_loop.set_control_flow(ControlFlow::Wait);
//
//     // === 渲染循环 ===
//     event_loop.run(move |event, _| {
//         match event {
//             Event::WindowEvent { event, .. } => {
//                 match event {
//                     WindowEvent::RedrawRequested => {
//                         let frame = surface.get_current_texture().unwrap();
//                         let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
//                         let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
//                             label: Some("Render Encoder"),
//                         });
//
//                         {
//                             let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                                 label: Some("Render Pass"),
//                                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                                     view: &view,
//                                     resolve_target: None,
//                                     ops: wgpu::Operations {
//                                         load: wgpu::LoadOp::Clear(wgpu::Color {
//                                             r: 0.1,
//                                             g: 0.1,
//                                             b: 0.1,
//                                             a: 1.0,
//                                         }),
//                                         store: wgpu::StoreOp::Store,
//                                     },
//                                 })],
//                                 depth_stencil_attachment: None,
//                                 timestamp_writes: None,
//                                 occlusion_query_set: None,
//                             });
//
//                             rpass.set_pipeline(&render_pipeline);
//                             rpass.set_bind_group(0, &uniform_bind_group, &[]);
//                             rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
//                             rpass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
//                             rpass.draw_indexed(0..indices.len() as u32, 0, 0..1);
//                         }
//
//                         queue.submit(Some(encoder.finish()));
//                         frame.present();
//                     }
//                     WindowEvent::Resized(size) => {
//                         config.width = size.width;
//                         config.height = size.height;
//                         surface.configure(&device, &config);
//                         window.request_redraw();
//                     }
//                     _ => {}
//                 }
//             }
//             _ => {}
//         }
//     }).unwrap();
// }
fn main() {

}