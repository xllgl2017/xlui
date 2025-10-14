pub mod param;
#[cfg(feature = "gpu")]
use wgpu::IndexFormat;
#[cfg(feature = "gpu")]
use crate::render::WrcRender;
#[cfg(feature = "gpu")]
use crate::Device;
use crate::render::RenderParam;
#[cfg(feature = "gpu")]
use crate::vertex::Vertex;

#[cfg(feature = "gpu")]
pub struct RectangleRender2 {
    pipeline: wgpu::RenderPipeline,
    pub bind_group_layout: wgpu::BindGroupLayout,
}

#[cfg(feature = "gpu")]
impl RectangleRender2 {
    pub fn new(device: &Device) -> RectangleRender2 {
        let shader = device.device.create_shader_module(wgpu::include_wgsl!("../../bin/1.wgsl"));
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
        let bind_group_layout = device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[bind_group_layout_entry],
        });
        let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let pipeline = super::create_pipeline(device, shader, pipeline_layout, &[Vertex::desc()]);
        RectangleRender2{
            pipeline,
            bind_group_layout,
        }
    }

    pub(crate) fn render(&self, param: &RenderParam, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, param.bind_group.as_ref().unwrap(), &[]);
        render_pass.set_vertex_buffer(0, param.vertices_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(param.indices_buffer.as_ref().unwrap().slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..param.rect_param().rect_shape.indices.len() as u32, 0, 0..1);
    }
}


#[cfg(feature = "gpu")]
pub struct RectangleRender {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}
#[cfg(feature = "gpu")]
impl RectangleRender {
    pub fn new(device: &Device) -> RectangleRender {
        let shader = device.device.create_shader_module(wgpu::include_wgsl!("rectangle2.wgsl"));
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
        RectangleRender {
            pipeline,
            bind_group_layout,
        }
    }
}
#[cfg(feature = "gpu")]
impl WrcRender for RectangleRender {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}
