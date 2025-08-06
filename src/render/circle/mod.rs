use crate::Device;
use wgpu::{include_wgsl};
use crate::render::WrcRender;

pub mod param;

pub struct CircleRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    bind_groups: Vec<wgpu::BindGroup>,
}

impl CircleRenderer {
    pub fn new(device: &Device) -> Self {
        let shader = device.device.create_shader_module(include_wgsl!("circle.wgsl"));
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

        let bind_group_layout_desc = wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[bind_group_layout_entry],
        };
        let bind_group_layout = device.device.create_bind_group_layout(&bind_group_layout_desc);
        let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = super::create_pipeline(device, shader, pipeline_layout);
        CircleRenderer {
            pipeline: render_pipeline,
            bind_group_layout,
            bind_groups: vec![],
        }
    }

}

impl WrcRender for CircleRenderer {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    fn bind_groups(&self) -> &Vec<wgpu::BindGroup> {
        &self.bind_groups
    }

    fn bind_groups_mut(&mut self) -> &mut Vec<wgpu::BindGroup> {
        &mut self.bind_groups
    }

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}
