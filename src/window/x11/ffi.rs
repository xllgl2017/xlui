use std::ffi::CString;
use std::marker::{PhantomData, PhantomPinned};
use std::mem::MaybeUninit;
use x11::xlib::{Display, Drawable, Visual};

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
    fn cairo_select_font_face(cr: *mut Cairo, family: *const i8, slant: i32, weight: i32);
    fn cairo_set_font_size(cr: *mut Cairo, size: f64);
    fn cairo_move_to(cr: *mut Cairo, x: f64, y: f64);
    fn cairo_show_text(cr: *mut Cairo, utf8: *const i8);
    fn cairo_font_extents(cr: *mut Cairo, extents: *mut CairoFontExtents);
    fn cairo_text_extents(cr: *mut Cairo, utf8: *const i8, extents: *mut CairoTextExtents);
    fn cairo_rectangle(cr: *mut Cairo, x: f64, y: f64, width: f64, height: f64);
    fn cairo_clip(cr: *mut Cairo);
    fn cairo_reset_clip(cr: *mut Cairo);
    fn cairo_line_to(cr: *mut Cairo, x: f64, y: f64);
    fn cairo_set_source_surface(cr: *mut Cairo, surface: *mut CairoSurface, x: f64, y: f64);
    fn cairo_image_surface_create_for_data(data: *mut u8, format: i32, width: i32, height: i32, stride: i32) -> *mut CairoSurface;
    fn cairo_save(cr: *mut Cairo);
    fn cairo_restore(cr: *mut Cairo);
    fn cairo_scale(cr: *mut Cairo, sx: f64, sy: f64);
    fn cairo_paint(cr: *mut Cairo);
    fn cairo_translate(cr: *mut Cairo, tx: f64, ty: f64);
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Format {
//     Invalid = -1,
//     ARgb32 = 0,
//     Rgb24 = 1,
//     A8 = 2,
//     A1 = 3,
//     Rgb16_565 = 4,
//     Rgb30 = 5,
//
// }

#[repr(C)]
pub struct CairoFontExtents {
    pub ascent: f64,
    pub descent: f64,
    pub height: f64,
    pub max_x_advance: f64,
    pub max_y_advance: f64,
}

#[repr(C)]
pub struct CairoTextExtents {
    pub x_bearing: f64,
    pub y_bearing: f64,
    pub width: f64,
    pub height: f64,
    pub x_advance: f64,
    pub y_advance: f64,
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

    pub fn show_text(&mut self, utf8: impl AsRef<str>) {
        let utf8 = CString::new(utf8.as_ref().to_string()).unwrap();
        unsafe { cairo_show_text(self, utf8.as_ptr()) }
    }

    pub fn font_extends(&mut self) -> CairoFontExtents {
        let mut extents = MaybeUninit::uninit();
        unsafe {
            cairo_font_extents(self, extents.as_mut_ptr());
            extents.assume_init()
        }
    }

    pub fn text_extents(&mut self, utf8: impl AsRef<str>) -> CairoTextExtents {
        let utf8 = CString::new(utf8.as_ref().to_string()).unwrap();
        let mut extents = MaybeUninit::uninit();
        unsafe {
            cairo_text_extents(self, utf8.as_ptr(), extents.as_mut_ptr());
            extents.assume_init()
        }
    }

    pub fn rectangle(&mut self, x: f64, y: f64, width: f64, height: f64) {
        unsafe { cairo_rectangle(self, x, y, width, height) }
    }

    pub fn clip(&mut self) {
        unsafe { cairo_clip(self) }
    }

    pub fn reset_clip(&mut self) {
        unsafe { cairo_reset_clip(self) }
    }

    pub fn line_to(&mut self, x: f64, y: f64) {
        unsafe { cairo_line_to(self, x, y) }
    }

    pub fn set_source_surface(&mut self, surface: *mut CairoSurface, x: f64, y: f64) {
        unsafe { cairo_set_source_surface(self, surface, x, y) }
    }

    pub fn save(&mut self) {
        unsafe { cairo_save(self) }
    }

    pub fn restore(&mut self) {
        unsafe { cairo_restore(self) }
    }

    pub fn scale(&mut self, sx: f64, sy: f64) {
        unsafe { cairo_scale(self, sx, sy) }
    }

    pub fn paint(&mut self) {
        unsafe { cairo_paint(self) }
    }

    pub fn translate(&mut self, tx: f64, ty: f64) {
        unsafe { cairo_translate(self, tx, ty) }
    }
}

#[repr(C)]
pub struct CairoSurface {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

impl CairoSurface {
    pub fn new(display: *mut Display, win: Drawable, visual: *mut Visual, width: i32, height: i32) -> *mut CairoSurface {
        unsafe { cairo_xlib_surface_create(display, win, visual, width, height) }
    }

    pub fn new_image(img: *mut u8, w: i32, h: i32) -> *mut CairoSurface {
        unsafe { cairo_image_surface_create_for_data(img, 0, w, h, w * 4) }
    }

    pub fn flush(&mut self) {
        unsafe { cairo_surface_flush(self) }
    }
}