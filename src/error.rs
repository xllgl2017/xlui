use std::error::Error;
use std::string::ToString;

#[derive(Debug)]
pub enum UiError {
    NullPtr,
    UNINIT,
    OptNone,
    Error(String),
}

impl UiError {
    pub const UNINIT: UiError = UiError::UNINIT;

    pub fn to_string(&self) -> &str {
        match self {
            UiError::NullPtr => "空指针",
            UiError::UNINIT => "值未初始化",
            UiError::OptNone => "Option值为None",
            UiError::Error(value) => value
        }
    }
}

impl<E: Into<Box<dyn Error>>> From<E> for UiError {
    fn from(e: E) -> UiError {
        let es = e.into().to_string();
        UiError::Error(es)
    }
}



pub type UiResult<T> = Result<T, UiError>;