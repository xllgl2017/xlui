use std::ffi::CString;
use std::marker::{PhantomData, PhantomPinned};
use std::ptr;
use x11::xlib::*;

#[link(name = "X11")]
unsafe extern "C" {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Debug, Copy)]
pub enum FontSlant {
    Normal = 0,
    Italic = 1,
    Oblique = 2,
}

pub enum FontWeight {
    Normal = 0,
    Bold = 1,
}

unsafe extern "C" {
    fn cairo_surface_reference(surface: *mut CairoSurface) -> *mut CairoSurface;
    fn cairo_create(target: *mut CairoSurface) -> *mut Cairo;
    fn cairo_new_path(cr: *mut Cairo);
    fn cairo_arc(cr: *mut Cairo, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64);
    fn cairo_close_path(cr: *mut Cairo);
    fn cairo_xlib_surface_create(dpy: *mut Display, drawable: Drawable, visual: *mut Visual, width: i32, height: i32) -> *mut CairoSurface;
    fn cairo_surface_flush(surface: *mut CairoSurface);
    fn cairo_set_source_rgba(cr: *mut Cairo, red: f64, green: f64, blue: f64, alpha: f64);
    fn cairo_fill(cr: *mut Cairo);
    fn cairo_fill_preserve(cr: *mut Cairo);
    fn cairo_set_line_width(cr: *mut Cairo, width: f64);
    fn cairo_stroke(cr: *mut Cairo);
    fn cairo_select_font_face(
        cr: *mut Cairo,
        family: *const i8,
        slant: i32,
        weight: i32,
    );
    fn cairo_set_font_size(
        cr: *mut Cairo,
        size: f64,
    );
    fn cairo_move_to(
        cr: *mut Cairo,
        x: f64,
        y: f64,
    );
    fn cairo_show_text(
        cr: *mut Cairo,
        utf8: *const i8,
    );
}

#[repr(C)]
pub struct Cairo {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

impl Cairo {
    pub fn new<'a>(surface: *mut CairoSurface) -> Option<&'a mut Cairo> {
        let cairo = unsafe { cairo_create(surface) };
        unsafe { cairo.as_mut() }
    }

    pub fn arc(&mut self, xc: f64, yc: f64, radius: f64, angle1: f64, angle2: f64) {
        unsafe { cairo_arc(self, xc, yc, radius, angle1, angle2) }
    }

    pub fn close_path(&mut self) {
        unsafe { cairo_close_path(self) }
    }

    pub fn new_path(&mut self) {
        unsafe { cairo_new_path(self) }
    }

    pub fn fill_preserve(&mut self) {
        unsafe { cairo_fill_preserve(self) }
    }

    pub fn set_line_width(&mut self, width: f64) {
        unsafe { cairo_set_line_width(self, width) }
    }

    pub fn stroke(&mut self) {
        unsafe { cairo_stroke(self) }
    }

    pub fn set_source_rgba(&mut self, red: f64, green: f64, blue: f64, alpha: f64) {
        unsafe { cairo_set_source_rgba(self, red, green, blue, alpha) }
    }

    pub fn select_font_face(&mut self, family: &str, slant: FontSlant, weight: FontWeight) {
        let family = CString::new(family).unwrap();
        unsafe { cairo_select_font_face(self, family.as_ptr(), slant as i32, weight as i32); }
    }

    pub fn set_font_size(&mut self, size: f64) {
        unsafe { cairo_set_font_size(self, size) }
    }

    pub fn move_to(&mut self, x: f64, y: f64) {
        unsafe { cairo_move_to(self, x, y) }
    }

