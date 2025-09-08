use crate::{Pos, Size};
use crate::key::Key;
use crate::window::ClipboardData;
use crate::window::ime::IMEData;

#[derive(Debug)]
pub enum WindowEvent {
    None,
    KeyPress(Key),
    KeyRelease(Key),
    MouseMove(Pos),
    MouseWheel(f32),
    MousePress(Pos),
    MouseRelease(Pos),
    Redraw,
    Reinit,
    Resize(Size),
    ReqClose,
    ReqUpdate,
    IME(IMEData),
    CreateChild,
    Clipboard(ClipboardData),
}