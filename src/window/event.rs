use crate::frame::context::UpdateType;
use crate::{Pos, Size};

#[derive(Debug)]
pub enum WindowEvent {
    KeyPress,
    KeyRelease,
    MouseMove(Pos),
    MouseWheel,
    MousePress(Pos),
    MouseRelease(Pos),
    Redraw,
    Reinit,
    Resize(Size),
    ReqClose,
    Update(UpdateType),
}