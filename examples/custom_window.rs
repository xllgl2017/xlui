use std::ffi::CString;
use std::os::raw::{c_long, c_ulong};
use std::ptr;
use x11::xlib;
use x11::xlib::*;
#[link(name = "X11")]
unsafe extern "C" {}

#[repr(C)]
struct MotifWmHints {
    flags: c_ulong,
    functions: c_ulong,
    decorations: c_ulong,
    input_mode: c_long,
    status: c_ulong,
}

const MWM_HINTS_DECORATIONS: c_ulong = 1 << 1;

fn main() {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("Cannot open display");
        }
        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);

        // 查找 32 位 ARGB visual
        let mut vinfo: XVisualInfo = std::mem::zeroed();
        vinfo.screen = screen;
        vinfo.depth = 32;
        vinfo.class = xlib::TrueColor;
        let mut n = 0;
        let vinfo = XGetVisualInfo(
            display,
            VisualScreenMask | VisualDepthMask | VisualClassMask,
            &mut vinfo,
            &mut n,
        );
        if vinfo.is_null() {
            panic!("No ARGB visual found");
        }
        // vinfo.visual = vlist.visual;
        // vinfo = *vlist;
        // let vis = vinfo.visual;
        // XFree(vlist as _);
        let colormap = XCreateColormap(display, root, (*vinfo).visual, AllocNone);

        let mut swa: XSetWindowAttributes = std::mem::zeroed();
        swa.colormap = colormap;
        swa.border_pixel = 0;
        swa.background_pixel = 0; //xlib::XWhitePixel(display,screen);

        let win = XCreateWindow(
            display,
            root,
            100, 100, 400, 300,
            0,
            (*vinfo).depth,
            InputOutput as u32,
            (*vinfo).visual,
            CWColormap | CWBorderPixel | CWBackPixel,
            &mut swa,
        );

        let title = CString::new("No Border / No Shadow Window").unwrap();
        XStoreName(display, win, title.as_ptr());

        // ========= 去掉装饰 =========
        // let motif_hints_atom = XInternAtom(display, b"_MOTIF_WM_HINTS\0".as_ptr() as *const i8, False);
        // let hints = MotifWmHints {
        //     flags: MWM_HINTS_DECORATIONS,
        //     functions: 0,
        //     decorations: 0, // 0 = no border, no title bar
        //     input_mode: 0,
        //     status: 0,
        // };
        // XChangeProperty(
        //     display,
        //     win,
        //     motif_hints_atom,
        //     motif_hints_atom,
        //     32,
        //     PropModeReplace,
        //     &hints as *const _ as *const u8,
        //     5,
        // );

        // ========= 禁用阴影 =========
        // let no_shadow: c_ulong = 0;
        // let shadow_atom =
        //     XInternAtom(display, b"_COMPTON_SHADOW\0".as_ptr() as *const i8, False);
        // XChangeProperty(
        //     display,
        //     win,
        //     shadow_atom,
        //     XA_CARDINAL,
        //     32,
        //     PropModeReplace,
        //     &no_shadow as *const _ as *const u8,
        //     1,
        // );

        // 显示窗口
        XMapWindow(display, win);
        XFlush(display);

        let mut event: XEvent = std::mem::zeroed();
        loop {
            XNextEvent(display, &mut event);
        }
    }
}
