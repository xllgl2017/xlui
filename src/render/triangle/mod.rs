pub mod param;

use crate::render::WrcRender;
use crate::Device;
use wgpu::{include_wgsl, BindGroupLayout, RenderPipeline};

pub struct TriangleRender {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

impl TriangleRender {
    pub fn new(device: &Device) -> TriangleRender {
        let shader = device.device.create_shader_module(include_wgsl!("triangle.wgsl"));
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
            label: None,
            entries: &[bind_group_layout_entry],
        });
        let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = super::create_pipeline(device, shader, pipeline_layout, &[]);
        TriangleRender {
            pipeline,
            bind_group_layout,
        }
    }
}

impl WrcRender for TriangleRender {
    fn pipeline(&self) -> &RenderPipeline {
        &self.pipeline
    }

    fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.bind_group_layout
    }
}