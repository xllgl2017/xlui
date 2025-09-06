pub mod buffer;
pub mod render;
pub mod rich;
pub mod cchar;

#[derive(PartialEq)]
pub enum TextWrap {
    NoWrap,
    WrapAny,
    WrapWorld,
}


impl TextWrap {
    pub fn as_gamma(&self) -> glyphon::Wrap {
        match self {
            TextWrap::NoWrap => glyphon::Wrap::None,
            TextWrap::WrapAny => glyphon::Wrap::Glyph,
            TextWrap::WrapWorld => glyphon::Wrap::Word
        }
    }

    pub fn is_wrap(&self) -> bool {
        match self {
            TextWrap::WrapWorld | TextWrap::WrapAny => true,
            _ => false
        }
    }
}