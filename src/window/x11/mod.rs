use crate::error::UiResult;
use crate::key::Key;
use crate::window::event::WindowEvent;
use crate::window::ime::{IMEData, IME};
use crate::window::x11::handle::X11WindowHandle;
use crate::window::x11::ime::flag::{Capabilities, Modifiers};
use crate::window::{WindowId, WindowKind, WindowType};
use crate::{App, Pos, Size, WindowAttribute};
use std::ffi::CString;
use std::os::raw::{c_long, c_uint, c_ulong};
use std::ptr::null_mut;
use std::sync::{Arc, RwLock};
use std::{mem, ptr};
use std::process::exit;
use x11::xlib;
use x11::xlib::{AllocNone, XCloseDisplay, XLookupString};
use crate::map::Map;
use crate::window::wino::{EventLoopHandle, LoopWindow};

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
    windows: Map<WindowId, LoopWindow>,
    wm_delete_atom: xlib::Atom,
    size: RwLock<Size>,
    root: xlib::Window,
    visual_info: xlib::XVisualInfo,
    modify_keys: Vec<c_uint>,
}

impl X11Window {
    pub fn new<A: App>(app: A) -> UiResult<Self> {
        let ime = Arc::new(IME::new_x11("xlui ime"));
        ime.set_capabilities(Capabilities::PreeditText | Capabilities::Focus);
        let ii = ime.clone();
        ime.create_binding(ii);
        let attr = app.window_attributes();
        unsafe {
            if xlib::XInitThreads() == 0 { return Err("XInitThreads failed".into()); }
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() { return Err("Cannot open X display".into()); }
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);
            let wm_delete = xlib::XInternAtom(display, b"WM_DELETE_WINDOW\0".as_ptr() as *const i8, 0);

            // 查找 32 位 ARGB visual
            let mut vinfo: xlib::XVisualInfo = mem::zeroed();
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
                windows: Map::new(),
                wm_delete_atom: wm_delete,
                size: RwLock::new(attr.inner_size),
                root,
                visual_info: *vinfo,
                modify_keys: vec![],
            };
            xlib::XFree(vinfo as *mut _);
            let handle = res.init(&attr, ime, screen)?;
            #[cfg(not(feature = "gpu"))]
            let window = LoopWindow::create_native_window(Box::new(app), Arc::new(handle), attr);
            #[cfg(feature = "gpu")]
            let window = pollster::block_on(async { LoopWindow::create_gpu_window(Box::new(app), Arc::new(handle), attr).await });
            res.windows.insert(window.window_id(), window);
            Ok(res)
        }
    }

    fn init(&mut self, attr: &WindowAttribute, ime: Arc<IME>, screen: i32) -> UiResult<WindowType> {
        let handle = self.create_window(screen, attr)?;
        let window = WindowType {
            kind: WindowKind::X11(handle),
            id: WindowId::unique_id(),
            type_: WindowType::ROOT,
            ime,
        };
        window.x11().set_size(attr.inner_size);
        Ok(window)
    }

    fn create_window(&mut self, screen: i32, attr: &WindowAttribute) -> UiResult<X11WindowHandle> {
        unsafe {
            let colormap = xlib::XCreateColormap(self.display, self.root, self.visual_info.visual, AllocNone);
            let mut swa: xlib::XSetWindowAttributes = std::mem::zeroed();
            swa.colormap = colormap;
            swa.border_pixel = if attr.transparent { 0 } else { xlib::XWhitePixel(self.display, screen) };
            swa.background_pixel = if attr.transparent { 0 } else { xlib::XWhitePixel(self.display, screen) }; //xlib::XWhitePixel(display,screen);
            let child = xlib::XCreateWindow(
                self.display,
                self.root,
                attr.position[0], attr.position[1], attr.inner_size.width_u32(), attr.inner_size.height_u32(),
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
            X11WindowHandle::new(self.display, child, 0, screen)
        }
    }

    pub fn create_child_window(&mut self, parent: &Arc<WindowType>, app: Box<dyn App>) -> UiResult<()> {
        let attr = app.window_attributes();
        let mut handle = self.create_window(parent.x11().screen, &attr)?;
        handle.update_atom = parent.x11().update_atom;
        let window = Arc::from(WindowType {
            id: WindowId::unique_id(),
            kind: WindowKind::X11(handle),
            type_: WindowType::CHILD,
            ime: parent.ime.clone(),
        });
        #[cfg(not(feature = "gpu"))]
        let window = LoopWindow::create_native_window(app, window, attr);
        #[cfg(feature = "gpu")]
        let window = pollster::block_on(async { LoopWindow::create_gpu_window(app, window, attr).await });
        self.windows.insert(window.window_id(), window);
        Ok(())
    }

    pub fn run(&mut self) -> UiResult<()> {
        loop {
            unsafe {
                let mut event: xlib::XEvent = mem::zeroed();
                xlib::XNextEvent(self.display, &mut event);
                let window = match self.windows.iter_mut().find(|x| x.handle().x11().window == event.expose.window) {
                    None => continue,
                    Some(window) => window,
                };
                window.handle().ime.update_working();
                if window.handle().ime.is_working() { window.handle().ime.update(); }
                let typ = event.get_type();
                match typ {
                    xlib::Expose => window.handle_event(WindowEvent::Redraw),
                    xlib::FocusIn => {
                        window.handle().ime.focus_in();
                        println!("focus in window");
                    }
                    xlib::FocusOut => {
                        window.handle().ime.focus_out();
                        println!("focus out");
                    }
                    xlib::ConfigureNotify => {
                        let xcfg: xlib::XConfigureEvent = event.configure;
                        let new_w = xcfg.width as f32;
                        let new_h = xcfg.height as f32;
                        let mut size = self.size.write().unwrap();
                        if new_w == 0.0 || new_h == 0.0 {
                            // ignore weird zero sizes
                        } else if new_w != size.width || new_h != size.height {
                            size.width = new_w;
                            size.height = new_h;
                            println!("resize {}-{}-{}-{}", xcfg.width, xcfg.height, new_w, new_h);
                            window.handle_event(WindowEvent::Resize(size.clone()));
                        }
                    }
                    xlib::ClientMessage => {
                        // Check for WM_DELETE_WINDOW
                        let xclient: xlib::XClientMessageEvent = event.client_message;
                        if xclient.message_type == window.handle().x11().update_atom {
                            match xclient.data.get_long(0) {
                                0 => window.handle_event(WindowEvent::ReqUpdate),
                                1 => window.handle_event(WindowEvent::CreateChild),
                                2 => window.handle_event(WindowEvent::ReInit),
                                3 => window.handle_event(WindowEvent::UserUpdate),
                                _ => {}
                            };
                        } else if xclient.data.get_long(0) as xlib::Atom == self.wm_delete_atom {
                            let wid = self.windows.iter().find(|x| x.handle().x11().window == event.expose.window);
                            if let Some(wid) = wid { self.windows.remove(&wid.window_id()); }
                            if self.windows.len() == 0 { exit(0); }
                            continue;
                        }
                    }
                    xlib::KeyPress => {
                        let mut keysym = 0;
                        let mut buffer: [i8; 32] = [0; 32];
                        // let mut keysym = xlib::XLookupKeysym(&mut event.key, 0);
                        let len = XLookupString(&mut event.key, buffer.as_mut_ptr(), 32, &mut keysym, null_mut());
                        let handle = window.handle().ime.post_key(keysym as u32, event.key.keycode, Modifiers::Empty).unwrap();
                        println!("press-handle-{}-{}-{}", handle, window.handle().ime.is_commited(), keysym);
                        if handle {
                            window.handle().ime.update();
                            window.handle_event(WindowEvent::IME(IMEData::Preedit(window.handle().ime.chars())))
                        } else {
                            let ctrl_press = (event.key.state & xlib::ControlMask) != 0;
                            if ctrl_press && keysym == x11::keysym::XK_c as u64 {
                                self.modify_keys.push(x11::keysym::XK_c);
                                window.handle_event(WindowEvent::KeyPress(Key::CtrlC));
                            } else if ctrl_press && (keysym == x11::keysym::XK_v as u64) {
                                self.modify_keys.push(x11::keysym::XK_v);
                                window.handle_event(WindowEvent::KeyPress(Key::CtrlV));
                            } else if ctrl_press && keysym == x11::keysym::XK_x as u64 {
                                self.modify_keys.push(x11::keysym::XK_x);
                                window.handle_event(WindowEvent::KeyPress(Key::CtrlX));
                            } else if ctrl_press && keysym == x11::keysym::XK_a as u64 {
                                self.modify_keys.push(x11::keysym::XK_a);
                                window.handle_event(WindowEvent::KeyPress(Key::CtrlA));
                            }


                            window.handle_event(WindowEvent::KeyPress(Key::from_c_ulong(event.key.keycode, &buffer[..len as usize])))
                        };
                    }
                    xlib::KeyRelease => {
                        let mut keysym = 0;
                        // let keysym = xlib::XLookupKeysym(&mut event.key, 0);
                        let mut buffer: [i8; 32] = [0; 32];
                        let len = XLookupString(&mut event.key, buffer.as_mut_ptr(), 32, &mut keysym, null_mut());
                        if let Some(pos) = self.modify_keys.iter().position(|x| *x == keysym as u32) {
                            self.modify_keys.remove(pos);
                            continue;
                        }
                        let handle = window.handle().ime.post_key(keysym as u32, event.key.keycode, Modifiers::Release).unwrap();
                        println!("release-handle-{}-{}", handle, window.handle().ime.is_commited());
                        if !handle {
                            if window.handle().ime.is_commited() {
                                window.handle_event(WindowEvent::IME(IMEData::Commit(window.handle().ime.ime_done())));
                            }
                            let ctrl_press = (event.key.state & xlib::ControlMask) != 0;
                            // if ctrl_press && keysym == x11::keysym::XK_c as u64 {
                            //     window.handle_event(WindowEvent::None);
                            // } else if ctrl_press && (keysym == x11::keysym::XK_v as u64) {
                            //     window.handle_event(WindowEvent::None);
                            // } else if ctrl_press {
                            //     window.handle_event(WindowEvent::None);
                            // }
                            if !ctrl_press {
                                window.handle_event(WindowEvent::KeyRelease(Key::from_c_ulong(event.key.keycode, &buffer[..len as usize])));
                            }
                        }
                    }
                    xlib::ButtonRelease => {
                        // window.x11().clipboard.request_get_clipboard(window.x11().window, window.x11().clipboard.utf8_atom);
                        let xb: xlib::XButtonEvent = event.button;
                        match xb.button {
                            1 => window.handle_event(WindowEvent::MouseRelease(Pos { x: xb.x as f32, y: xb.y as f32 })),
                            2 => {} //鼠标中间键
                            3 => {} //鼠标右键
                            4 => window.handle_event(WindowEvent::MouseWheel(1.0)), //向上滚动
                            5 => window.handle_event(WindowEvent::MouseWheel(-1.0)), //向下动
                            _ => {}
                        }
                    }
                    xlib::ButtonPress => {
                        let xb: xlib::XButtonEvent = event.button;
                        match xb.button {
                            1 => window.handle_event(WindowEvent::MousePress(Pos { x: xb.x as f32, y: xb.y as f32 })),
                            _ => {}
                        }
                    }
                    xlib::MotionNotify => {
                        let xm: xlib::XMotionEvent = event.motion;
                        if window.handle().ime.is_commited() {
                            window.handle_event(WindowEvent::IME(IMEData::Commit(window.handle().ime.ime_done())))
                        } else {
                            let mut x: i32 = 0;
                            let mut y: i32 = 0;
                            xlib::XQueryPointer(self.display, self.root, &mut event.button.root, &mut event.button.subwindow, &mut x, &mut y, &mut event.button.x, &mut event.button.y, &mut event.button.state);
                            window.handle_event(WindowEvent::MouseMove((xm.x, xm.y, x, y).into()))
                        };
                    }
                    xlib::SelectionRequest => window.handle().x11().clipboard.handle_request(&event).unwrap(),
                    xlib::SelectionNotify => {
                        let res = window.handle().x11().clipboard.get_clipboard_data(event, window.handle().x11().window).unwrap();
                        println!("clipboard_res: {:?}", res);
                        window.handle_event(WindowEvent::Clipboard(res));
                    }
                    _ => {}
                }
                if window.handle().ime.is_working() { window.handle().ime.update(); }
            }
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

