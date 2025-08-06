use wgpu::util::DeviceExt;
use crate::{Device, SAMPLE_COUNT};

pub mod rectangle;
pub mod circle;


fn create_pipeline(device: &Device, shader: wgpu::ShaderModule, layout: wgpu::PipelineLayout) -> wgpu::RenderPipeline {
    device.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&layout),
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
    })
}


pub(crate) trait WrcRender {
    fn pipeline(&self) -> &wgpu::RenderPipeline;

    fn bind_groups(&self) -> &Vec<wgpu::BindGroup>;

    fn bind_groups_mut(&mut self) -> &mut Vec<wgpu::BindGroup>;

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout;

    fn create_bind_group(&mut self, device: &Device, buffer: &wgpu::Buffer) -> usize {
        let bind_group_entry = wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        };
        let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: self.bind_group_layout(),
            entries: &[bind_group_entry],
            label: None,
        });
        self.bind_groups_mut().push(bind_group);
        self.bind_groups().len() - 1
    }

    fn create_buffer(&self, device: &Device, param: &[u8]) -> wgpu::Buffer {
        device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: param,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        })
    }

    fn render(&self, index: usize, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(self.pipeline());
        render_pass.set_bind_group(0, &self.bind_groups()[index], &[]);
        render_pass.draw(0..6, 0..1);
    }
}