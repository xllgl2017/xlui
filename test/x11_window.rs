use std::ffi::CString;
use std::mem;
use std::ptr::null_mut;
use x11::xlib;
use x11::xlib::{ButtonPressMask, ExposureMask, FocusChangeMask, XBlackPixel, XCreateSimpleWindow, XDefaultScreen, XLookupKeysym, XMapWindow, XOpenDisplay, XRootWindow, XSelectInput, XStoreName, XWhitePixel};

mod x11_ime;

#[link(name = "X11")]
unsafe extern {}


fn main() {
    unsafe {
        let display = XOpenDisplay(null_mut());
        if display.is_null() {
            panic!("XOpenDisplay failed");
        }
        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);
        let window = XCreateSimpleWindow(
            display, root, 100, 100, 500, 300, 1,
            XBlackPixel(display, screen), XWhitePixel(display, screen));
        let events = xlib::ExposureMask
            | FocusChangeMask
            | xlib::KeyPressMask
            | xlib::KeyReleaseMask
            | xlib::ButtonPressMask
            | xlib::ButtonReleaseMask
            | xlib::PointerMotionMask
            | xlib::StructureNotifyMask;
        XSelectInput(display, window, events);
        XMapWindow(display, window);
        let mut x = 100.0;
        let mut y = 100.0;
        // let bus = Bus::new("input ctx lel").unwrap();
        // let ctx = bus.ctx(); //bus.create_input_context("input ctx lel").unwrap();
        // ctx.set_capabilities(Capabilities::PreeditText | Capabilities::Focus).unwrap();
        // //
        // ctx.on_update_preedit_text(|s, _, _| {
        //     println!("preedit: {:?}", s);
        //     true
        // }).unwrap();
        // ctx.on_commit_text(|s, _, _| {
        //     println!("commit: {:?}", s);
        //     true
        // }).unwrap();

        // ctx.focus_in().unwrap();
        let mut child_window =0;
        loop {
            let mut event = mem::zeroed();
            xlib::XNextEvent(display, &mut event);
            // bus.process(std::time::Duration::from_secs(0)).unwrap();
            match event.get_type() {
                xlib::Expose => {}
                xlib::FocusIn => {
                    // self.ime.focus_in();
                    println!("focus in window");
                }
                xlib::FocusOut => {
                    // self.ime.focus_out();
                    println!("focus out");
                }
                xlib::ButtonPress => {
                    x += 50.0;
                    y += 50.0;
                    if event.expose.window==window {
                        println!("press root");
                    }else if event.expose.window==child_window {
                        println!("press child");
                    }
                    // ctx.set_cursor_location(x as i32, y as i32, 1, 1).unwrap();
                }
                xlib::KeyPress => {
                    child_window = XCreateSimpleWindow(
                        display,
                        root, // 父窗口
                        100, 100,
                        100, 80,       // 子窗口大小
                        1,
                        XBlackPixel(display, screen),   // 边框颜色
                        0xff0000,                       // 红色背景
                    );

                    let child_title = CString::new("动态子窗口").unwrap();
                    XStoreName(display, child_window, child_title.as_ptr());

                    XSelectInput(display, child_window, ExposureMask | ButtonPressMask);
                    XMapWindow(display, child_window);
                    let s = XLookupKeysym(&mut event.key, 0);
                    println!("kpress-{}-{}", s, event.key.keycode);
                    // ibus_input_context_process_key_event(g_context, s as u32, event.key.keycode, 0);
                    // ctx.process_key_event(s as u32, 50, Modifiers::Empty).unwrap();
                }
                xlib::KeyRelease => {
                    let s = XLookupKeysym(&mut event.key, 0);
                    // ibus_input_context_process_key_event(g_context, s as u32, event.key.keycode, 1 << 30);
                    // ctx.process_key_event(s as u32, event.key.keycode, Modifiers::Release).unwrap();
                }
                _ => {}
            }
            // bus.process(std::time::Duration::from_secs(0)).unwrap();
        }
    }
}
