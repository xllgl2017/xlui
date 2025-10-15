use crate::text::cchar::{CChar, LineChar};
use crate::window::win32::until;
use crate::{RichText, Ui, UiResult};
use std::mem;
use std::mem::zeroed;
use std::ptr::null_mut;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::{CreateCompatibleDC, CreateFontW, DeleteDC, DeleteObject, GetCharWidth32W, GetDeviceCaps, GetTextMetricsW, SelectObject, FONT_CHARSET, FONT_CLIP_PRECISION, FONT_OUTPUT_PRECISION, FONT_QUALITY, HDC, HFONT, HGDIOBJ, LOGPIXELSY};

pub struct Win32Font {
    family: String,
    size: f32,
    hdc: HDC,
}

impl Win32Font {
    pub fn new() -> Win32Font {
        Win32Font {
            family: "FangSong".to_string(),
            size: 14.0,
            hdc: HDC(null_mut()),
        }
    }

    pub fn set_family_size(&mut self, ui: &mut Ui, text: &mut RichText) -> UiResult<()> {
        let family = text.family.get_or_insert_with(|| ui.context.font.family().to_string());
        let size = text.size.get_or_insert_with(|| ui.context.font.size());
        self.family = family.to_string();
        self.size = *size;
        self.init();
        Ok(())
    }

    pub fn init(&mut self) {
        self.hdc = unsafe { CreateCompatibleDC(None) };
    }

    fn create_font(&self, height: i32, family: &str) -> HFONT {
        let font_name = until::to_wstr(family);
        // 创建字体
        let hfont = unsafe {
            CreateFontW(
                height,                 // 字体高度（像素）
                0,                  // 宽度（0 = 自动）
                0,                  // 角度
                0,                  // 基线角度
                500,                // 粗细（FW_BOLD = 700）
                0,                  // 斜体 (1 = TRUE)
                0,                  // 下划线
                0,                  // 删除线
                FONT_CHARSET(0),                  // 字体集 (DEFAULT_CHARSET)
                FONT_OUTPUT_PRECISION(0),                  // 输出精度
                FONT_CLIP_PRECISION(0),                  // 剪辑精度
                FONT_QUALITY(0),                  // 输出质量
                0,                  // 字体 pitch & family
                PCWSTR(font_name.as_ptr()), // 字体名称
            )
        };
        hfont
    }

    pub fn measure_text(&self, text: &RichText, wrap: bool, max_wrap: f32) -> UiResult<Vec<LineChar>> {
        unsafe {
            let hfont = self.create_font(text.height as i32, text.family.as_ref().unwrap());
            let old_font = SelectObject(self.hdc, HGDIOBJ::from(hfont));
            let lines = text.text.replace("\r\n", "\n");
            let mut res = vec![];
            for line in lines.split("\n") {
                let mut lines = self.measure_line(line, wrap, max_wrap).unwrap();
                res.append(&mut lines);
            }
            if let Some(last) = res.last_mut() {
                last.auto_wrap = true;
            }
            // 清理
            SelectObject(self.hdc, old_font);
            DeleteObject(HGDIOBJ::from(hfont)).ok()?;
            Ok(res)
        }
    }

    fn mul_div(&self, a: i32, b: i32, c: i32) -> i32 {
        ((a as i64 * b as i64 + (c as i64 / 2)) / c as i64) as i32
    }

    pub fn line_height(&self) -> UiResult<f32> {
        unsafe {
            let dpi = unsafe { GetDeviceCaps(Some(self.hdc), LOGPIXELSY) };
            let height = -self.mul_div(self.size as i32, dpi, 112);
            let font = self.create_font(height, &self.family);
            let old = SelectObject(self.hdc, HGDIOBJ::from(font));
            let mut tm = zeroed();
            GetTextMetricsW(self.hdc, &mut tm).ok()?;
            SelectObject(self.hdc, old);
            let height = tm.tmAscent + tm.tmDescent + tm.tmExternalLeading;
            DeleteObject(HGDIOBJ::from(font)).ok()?;
            Ok(height as f32)
        }
    }

    pub fn measure_line(&self, line: &str, wrap: bool, max_wrap: f32) -> UiResult<Vec<LineChar>> {
        let mut res = vec![];
        let mut line_char = LineChar::new();
        for ch in line.chars() {
            let cchar = self.measure_char(ch).unwrap();
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

    pub fn measure_char(&self, cc: char) -> UiResult<CChar> {
        let mut w = 0;
        let ch = cc.to_string().encode_utf16().collect::<Vec<u16>>();
        let ch = ch[0];
        unsafe { GetCharWidth32W(self.hdc, ch as u32, ch as u32, &mut w).ok().unwrap() };
        Ok(CChar::new(cc, w as f32))
    }
}

impl Drop for Win32Font {
    fn drop(&mut self) {
        unsafe {
            let _ = DeleteDC(self.hdc);
        }
    }
}