use crate::{MousePos, Pos, Size};
use crate::key::Key;
#[cfg(not(feature = "gpu"))]
use crate::ui::PaintParam;
use crate::window::ClipboardData;
use crate::window::ime::IMEData;

#[derive(Debug)]
pub enum WindowEvent {
    KeyPress(Key),
    KeyRelease(Key),
    MouseMove(MousePos),
    MouseWheel(f32),
    MousePress(Pos),
    MouseRelease(Pos),
    #[cfg(feature = "gpu")]
    Redraw,
    #[cfg(not(feature = "gpu"))]
    Redraw(PaintParam<'static>),
    ReInit,
    Resize(Size),
    ReqUpdate,
    IME(IMEData),
    Clipboard(ClipboardData),
    UserUpdate,
}