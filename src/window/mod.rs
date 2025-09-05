pub mod attribute;
pub mod inner;
#[cfg(target_os = "linux")]
mod x11;
#[cfg(not(feature = "winit"))]
pub mod wino;
pub mod event;
#[cfg(feature = "winit")]
pub mod winit_app;
#[cfg(not(feature = "winit"))]
pub mod application;
#[cfg(feature = "winit")]
mod winit_window;
mod ime;

use crate::window::ime::IME;
use crate::window::x11::{UserEvent, X11WindowType};
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Hash, Debug, Eq)]
pub struct WindowId(u32);

impl WindowId {
    pub fn unique_id() -> WindowId {
        WindowId(crate::unique_id_u32())
    }
    #[cfg(feature = "winit")]
    pub fn from_winit_id(id: winit::window::WindowId) -> Self {
        let id = format!("{:?}", id).replace("WindowId(", "").replace(")", "");
        WindowId(id.parse().unwrap())
    }
}

impl Display for WindowId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.0.to_string().as_str())
    }
}

pub enum WindowKind {
    X11(X11WindowType)
}

pub struct WindowType {
    kind: WindowKind,
    id: WindowId,
    type_: u32,
    ime: Arc<IME>,

}


impl WindowType {
    pub const ROOT: u32 = 0;
    pub const CHILD: u32 = 1;

    pub fn x11(&self) -> &X11WindowType {
        match self.kind {
            WindowKind::X11(ref window) => window,
        }
    }

    pub fn set_ime_position(&self, x: f32, y: f32) {
        match self.kind {
            WindowKind::X11(ref window) => window.set_ime_position(&self.ime, x, y)
        }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn request_redraw(&self) {
        match self.kind {
            WindowKind::X11(ref window) => window.request_redraw(),
        }
    }

    pub fn request_update(&self) {
        match self.kind {
            WindowKind::X11(ref window) => window.send_update(UserEvent::ReqUpdate)
        }
    }

    pub fn ime(&self) -> &Arc<IME> {
        &self.ime
    }

    pub fn create_window(&self) {
        match self.kind {
            WindowKind::X11(ref window) => window.send_update(UserEvent::CreateChild)
        }
    }
}

impl HasWindowHandle for WindowType {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self.kind {
            WindowKind::X11(ref window) => Ok(window.window_handle()),
        }
    }
}

impl HasDisplayHandle for WindowType {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        match self.kind {
            WindowKind::X11(ref window) => Ok(window.display_handle()),
        }
    }
}