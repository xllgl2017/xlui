#[cfg(not(feature = "gpu"))]
use crate::error::UiResult;
#[cfg(not(feature = "gpu"))]
use crate::render::image::texture::ImageTexture;
#[cfg(not(feature = "gpu"))]
use crate::text::cchar::LineChar;
#[cfg(not(feature = "gpu"))]
use crate::ui::PaintParam;
use crate::window::ime::IME;
use crate::window::x11::clipboard::X11ClipBoard;
#[cfg(not(feature = "gpu"))]
use crate::window::x11::ffi::CairoAntialias;
#[cfg(not(feature = "gpu"))]
use crate::window::x11::ffi::{Cairo, CairoSurface};
use crate::window::{ClipboardData, UserEvent};
use crate::*;
#[cfg(feature = "gpu")]
use raw_window_handle::{DisplayHandle, RawDisplayHandle, RawWindowHandle, WindowHandle, XlibDisplayHandle, XlibWindowHandle};
use std::cell::RefCell;
#[cfg(feature = "gpu")]
use std::ffi::c_void;
use std::mem;
use std::os::raw::c_long;
#[cfg(feature = "gpu")]
use std::ptr::NonNull;
use std::sync::{Arc, RwLock};
use x11::xlib;
use x11::xlib::{XFreeColormap, XMoveWindow};
use crate::size::font::{FontSlant, FontWeight};

pub struct X11WindowHandle {
    pub(crate) display: *mut xlib::Display,
    pub(crate) window: xlib::Window,
    pub(crate) update_atom: xlib::Atom,
    pub(crate) screen: i32,
    pub(crate) clipboard: X11ClipBoard,
    pub(crate) visual_info: xlib::XVisualInfo,
    pub(crate) size: RwLock<Size>,
    pub(crate) colormap: u64,
}


impl X11WindowHandle {
    pub fn request_redraw(&self) {
        let s = RefCell::new(12);
        *s.borrow_mut() = 10;
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

    #[cfg(feature = "gpu")]
    pub fn window_handle(&self) -> WindowHandle<'_> {
        let xlib_window_handle = XlibWindowHandle::new(self.window);
        let raw_window_handle = RawWindowHandle::Xlib(xlib_window_handle);
        unsafe { WindowHandle::borrow_raw(raw_window_handle) }
    }

