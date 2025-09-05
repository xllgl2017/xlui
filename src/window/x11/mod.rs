use crate::key::Key;
use crate::window::event::WindowEvent;
use crate::window::ime::IME;
use crate::window::x11::ime::flag::Modifiers;
use crate::window::{WindowId, WindowKind, WindowType};
use crate::{Pos, Size, WindowAttribute};
use raw_window_handle::*;
use std::ffi::{c_void, CString};
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};
use std::{mem, ptr};
use std::os::raw::c_long;
use x11::xlib;
use x11::xlib::XCloseDisplay;

pub mod ime;

pub enum UserEvent {
    ReqUpdate = 0,
    CreateChild = 1,
    IMECommit = 2,
}

pub struct X11WindowType {
    display: *mut xlib::Display,
    window: xlib::Window,
    update_atom: xlib::Atom,
    screen: i32,
}


impl X11WindowType {
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

impl Drop for X11WindowType {
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
        }
    }
}

unsafe impl Send for X11WindowType {}
unsafe impl Sync for X11WindowType {}


pub struct X11Window {
    display: *mut xlib::Display,
    windows: Vec<Arc<WindowType>>,
    wm_delete_atom: xlib::Atom,

    size: RwLock<Size>,
    root: xlib::Window,
}

impl X11Window {
    pub fn new(attr: &WindowAttribute, ime: Arc<IME>) -> Result<Self, String> {
        unsafe {
            if xlib::XInitThreads() == 0 {
                panic!("XInitThreads failed");
            }
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err("Cannot open X display".into());
            }
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
            let mut wm_delete = xlib::XInternAtom(display, b"WM_DELETE_WINDOW\0".as_ptr() as *const i8, 0);
            let mut window = X11Window::create_window(display, screen, root, attr, &mut wm_delete);

            // WM_DELETE_WINDOW
            // let wm_protocols = xlib::XInternAtom(display, b"WM_PROTOCOLS\0".as_ptr() as *const i8, 0);
            let update_atom = xlib::XInternAtom(display, b"MY_CUSTOM_MESSAGE\0".as_ptr() as *const i8, 0);
            window.update_atom = update_atom;
            let p = CString::new("@im=none").unwrap();
            xlib::XSetLocaleModifiers(p.as_ptr());
            let window = WindowType {
                kind: WindowKind::X11(window),
                id: WindowId::unique_id(),
                type_: WindowType::ROOT,
                ime,
            };
            Ok(Self {
                display,
                windows: vec![Arc::new(window)],
                wm_delete_atom: wm_delete,
                size: RwLock::new(attr.inner_size),
                root,
            })
        }
    }


    pub fn last_window(&self) -> Arc<WindowType> {
        self.windows.last().cloned().unwrap()
    }

    fn create_window(display: *mut xlib::Display, screen: i32, root: xlib::Window, attr: &WindowAttribute, wm_delete: &mut xlib::Atom) -> X11WindowType {
        unsafe {
            let child = xlib::XCreateSimpleWindow(
                display, root,
                attr.position[0], attr.position[1],
                attr.inner_size.width, attr.inner_size.height,
                1, // 边框宽度
                xlib::XBlackPixel(display, screen),
                xlib::XWhitePixel(display, screen),
            );
            let c_title = CString::new(attr.title.clone()).unwrap();
            xlib::XStoreName(display, child, c_title.as_ptr());
            let events = xlib::ExposureMask
                | xlib::FocusChangeMask
                | xlib::KeyPressMask
                | xlib::KeyReleaseMask
                | xlib::ButtonPressMask
                | xlib::ButtonReleaseMask
                | xlib::PointerMotionMask
                | xlib::StructureNotifyMask;
            xlib::XSelectInput(display, child, events as i64);
            let mut attrs: xlib::XSetWindowAttributes = mem::zeroed();
            attrs.background_pixel = 0;
            xlib::XChangeWindowAttributes(display, child, xlib::CWBackPixmap, &mut attrs);
            xlib::XSetWindowBackgroundPixmap(display, child, 0); // 0 == None
            // xlib::XMapWindow(display, root);
            xlib::XMapWindow(display, child);
            xlib::XFlush(display);
            xlib::XSetWMProtocols(display, child, wm_delete, 1);
            X11WindowType {
                display,
                window: child,
                update_atom: 0,
                screen,
            }
        }
    }

    pub fn create_child_window(&mut self, parent: &Arc<WindowType>, attr: &WindowAttribute) -> Arc<WindowType> {
        let mut window = X11Window::create_window(
            parent.x11().display, parent.x11().screen, self.root, attr,
            &mut self.wm_delete_atom);
        window.update_atom = parent.x11().update_atom;
        let window = Arc::from(WindowType {
            id: WindowId::unique_id(),
            kind: WindowKind::X11(window),
            type_: WindowType::CHILD,
            ime: parent.ime.clone(),
        });
        self.windows.push(window.clone());
        window
    }

    pub fn run(&mut self) -> (WindowId, WindowEvent) {
        unsafe {
            let mut event: xlib::XEvent = mem::zeroed();
            xlib::XNextEvent(self.display, &mut event);
            let window = self.windows.iter().find(|x| x.x11().window == event.expose.window);
            if window.is_none() { return (WindowId::unique_id(), WindowEvent::None); }
            let window = window.unwrap();
            window.ime.update_working();
            if window.ime.is_available() && window.ime.is_working() { window.ime.update(); }
            let typ = event.get_type();
            match typ {
                xlib::Expose => return (window.id, WindowEvent::Redraw),
                xlib::FocusIn => {
                    window.ime.focus_in();
                    println!("focus in window");
                }
                xlib::FocusOut => {
                    window.ime.focus_out();
                    println!("focus out");
                }
                xlib::ConfigureNotify => {
                    let xcfg: xlib::XConfigureEvent = event.configure;
                    let new_w = xcfg.width as u32;
                    let new_h = xcfg.height as u32;
                    let mut size = self.size.write().unwrap();
                    if new_w == 0 || new_h == 0 {
                        // ignore weird zero sizes
                    } else if new_w != size.width || new_h != size.height {
                        size.width = new_w;
                        size.height = new_h;
                        println!("resize {}-{}-{}-{}", xcfg.width, xcfg.height, new_w, new_h);
                        return (window.id, WindowEvent::Resize(size.clone()));
                    }
                }
                xlib::ClientMessage => {
                    // Check for WM_DELETE_WINDOW
                    let xclient: xlib::XClientMessageEvent = event.client_message;
                    if xclient.message_type == window.x11().update_atom {
                        return match xclient.data.get_long(0) {
                            0 => (window.id, WindowEvent::ReqUpdate),
                            1 => (window.id, WindowEvent::CreateChild),
                            _ => (window.id, WindowEvent::None)
                        };
                    } else if xclient.data.get_long(0) as xlib::Atom == self.wm_delete_atom {
                        let pos = self.windows.iter().position(|x| x.x11().window == event.expose.window).unwrap();
                        let window = self.windows.remove(pos);
                        return (window.id, WindowEvent::ReqClose);
                    }
                }
                xlib::KeyPress => {
                    println!("key-press");
                    return match window.ime.is_available() && window.ime.is_working() {
                        true => {
                            let keysym = xlib::XLookupKeysym(&mut event.key, 0);
                            window.ime.post_key(keysym as u32, event.key.keycode, Modifiers::Empty).unwrap();
                            if window.ime.is_working() {
                                window.ime.update();
                                (window.id, WindowEvent::IME)
                            } else { (window.id, WindowEvent::KeyPress(Key::from_c_ulong(event.key.keycode))) }
                        }
                        false => (window.id, WindowEvent::KeyPress(Key::from_c_ulong(event.key.keycode)))
                    };
                }
                xlib::KeyRelease => {
                    println!("key-release");

                    match window.ime.is_available() {
                        true => {
                            if window.ime.is_working() {
                                let keysym = xlib::XLookupKeysym(&mut event.key, 0);
                                let handle = window.ime.post_key(keysym as u32, event.key.keycode, Modifiers::Release).unwrap();
                                println!("release-handle-{}-{}", handle, window.ime.is_commited());
                                if !handle && !window.ime.is_working() {
                                    let key = Key::from_c_ulong(event.key.keycode);
                                    return (window.id, WindowEvent::KeyRelease(key));
                                }
                            }
                        }
                        false => return (window.id, WindowEvent::KeyRelease(Key::from_c_ulong(event.key.keycode)))
                    }
                }
                xlib::ButtonRelease => {
                    let xb: xlib::XButtonEvent = event.button;
                    return (window.id, WindowEvent::MouseRelease(Pos { x: xb.x as f32, y: xb.y as f32 }));
                }
                xlib::ButtonPress => {
                    let xb: xlib::XButtonEvent = event.button;
                    return (window.id, WindowEvent::MousePress(Pos { x: xb.x as f32, y: xb.y as f32 }));
                }
                xlib::MotionNotify => {
                    let xm: xlib::XMotionEvent = event.motion;
                    return if window.ime.is_commited() {
                        (window.id, WindowEvent::IME)
                    } else {
                        (window.id, WindowEvent::MouseMove(Pos { x: xm.x as f32, y: xm.y as f32 }))
                    };
                }
                _ => {}
            }
            if window.ime.is_available() && window.ime.is_working() { window.ime.update(); }
            (window.id, WindowEvent::None)
        }
    }
}

impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            XCloseDisplay(self.display);
            let _ = Box::from_raw(self.display);
        }
    }
}

