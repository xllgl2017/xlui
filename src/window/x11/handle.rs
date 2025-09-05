use std::ffi::c_void;
use std::mem;
use std::os::raw::c_long;
use std::ptr::NonNull;
use std::sync::Arc;
use raw_window_handle::{DisplayHandle, RawDisplayHandle, RawWindowHandle, WindowHandle, XlibDisplayHandle, XlibWindowHandle};
use x11::xlib;
use crate::window::ime::IME;
use crate::window::x11::UserEvent;

pub struct X11WindowHandle {
    pub(crate) display: *mut xlib::Display,
    pub(crate) window: xlib::Window,
    pub(crate) update_atom: xlib::Atom,
    pub(crate) screen: i32,
}


impl X11WindowHandle {
    pub fn request_redraw(&self) {
        unsafe {
            xlib::XClearArea(self.display, self.window, 0, 0, 0, 0, xlib::True);
            xlib::XFlush(self.display);
        }
    }

    pub fn send_update(&self, ue: UserEvent) {
        let mut event: xlib::XClientMessageEvent = unsafe { mem::zeroed() };
        event.type_ = xlib::ClientMessage;
        event.display = self.display;
        event.window = self.window;
        event.message_type = self.update_atom;
        event.format = 32;
        event.data.set_long(0, ue as c_long);
        let mask = xlib::NoEventMask;
        unsafe { xlib::XSendEvent(self.display, self.window, 0, mask, &mut event as *mut _ as *mut _); }
        unsafe { xlib::XFlush(self.display); }
    }

    pub fn window_handle(&self) -> WindowHandle {
        let xlib_window_handle = XlibWindowHandle::new(self.window);
        let raw_window_handle = RawWindowHandle::Xlib(xlib_window_handle);
        unsafe { WindowHandle::borrow_raw(raw_window_handle) }
    }

    pub fn display_handle(&self) -> DisplayHandle {
        let display = NonNull::new(self.display as *mut c_void);
        let x11_display_handle = XlibDisplayHandle::new(display, self.screen);
        let raw_display_handle = RawDisplayHandle::Xlib(x11_display_handle);
        unsafe { DisplayHandle::borrow_raw(raw_display_handle) }
    }

    pub fn set_ime_position(&self, ime: &Arc<IME>, x: f32, y: f32) {
        let root = unsafe { xlib::XRootWindow(self.display, self.screen) };
        let mut child_return: xlib::Window = 0;
        let mut ax: i32 = 0;
        let mut ay: i32 = 0;
        let status = unsafe {
            xlib::XTranslateCoordinates(
                self.display, self.window, root, 0, 0, &mut ax, &mut ay,
                &mut child_return,
            )
        };
        if status == 0 { return; }
        ime.set_cursor_position(ax + x as i32, ay + y as i32);
    }
}

impl Drop for X11WindowHandle {
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
        }
    }
}

unsafe impl Send for X11WindowHandle {}
unsafe impl Sync for X11WindowHandle {}
