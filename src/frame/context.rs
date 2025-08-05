use crate::font::Font;
use crate::render::rectangle::RectangleRender;
use crate::size::Size;
use crate::text::text_render::TextRender;
use crate::Device;
use glyphon::Viewport;
use std::sync::Arc;

pub struct Context {
    pub size: Size,
    pub viewport: Viewport,
    pub window: Arc<winit::window::Window>,
    pub font: Arc<Font>,
    pub surface: wgpu::Surface<'static>,
    pub resize: bool,
    pub render: Render,

}


pub struct Render {
    pub(crate) rectangle: RectangleRender,
    pub(crate) text: TextRender,
}

impl Render {
    pub fn new(device: &Device) -> Render {
        Render {
            rectangle: RectangleRender::new(device),
            text: TextRender::new(device),
        }
    }
}


