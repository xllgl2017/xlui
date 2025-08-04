pub mod text_buffer;
pub mod text_render;

pub struct TextSize {
    pub font_size: f32,
    pub line_width: f32,
    pub line_height: f32,
}

impl TextSize {
    pub fn new() -> TextSize {
        TextSize {
            font_size: 14.0,
            line_width: 0.0,
            line_height: 0.0,
        }
    }
}

pub enum TextWrap {
    NoWrap,
    WrapAnyWhere,
    WrapWorld,
}


impl TextWrap {
    pub fn as_gamma(&self) -> glyphon::Wrap {
        match self {
            TextWrap::NoWrap => glyphon::Wrap::None,
            TextWrap::WrapAnyWhere => glyphon::Wrap::Glyph,
            TextWrap::WrapWorld => glyphon::Wrap::Word
        }
    }
}