pub mod attribute;
pub mod inner;
#[cfg(all(target_os = "linux", not(feature = "winit")))]
pub mod x11;
#[cfg(not(feature = "winit"))]
pub mod wino;
#[cfg(not(feature = "winit"))]
pub mod event;
#[cfg(feature = "winit")]
pub mod winit_app;
pub mod ime;
#[cfg(feature = "winit")]
mod wnit;

#[cfg(all(not(feature = "winit"), target_os = "windows"))]
pub(crate) mod win32;

use crate::window::ime::IME;
#[cfg(feature = "winit")]
use crate::window::wnit::handle::WInitWindowHandle;
#[cfg(all(target_os = "linux", not(feature = "winit")))]
use crate::window::x11::handle::X11WindowHandle;
#[cfg(feature = "gpu")]
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};
use std::fmt::{Display, Formatter};
use std::sync::Arc;
#[cfg(all(target_os = "windows", not(feature = "gpu")))]
use windows::Win32::Graphics::Gdi::{CreateRectRgn, DeleteObject, RestoreDC, SaveDC, SelectClipRgn, HGDIOBJ};
use crate::*;
use crate::ui::PaintParam;

#[derive(Copy, Clone, PartialEq, Hash, Debug, Eq)]
pub struct WindowId(u32);

impl WindowId {
    pub fn unique_id() -> WindowId {
        WindowId(crate::unique_id_u32())
    }

    #[cfg(feature = "winit")]
    pub fn from_winit_id(id: winit::window::WindowId) -> Self {
        let mut id = format!("{:?}", id).replace("WindowId(", "").replace(")", "");
        if id.len() >= 8 { id = id[..8].to_string(); }
        WindowId(id.parse().unwrap())
    }
}

impl Display for WindowId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

pub enum WindowKind {
    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    X11(X11WindowHandle),
    #[cfg(feature = "winit")]
    Winit(WInitWindowHandle),
    #[cfg(all(not(feature = "winit"), target_os = "windows"))]
    Win32(win32::handle::Win32WindowHandle),
}

#[derive(Debug, Clone, Default)]
pub enum ClipboardData {
    #[default]
    Unsupported,
    Text(String),
    Image(Vec<u8>),
    Url(String),
}

impl ClipboardData {
    pub fn text(&self) -> &str {
        match self {
            ClipboardData::Unsupported => "unsupported data",
            ClipboardData::Text(text) => text.as_str(),
            ClipboardData::Image(_) => "image",
            ClipboardData::Url(_) => "url"
        }
    }
}


#[derive(Debug)]
pub enum UserEvent {
    ReqUpdate = 0,
    CreateChild = 1,
    ReInit = 2,
    UserUpdate = 3,
}

pub struct WindowType {
    kind: WindowKind,
    id: WindowId,
    type_: u32,
    ime: Arc<IME>,

}


impl WindowType {
    pub(crate) const ROOT: u32 = 0;
    #[cfg(not(feature = "winit"))]
    pub(crate) const CHILD: u32 = 1;

    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    pub(crate) fn x11(&self) -> &X11WindowHandle {
        match self.kind {
            WindowKind::X11(ref window) => window,
        }
    }

    #[cfg(feature = "winit")]
    pub(crate) fn winit(&self) -> &WInitWindowHandle {
        match self.kind {
            WindowKind::Winit(ref window) => window,
        }
    }

    #[cfg(all(not(feature = "winit"), target_os = "windows"))]
    pub(crate) fn win32(&self) -> &win32::handle::Win32WindowHandle {
        match self.kind {
            WindowKind::Win32(ref window) => window
        }
    }

