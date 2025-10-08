#[cfg(all(windows, not(feature = "gpu")))]
use windows::Win32::Graphics::Gdi::{HDC, PAINTSTRUCT};
use crate::{MousePos, Pos, Size};
use crate::key::Key;
use crate::window::ClipboardData;
use crate::window::ime::IMEData;

#[derive(Debug)]
pub enum WindowEvent {
    None,
    KeyPress(Key),
    KeyRelease(Key),
    MouseMove(MousePos),
    MouseWheel(f32),
    MousePress(Pos),
    MouseRelease(Pos),
    #[cfg(any(target_os = "linux", feature = "gpu"))]
    Redraw,
    #[cfg(all(windows, not(feature = "gpu")))]
    Redraw(PAINTSTRUCT, HDC),
    ReInit,
    Resize(Size),
    ReqClose,
    ReqUpdate,
    IME(IMEData),
    CreateChild,
    Clipboard(ClipboardData),
    UserUpdate,
}