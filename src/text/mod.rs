pub mod text_buffer;
pub mod text_render;
pub mod rich;

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