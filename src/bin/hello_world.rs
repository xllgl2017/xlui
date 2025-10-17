#[cfg(all(feature = "gpu", feature = "winit"))]
use std::sync::Arc;
#[cfg(all(feature = "gpu", feature = "winit"))]
use wgpu::util::DeviceExt;
#[cfg(all(feature = "gpu", feature = "winit"))]
use wgpu::{include_wgsl, IndexFormat, RenderPassDescriptor, StoreOp, TextureDimension};
#[cfg(all(feature = "gpu", feature = "winit"))]
use winit::window::WindowAttributes;
#[cfg(all(feature = "gpu", feature = "winit"))]
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
};
#[cfg(all(feature = "gpu", feature = "winit"))]
use xlui::vertex::Vertex;
#[cfg(all(feature = "gpu", feature = "winit"))]
use xlui::{Border, Color, Rect};
#[cfg(all(feature = "gpu", feature = "winit"))]
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Screen {
    size: [f32; 2],
}
#[cfg(all(feature = "gpu", feature = "winit"))]
fn draw_rect() -> (Vec<Vertex>, Vec<u16>) {
    let mut rect = Rect::new();
    rect.set_x_min(153.0);
    rect.set_x_max(383.0);
    rect.set_y_min(58.34375);
    rect.set_y_max(288.34375);
    // let mut rectangle = RectangleShape::new(); //CircleShape::new(); //
    // rectangle.update(&rect, &Color::rgb(230, 230, 230), &Border::same(2.0));
    (vec![], vec![])
}
#[cfg(all(feature = "gpu", feature = "winit"))]
fn main() {
    // 创建窗口
    let event_loop = EventLoop::new().unwrap();
    let window = Arc::new(event_loop.create_window(WindowAttributes::default()).unwrap());

    // 初始化 GPU
    let instance = wgpu::Instance::default();
    let surface = unsafe { instance.create_surface(&window) }.unwrap();
    let adapter = pollster::block_on(async {
        instance.request_adapter(&wgpu::RequestAdapterOptions {
            compatible_surface: Some(&surface),
            ..Default::default()
        }).await.unwrap()
    });
    let (device, queue) = pollster::block_on(async { adapter.request_device(&Default::default()).await.unwrap() });

    // 配置表面
    let size = window.inner_size();
    let surface_format = surface.get_capabilities(&adapter).formats[0];
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: surface_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        desired_maximum_frame_latency: 0,
        alpha_mode: wgpu::CompositeAlphaMode::Opaque,
        view_formats: vec![],
    };
    surface.configure(&device, &config);
    let shader = device.create_shader_module(include_wgsl!("1.wgsl"));
    let bind_group_layout_entry = wgpu::BindGroupLayoutEntry {
        binding: 0,
        visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    };
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        label: None,
        entries: &[bind_group_layout_entry],
    });


    // 创建渲染管线
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Pipeline Layout"),
        bind_group_layouts: &[&bind_group_layout],
        push_constant_ranges: &[],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Option::from("vs_main"),
            compilation_options: Default::default(),
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Option::from("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: surface_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 4,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    });
    let mut screen = Screen { size: [800.0, 600.0] };
    let bind_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: None,
        contents: bytemuck::bytes_of(&screen),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });
    let bind_group_entry = wgpu::BindGroupEntry {
        binding: 0,
        resource: bind_buffer.as_entire_binding(),
    };
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        entries: &[bind_group_entry],
        label: None,
    });
    // let (points, indices) = draw_ring(1.0);
    // let (points, indices) = draw_line();
    let (points, indices) = draw_rect();

    let indices_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        // contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
        size: 8096,
        mapped_at_creation: false,
    });
    // 创建顶点缓冲
    let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Vertex Buffer"),
        // contents: bytemuck::cast_slice(&points),
        usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        size: 8096,
        mapped_at_creation: false,
    });
    queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(&points));
    queue.write_buffer(&indices_buffer, 0, bytemuck::cast_slice(&indices));
    event_loop.set_control_flow(ControlFlow::Wait);
    // 事件循环
    event_loop.run(move |event, _| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::RedrawRequested => {
                        let frame = surface.get_current_texture().unwrap();
                        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        let msaa_texture = device.create_texture(&wgpu::TextureDescriptor {
                            label: None,
                            size: wgpu::Extent3d {
                                width: config.width,
                                height: config.height,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 4,
                            dimension: TextureDimension::D2,
                            format: surface_format,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            view_formats: &[],
                        });
                        let msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());

                        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

                        {
                            let pass_desc = RenderPassDescriptor {
                                label: None,
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &msaa_view,
                                    resolve_target: Some(&view),
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color::WHITE),
                                        store: StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: None,
                                timestamp_writes: None,
                                occlusion_query_set: None,
                            };
                            let mut pass = encoder.begin_render_pass(&pass_desc);
                            pass.set_pipeline(&render_pipeline);
                            pass.set_bind_group(0, &bind_group, &[]);
                            pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            pass.set_index_buffer(indices_buffer.slice(..), IndexFormat::Uint16);
                            pass.draw_indexed(0..indices.len() as u32, 0, 0..1);
                        }

                        queue.submit(Some(encoder.finish()));
                        frame.present();
                    }
                    WindowEvent::Resized(size) => {
                        config.width = size.width;
                        config.height = size.height;
                        surface.configure(&device, &config);
                        screen.size = [config.width as f32, config.height as f32];
                        queue.write_buffer(&bind_buffer, 0, bytemuck::bytes_of(&screen));
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }).unwrap();
}


#[cfg(any(not(feature = "gpu"), not(feature = "winit")))]
fn main() {}

// fn main() {
//     // let p1 = Pos { x: 1.0, y: 1.0 };
//     // let p2 = Pos { x: 2.0, y: 2.0 };
//     // let w = 1.0;
//     //
//     // let (c1, c2) = get_circle_pos(p1, p2, w);
//     // println!("Circle centers: {:?} and {:?}", c1, c2);
//     let center = Pos { x: 2.0, y: 1.0 };
//     let a = Pos { x: 5.0, y: 1.0 }; // 假设 A 在圆上，且距离 center 正好是半径 r = 3
//     let angle_deg = 90.0; // 把 A 逆时针转 90 度
//
//     let b = rotate_point_deg(a, center, angle_deg);
//     println!("B = {:?}", b); // 结果应接近 (2.0, 4.0)
// }