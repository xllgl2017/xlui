use std::ffi::CString;
use std::ptr;
use x11::xft::{XftColor, XftColorAllocValue, XftDraw, XftDrawCreate, XftDrawDestroy, XftDrawStringUtf8, XftFont, XftFontClose, XftFontOpenName};
use x11::xlib::*;
use x11::xrender::XRenderColor;

#[link(name = "X11")]
unsafe extern "C" {}

#[link(name = "Xft")]
unsafe extern "C" {}

fn create_x11_window() -> (*mut Display, Window, i32) {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("Failed to open X display");
        }

        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);
        let white = XWhitePixel(display, screen);
        let black = XBlackPixel(display, screen);

        let win = XCreateSimpleWindow(
            display,
            root,
            0,
            0,
            800,
            600,
            1,
            black,
            white,
        );

        XStoreName(display, win, b"X11 + Xft Text\0".as_ptr() as *const i8);
        XSelectInput(display, win, ExposureMask | KeyPressMask);
        XMapWindow(display, win);
        (display, win, screen)
    }
}


fn create_xft_resources(display: *mut Display, win: Window) -> (*mut XftDraw, *mut XftFont, XftColor) {
    unsafe {
        let screen = XDefaultScreen(display);
        let visual = XDefaultVisual(display, screen);
        let colormap = XDefaultColormap(display, screen);

        // 创建 XftDraw 对象
        let draw = XftDrawCreate(display, win, visual, colormap);
        if draw.is_null() {
            panic!("Failed to create XftDraw");
        }
        let font = CString::new("仿宋").unwrap();
        // 加载字体
        let font = XftFontOpenName(display, screen, font.as_ptr() as *const i8);
        if font.is_null() {
            panic!("Failed to open font");
        }

        // 创建颜色（黑色）
        let mut xft_color: XftColor = std::mem::zeroed();
        let mut render_color: XRenderColor = XRenderColor {
            red: 0x0000,
            green: 0x0000,
            blue: 0x0000,
            alpha: 0xffff,
        };

        if XftColorAllocValue(display, visual, colormap, &mut render_color, &mut xft_color) == 0 {
            panic!("Failed to alloc color");
        }

        (draw, font, xft_color)
    }
}


fn draw_text(display: *mut Display, draw: *mut XftDraw, font: *mut XftFont, color: &mut XftColor, x: i32, y: i32, text: &str) {
    unsafe {
        let c_str = std::ffi::CString::new(text).unwrap();
        XftDrawStringUtf8(
            draw,
            color,
            font,
            x,
            y,
            c_str.as_ptr() as *const u8,
            text.len() as i32,
        );
        XFlush(display);
    }
}

fn set_gc_color(display: *mut Display, gc: GC, screen: i32, r: u8, g: u8, b: u8) {
    unsafe {
        let cmap = XDefaultColormap(display, screen);
        let mut color: XColor = std::mem::zeroed();
        let name = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let cname = CString::new(name).unwrap();
        // Alloc named color; if fail, fallback to simple pixel
        if XParseColor(display, cmap, cname.as_ptr() as *const i8, &mut color) != 0 {
            XAllocColor(display, cmap, &mut color);
            XSetForeground(display, gc, color.pixel);
        } else {
            // fallback: compose pixel from r/g/b into 24-bit value (may not be perfect)
            let pixel = ((r as u32) << 16) | ((g as u32) << 8) | (b as u32);
            XSetForeground(display, gc, pixel.into());
        }
    }
}

