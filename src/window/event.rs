use crate::{Pos, Size};
use crate::key::Key;

#[derive(Debug)]
pub enum WindowEvent {
    None,
    KeyPress(Key),
    KeyRelease(Key),
    MouseMove(Pos),
    MouseWheel,
    MousePress(Pos),
    MouseRelease(Pos),
    Redraw,
    Reinit,
    Resize(Size),
    ReqClose,
    ReqUpdate,
    IME(Vec<char>),
}