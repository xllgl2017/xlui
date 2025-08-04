use wgpu::util::DeviceExt;
use crate::{Device, SAMPLE_COUNT};
use crate::paint::rectangle::param::RectangleParam;

pub struct RectangleRender {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl RectangleRender {
    pub fn new(device: &Device) -> RectangleRender {
        let shader = device.device.create_shader_module(wgpu::include_wgsl!("rectangle.wgsl"));
        let bind_group_layout_entry = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };
        let bind_group_layout = device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Rounded Border Layout"),
            entries: &[bind_group_layout_entry],
        });
        let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Rounded Border Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = device.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rounded Border Pipeline"),
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
            multisample: wgpu::MultisampleState {
                count: SAMPLE_COUNT,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });
        RectangleRender {
            pipeline,
            bind_group_layout,
            bind_groups: vec![],
        }
    }

    pub fn create_buffer(&self, device: &Device, param: &RectangleParam) -> wgpu::Buffer {
        device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&param.as_draw_param(false, false)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    pub fn create_bind_group(&mut self, device: &Device, buffer: &wgpu::Buffer) -> usize {
        let bind_group_entry = wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        };
        let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.bind_group_layout,
            entries: &[bind_group_entry],
            label: Some("Rounded Border Bind Group"),
        });
        self.bind_groups.push(bind_group);
        self.bind_groups.len() - 1
    }

    pub fn render(&self, index: usize, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_groups[index], &[]);
        render_pass.draw(0..6, 0..1);
    }
}
