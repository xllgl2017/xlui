#[cfg(feature = "gpu")]
use ab_glyph::{Font as AbFont, PxScale, ScaleFont};
#[cfg(feature = "gpu")]
use std::fs;
use std::mem;
#[cfg(all(target_os = "linux", not(feature = "gpu")))]
use std::ffi::CString;
#[cfg(feature = "gpu")]
use std::path::Path;
use std::sync::Arc;
#[cfg(feature = "gpu")]
use cosmic_text::fontdb::Source;
#[cfg(all(target_os = "linux", not(feature = "gpu")))]
use x11::xft::{XftFont, XftFontClose, XftFontOpenName};
#[cfg(all(target_os = "linux", not(feature = "gpu")))]
use x11::xft::XftTextExtentsUtf8;
#[cfg(all(target_os = "linux", not(feature = "gpu")))]
use x11::xrender::XGlyphInfo;
use crate::error::UiResult;
use crate::*;
use crate::text::cchar::{CChar, LineChar};
#[cfg(all(target_os = "linux", not(feature = "gpu")))]
use crate::window::WindowType;

pub struct Font {
    family: String,
    #[cfg(feature = "gpu")]
    glyph_font: ab_glyph::FontArc,
    size: f32,
    #[cfg(feature = "gpu")]
    font_system: cosmic_text::FontSystem,
}

impl Font {
    ///默认字体-仿宋
    pub fn default() -> UiResult<Font> {
        Font::from_family("FangSong")
    }


    ///设置全局字体大小
    pub fn with_size(mut self, font_size: f32) -> Self {
        self.size = font_size;
        self
    }
}

#[cfg(feature = "gpu")]
impl Font {
    fn get_font_by_family(system: &cosmic_text::FontSystem, family: &str) -> UiResult<Vec<u8>> {
        let face = system.db().faces().find(|x| {
            for (font_family, _) in &x.families {
                if font_family == family { return true; };
            }
            false
        }).ok_or(format!("字体'{}'未找到", family))?;
        let data = match &face.source {
            Source::Binary(data) => data.as_ref().as_ref().to_vec(),
            Source::File(data) => fs::read(data)?,
            Source::SharedFile(_, data) => data.as_ref().as_ref().to_vec(),
        };
        Ok(data)
    }

    ///根据字体名称调用系统字体
    pub fn from_family(family: &str) -> UiResult<Font> {
        let mut font_system = cosmic_text::FontSystem::new();
        let font = Font::get_font_by_family(&mut font_system, family)?;
        let glyph_font = ab_glyph::FontArc::try_from_vec(font)?;
        Ok(Font {
            family: family.to_string(),
            glyph_font,
            // font,
            size: 14.0,
            font_system,
        })
    }

    ///使用自定义字体文件
    pub fn from_file(fp: impl AsRef<Path>) -> UiResult<Font> {
        let data = fs::read(fp)?;
        Font::from_vec(data)
    }


    ///使用字体字节集
    pub fn from_vec(data: Vec<u8>) -> UiResult<Font> {
        let mut res = Font::default()?;
        res.glyph_font = ab_glyph::FontArc::try_from_vec(data.to_vec())?;
        let mut font_system = cosmic_text::FontSystem::new();
        let id = font_system.db_mut().load_font_source(Source::Binary(Arc::new(data)));
        for face in font_system.db().faces() {
            if face.id.to_string() != id[0].to_string() { continue; }
            res.family = face.families[0].0.clone();
            break;
        }
        Ok(res)
    }

    pub(crate) fn line_height(&self, text: &mut RichText) -> UiResult<f32> {
        let size = text.size.get_or_insert(self.size);
        let family = text.family.get_or_insert_with(|| self.family.clone());
        let font = Self::get_font_by_family(&self.font_system, family)?;
        let glyph_font = ab_glyph::FontRef::try_from_slice(&font)?;
        let scale = PxScale::from(*size);
        let scale_font = glyph_font.as_scaled(scale);
        let ascent = scale_font.ascent();
        let descent = scale_font.descent();
        let line_gap = scale_font.line_gap();
        Ok(ascent - descent + line_gap)
    }

