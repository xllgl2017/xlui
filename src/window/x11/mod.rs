use crate::error::UiResult;
use crate::key::Key;
use crate::window::event::WindowEvent;
use crate::window::ime::{IMEData, IME};
use crate::window::x11::clipboard::X11ClipBoard;
use crate::window::x11::handle::X11WindowHandle;
use crate::window::x11::ime::flag::Modifiers;
use crate::window::{WindowId, WindowKind, WindowType};
use crate::{Pos, Size, WindowAttribute};
use std::ffi::CString;
use std::os::raw::{c_long, c_ulong};
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};
use std::{mem, ptr};
use x11::xlib;
use x11::xlib::{AllocNone, XCloseDisplay, XLookupString};

pub mod ime;
pub mod handle;
mod clipboard;

#[repr(C)]
struct X11WmHints {
    flags: c_ulong,
    functions: c_ulong,
    decorations: c_ulong,
    input_mode: c_long,
    status: c_ulong,
}

pub struct X11Window {
    display: *mut xlib::Display,
    handles: Vec<Arc<WindowType>>,
    wm_delete_atom: xlib::Atom,
    size: RwLock<Size>,
    root: xlib::Window,
    visual_info: xlib::XVisualInfo,
}

impl X11Window {
    pub fn new(attr: &WindowAttribute, ime: Arc<IME>) -> Result<Self, String> {
        unsafe {
            if xlib::XInitThreads() == 0 { return Err("XInitThreads failed".into()); }
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() { return Err("Cannot open X display".into()); }
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
            let wm_delete = xlib::XInternAtom(display, b"WM_DELETE_WINDOW\0".as_ptr() as *const i8, 0);

            // 查找 32 位 ARGB visual
            let mut vinfo: xlib::XVisualInfo = std::mem::zeroed();
            vinfo.screen = screen;
            vinfo.depth = 32;
            vinfo.class = xlib::TrueColor;
            let mut n = 0;
            let vinfo = xlib::XGetVisualInfo(
                display,
                xlib::VisualScreenMask | xlib::VisualDepthMask | xlib::VisualClassMask,
                &mut vinfo,
                &mut n,
            );
            if vinfo.is_null() { return Err("No ARGB visual found".into()); }
            let p = CString::new("@im=none").unwrap();
            xlib::XSetLocaleModifiers(p.as_ptr());
            let mut res = Self {
                display,
                handles: vec![],
                wm_delete_atom: wm_delete,
                size: RwLock::new(attr.inner_size),
                root,
                visual_info: *vinfo,
            };
            xlib::XFree(vinfo as *mut _);
            res.init(attr, ime, screen);
            Ok(res)
        }
    }

    fn init(&mut self, attr: &WindowAttribute, ime: Arc<IME>, screen: i32) {
        let handle = self.create_window(screen, attr);
        let window = WindowType {
            kind: WindowKind::X11(handle),
            id: WindowId::unique_id(),
            type_: WindowType::ROOT,
            ime,
        };
        self.handles.push(Arc::new(window));
    }


    pub fn last_window(&self) -> Arc<WindowType> {
        self.handles.last().cloned().unwrap()
    }

