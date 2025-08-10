use crate::font::Font;
use crate::size::Size;
use crate::text::text_render::TextRender;
use crate::Device;
use glyphon::Viewport;
use std::sync::Arc;
use crate::map::Map;
use crate::render::circle::CircleRender;
use crate::render::image::ImageRender;
use crate::render::rectangle::RectangleRender;
use crate::render::triangle::TriangleRender;

pub enum ContextUpdate {
    Text(String),
    Combo(usize),
    // Popup(bool),
}

impl ContextUpdate {
    pub fn text(self) -> String {
        match self {
            ContextUpdate::Text(text) => text,
            _ => panic!("应该是:ContextUpdate::Text")
        }
    }

    pub fn combo(self) -> usize {
        match self {
            ContextUpdate::Combo(v) => v,
            _ => panic!("应该是:ContextUpdate::Text")
        }
    }

    // pub fn popup(self) -> bool {
    //     match self {
    //         ContextUpdate::Popup(v) => v,
    //         _ => panic!("应该是:ContextUpdate::Text")
    //     }
    // }
}

pub struct Context {
    pub size: Size,
    pub viewport: Viewport,
    pub window: Arc<winit::window::Window>,
    pub font: Arc<Font>,
    pub surface: wgpu::Surface<'static>,
    pub resize: bool,
    pub render: Render,
    pub updates: Map<ContextUpdate>,
}

impl Context {
    pub fn send_update(&mut self, id: String, update: ContextUpdate) {
        self.updates.insert(id, update);
        self.window.request_redraw(); //更新ui
    }
}


pub struct Render {
    pub(crate) rectangle: RectangleRender,
    pub(crate) text: TextRender,
    pub(crate) circle: CircleRender,
    pub(crate) image: ImageRender,
    pub(crate) triangle: TriangleRender,
}

impl Render {
    pub fn new(device: &Device) -> Render {
        Render {
            rectangle: RectangleRender::new(device),
            text: TextRender::new(device),
            circle: CircleRender::new(device),
            image: ImageRender::new(device),
            triangle: TriangleRender::new(device),
        }
    }
}