    #[cfg(feature = "gpu")]
    pub fn display_handle(&self) -> DisplayHandle<'_> {
        let display = NonNull::new(self.display as *mut c_void);
        let x11_display_handle = XlibDisplayHandle::new(display, self.screen);
        let raw_display_handle = RawDisplayHandle::Xlib(x11_display_handle);
        unsafe { DisplayHandle::borrow_raw(raw_display_handle) }
    }

    /// * 设置输入法显示的位置
    /// * x,y为窗口内部的位置
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

    /// 请求系统粘贴板，需要指定类型
    pub fn request_clipboard(&self, clipboard: ClipboardData) {
        match clipboard {
            ClipboardData::Unsupported => {}
            ClipboardData::Text(_) => self.clipboard.request_get_clipboard(self.window, self.clipboard.utf8_atom),
            ClipboardData::Image(_) => self.clipboard.request_get_clipboard(self.window, self.clipboard.png_atom),
            ClipboardData::Url(_) => self.clipboard.request_get_clipboard(self.window, self.clipboard.url_atom)
        }
    }

    /// 设置系统粘贴板
    pub fn set_clipboard(&self, clipboard: ClipboardData) {
        self.clipboard.request_set_clipboard(self.window, clipboard);
    }

    ///* 移动窗口
    ///* x,y是窗口在桌面的位置
    pub fn move_window(&self, x: f32, y: f32) {
        unsafe { XMoveWindow(self.display, self.window, x as i32, y as i32) };
    }

    pub fn size(&self) -> Size {
        self.size.read().unwrap().clone()
    }

    pub fn set_size(&self, size: Size) {
        *self.size.write().unwrap() = size;
    }

    #[cfg(not(feature = "gpu"))]
    pub fn paint_text_by_cairo(&self, paint: &mut PaintParam, text: &RichText, lines: &Vec<LineChar>, rect: Rect, clip_x: f32, clip_y: f32) { //功能异常
        paint.cairo.save();
        paint.cairo.select_font_face(text.family.as_ref().unwrap(), FontSlant::Normal, FontWeight::Normal);
        paint.cairo.set_font_size(text.font_size() as f64);
        paint.cairo.set_source_rgba(text.color.r_f64(), text.color.g_f64(), text.color.b_f64(), text.color.a_f64());
        let font_extents = paint.cairo.font_extends();
        paint.cairo.rectangle(rect.dx().min as f64, rect.dy().min as f64 - font_extents.ascent, rect.width() as f64, rect.height() as f64 + font_extents.ascent + font_extents.descent);
        paint.cairo.clip();
        let x = (rect.dx().min + clip_x) as f64;
        let mut y = (rect.dy().min + clip_y) as f64 + font_extents.ascent;
        for line in lines {
            paint.cairo.move_to(x, y);
            paint.cairo.show_text(line.line_text.as_str());
            y += text.height as f64;
        }

        // paint.cairo.move_to(rect.dx().min as f64, rect.dy().min as f64 + font_extents.ascent);
        // paint.cairo.show_text(text.text.as_str());
        paint.cairo.reset_clip();
        paint.cairo.restore();
    }

    // #[cfg(not(feature = "gpu"))]
    // pub fn set_clip_rect(&self, paint: &mut PaintParam, rect: &Rect) {
    //     paint.cairo.rectangle(rect.dx().min as f64, rect.dy().min as f64, rect.width() as f64, rect.height() as f64);
    //     // unsafe { XftDrawSetClipRectangles(draw, 0, 0, &rect.as_x_rect() as *const XRectangle, 1); }
    // }


    #[cfg(not(feature = "gpu"))]
    pub fn paint_text(&self, paint: &mut PaintParam, text: &RichText, lines: &Vec<LineChar>, rect: Rect, clip_x: f32, clip_y: f32) -> UiResult<()> {
        return Ok(self.paint_text_by_cairo(paint, text, lines, rect, clip_x, clip_y));
        // unsafe {
        //     // let colormap = xlib::XCreateColormap(self.display, self.root, self.visual_info.visual, xlib::AllocNone);
        //     let font = CString::new(format!("{}:pixelsize={}", text.family.as_ref().unwrap(), text.font_size() as i32))?;
        //     // 加载字体
        //     let font = XftFontOpenName(self.display, self.screen, font.as_ptr() as *const i8);
        //     if font.is_null() { return Err("Failed to load font".into()); }
        //
        //     // 创建颜色（黑色）
        //     let mut xft_color: XftColor = mem::zeroed();
        //     let mut render_color: XRenderColor = XRenderColor {
        //         red: text.color.r as u16 * 257,
        //         green: text.color.g as u16 * 257,
        //         blue: text.color.b as u16 * 257,
        //         alpha: text.color.a as u16 * 257,
        //     };
        //
        //     if XftColorAllocValue(self.display, self.visual_info.visual, self.colormap, &mut render_color, &mut xft_color) == 0 {
        //         return Err("Failed to alloc color".into());
        //     }
        //     // 设置裁剪区域
        //     XftDrawSetClipRectangles(paint.draw, 0, 0, &rect.as_x_rect() as *const XRectangle, 1);
        //     let font_ascent = font.as_ref().ok_or("获取字体ascent失败")?.ascent;
        //     let line_height = text.height as i32;
        //     let x = (rect.dx().min + clip_x) as i32;
        //     let mut y = (rect.dy().min + clip_y) as i32 + font_ascent;
        //     for line in lines {
        //         let c_str = CString::new(line.line_text.clone())?;
        //         XftDrawStringUtf8(
        //             paint.draw, &mut xft_color, font,
        //             x, y,
        //             c_str.as_ptr() as *const u8,
        //             line.line_text.len() as i32,
        //         );
        //         y += line_height;
        //     }
        //     // 恢复裁剪（清空剪裁区域）
        //     XftDrawSetClip(paint.draw, std::ptr::null_mut());
        //     XftFontClose(self.display, font);
        //     Ok(())
        // }
    }


    #[cfg(not(feature = "gpu"))]
    pub fn paint_rect(&self, cairo: &mut Cairo, fill: &Color, border: &Border, rect: &Rect) {
        cairo.save();
        let x1 = rect.dx().min;
        let y1 = rect.dy().min;
        let x2 = rect.dx().max;
        let y2 = rect.dy().max;
        cairo.set_antialias(CairoAntialias::Best);
        cairo.new_path();
        cairo.arc(
            (x2 - border.radius.right_top as f32) as f64,
            (y1 + border.radius.right_top as f32) as f64,
            border.radius.right_top as f64,
            -90_f64.to_radians(), 0_f64.to_radians(),
        );
        cairo.arc(
            (x2 - border.radius.right_bottom as f32) as f64,
            (y2 - border.radius.right_bottom as f32) as f64,
            border.radius.right_bottom as f64,
            0_f64.to_radians(), 90_f64.to_radians());
        cairo.arc(
            (x1 + border.radius.left_bottom as f32) as f64,
            (y2 - border.radius.left_bottom as f32) as f64,
            border.radius.left_bottom as f64,
            90_f64.to_radians(), 180_f64.to_radians());
        cairo.arc(
            (x1 + border.radius.left_top as f32) as f64,
            (y1 + border.radius.left_top as f32) as f64,
            border.radius.left_top as f64,
            180_f64.to_radians(), 270_f64.to_radians());
        cairo.close_path();
        cairo.set_source_rgba(fill.r_f64(), fill.g_f64(), fill.b_f64(), fill.a_f64());
        cairo.fill_preserve();
        cairo.set_line_width(border.width() as f64);
        cairo.set_source_rgba(border.color.r_f64(), border.color.g_f64(), border.color.b_f64(), border.color.a_f64());
        cairo.stroke();
        cairo.restore();
    }

    #[cfg(not(feature = "gpu"))]
    pub fn paint_circle(&self, cairo: &mut Cairo, fill: &Color, border: &Border, rect: &Rect) {
        cairo.save();
        cairo.new_path();
        cairo.arc(
            rect.dx().center() as f64,
            rect.dy().center() as f64,
            (rect.height() / 2.0) as f64,
            0f64.to_radians(), 360f64.to_radians(),
        );
        cairo.close_path();
        cairo.set_source_rgba(fill.r_f64(), fill.g_f64(), fill.b_f64(), fill.a_f64());
        cairo.fill_preserve();
        cairo.set_line_width(border.width() as f64);
        cairo.set_source_rgba(border.color.r_f64(), border.color.g_f64(), border.color.b_f64(), border.color.a_f64());
        cairo.stroke();
        cairo.restore();
    }

    #[cfg(not(feature = "gpu"))]
    pub fn paint_triangle(&self, cairo: &mut Cairo, pos0: Pos, pos1: Pos, pos2: Pos, fill: &Color, border: &Border) {
        cairo.save();
        cairo.new_path();
        cairo.move_to(pos0.x as f64, pos0.y as f64);
        cairo.line_to(pos1.x as f64, pos1.y as f64);
        cairo.line_to(pos2.x as f64, pos2.y as f64);
        cairo.close_path();
        cairo.set_source_rgba(fill.r_f64(), fill.g_f64(), fill.b_f64(), fill.a_f64());
        cairo.fill_preserve();
        cairo.set_line_width(border.width() as f64);
        cairo.set_source_rgba(border.color.r_f64(), border.color.g_f64(), border.color.b_f64(), border.color.a_f64());
        cairo.stroke();
        cairo.restore();
    }

    #[cfg(not(feature = "gpu"))]
    pub fn paint_image(&self, cairo: &mut Cairo, texture: &mut ImageTexture, rect: Rect) {
        cairo.save();
        let img = texture.raw_mut().as_mut_ptr();
        let sx = rect.width() / texture.size().width;
        let sy = rect.height() / texture.size().height;
        cairo.translate(rect.dx().min as f64, rect.dy().min as f64);
        cairo.scale(sx as f64, sy as f64);
        let surface = CairoSurface::new_image(img, texture.size().width as i32, texture.size().height as i32);
        cairo.set_source_surface(surface, 0.0, 0.0);
        cairo.paint();
        cairo.restore();
    }
}

impl Drop for X11WindowHandle {
    fn drop(&mut self) {
        unsafe {
            XFreeColormap(self.display, self.colormap);
            xlib::XDestroyWindow(self.display, self.window);
        }
    }
}

unsafe impl Send for X11WindowHandle {}
unsafe impl Sync for X11WindowHandle {}