    pub(crate) fn measure_text(&self, buffer: &cosmic_text::Buffer, wrap: bool, max_wrap_width: f32) -> Vec<LineChar> {
        let mut res = vec![];
        for buffer_line in &buffer.lines {
            let mut line = LineChar::new(buffer_line.text());
            line.auto_wrap = false;
            for layout in buffer_line.layout_opt().unwrap() {
                for glyph in &layout.glyphs {
                    let cchar = buffer_line.text()[glyph.start..glyph.end].chars().next().unwrap();
                    if wrap && line.width + glyph.w >= max_wrap_width {
                        let mut line = mem::take(&mut line);
                        line.auto_wrap = true;
                        res.push(line);
                    }
                    line.push(CChar::new(cchar, glyph.w));
                }
            }
            res.push(line);
        }
        if let Some(line) = res.last_mut() { line.auto_wrap = true; }
        res
    }

    pub(crate) fn system_mut(&mut self) -> &mut cosmic_text::FontSystem {
        &mut self.font_system
    }
}

#[cfg(all(target_os = "linux", not(feature = "gpu")))]
impl Font {
    ///根据字体名称调用系统字体
    pub fn from_family(family: &str) -> UiResult<Font> {
        Ok(Font {
            family: family.to_string(),
            size: 14.0,
        })
    }

    // 打开字体
    fn get_xft_font(&self, handle: &Arc<WindowType>, family: &str, size: f32) -> UiResult<*mut XftFont> {
        let font_name = CString::new(format!("{}:pixelsize={}", family, size))?;
        let display = handle.x11().display;
        let screen = handle.x11().screen;
        let xft_font = unsafe { XftFontOpenName(display, screen, font_name.as_ptr()) };
        Ok(xft_font)
    }

    pub(crate) fn line_height(&self, handle: &Arc<WindowType>, text: &mut RichText) -> UiResult<f32> {
        let size = text.size.get_or_insert(self.size);
        let family = text.family.get_or_insert_with(|| self.family.clone());
        let font = self.get_xft_font(handle, family, *size)?;
        let font_ref = unsafe { font.as_ref() }.ok_or(format!("字体'{}'未找到", family))?;
        let height = font_ref.height as f32;
        unsafe { XftFontClose(handle.x11().display, font) };
        Ok(height)
    }

    pub fn measure_text(&self, handle: &Arc<WindowType>, text: &RichText, wrap: bool, max_wrap: f32) -> UiResult<Vec<LineChar>> {
        let mut res = vec![];
        let xft_font = self.get_xft_font(handle, text.family.as_ref().ok_or("字体为空")?, text.font_size())?;
        let text = text.text.replace("\r\n", "\n");

        for line in text.split("\n") {
            let mut lines = self.measure_line(line, handle, xft_font, wrap, max_wrap)?;
            res.append(&mut lines);
        }
        println!("{:#?}", res);
        unsafe { XftFontClose(handle.x11().display, xft_font) };
        Ok(res)
    }

    fn measure_line(&self, line: &str, handle: &Arc<WindowType>, xft_font: *mut XftFont, wrap: bool, max_wrap: f32) -> UiResult<Vec<LineChar>> {
        let mut res = vec![];
        let mut line_char = LineChar::new();
        for ch in line.chars() {
            let cchar = self.measure_char(ch, handle, xft_font)?;
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

    fn measure_char(&self, ch: char, handle: &Arc<WindowType>, xft_font: *mut XftFont) -> UiResult<CChar> {
        let char_str = ch.to_string();
        let char_len = char_str.len() as i32;
        let c_char_str = CString::new(char_str)?;
        let c_char_ptr = c_char_str.as_ptr() as *const u8;
        let mut extents: XGlyphInfo = unsafe { mem::zeroed() };
        unsafe {
            XftTextExtentsUtf8(handle.x11().display, xft_font, c_char_ptr, char_len, &mut extents);
        }
        Ok(CChar::new(ch, extents.xOff as f32))
    }
}

impl From<&Font> for Font {
    fn from(value: &Font) -> Self {
        Font {
            family: value.family.clone(),
            #[cfg(feature = "gpu")]
            glyph_font: value.glyph_font.clone(),
            size: value.size,
            #[cfg(feature = "gpu")]
            font_system: cosmic_text::FontSystem::new(),
        }
    }
}