    fn create_window(&mut self, screen: i32, attr: &WindowAttribute) -> X11WindowHandle {
        unsafe {
            let colormap = xlib::XCreateColormap(self.display, self.root, self.visual_info.visual, AllocNone);
            let mut swa: xlib::XSetWindowAttributes = std::mem::zeroed();
            swa.colormap = colormap;
            swa.border_pixel = if attr.transparent { 0 } else { xlib::XWhitePixel(self.display, screen) };
            swa.background_pixel = if attr.transparent { 0 } else { xlib::XWhitePixel(self.display, screen) }; //xlib::XWhitePixel(display,screen);
            let child = xlib::XCreateWindow(
                self.display,
                self.root,
                attr.position[0], attr.position[1], attr.inner_size.width, attr.inner_size.height,
                1,
                self.visual_info.depth,
                xlib::InputOutput as u32,
                self.visual_info.visual,
                xlib::CWColormap | xlib::CWBorderPixel | xlib::CWBackPixel,
                &mut swa,
            );
            let c_title = CString::new(attr.title.clone()).unwrap();
            xlib::XStoreName(self.display, child, c_title.as_ptr());
            // ========= 去掉装饰 =========
            let motif_hints_atom = xlib::XInternAtom(self.display, b"_MOTIF_WM_HINTS\0".as_ptr() as *const i8, xlib::False);
            let hints = X11WmHints {
                flags: 1 << 1,
                functions: 0,
                decorations: if attr.decorations { 1 } else { 0 }, // 0 = no border, no title bar
                input_mode: 0,
                status: 0,
            };
            xlib::XChangeProperty(
                self.display,
                child,
                motif_hints_atom,
                motif_hints_atom,
                32,
                xlib::PropModeReplace,
                &hints as *const _ as *const u8,
                5,
            );
            let events = xlib::ExposureMask
                | xlib::FocusChangeMask
                | xlib::KeyPressMask
                | xlib::KeyReleaseMask
                | xlib::ButtonPressMask
                | xlib::ButtonReleaseMask
                | xlib::PointerMotionMask
                | xlib::StructureNotifyMask;
            xlib::XSelectInput(self.display, child, events as i64);
            let mut attrs: xlib::XSetWindowAttributes = mem::zeroed();
            attrs.background_pixel = 0;
            xlib::XChangeWindowAttributes(self.display, child, xlib::CWBackPixmap, &mut attrs);
            xlib::XSetWindowBackgroundPixmap(self.display, child, 0); // 0 == None
            xlib::XMapWindow(self.display, child);
            xlib::XFlush(self.display);
            xlib::XSetWMProtocols(self.display, child, &mut self.wm_delete_atom, 1);
            X11WindowHandle {
                display: self.display,
                window: child,
                update_atom: 0,
                screen,
                clipboard: X11ClipBoard::new(self.display).unwrap(),
            }
        }
    }

    pub fn create_child_window(&mut self, parent: &Arc<WindowType>, attr: &WindowAttribute) -> UiResult<Arc<WindowType>> {
        let mut window = self.create_window(parent.x11().screen, attr);
        window.update_atom = parent.x11().update_atom;
        let window = Arc::from(WindowType {
            id: WindowId::unique_id(),
            kind: WindowKind::X11(window),
            type_: WindowType::CHILD,
            ime: parent.ime.clone(),
        });
        self.handles.push(window.clone());
        Ok(window)
    }

