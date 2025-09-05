use std::ffi::{c_char, c_void, CString};
use std::{mem, slice};
use std::os::raw;
use std::os::raw::{c_int, c_ulong};
use std::ptr::null_mut;
use x11::xlib;
use x11::xlib::{SelectionNotify, XChangeProperty, XConvertSelection, XEvent, XFlush, XFree, XGetWindowProperty, XInternAtom, XSelectionEvent, XSendEvent, XSetSelectionOwner};
use crate::error::UiResult;

pub struct X11ClipBoard {
    clipboard_atom: xlib::Atom,
    utf8_atom: xlib::Atom,
    targets_atom: xlib::Atom,
    display: *mut xlib::Display,
}


impl X11ClipBoard {
    pub fn new(display: *mut xlib::Display) -> UiResult<X11ClipBoard> {
        let clipboard = CString::new("CLIPBOARD")?;
        let clipboard_atom = unsafe { XInternAtom(display, clipboard.as_ptr(), 0) };
        let utf8 = CString::new("UTF8_STRING")?;
        let utf8_atom = unsafe { XInternAtom(display, utf8.as_ptr(), 0) };
        let targets = CString::new("TARGETS")?;
        let targets_atom = unsafe { XInternAtom(display, targets.as_ptr(), 0) };
        Ok(X11ClipBoard {
            clipboard_atom,
            utf8_atom,
            targets_atom,
            display,
        })
    }

    pub fn request_set_clipboard(&self, window: xlib::Window) {
        unsafe { XSetSelectionOwner(self.display, self.clipboard_atom, window, xlib::CurrentTime); }
    }

    pub fn request_get_clipboard(&self, window: xlib::Window) {
        unsafe { XConvertSelection(self.display, self.clipboard_atom, self.utf8_atom, self.clipboard_atom, window, xlib::CurrentTime) };
        unsafe { XFlush(self.display); };
    }

    pub fn handle_request(&self, event: &XEvent) -> UiResult<()> {
        let xsr = unsafe { event.selection_request };
        let mut xs = unsafe { mem::zeroed::<XSelectionEvent>() };
        xs.type_ = SelectionNotify;
        xs.display = xsr.display;
        xs.requestor = xsr.requestor;
        xs.selection = xsr.selection;
        xs.target = xsr.target;
        xs.time = xsr.time;
        xs.property = xsr.property;
        if xsr.target == self.targets_atom {
            let data = [self.utf8_atom];
            unsafe {
                //返回粘贴板支持的格式
                XChangeProperty(self.display, xsr.requestor, xsr.property, xlib::XA_ATOM, 32,
                                xlib::PropModeReplace, data.as_ptr() as *const u8, data.len() as i32);
            }
        } else if xsr.target == self.utf8_atom {
            let cstr = CString::new("Hello Rust Clipboard")?;
            unsafe {
                //返回文本数据
                XChangeProperty(self.display, xsr.requestor, xsr.property, xlib::XA_ATOM, 8,
                                xlib::PropModeReplace, cstr.as_bytes().as_ptr(), cstr.as_bytes().len() as i32);
            }
        } else {
            xs.property = 0; //不支持的格式
        }
        let mut out_event = unsafe { mem::zeroed::<XEvent>() };
        out_event.selection = xs;
        unsafe { XSendEvent(self.display, xsr.requestor, 0, 0, &mut out_event); }
        unsafe { XFlush(self.display); }
        Ok(())
    }

    pub fn get_clipboard_data(&self, event: XEvent, window: xlib::Window) -> String {
        let xs = unsafe { event.selection };
        if xs.property == 0 { return String::new(); }
        let mut actual_atom: c_ulong = 0;
        let mut actual_format: c_int = 0;
        let mut nitems: c_ulong = 0;
        let mut bytes_after: u64 = 0;
        let mut prop: *mut u8 = null_mut();
        unsafe {
            XGetWindowProperty(
                self.display, window, self.clipboard_atom,
                0, i64::MAX, 0, xlib::AnyPropertyType as c_ulong,
                &mut actual_atom, &mut actual_format, &mut nitems, &mut bytes_after, &mut prop,
            );
        }
        if !prop.is_null() {
            let len = match actual_format {
                8 => nitems as usize,
                16 => nitems as usize * 2,
                32 => nitems as usize * 4,
                _ => nitems as usize
            };
            let slice = unsafe { slice::from_raw_parts(prop, len) };
            let result = String::from_utf8_lossy(slice).to_owned().to_string();
            println!("get_clipboard-{}-{}-{}-{:?}", self.utf8_atom, actual_atom, actual_format, result);
            unsafe { XFree(prop as *mut c_void); }
            return result.to_string();
        }
        String::new()
    }
}