/// Draw a filled rounded rectangle on `drawable` using `gc`.
/// x,y : top-left, w/h : size, rx/ry : corner radius (in pixels)
unsafe fn fill_rounded_rect(display: *mut Display, drawable: Drawable, gc: GC, x: i32, y: i32, w: u32, h: u32, rx: u32, ry: u32) {
    let rx = rx.min(w / 2);
    let ry = ry.min(h / 2);

    // center rect
    if w > 2 * rx && h > 2 * ry {
        XFillRectangle(display, drawable, gc, x + rx as i32, y + ry as i32, w - 2 * rx, h - 2 * ry);
    }

    // top rect
    if w > 2 * rx {
        XFillRectangle(display, drawable, gc, x + rx as i32, y, w - 2 * rx, ry);
        // bottom rect
        XFillRectangle(display, drawable, gc, x + rx as i32, y + (h - ry) as i32, w - 2 * rx, ry);
    }

    // left rect
    if h > 2 * ry {
        XFillRectangle(display, drawable, gc, x, y + ry as i32, rx, h - 2 * ry);
        // right rect
        XFillRectangle(display, drawable, gc, x + (w - rx) as i32, y + ry as i32, rx, h - 2 * ry);
    }

    // four corner arcs (filled)
    // XFillArc expects width,height for the full ellipse; start angle in 64ths of degrees
    if rx > 0 && ry > 0 {
        // top-left
        XFillArc(
            display,
            drawable,
            gc,
            x,
            y,
            2 * rx,
            2 * ry,
            90 * 64,
            90 * 64,
        );
        // top-right
        XFillArc(
            display,
            drawable,
            gc,
            x + (w - 2 * rx) as i32,
            y,
            2 * rx,
            2 * ry,
            0 * 64,
            90 * 64,
        );
        // bottom-right
        XFillArc(
            display,
            drawable,
            gc,
            x + (w - 2 * rx) as i32,
            y + (h - 2 * ry) as i32,
            2 * rx,
            2 * ry,
            270 * 64,
            90 * 64,
        );
        // bottom-left
        XFillArc(
            display,
            drawable,
            gc,
            x,
            y + (h - 2 * ry) as i32,
            2 * rx,
            2 * ry,
            180 * 64,
            90 * 64,
        );
    }
}

/// Draw rounded rectangle with border by painting outer rounded rect with border color,
/// then inset and paint inner rounded rect with fill color (carving out the center).
unsafe fn draw_rounded_rect_with_border(
    display: *mut Display,
    drawable: Drawable,
    screen: i32,
    gc_border: GC,
    gc_fill: GC,
    x: i32,
    y: i32,
    w: u32,
    h: u32,
    rx: u32,
    ry: u32,
    border: u32,
) {
    if w == 0 || h == 0 {
        return;
    }
    // draw outer (border color) as filled rounded rect
    fill_rounded_rect(display, drawable, gc_border, x, y, w, h, rx, ry);

    // compute inner rect (inset by border)
    if border == 0 {
        return;
    }
    let inset = border as i32;
    let ix = x + inset;
    let iy = y + inset;
    let iw = if (w as i32 - 2 * inset) > 0 {
        (w as i32 - 2 * inset) as u32
    } else {
        0
    };
    let ih = if (h as i32 - 2 * inset) > 0 {
        (h as i32 - 2 * inset) as u32
    } else {
        0
    };

    if iw > 0 && ih > 0 {
        // inner radii = outer radii - inset (not less than 0)
        let irx = if rx as i32 - inset > 0 { (rx as i32 - inset) as u32 } else { 0 };
        let iry = if ry as i32 - inset > 0 { (ry as i32 - inset) as u32 } else { 0 };
        // paint inner with fill color -> leaves border visible
        fill_rounded_rect(display, drawable, gc_fill, ix, iy, iw, ih, irx, iry);
    }
}

fn main() {
    let (display, win, screen) = create_x11_window();
    let (draw, font, mut color) = create_xft_resources(display, win);
    let gc_border = unsafe { XCreateGC(display, win, 0, ptr::null_mut()) };
    let gc_fill = unsafe { XCreateGC(display, win, 0, ptr::null_mut()) };
    // background GC if needed
    let gc_bg = unsafe { XCreateGC(display, win, 0, ptr::null_mut()) };

    set_gc_color(display, gc_border, screen, 0x1f, 0x77, 0xb4); // border color (blue-ish)
    set_gc_color(display, gc_fill, screen, 0xff, 0xff, 0xff);   // fill color (white)
    set_gc_color(display, gc_bg, screen, 0xff, 0xff, 0xff);     // background (same as fill here)

    unsafe {
        let mut event: XEvent = std::mem::zeroed();
        loop {
            XNextEvent(display, &mut event);

            match event.get_type() {
                Expose => {
                    // draw_text(display, draw, font, &mut color, 50, 100, "Hello X11 + Xft 中文测试");
                    XFillRectangle(
                        display,
                        win,
                        gc_bg,
                        0,
                        0,
                        100 as u32,
                        100 as u32,
                    );

                    set_gc_color(display, gc_border, screen, 0x2c, 0xa0, 0x2c); // green border
                    set_gc_color(display, gc_fill, screen, 0xf8, 0xff, 0xf8);   // fill slightly off-white
                    draw_rounded_rect_with_border(
                        display,
                        win,
                        screen,
                        gc_border,
                        gc_fill,
                        300, 60,
                        240, 180,
                        40, 40,
                        4,
                    );


                    XFlush(display);
                }
                KeyPress => break,
                _ => {}
            }
        }

        // 释放资源
        XftDrawDestroy(draw);
        XftFontClose(display, font);
        XCloseDisplay(display);
    }
}