    pub(crate) fn set_ime_position(&self, x: f32, y: f32, cursor_height: f32) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.set_ime_position(&self.ime, x, y + cursor_height),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.set_ime_position(x, y + cursor_height),
            #[cfg(all(not(feature = "winit"), target_os = "windows"))]
            WindowKind::Win32(ref window) => window.set_ime_position(x, y, cursor_height).unwrap(),
        }
    }

    pub(crate) fn id(&self) -> WindowId {
        self.id
    }

    pub(crate) fn request_redraw(&self) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.request_redraw(),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.request_redraw(),
            #[cfg(all(not(feature = "winit"), target_os = "windows"))]
            WindowKind::Win32(ref window) => window.request_redraw().unwrap()
        }
    }

    pub(crate) fn request_update_event(&self, event: UserEvent) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.send_update(event),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.send_user_event(self.id, event).unwrap(),
            #[cfg(all(not(feature = "winit"), target_os = "windows"))]
            WindowKind::Win32(ref window) => window.send_update(event),
        }
    }
    ///仅调用当前window下的App::update
    pub fn request_update(&self) {
        self.request_update_event(UserEvent::UserUpdate);
    }

    pub(crate) fn ime(&self) -> &Arc<IME> {
        &self.ime
    }

    #[cfg(all(not(feature = "winit"), target_os = "windows"))]
    pub(crate) fn set_visible(&self, visible: bool) {
        match self.kind {
            WindowKind::Win32(ref window) => window.set_visible(visible).unwrap(),
        }
    }

    #[cfg(not(feature = "winit"))]
    pub(crate) fn request_clipboard(&self, clipboard: ClipboardData) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.request_clipboard(clipboard),
            #[cfg(all(target_os = "windows", not(feature = "winit")))]
            WindowKind::Win32(_) => {}
        }
    }

    #[cfg(not(feature = "winit"))]
    pub(crate) fn set_clipboard(&self, clipboard: ClipboardData) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.set_clipboard(clipboard),
            #[cfg(all(target_os = "windows", not(feature = "winit")))]
            WindowKind::Win32(ref window) => window.clipboard.set_clipboard_data(clipboard).unwrap()
        }
    }

    pub(crate) fn size(&self) -> Size {
        match self.kind {
            #[cfg(all(windows, not(feature = "winit")))]
            WindowKind::Win32(ref window) => window.size(),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.size(),
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.size()
        }
    }

    pub(crate) fn set_clip_rect(&self, paint: &mut PaintParam, clip: Rect) {
        #[cfg(feature = "gpu")]
        paint.pass.set_scissor_rect(clip.dx().min as u32, clip.dy().min as u32, clip.width() as u32, clip.height() as u32);
        #[cfg(all(target_os = "linux", not(feature = "gpu")))]
        paint.cairo.rectangle(clip.dx().min as f64, clip.dy().min as f64, clip.width() as f64, clip.height() as f64);
        #[cfg(all(target_os = "linux", not(feature = "gpu")))]
        paint.cairo.clip();
        #[cfg(all(target_os = "windows", not(feature = "gpu")))]
        unsafe {
            paint.saved_hdc = SaveDC(paint.hdc);
            // 先创建矩形区域
            let hrgn = CreateRectRgn(
                clip.dx().min as i32,
                clip.dy().min as i32,
                clip.dx().max as i32,
                clip.dy().max as i32,
            );

            // 设置裁剪区域（会替换当前裁剪）
            SelectClipRgn(paint.hdc, Some(hrgn));

            // 删除区域对象，GDI 内部会复制
            DeleteObject(HGDIOBJ::from(hrgn)).ok().unwrap();
        }
    }

    pub(crate) fn reset_clip(&self, paint: &mut PaintParam) {
        #[cfg(feature = "gpu")]
        paint.pass.set_scissor_rect(0, 0, self.size().width_u32(), self.size().height_u32());
        #[cfg(all(target_os = "linux", not(feature = "gpu")))]
        paint.cairo.reset_clip();
        #[cfg(all(target_os = "windows", not(feature = "gpu")))]
        unsafe { RestoreDC(paint.hdc, paint.saved_hdc).ok().unwrap(); }
    }
}
#[cfg(feature = "gpu")]
impl HasWindowHandle for WindowType {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => Ok(window.window_handle()),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.window_handle(),
            #[cfg(all(not(feature = "winit"), target_os = "windows"))]
            WindowKind::Win32(ref window) => Ok(window.window_handle()),
        }
    }
}
#[cfg(feature = "gpu")]
impl HasDisplayHandle for WindowType {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => Ok(window.display_handle()),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.display_handle(),
            #[cfg(all(not(feature = "winit"), target_os = "windows"))]
            WindowKind::Win32(ref window) => Ok(window.display_handle())
        }
    }
}

