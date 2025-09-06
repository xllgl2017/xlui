pub mod buffer;
pub mod render;
pub mod rich;
pub mod cchar;

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