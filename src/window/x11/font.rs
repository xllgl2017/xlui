use crate::error::{UiError, UiResult};
use crate::text::cchar::{CChar, LineChar};
use crate::{RichText, Ui};
use std::ffi::CString;
use std::mem;
use std::ptr::null_mut;
use x11::xft::{XftFont, XftFontClose, XftFontOpenName, XftTextExtentsUtf8};
use x11::xlib::Display;
use x11::xrender::XGlyphInfo;

pub struct X11Font {
    display: *mut Display,
    font: *mut XftFont,
    screen: i32,
    family: String,
    size: f32,
}

impl X11Font {
    pub fn new_empty() -> X11Font {
        X11Font {
            display: null_mut(),
            font: null_mut(),
            screen: 0,
            family: "FangSong".to_string(),
            size: 14.0,
        }
    }

    pub fn set_family_size(&mut self, ui: &mut Ui, text: &mut RichText) -> UiResult<()> {
        let family = text.family.get_or_insert_with(|| ui.context.font.family().to_string());
        let size = text.size.get_or_insert_with(|| ui.context.font.size());
        self.family = family.to_string();
        self.size = *size;
        self.init(ui)
    }

    pub fn init(&mut self, ui: &mut Ui) -> UiResult<()> {
        let handle = ui.context.window.x11();
        if !self.display.is_null() && !self.font.is_null() { unsafe { XftFontClose(self.display, self.font); } }
        self.display = handle.display;
        self.font = self.get_xft_font(&self.family, self.size)?;
        Ok(())
    }


    // 打开字体
    pub(crate) fn get_xft_font(&self, family: &str, size: f32) -> UiResult<*mut XftFont> {
        let font_name = CString::new(format!("{}:pixelsize={}", family, size))?;
        let xft_font = unsafe { XftFontOpenName(self.display, self.screen, font_name.as_ptr()) };
        Ok(xft_font)
    }

    pub(crate) fn line_height(&self) -> UiResult<f32> {
        if self.font.is_null() || self.display.is_null() { return Err(UiError::NullPtr); }
        let font = unsafe { self.font.as_ref().ok_or(format!("字体'{}'为初始化", self.family)) }?;
        Ok(font.height as f32)
    }

    pub fn measure_text(&self, text: &RichText, wrap: bool, max_wrap: f32) -> UiResult<Vec<LineChar>> {
        let mut res = vec![];
        let text = text.text.replace("\r\n", "\n");
        for line in text.split("\n") {
            let mut lines = self.measure_line(line, wrap, max_wrap)?;
            res.append(&mut lines);
        }
        if let Some(last) = res.last_mut() {
            last.auto_wrap = true;
        }
        Ok(res)
    }

    fn measure_line(&self, line: &str, wrap: bool, max_wrap: f32) -> UiResult<Vec<LineChar>> {
        let mut res = vec![];
        let mut line_char = LineChar::new();
        for ch in line.chars() {
            let cchar = self.measure_char(ch)?;
            if wrap && line_char.width + cchar.width >= max_wrap {
                let mut line = mem::take(&mut line_char);
                line.auto_wrap = true;
                line.line_text = line.chars.iter().map(|x| x.cchar.to_string()).collect();
                res.push(line);
            }
            line_char.push(cchar);
        }
        line_char.auto_wrap = false;
        line_char.line_text = line_char.chars.iter().map(|x| x.cchar.to_string()).collect();
        res.push(line_char);
        Ok(res)
    }

    pub(crate) fn measure_char(&self, ch: char) -> UiResult<CChar> {
        let char_str = ch.to_string();
        let char_len = char_str.len() as i32;
        let c_char_str = CString::new(char_str)?;
        let c_char_ptr = c_char_str.as_ptr() as *const u8;
        let mut extents: XGlyphInfo = unsafe { mem::zeroed() };
        unsafe {
            XftTextExtentsUtf8(self.display, self.font, c_char_ptr, char_len, &mut extents);
        }
        Ok(CChar::new(ch, extents.xOff as f32))
    }
}

impl Drop for X11Font {
    fn drop(&mut self) {
        unsafe { XftFontClose(self.display, self.font); }
    }
}