    pub fn show_text(&mut self, utf8: &str) {
        let text = CString::new(utf8).unwrap();
        unsafe { cairo_show_text(self, text.as_ptr()) }
    }
}

#[repr(C)]
pub struct CairoSurface {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[link(name = "cairo")]
unsafe extern "C" {}


fn main() {
    unsafe {
        let display = XOpenDisplay(ptr::null());
        if display.is_null() {
            panic!("Cannot open display");
        }
        let screen = XDefaultScreen(display);
        let root = XRootWindow(display, screen);

        let window = XCreateSimpleWindow(
            display,
            root,
            100,
            100,
            400,
            300,
            1,
            XBlackPixel(display, screen),
            XWhitePixel(display, screen),
        );

        XSelectInput(display, window, ExposureMask | KeyPressMask);
        XMapWindow(display, window);
        let surface = cairo_xlib_surface_create(display, window, XDefaultVisual(display, screen), 400, 300);
        let cr = Cairo::new(surface).unwrap();

        loop {
            let mut event: XEvent = std::mem::zeroed();
            XNextEvent(display, &mut event);

            match event.get_type() {
                Expose => {
                    draw_text(cr, "test 中文", 100.0, 100.0, 14.0);

                    // draw_rounded_rect(cr, 50.0, 50.0, 300.0, 200.0, 30.0);
                    cairo_surface_flush(surface);
                }
                KeyPress => break,
                _ => {}
            }
        }

        XDestroyWindow(display, window);
        XCloseDisplay(display);
    }
}

fn draw_text(cr: &mut Cairo, text: &str, x: f64, y: f64, font_size: f64) {
    cr.select_font_face("仿宋", FontSlant::Normal, FontWeight::Normal);
    cr.set_font_size(font_size);
    cr.set_source_rgba(0.0, 0.0, 0.0, 1.0);
    cr.move_to(x, y);
    cr.show_text(text);
}

/// Cairo 绘制抗锯齿圆角矩形
fn draw_rounded_rect(cr: &mut Cairo, x: f64, y: f64, width: f64, height: f64, radius: f64) {
    let x1 = x;
    let y1 = y;
    let x2 = x + width;
    let y2 = y + height;
    cr.new_path();
    cr.arc(x2 - radius, y1 + radius, radius, -90_f64.to_radians(), 0_f64.to_radians());
    cr.arc(x2 - radius, y2 - radius, radius, 0_f64.to_radians(), 90_f64.to_radians());
    cr.arc(x1 + radius, y2 - radius, radius, 90_f64.to_radians(), 180_f64.to_radians());
    cr.arc(x1 + radius, y1 + radius, radius, 180_f64.to_radians(), 270_f64.to_radians());
    cr.close_path();
    cr.set_source_rgba(0.5, 0.3, 0.2, 0.4);
    cr.fill_preserve();
    cr.set_line_width(2.0);
    cr.set_source_rgba(0.5, 0.8, 0.6, 0.7);
    cr.stroke();
    // cr.new_path();
    // unsafe {
    //     cairo_new_path(cr);
    //     // cairo_arc(cr, x1, y1, x2, y2, radius);
    //     cairo_arc(cr, x2 - radius, y1 + radius, radius, -90_f64.to_radians(), 0_f64.to_radians());
    //     cairo_arc(cr, x2 - radius, y2 - radius, radius, 0_f64.to_radians(), 90_f64.to_radians());
    //     cairo_arc(cr, x1 + radius, y2 - radius, radius, 90_f64.to_radians(), 180_f64.to_radians());
    //     cairo_arc(cr, x1 + radius, y1 + radius, radius, 180_f64.to_radians(), 270_f64.to_radians());
    //     cairo_close_path(cr);
    //     cairo_set_source_rgba(cr, 0.5, 0.3, 0.2, 0.4);
    //     cairo_fill_preserve(cr);
    //     cairo_set_line_width(cr, 2.0);
    //     cairo_set_source_rgba(cr, 0.5, 0.8, 0.6, 0.7);
    //     cairo_stroke(cr);
    //
    //
    //     // cairo_fill(cr);
    //     // cr.set_source_rgb(0.5, 0.3, 0.2);
    //     // cr.cairo();
    // }
    // cr.arc(x2 - radius, y1 + radius, radius, -90_f64.to_radians(), 0_f64.to_radians());
    // cr.arc(x2 - radius, y2 - radius, radius, 0_f64.to_radians(), 90_f64.to_radians());
    // cr.arc(x1 + radius, y2 - radius, radius, 90_f64.to_radians(), 180_f64.to_radians());
    // cr.arc(x1 + radius, y1 + radius, radius, 180_f64.to_radians(), 270_f64.to_radians());

    // cr.close_path();

    // cr.set_source_rgb(0.5, 0.3, 0.2);
    // cr.fill();
}