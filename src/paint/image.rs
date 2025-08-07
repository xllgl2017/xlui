use crate::Device;
use crate::frame::context::Context;
use crate::size::rect::Rect;
use crate::widgets::image::ImageReader;

pub struct PaintImage {
    id: String,
    pub(crate) rect: Rect,
    render: ImageReader,
}

impl PaintImage {
    pub fn new(render: ImageReader, rect: Rect) -> PaintImage {
        PaintImage {
            id: "".to_string(),
            rect,
            render,
        }
    }


    pub fn render(&mut self, device: &Device, context: &Context, render_pass: &mut wgpu::RenderPass) {
        self.render.prepare(device, context);
        self.render.render(render_pass);
    }

    pub fn offset(&mut self, device: &Device, ox: f32, oy: f32) -> Vec<(String, Rect)> {
        self.rect.offset(ox, oy);
        self.render.offset(device, &self.rect);
        vec![(self.id.clone(), self.rect.clone())]
    }
}