use crate::vertex::Vertex;
use crate::{Device, Offset};
use std::ops::Range;
use wgpu::include_wgsl;

pub struct TriangleRender {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl TriangleRender {
    pub fn new(device: &Device) -> TriangleRender {
        let vertices = vec![];
        let indices = vec![];
        let shader = device.device.create_shader_module(include_wgsl!("triangle.wgsl"));
        let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });
        let render_pipeline = super::create_pipeline(device, shader, pipeline_layout, &[Vertex::desc()]);
        let vertex_buffer = device.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 1024,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = device.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 1024,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        TriangleRender {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            render_pipeline,
        }
    }

    pub fn add_triangle(&mut self, mut vs: Vec<Vertex>, device: &Device) -> Range<usize> {
        let current = self.vertices.len() as u32;
        self.vertices.append(&mut vs);
        assert!(self.vertices.len() < 1024);
        self.indices.extend_from_slice(&[current, current + 1, current + 2]);
        device.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
        device.queue.write_buffer(&self.index_buffer, 0, bytemuck::cast_slice(&self.indices));
        current as usize..current as usize + 3
    }

    pub fn offset(&mut self, range: Range<usize>, offset: &Offset) {
        self.vertices[range].iter_mut().for_each(|x| {
            x.position[0] += offset.x;
            x.position[1] += offset.y;
        })
    }

    pub fn prepare(&mut self, range: Range<usize>, device: &Device, size: [f32; 2], c: [f32; 4]) {
        self.vertices[range].iter_mut().for_each(|x| {
            x.screen_size = size;
            x.color = c;
        });
        device.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }

    pub fn render(&mut self, range: Range<usize>, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(range.start as u32..range.end as u32, 0, 0..1);
    }
}