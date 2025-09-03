use std::ffi::{c_char, c_short, c_void, CStr, CString};
use std::ptr::null_mut;
use x11::xlib::{Display, Window, XCreateIC, XFree, XIMCallback, XIMPreeditCallbacks, XIMPreeditDrawCallbackStruct, XIMPreeditNothing, XIMPreeditPosition, XIMStatusNothing, XNClientWindow_0, XNFocusWindow_0, XNInputStyle_0, XNPreeditAttributes_0, XNPreeditCaretCallback_0, XNPreeditDoneCallback_0, XNPreeditDrawCallback_0, XNPreeditStartCallback_0, XNSpotLocation_0, XOpenIM, XPoint, XPointer, XSetICFocus, XSetICValues, XSetLocaleModifiers, XVaCreateNestedList, XIC, XIM};

extern "C" fn preedit_start_cb(xic: XIM, client_data: XPointer, call_data: XPointer) {
    println!("preedit_start_cb");
}
extern "C" fn preedit_done_cb(xic: XIM, client_data: XPointer, call_data: XPointer) {
    println!("preedit_done_cb");
}
extern "C" fn preedit_draw_cb(xic: XIM, client_data: XPointer, call_data: XPointer) {
    println!("preedit_draw_cb");
    let client_data = unsafe { (client_data as *mut IMEData).as_mut() }.unwrap();
    let call_data = unsafe { (call_data as *mut XIMPreeditDrawCallbackStruct).as_mut() }.unwrap();
    if call_data.text.is_null() { return; }
    let xim_text = unsafe { &mut *(call_data.text) };
    let new_text = unsafe { CStr::from_ptr(xim_text.string.multi_byte) };
    client_data.text = new_text.to_string_lossy().chars().collect();
    println!("new text: {}", new_text.to_string_lossy());
}
extern "C" fn preedit_caret_cb(xic: XIM, client_data: XPointer, call_data: XPointer) {
    println!("preedit_caret_cb");
}

pub struct X11IME {
    xim: XIM,
    xic: XIC,
    data: *mut IMEData,
}

impl X11IME {
    pub fn new(display: *mut Display, window: Window) -> X11IME {
        // let p = CString::new("").unwrap();
        // unsafe { libc::setlocale(LC_ALL, p.as_ptr()); }
        let modifier = CString::new("").unwrap();
        unsafe { XSetLocaleModifiers(modifier.as_ptr()); }
        let xim = unsafe { XOpenIM(display, null_mut(), null_mut(), null_mut()) };
        if xim.is_null() { panic!("XOpenIM error"); }
        let ime_data = Box::into_raw(Box::new(IMEData { text: vec![] }));
        let attrs = X11IME::gen_ime_attr(ime_data as XPointer);
        let xic = unsafe {
            XCreateIC(
                xim,
                XNInputStyle_0.as_ptr() as *const c_char, XIMPreeditNothing | XIMStatusNothing,
                XNClientWindow_0.as_ptr() as *const c_char, window,
                XNFocusWindow_0.as_ptr() as *const c_char, window,
                XNPreeditAttributes_0.as_ptr() as *const c_char, attrs,
                null_mut::<c_char>(),
            )
        };
        X11IME {
            xim,
            xic,
            data: ime_data,
        }
    }


    pub fn gen_ime_attr(ime_data: XPointer) -> *mut c_void {
        let point = Box::into_raw(Box::new(XPoint { x: 100, y: 100 }));
        let preedit_start_callback = Box::into_raw(Box::new(XIMCallback { client_data: ime_data, callback: Some(preedit_start_cb) }));
        let preedit_done_callback = Box::into_raw(Box::new(XIMCallback { client_data: ime_data, callback: Some(preedit_done_cb) }));
        let preedit_draw_callback = Box::into_raw(Box::new(XIMCallback { client_data: ime_data, callback: Some(preedit_draw_cb) }));
        let preedit_caret_callback = Box::into_raw(Box::new(XIMCallback { client_data: ime_data, callback: Some(preedit_caret_cb) }));
        let attrs = unsafe {
            XVaCreateNestedList(
                0,
                XNSpotLocation_0.as_ptr() as *const c_char, point,
                XNPreeditStartCallback_0.as_ptr() as *const c_char, preedit_start_callback,
                XNPreeditDoneCallback_0.as_ptr() as *const c_char, preedit_done_callback,
                XNPreeditDrawCallback_0.as_ptr() as *const c_char, preedit_draw_callback,
                XNPreeditCaretCallback_0.as_ptr() as *const c_char, preedit_caret_callback,
                null_mut::<c_char>(),
            )
        };

        attrs
    }

    pub fn request_focus(&self) {
        unsafe { XSetICFocus(self.xic); }
    }

    pub fn set_position(&self, x: f32, y: f32) {
        println!("ime: {}-{}", x, y);
        let point = Box::into_raw(Box::new(XPoint { x: x as c_short, y: y as c_short }));
        let attr = unsafe { XVaCreateNestedList(0, XNSpotLocation_0.as_ptr() as *const c_char, point, null_mut::<c_char>()) };
        unsafe { XSetICValues(self.xic, XNPreeditAttributes_0.as_ptr() as *const c_char, attr, null_mut::<c_char>()) };
        unsafe { XFree(attr); }
    }

    pub fn ime_chars(&self) -> Vec<char> {
        let data = unsafe { self.data.as_mut() }.unwrap();
        data.text.clone()
    }
}


struct IMEData {
    text: Vec<char>,
}
