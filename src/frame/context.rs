use crate::frame::App;
use crate::key::Key;
use crate::map::Map;
#[cfg(feature = "gpu")]
use crate::render::circle::CircleRender;
#[cfg(feature = "gpu")]
use crate::render::image::ImageRender;
#[cfg(feature = "gpu")]
use crate::render::rectangle::RectangleRender;
#[cfg(feature = "gpu")]
use crate::render::triangle::TriangleRender;
#[cfg(feature = "gpu")]
use crate::text::render::TextRender;
use crate::window::ime::IMEData;
use crate::window::{ClipboardData, WindowId, WindowType};
use crate::{Font, NumCastExt};
#[cfg(feature = "gpu")]
use crate::Device;
#[cfg(feature = "gpu")]
use glyphon::Viewport;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone)]
pub enum ContextUpdate {
    String(String),
    // I32(i32),
    F32(f32),
    // U8(u8),
    Bool(bool),
}

impl ContextUpdate {
    // pub fn update_i32(&self, value: &mut i32) {
    //     match self {
    //         ContextUpdate::String(v) => *value = v.parse::<i32>().unwrap_or(*value),
    //         // ContextUpdate::I32(v) => *value = *v,
    //         ContextUpdate::F32(v) => *value = *v as i32,
    //         // ContextUpdate::U8(v) => *value = *v as i32,
    //         _ => {}
    //     }
    // }
    pub fn update_f32(&self, value: &mut f32) {
        match self {
            ContextUpdate::String(v) => *value = v.parse::<f32>().unwrap_or(*value),
            // ContextUpdate::I32(v) => *value = *v as f32,
            ContextUpdate::F32(v) => *value = *v,
            // ContextUpdate::U8(v) => *value = *v as f32,
            _ => {}
        }
    }

    pub fn update_t<T: NumCastExt>(&self, value: &mut T) {
        match self {
            ContextUpdate::String(v) => *value = T::from_num(v.parse::<f64>().unwrap_or(value.as_f32() as f64)),
            // ContextUpdate::I32(v) => *value = T::from_num(*v as f64),
            ContextUpdate::F32(v) => *value = T::from_num(*v as f64),
            // ContextUpdate::U8(v) => *value = T::from_num(*v as f64),
            _ => {}
        }
    }

    pub fn update_bool(&self, value: &mut bool) {
        match self {
            ContextUpdate::Bool(v) => *value = *v,
            _ => {}
        }
    }

    pub fn update_str(self, value: &mut String) {
        match self {
            ContextUpdate::String(v) => *value = v,
            // ContextUpdate::I32(v) => *value = v.to_string(),
            ContextUpdate::F32(v) => *value = v.to_string(),
            // ContextUpdate::U8(v) => *value = v.to_string(),
            ContextUpdate::Bool(v) => *value = v.to_string(),
        }
    }

    pub fn to_string(self) -> String {
        match self {
            ContextUpdate::String(v) => v,
            ContextUpdate::F32(v) => v.to_string(),
            ContextUpdate::Bool(v) => v.to_string()
        }
    }
}

#[derive(Clone)]
pub enum UpdateType {
    Draw,
    None,
    Init,
    ReInit,
    MouseMove,
    MousePress,
    MouseRelease,
    MouseWheel,
    KeyPress(Key),
    KeyRelease(Key),
    IME(IMEData),
    CreateWindow,
    Clipboard(ClipboardData),
}


impl Debug for UpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UpdateType::None => f.write_str("None"),
            UpdateType::Init => f.write_str("Init"),
            UpdateType::ReInit => f.write_str("ReInit"),
            UpdateType::MouseMove => f.write_str("MouseMove"),
            UpdateType::MousePress => f.write_str("MousePress"),
            UpdateType::MouseRelease => f.write_str("MouseRelease"),
            UpdateType::MouseWheel => f.write_str("MouseWheel"),
            UpdateType::KeyRelease(_) => f.write_str("KeyRelease"),
            UpdateType::IME(_) => f.write_str("IME"),
            UpdateType::CreateWindow => f.write_str("CreateWindow"),
            UpdateType::Clipboard(_) => f.write_str("Clipboard"),
            UpdateType::KeyPress(_) => f.write_str("KeyPress"),
            UpdateType::Draw => f.write_str("Draw"),
        }
    }
}

pub struct Context {
    #[cfg(feature = "gpu")]
    pub viewport: Viewport,
    pub window: Arc<WindowType>,
    pub font: Arc<Font>,
    #[cfg(feature = "gpu")]
    pub render: Render,
    pub updates: Map<String, ContextUpdate>,
    pub user_update: (WindowId, UpdateType),
    pub new_window: Option<Box<dyn App>>,
}
#[cfg(feature = "gpu")]
pub struct Render {
    pub(crate) rectangle: RectangleRender,
    pub(crate) text: TextRender,
    pub(crate) circle: CircleRender,
    pub(crate) image: ImageRender,
    pub(crate) triangle: TriangleRender,
}
#[cfg(feature = "gpu")]
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


