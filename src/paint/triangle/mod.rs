use crate::frame::context::Context;
use crate::paint::color::Color;
use crate::paint::gen_render_pipeline;
use crate::ui::Ui;
use crate::vertex::Vertex;
use crate::Device;
use std::ops::Range;

pub struct PaintTriangle {
    pub(crate) vertices: Vec<Vertex>,
    indices: Vec<u32>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl PaintTriangle {
    pub fn new(ui: &mut Ui) -> PaintTriangle {
        let vertices = vec![];
        let indices = vec![];
        let render_pipeline = gen_render_pipeline(&ui.device, wgpu::PrimitiveTopology::TriangleList);
        let vertex_buffer = ui.device.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 1024,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let index_buffer = ui.device.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: 1024,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        PaintTriangle {
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

    pub fn set_color(&mut self, rgn: Range<usize>, color: &Color, device: &Device) {
        self.vertices[rgn].iter_mut().for_each(|x| x.color = color.as_gamma_rgba());
        device.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }

    pub fn prepare(&mut self, device: &Device, context: &Context) {
        if self.vertices.len() == 0 { return; }
        self.vertices.iter_mut().for_each(|x| {
            x.screen_size = context.size.as_gamma_size();
        });
        device.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        if self.vertices.len() == 0 { return; }
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
    }
}