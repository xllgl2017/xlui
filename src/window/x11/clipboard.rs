use crate::error::UiResult;
use std::ffi::{c_void, CString};
use std::os::raw::{c_int, c_ulong};
use std::ptr::null_mut;
use std::{mem, slice};
use std::sync::atomic::{AtomicBool, Ordering};
use x11::xlib;

pub struct X11ClipBoard {
    clipboard_atom: xlib::Atom,
    utf8_atom: xlib::Atom,
    pub(crate) targets_atom: xlib::Atom,
    display: *mut xlib::Display,
    png_atom: xlib::Atom,
    text_atom: xlib::Atom,
    timestamp_atom: xlib::Atom,
    url_atom: xlib::Atom,
    req_targets: AtomicBool,
}


impl X11ClipBoard {
    pub fn new(display: *mut xlib::Display) -> UiResult<X11ClipBoard> {
        let clipboard = CString::new("CLIPBOARD")?;
        let clipboard_atom = unsafe { xlib::XInternAtom(display, clipboard.as_ptr(), 0) };
        let utf8 = CString::new("UTF8_STRING")?;
        let utf8_atom = unsafe { xlib::XInternAtom(display, utf8.as_ptr(), 0) };
        let png = CString::new("image/png")?;
        let png_atom = unsafe { xlib::XInternAtom(display, png.as_ptr(), 0) };
        let text = CString::new("TEXT")?;
        let text_atom = unsafe { xlib::XInternAtom(display, text.as_ptr(), 0) };
        let timestamp = CString::new("TIMESTAMP")?;
        let timestamp_atom = unsafe { xlib::XInternAtom(display, timestamp.as_ptr(), 0) };
        let url = CString::new("text/uri-list")?;
        let url_atom = unsafe { xlib::XInternAtom(display, url.as_ptr(), 0) };

        let targets = CString::new("TARGETS")?;
        let targets_atom = unsafe { xlib::XInternAtom(display, targets.as_ptr(), 0) };
        Ok(X11ClipBoard {
            clipboard_atom,
            utf8_atom,
            targets_atom,
            display,
            png_atom,
            text_atom,
            timestamp_atom,
            url_atom,
            req_targets: AtomicBool::new(false),
        })
    }

    pub fn request_set_clipboard(&self, window: xlib::Window) {
        unsafe { xlib::XSetSelectionOwner(self.display, self.clipboard_atom, window, xlib::CurrentTime); }
    }

    pub fn request_get_clipboard(&self, window: xlib::Window, atom: xlib::Atom) {
        self.req_targets.store(atom == self.targets_atom, Ordering::SeqCst);
        unsafe { xlib::XConvertSelection(self.display, self.clipboard_atom, atom, self.clipboard_atom, window, xlib::CurrentTime) };
        unsafe { xlib::XFlush(self.display); };
    }

    pub fn handle_request(&self, event: &xlib::XEvent) -> UiResult<()> {
        let xsr = unsafe { event.selection_request };
        let mut xs = unsafe { mem::zeroed::<xlib::XSelectionEvent>() };
        xs.type_ = xlib::SelectionNotify;
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
                xlib::XChangeProperty(self.display, xsr.requestor, xsr.property, xlib::XA_ATOM, 32,
                                      xlib::PropModeReplace, data.as_ptr() as *const u8, data.len() as i32);
            }
        } else if xsr.target == self.utf8_atom {
            let cstr = CString::new("Hello Rust Clipboard")?;
            unsafe {
                //返回文本数据
                xlib::XChangeProperty(self.display, xsr.requestor, xsr.property, xlib::XA_ATOM, 8,
                                      xlib::PropModeReplace, cstr.as_bytes().as_ptr(), cstr.as_bytes().len() as i32);
            }
        } else {
            xs.property = 0; //不支持的格式
        }
        let mut out_event = unsafe { mem::zeroed::<xlib::XEvent>() };
        out_event.selection = xs;
        unsafe { xlib::XSendEvent(self.display, xsr.requestor, 0, 0, &mut out_event); }
        unsafe { xlib::XFlush(self.display); }
        Ok(())
    }

    pub fn get_clipboard_data(&self, event: xlib::XEvent, window: xlib::Window) -> UiResult<ClipboardData> {
        let xs = unsafe { event.selection };
        if xs.property == 0 { return Ok(ClipboardData::Unsupported); }
        let mut actual_atom: c_ulong = 0;
        let mut actual_format: c_int = 0;
        let mut nitems: c_ulong = 0;
        let mut bytes_after: u64 = 0;
        let mut prop: *mut u8 = null_mut();
        let status = unsafe {
            xlib::XGetWindowProperty(
                self.display, window, self.clipboard_atom,
                0, i64::MAX, 0, xlib::AnyPropertyType as c_ulong,
                &mut actual_atom, &mut actual_format, &mut nitems, &mut bytes_after, &mut prop,
            )
        };
        if status != xlib::Success as i32 || prop.is_null() { return Err("获取粘贴板失败".into()); }
        match self.req_targets.load(Ordering::SeqCst) {
            true => {
                let atoms = unsafe { slice::from_raw_parts(prop as *const xlib::Atom, nitems as usize) };
                if atoms.contains(&self.png_atom) {
                    self.request_get_clipboard(window, self.png_atom);
                } else if atoms.contains(&self.url_atom) {
                    self.request_get_clipboard(window, self.url_atom);
                } else if atoms.contains(&self.text_atom) {
                    self.request_get_clipboard(window, self.text_atom);
                } else if atoms.contains(&self.timestamp_atom) {
                    self.request_get_clipboard(window, self.timestamp_atom);
                } else if atoms.contains(&self.utf8_atom) {
                    self.request_get_clipboard(window, self.utf8_atom)
                }
                println!("{}-{}-{}-{}-{:?}", self.utf8_atom, self.text_atom, self.url_atom, self.png_atom, atoms);
            }
            false => {
                let len = match actual_format {
                    8 => nitems as usize,
                    16 => nitems as usize * 2,
                    32 => nitems as usize * 4,
                    _ => nitems as usize
                };
                let slice = unsafe { slice::from_raw_parts(prop, len) }.to_owned();
                println!("get_clipboard-{}-{}-{}-{}-{}", self.utf8_atom, actual_atom, actual_format, self.url_atom, self.utf8_atom == actual_atom);
                if actual_atom == self.utf8_atom || actual_atom == self.text_atom {
                    let result = String::from_utf8(slice)?;
                    println!("get_clipboard-{}-{}-{}-{:?}", self.utf8_atom, actual_atom, actual_format, result);
                    unsafe { xlib::XFree(prop as *mut c_void); }
                    return Ok(ClipboardData::Text(result));
                } else if actual_atom == self.png_atom {
                    return Ok(ClipboardData::Image(slice))
                } else if actual_atom == self.url_atom {
                    let result = String::from_utf8(slice)?;
                    println!("get_clipboard-{}-{}-{}-{:?}", self.utf8_atom, actual_atom, actual_format, result);
                    return Ok(ClipboardData::Url(result));
                } else if actual_atom == self.timestamp_atom {
                    println!("{:?} {:?}", slice, String::from_utf8_lossy(slice.as_slice()));
                }
            }
        }
        Ok(ClipboardData::Unsupported)
    }
}

#[derive(Debug)]
pub enum ClipboardData {
    Unsupported,
    Text(String),
    Image(Vec<u8>),
    Url(String),
}


