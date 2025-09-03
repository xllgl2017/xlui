use std::error::Error;

#[derive(Debug)]
pub struct UiError {
    pub error: String,
}

impl<E: Into<Box<dyn Error>>> From<E> for UiError {
    fn from(e: E) -> UiError {
        let es = e.into().to_string();
        UiError { error: es }
    }
}

pub type UiResult<T> = Result<T, UiError>;