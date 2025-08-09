pub mod param;
use crate::Device;
use crate::render::WrcRender;

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
            bind_groups: vec![],
        }
    }

    // pub fn create_buffer(&self, device: &Device, param: &RectParam) -> wgpu::Buffer {
    //     device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //         label: None,
    //         contents: bytemuck::bytes_of(&param.as_draw_param(false, false)),
    //         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    //     })
    // }
    //
    // pub fn create_bind_group(&mut self, device: &Device, buffer: &wgpu::Buffer) -> usize {
    //     let bind_group_entry = wgpu::BindGroupEntry {
    //         binding: 0,
    //         resource: buffer.as_entire_binding(),
    //     };
    //     let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
    //         layout: &self.bind_group_layout,
    //         entries: &[bind_group_entry],
    //         label: None,
    //     });
    //     self.bind_groups.push(bind_group);
    //     self.bind_groups.len() - 1
    // }

    pub fn render(&self, index: usize, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.bind_groups[index], &[]);
        render_pass.draw(0..6, 0..1);
    }
}

impl WrcRender for RectangleRender {
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
