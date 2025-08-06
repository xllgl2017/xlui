use wgpu::util::DeviceExt;
use xlui::Device;
use xlui::frame::context::Context;
use xlui::paint::color::Color;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct CircleParams {
    center: [f32; 2],
    radius: f32,
    border_thickness: f32,
    fill_color: [f32; 4],
    border_color: [f32; 4],
    // resolution: [f32; 2],
    // _pad: [f32; 2],
}


pub struct CircleRender {
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,

}

impl CircleRender {
    pub fn new(device: &Device) -> Self {
        let shader = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("circle_shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../src/render/circle/circle.wgsl").into()),
        });
        let params = CircleParams {
            center: [300.0, 300.0],
            radius: 5.0,
            border_thickness: 0.0,
            fill_color: Color::GREEN.as_gamma_rgba(),
            border_color: Color::RED.as_gamma_rgba(),
            // resolution: [context.size.width as f32, context.size.height as f32],
            // _pad: [0.0;2],
        };

        let uniform_buffer = device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("uniform_buffer"),
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("uniform_bind_group_layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        let uniform_bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("uniform_bind_group"),
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("pipeline_layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: device.texture_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });
        Self {
            render_pipeline,
            bind_group: uniform_bind_group,
        }
    }

    pub(crate) fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}