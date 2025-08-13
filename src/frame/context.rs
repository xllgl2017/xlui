use crate::font::Font;
use crate::size::Size;
use crate::text::text_render::TextRender;
use crate::{Device, NumCastExt, Offset};
use glyphon::Viewport;
use std::sync::Arc;
use crate::map::Map;
use crate::render::circle::CircleRender;
use crate::render::image::ImageRender;
use crate::render::rectangle::RectangleRender;
use crate::render::triangle::TriangleRender;

#[derive(Clone)]
pub enum ContextUpdate {
    String(String),
    I32(i32),
    F32(f32),
    U8(u8),
    Bool(bool),
}

impl ContextUpdate {
    pub fn update_i32(&self, value: &mut i32) {
        match self {
            ContextUpdate::String(v) => *value = v.parse::<i32>().unwrap_or(*value),
            ContextUpdate::I32(v) => *value = *v,
            ContextUpdate::F32(v) => *value = *v as i32,
            ContextUpdate::U8(v) => *value = *v as i32,
            _ => {}
        }
    }
    pub fn update_f32(&self, value: &mut f32) {
        match self {
            ContextUpdate::String(v) => *value = v.parse::<f32>().unwrap_or(*value),
            ContextUpdate::I32(v) => *value = *v as f32,
            ContextUpdate::F32(v) => *value = *v,
            ContextUpdate::U8(v) => *value = *v as f32,
            _ => {}
        }
    }

    pub fn update_t<T: NumCastExt>(&self, value: &mut T) {
        match self {
            ContextUpdate::String(v) => *value = T::from_num(v.parse::<f64>().unwrap_or(value.as_f32() as f64)),
            ContextUpdate::I32(v) => *value = T::from_num(*v as f64),
            ContextUpdate::F32(v) => *value = T::from_num(*v as f64),
            ContextUpdate::U8(v) => *value = T::from_num(*v as f64),
            _ => {}
        }
    }

    pub fn update_bool(&self, value: &mut bool) {
        match self {
            ContextUpdate::Bool(v) => *value = *v,
            _ => {}
        }
    }
}

pub enum UpdateType {
    None,
    MouseMove,
    MousePress,
    MouseRelease,
    MouseWheel,
    KeyRelease(Option<winit::keyboard::Key>),
    Offset(Offset),
}

impl UpdateType {
    pub(crate) fn is_offset(&self) -> Option<&Offset> {
        match self {
            UpdateType::Offset(o) => Some(o),
            _ => None,
        }
    }
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


