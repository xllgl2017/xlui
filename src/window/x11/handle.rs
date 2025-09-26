use crate::window::ime::IME;
use crate::window::x11::clipboard::X11ClipBoard;
use crate::window::{ClipboardData, UserEvent};
use raw_window_handle::{DisplayHandle, RawDisplayHandle, RawWindowHandle, WindowHandle, XlibDisplayHandle, XlibWindowHandle};
use std::cell::RefCell;
use std::ffi::c_void;
use std::mem;
use std::os::raw::c_long;
use std::ptr::NonNull;
use std::sync::Arc;
use x11::xlib;
use x11::xlib::XMoveWindow;
use crate::error::UiResult;

pub struct X11WindowHandle {
    display: *mut xlib::Display,
    pub(crate) window: xlib::Window,
    pub(crate) update_atom: xlib::Atom,
    pub(crate) screen: i32,
    pub(crate) clipboard: X11ClipBoard,
}


impl X11WindowHandle {
    pub fn new(display: *mut xlib::Display, window: xlib::Window, update_atom: xlib::Atom, screen: i32) -> UiResult<X11WindowHandle> {
        Ok(X11WindowHandle {
            display,
            window,
            update_atom,
            screen,
            clipboard: X11ClipBoard::new(display)?,
        })
    }


    pub fn request_redraw(&self) {
        let s = RefCell::new(12);
        *s.borrow_mut() = 10;
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

    pub fn window_handle(&self) -> WindowHandle<'_> {
        let xlib_window_handle = XlibWindowHandle::new(self.window);
        let raw_window_handle = RawWindowHandle::Xlib(xlib_window_handle);
        unsafe { WindowHandle::borrow_raw(raw_window_handle) }
    }

    pub fn display_handle(&self) -> DisplayHandle<'_> {
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

    pub fn request_clipboard(&self, clipboard: ClipboardData) {
        match clipboard {
            ClipboardData::Unsupported => {}
            ClipboardData::Text(_) => self.clipboard.request_get_clipboard(self.window, self.clipboard.utf8_atom),
            ClipboardData::Image(_) => self.clipboard.request_get_clipboard(self.window, self.clipboard.png_atom),
            ClipboardData::Url(_) => self.clipboard.request_get_clipboard(self.window, self.clipboard.url_atom)
        }
    }

    pub fn set_clipboard(&self, clipboard: ClipboardData) {
        self.clipboard.request_set_clipboard(self.window, clipboard);
    }

    pub fn move_window(&self, x: f32, y: f32) {
        unsafe { XMoveWindow(self.display, self.window, x as i32, y as i32) };
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
