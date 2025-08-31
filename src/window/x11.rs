use crate::window::event::WindowEvent;
use crate::window::WindowId;
use crate::{Pos, Size};
use raw_window_handle::*;
use std::ffi::c_void;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::RwLock;
use std::{mem, ptr};
use x11::xlib;
use x11::xlib::{Atom, XInitThreads};

pub struct X11Window {
    pub(crate) display: *mut xlib::Display,
    window: xlib::Window,
    screen: i32,
    pub(crate) wm_delete_atom: Atom,
    size: RwLock<Size>,
    id: WindowId,
    resized: AtomicBool,
    update_atom: Atom,
}

impl X11Window {
    pub fn new(size: Size, title: &str) -> Result<Self, String> {
        unsafe {
            if XInitThreads() == 0 {
                panic!("XInitThreads failed");
            }
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err("Cannot open X display".into());
            }
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);

            let black = xlib::XBlackPixel(display, screen);
            let white = xlib::XWhitePixel(display, screen);

            let window = xlib::XCreateSimpleWindow(
                display,
                root,
                0,
                0,
                size.width,
                size.height,
                1,
                black,
                white,
            );

            // Set window title
            let c_title = std::ffi::CString::new(title).unwrap();
            xlib::XStoreName(display, window, c_title.as_ptr());

            // Select events: expose, key, mouse, structure notify (resize), pointer motion
            let events = xlib::ExposureMask
                | xlib::KeyPressMask
                | xlib::KeyReleaseMask
                | xlib::ButtonPressMask
                | xlib::ButtonReleaseMask
                | xlib::PointerMotionMask
                | xlib::StructureNotifyMask;
            xlib::XSelectInput(display, window, events as i64);

            // WM_DELETE_WINDOW
            // let wm_protocols = xlib::XInternAtom(display, b"WM_PROTOCOLS\0".as_ptr() as *const i8, 0);
            let wm_delete = xlib::XInternAtom(display, b"WM_DELETE_WINDOW\0".as_ptr() as *const i8, 0);
            xlib::XSetWMProtocols(display, window, &wm_delete as *const xlib::Atom as *mut xlib::Atom, 1);

            let mut attrs: xlib::XSetWindowAttributes = mem::zeroed();
            attrs.background_pixel = 0;
            xlib::XChangeWindowAttributes(display, window, xlib::CWBackPixmap, &mut attrs);
            xlib::XSetWindowBackgroundPixmap(display, window, 0); // 0 == None
            xlib::XMapWindow(display, window);
            xlib::XFlush(display);
            let update_atom = xlib::XInternAtom(display, b"MY_CUSTOM_MESSAGE\0".as_ptr() as *const i8, 0);

