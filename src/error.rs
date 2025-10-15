use std::error::Error;
use std::string::ToString;

#[derive(Debug)]
pub enum UiError {
    NullPtr,
    UNINIT,
    OptNone,
    SendErr,
    Error(String),
}

impl UiError {
    pub const UNINIT: UiError = UiError::UNINIT;

    pub fn to_string(&self) -> &str {
        match self {
            UiError::NullPtr => "空指针",
            UiError::UNINIT => "值未初始化",
            UiError::OptNone => "Option值为None",
            UiError::SendErr => "通道发送失败",
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

#[cfg(target_os = "windows")]
impl From<UiError> for windows::core::Error{
    fn from(value: UiError) -> Self {
        windows::core::Error::new(windows::core::HRESULT(-1),value.to_string())
    }
}

pub type UiResult<T> = Result<T, UiError>;