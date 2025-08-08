use crate::frame::context::Context;
use crate::size::rect::Rect;
use crate::ui::{DrawParam, Ui};
use crate::vertex::ImageVertex;
use crate::Device;
use wgpu::util::DeviceExt;
use crate::widgets::image::Image;

pub struct PaintImage {
    id: String,
    uri: String,
    pub(crate) rect: Rect,
    vertices: Vec<ImageVertex>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
}

impl PaintImage {
    pub fn new(ui: &mut Ui, image: &Image) -> PaintImage {
        let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
        let vertices = vec![
            ImageVertex::new_coord(image.rect.left_top(), [0.0, 0.0], &ui.ui_manage.context.size),
            ImageVertex::new_coord(image.rect.left_bottom(), [0.0, 1.0], &ui.ui_manage.context.size),
            ImageVertex::new_coord(image.rect.right_bottom(), [1.0, 1.0], &ui.ui_manage.context.size),
            ImageVertex::new_coord(image.rect.right_top(), [1.0, 0.0], &ui.ui_manage.context.size)
        ];


        let vertex_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let index_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        PaintImage {
            id: image.id.clone(),
            uri: image.source.to_string(),
            rect: image.rect.clone(),
            vertex_buffer,
            index_buffer,
            vertices,
        }
    }


    pub fn render<A>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        self.prepare(param.device, Some(param.context));
        param.context.render.image.render(&self.uri, &self.vertex_buffer, &self.index_buffer, pass);
        // self.render.prepare(device, context);
        // self.render.render(render_pass);
    }

    fn prepare(&mut self, device: &Device, context: Option<&Context>) {
        for (index, v) in self.vertices.iter_mut().enumerate() {
            match index {
                0 => v.position = self.rect.left_top(),
                1 => v.position = self.rect.left_bottom(),
                2 => v.position = self.rect.right_bottom(),
                3 => v.position = self.rect.right_top(),
                _ => {}
            }
            if let Some(context) = context {
                v.screen_size = context.size.as_gamma_size()
            }
        }
        device.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(self.vertices.as_slice()));
    }

    pub fn offset(&mut self, device: &Device, ox: f32, oy: f32) -> Vec<(String, Rect)> {
        self.rect.offset(ox, oy);
        self.prepare(device, None);
        vec![(self.id.clone(), self.rect.clone())]
    }
}