            Ok(Self {
                display,
                window,
                screen,
                wm_delete_atom: wm_delete,
                size: RwLock::new(size),
                id: WindowId(crate::unique_id_u32()),
                resized: AtomicBool::new(false),
                update_atom,
            })
        }
    }

    pub fn request_redraw(&self) {
        unsafe {
            xlib::XClearArea(self.display, self.window, 0, 0, 0, 0, xlib::True);
            xlib::XFlush(self.display);
        }
    }

    pub fn send_update(&self) {
        let mut event: xlib::XClientMessageEvent = unsafe { mem::zeroed() };
        event.type_ = xlib::ClientMessage;
        event.display = self.display;
        event.window = self.window;
        event.message_type = self.update_atom;
        event.format = 32;
        // event.data.set_long(0, data as i64);
        let mask = xlib::NoEventMask;
        unsafe { xlib::XSendEvent(self.display, self.window, 0, mask, &mut event as *mut _ as *mut _); }
        unsafe { xlib::XFlush(self.display); }
    }

    pub fn resized(&self) -> bool {
        let b = self.resized.load(Ordering::SeqCst);
        self.resized.store(false, Ordering::SeqCst);
        b
    }

    pub fn run(&self) -> WindowEvent {
        unsafe {
            // if xlib::XPending(self.display) <= 0 { return false; }
            let mut event: xlib::XEvent = std::mem::zeroed();
            xlib::XNextEvent(self.display, &mut event);
            let typ = event.get_type();
            match typ {
                xlib::Expose => return WindowEvent::Redraw,
                xlib::ConfigureNotify => {
                    let xcfg: xlib::XConfigureEvent = event.configure;
                    let new_w = xcfg.width as u32;
                    let new_h = xcfg.height as u32;
                    // let mut attrs: xlib::XWindowAttributes = std::mem::zeroed();
                    // xlib::XGetWindowAttributes(self.display, self.window, &mut attrs);
                    // let new_w = attrs.width as u32;
                    // let new_h = attrs.height as u32;

                    let mut size = self.size.write().unwrap();
                    if new_w == 0 || new_h == 0 {
                        // ignore weird zero sizes
                    } else if new_w != size.width || new_h != size.height {
                        size.width = new_w;
                        size.height = new_h;
                        println!("resize {}-{}-{}-{}", xcfg.width, xcfg.height, new_w, new_h);
                        return WindowEvent::Resize(size.clone());
                        // self.resized.store(true, Ordering::SeqCst);
                        // let _ = self.sender.try_send((self.id, WindowEvent::Resize(size.clone())));
                    }
                }
                xlib::ClientMessage => {
                    // Check for WM_DELETE_WINDOW
                    let xclient: xlib::XClientMessageEvent = event.client_message;
                    if xclient.message_type == self.update_atom { return WindowEvent::ReqUpdate; }
                    if xclient.data.get_long(0) as Atom == self.wm_delete_atom { return WindowEvent::ReqClose; }
                }
                xlib::KeyPress => {
                    // Map key to keysym
                    let xkey: xlib::XKeyEvent = event.key;
                    let ks = xlib::XLookupKeysym(&xkey as *const xlib::XKeyEvent as *mut _, 0);
                    // XK_Escape constant from x11 crate keysym
                    // if ks == x11::keysym::XK_Escape {
                    //     running = false;
                    // } else {
                    //     // print pressed key code/keysym for debug
                    //     eprintln!("KeyPress: keycode={} keysym={}", xkey.keycode, ks);
                    // }
                }
                xlib::ButtonRelease => {
                    let xb: xlib::XButtonEvent = event.button;
                    return WindowEvent::MouseRelease(Pos { x: xb.x as f32, y: xb.y as f32 });
                    // self.sender.send((self.id, WindowEvent::MouseRelease(Pos { x: xb.x as f32, y: xb.y as f32 }))).unwrap();
                    // eprintln!("Mouse Release {} at ({}, {})", xb.button, xb.x, xb.y);
                }
                xlib::ButtonPress => {
                    let xb: xlib::XButtonEvent = event.button;
                    return WindowEvent::MousePress(Pos { x: xb.x as f32, y: xb.y as f32 });
                    // self.sender.try_send((self.id, WindowEvent::MousePress(Pos { x: xb.x as f32, y: xb.y as f32 })));
                    // eprintln!("Mouse Press {} at ({}, {})", xb.button, xb.x, xb.y);
                }
                xlib::MotionNotify => {
                    let xm: xlib::XMotionEvent = event.motion;
                    return WindowEvent::MouseMove(Pos { x: xm.x as f32, y: xm.y as f32 });
                    // self.sender.send((self.id, WindowEvent::MouseMove(Pos { x: xm.x as f32, y: xm.y as f32 }))).unwrap();
                    // eprintln!("Mouse move: ({}, {})", xm.x, xm.y);
                }
                _ => {}
            }
        }
        WindowEvent::None
    }

    pub fn size(&self) -> Size {
        *self.size.read().unwrap()
    }

    pub fn id(&self) -> WindowId {
        self.id
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
}

impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
            xlib::XCloseDisplay(self.display);
        }
    }
}

unsafe impl Send for X11Window {}
unsafe impl Sync for X11Window {}