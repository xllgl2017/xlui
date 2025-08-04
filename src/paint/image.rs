use crate::Device;
use crate::frame::context::Context;
use crate::size::rect::Rect;
use crate::widgets::image::ImageReader;

pub struct PaintImage {
    pub(crate) rect: Rect,
    render: ImageReader,
}

impl PaintImage {
    pub fn new(render: ImageReader, rect: Rect) -> PaintImage {
        PaintImage {
            rect,
            render,
        }
    }


    pub fn render(&mut self, device: &Device, context: &Context, render_pass: &mut wgpu::RenderPass) {
        self.render.prepare(device, context);
        self.render.render(render_pass);
    }
}