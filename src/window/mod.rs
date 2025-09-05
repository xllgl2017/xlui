pub mod attribute;
pub mod inner;
#[cfg(all(target_os = "linux", not(feature = "winit")))]
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
#[cfg(all(target_os = "linux", not(feature = "winit")))]
use crate::window::x11::UserEvent;
use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, WindowHandle};
use std::fmt::{Display, Formatter};
use std::sync::Arc;
#[cfg(all(target_os = "linux", not(feature = "winit")))]
use crate::window::x11::handle::X11WindowHandle;

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
    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    X11(X11WindowHandle),
    #[cfg(feature = "winit")]
    Winit(winit::window::Window),
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

    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    pub fn x11(&self) -> &X11WindowHandle {
        match self.kind {
            WindowKind::X11(ref window) => window,
        }
    }

    #[cfg(feature = "winit")]
    pub fn winit(&self) -> &winit::window::Window {
        match self.kind {
            WindowKind::Winit(ref window) => window,
        }
    }

    pub fn set_ime_position(&self, x: f32, y: f32) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.set_ime_position(&self.ime, x, y),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => {
                let pos = winit::dpi::LogicalPosition::new(x, y);
                let size = winit::dpi::LogicalSize::new(100.0, 100.0);
                window.set_ime_cursor_area(pos, size);
            }
        }
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn request_redraw(&self) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.request_redraw(),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.request_redraw()
        }
    }

    pub fn request_update(&self) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.send_update(UserEvent::ReqUpdate),
            #[cfg(feature = "winit")]
            WindowKind::Winit(_) => {}
        }
    }

    pub fn ime(&self) -> &Arc<IME> {
        &self.ime
    }

    pub fn create_window(&self) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.send_update(UserEvent::CreateChild),
            #[cfg(feature = "winit")]
            WindowKind::Winit(_) => {}
        }
    }
}

impl HasWindowHandle for WindowType {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => Ok(window.window_handle()),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.window_handle(),
        }
    }
}

impl HasDisplayHandle for WindowType {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => Ok(window.display_handle()),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.display_handle(),
        }
    }
}