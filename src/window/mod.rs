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
mod ime;
#[cfg(feature = "winit")]
mod wnit;

use crate::window::ime::IME;
#[cfg(feature = "winit")]
use crate::window::wnit::handle::WInitWindowHandle;
#[cfg(all(target_os = "linux", not(feature = "winit")))]
use crate::window::x11::handle::X11WindowHandle;
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
    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    X11(X11WindowHandle),
    #[cfg(feature = "winit")]
    Winit(WInitWindowHandle),
}

#[derive(Debug)]
pub enum UserEvent {
    ReqUpdate = 0,
    CreateChild = 1,
    ReInit = 2,
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
    pub fn winit(&self) -> &WInitWindowHandle {
        match self.kind {
            WindowKind::Winit(ref window) => window,
        }
    }

    pub fn set_ime_position(&self, x: f32, y: f32) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.set_ime_position(&self.ime, x, y),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.set_ime_position(x, y)
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

    pub fn request_update(&self, event: UserEvent) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            WindowKind::X11(ref window) => window.send_update(event),
            #[cfg(feature = "winit")]
            WindowKind::Winit(ref window) => window.send_user_event(self.id, event).unwrap()
        }
    }

    pub fn ime(&self) -> &Arc<IME> {
        &self.ime
    }

    // pub fn create_window(&self) {
    //     match self.kind {
    //         #[cfg(all(target_os = "linux", not(feature = "winit")))]
    //         WindowKind::X11(ref window) => window.send_update(UserEvent::CreateChild),
    //         #[cfg(feature = "winit")]
    //         WindowKind::Winit(_) => {}
    //     }
    // }
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