    pub fn run(&mut self) -> (WindowId, WindowEvent) {
        unsafe {
            let mut event: xlib::XEvent = mem::zeroed();
            xlib::XNextEvent(self.display, &mut event);
            let window = self.handles.iter().find(|x| x.x11().window == event.expose.window);
            if window.is_none() { return (WindowId::unique_id(), WindowEvent::None); }
            let window = window.unwrap();
            window.ime.update_working();
            if window.ime.is_working() { window.ime.update(); }
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
                            2 => (window.id, WindowEvent::ReInit),
                            3 => (window.id, WindowEvent::UserUpdate),
                            _ => (window.id, WindowEvent::None)
                        };
                    } else if xclient.data.get_long(0) as xlib::Atom == self.wm_delete_atom {
                        let pos = self.handles.iter().position(|x| x.x11().window == event.expose.window).unwrap();
                        let window = self.handles.remove(pos);
                        return (window.id, WindowEvent::ReqClose);
                    }
                }
                xlib::KeyPress => {
                    let mut keysym = 0;
                    let mut buffer: [i8; 32] = [0; 32];
                    // let mut keysym = xlib::XLookupKeysym(&mut event.key, 0);
                    let len = XLookupString(&mut event.key, buffer.as_mut_ptr(), 32, &mut keysym, null_mut());
                    let handle = window.ime.post_key(keysym as u32, event.key.keycode, Modifiers::Empty).unwrap();
                    println!("press-handle-{}-{}-{}", handle, window.ime.is_commited(), keysym);
                    return if handle {
                        window.ime.update();
                        (window.id, WindowEvent::IME(IMEData::Preedit(window.ime.chars())))
                    } else {
                        let ctrl_press = (event.key.state & xlib::ControlMask) != 0;
                        if ctrl_press && keysym == x11::keysym::XK_c as u64 {
                            return (window.id, WindowEvent::KeyPress(Key::CtrlC));
                        } else if ctrl_press && (keysym == x11::keysym::XK_v as u64) {
                            return (window.id, WindowEvent::KeyPress(Key::CtrlV));
                        }


                        (window.id, WindowEvent::KeyPress(Key::from_c_ulong(event.key.keycode, &buffer[..len as usize])))
                    };
                }
                xlib::KeyRelease => {
                    let mut keysym = 0;
                    // let keysym = xlib::XLookupKeysym(&mut event.key, 0);
                    let mut buffer: [i8; 32] = [0; 32];
                    let len = XLookupString(&mut event.key, buffer.as_mut_ptr(), 32, &mut keysym, null_mut());
                    let handle = window.ime.post_key(keysym as u32, event.key.keycode, Modifiers::Release).unwrap();
                    println!("release-handle-{}-{}", handle, window.ime.is_commited());
                    if !handle {
                        if window.ime.is_commited() {
                            return (window.id, WindowEvent::IME(IMEData::Commit(window.ime.ime_done())));
                        }
                        let ctrl_press = (event.key.state & xlib::ControlMask) != 0;
                        if ctrl_press && keysym == x11::keysym::XK_c as u64 {
                            return (window.id, WindowEvent::None);
                        } else if ctrl_press && (keysym == x11::keysym::XK_v as u64) {
                            return (window.id, WindowEvent::None);
                        } else if ctrl_press {
                            return (window.id, WindowEvent::None);
                        }
                        return (window.id, WindowEvent::KeyRelease(Key::from_c_ulong(event.key.keycode, &buffer[..len as usize])));
                    }
                }
                xlib::ButtonRelease => {
                    // window.x11().clipboard.request_get_clipboard(window.x11().window, window.x11().clipboard.utf8_atom);
                    let xb: xlib::XButtonEvent = event.button;
                    match xb.button {
                        1 => return (window.id, WindowEvent::MouseRelease(Pos { x: xb.x as f32, y: xb.y as f32 })),
                        2 => {} //鼠标中间键
                        3 => {} //鼠标右键
                        4 => return (window.id, WindowEvent::MouseWheel(1.0)), //向上滚动
                        5 => return (window.id, WindowEvent::MouseWheel(-1.0)), //向下动
                        _ => {}
                    }
                }
                xlib::ButtonPress => {
                    let xb: xlib::XButtonEvent = event.button;
                    match xb.button {
                        1 => return (window.id, WindowEvent::MousePress(Pos { x: xb.x as f32, y: xb.y as f32 })),
                        _ => {}
                    }
                }
                xlib::MotionNotify => {
                    let xm: xlib::XMotionEvent = event.motion;
                    return if window.ime.is_commited() {
                        (window.id, WindowEvent::IME(IMEData::Commit(window.ime.ime_done())))
                    } else {
                        (window.id, WindowEvent::MouseMove(Pos { x: xm.x as f32, y: xm.y as f32 }))
                    };
                }
                xlib::SelectionRequest => window.x11().clipboard.handle_request(&event).unwrap(),
                xlib::SelectionNotify => {
                    let res = window.x11().clipboard.get_clipboard_data(event, window.x11().window).unwrap();
                    println!("clipboard_res: {:?}", res);
                    return (window.id, WindowEvent::Clipboard(res));
                }
                _ => {}
            }
            if window.ime.is_working() { window.ime.update(); }
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

