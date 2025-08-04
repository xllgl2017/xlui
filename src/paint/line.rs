use wgpu::util::DeviceExt;
use crate::Device;
use crate::frame::context::Context;
use crate::paint::color::Color;
use crate::paint::gen_render_pipeline;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::vertex::Vertex;

pub struct PaintLine {
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    render_pipeline: wgpu::RenderPipeline,
}

impl PaintLine {
    pub fn new(ui: &mut Ui, rect: &Rect, border: &Border) -> PaintLine {
        let (vertices, indices) = border.radius_border_vertex(&ui.ui_manage.context.size, rect);
        let render_pipeline = gen_render_pipeline(&ui.device, wgpu::PrimitiveTopology::LineStrip);
        let vertex_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let index_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        PaintLine {
            vertices,
            indices,
            vertex_buffer,
            index_buffer,
            render_pipeline,
        }
    }

    pub fn prepare(&mut self, device: &Device, context: &Context, fill: &Color) {
        if self.vertices.len() == 0 { return; }
        self.vertices.iter_mut().for_each(|x| {
            x.color = fill.as_gamma_rgba();



            x.screen_size = context.size.as_gamma_size();
        });

        self.vertex_buffer = device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::cast_slice(&self.vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        // device.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(self.vertices.as_slice()));
    }

    pub fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
        if self.vertices.len() == 0 { return; }
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indices.len() as u32, 0, 0..1);
    }
}