use crate::text::cchar::{CChar, LineChar};
#[cfg(all(target_os = "linux", not(feature = "gpu")))]
use crate::window::x11::font::X11Font;
use crate::*;
#[cfg(feature = "gpu")]
use ab_glyph::{Font as AbFont, PxScale, ScaleFont};
#[cfg(feature = "gpu")]
use cosmic_text::fontdb::Source;
#[cfg(feature = "gpu")]
use cosmic_text::rustybuzz;
#[cfg(feature = "gpu")]
use cosmic_text::rustybuzz::{Direction, ShapePlan, UnicodeBuffer};
#[cfg(feature = "gpu")]
use std::fs;
#[cfg(feature = "gpu")]
use std::mem;
#[cfg(feature = "gpu")]
use std::path::Path;
#[cfg(feature = "gpu")]
use std::sync::Arc;
#[cfg(all(target_os = "windows",not(feature = "gpu")))]
use crate::window::win32::font::Win32Font;

pub enum FontSlant {
    Normal = 0,
    Italic = 1,
    Oblique = 2,
}

pub enum FontWeight {
    Normal = 0,
    Bold = 1,
}

/// ### Font全局字体
/// * 在wgpu模式下支持使用自定义字体ttf文件和bytes
/// * 在native模式下仅支持调用系统已有的字体
///
/// ### Font示例
/// ```rust
/// use std::fs;
/// use xlui::Font;
///
/// fn draw(){
///     let font=Font::from_family("微软雅黑");
///     #[cfg(feature = "gpu")]
///     let font=Font::from_file("1.ttf");
///     #[cfg(feature = "gpu")]
///     let font=Font::from_vec(fs::read("1.ttf"));
/// }
/// ```

pub struct Font {
    family: String,
    size: f32,
    #[cfg(feature = "gpu")]
    font_system: cosmic_text::FontSystem,
}

impl Font {
    ///默认字体-仿宋
    pub fn default() -> UiResult<Font> {
        Font::from_family("FangSong")
    }
    ///根据字体名称调用系统字体
    pub fn from_family(family: &str) -> UiResult<Font> {
        Ok(Font {
            family: family.to_string(),
            size: 14.0,
            #[cfg(feature = "gpu")]
            font_system: cosmic_text::FontSystem::new(),
        })
    }

    ///设置全局字体大小
    pub fn with_size(mut self, font_size: f32) -> Self {
        self.size = font_size;
        self
    }

    pub fn family(&self) -> &str {
        &self.family
    }

    pub fn size(&self) -> f32 {
        self.size
    }
}

#[cfg(feature = "gpu")]
impl Font {
    ///使用自定义字体文件
    pub fn from_file(fp: impl AsRef<Path>) -> UiResult<Font> {
        let data = fs::read(fp)?;
        Font::from_vec(data)
    }


    ///使用字体字节集
    pub fn from_vec(data: Vec<u8>) -> UiResult<Font> {
        let mut res = Font::default()?;
        // res.glyph_font = ab_glyph::FontArc::try_from_vec(data.to_vec())?;
        let mut font_system = cosmic_text::FontSystem::new();
        let id = font_system.db_mut().load_font_source(Source::Binary(Arc::new(data)));
        for face in font_system.db().faces() {
            if face.id.to_string() != id[0].to_string() { continue; }
            res.family = face.families[0].0.clone();
            break;
        }
        res.font_system = font_system;
        Ok(res)
    }


    pub(crate) fn system_mut(&mut self) -> &mut cosmic_text::FontSystem {
        &mut self.font_system
    }
}

impl From<&Font> for Font {
    fn from(value: &Font) -> Self {
        Font {
            family: value.family.clone(),
            size: value.size,
            #[cfg(feature = "gpu")]
            font_system: cosmic_text::FontSystem::new(),
        }
    }
}

#[cfg(feature = "gpu")]
pub(crate) struct WGpuFont {
    glyph_font: Option<ab_glyph::FontArc>,
    family: String,
    size: f32,
    font: Option<Arc<cosmic_text::Font>>,
}

#[cfg(feature = "gpu")]
impl WGpuFont {
    pub fn new() -> WGpuFont {
        WGpuFont {
            glyph_font: None,

            family: "".to_string(),
            size: 0.0,
            font: None,
        }
    }

    fn get_font_by_family(&self, system: &mut cosmic_text::FontSystem, family: &str) -> UiResult<Arc<cosmic_text::Font>> {
        let face = system.db().faces().find(|x| {
            for (font_family, _) in &x.families {
                if font_family == family { return true; };
            }
            false
        }).ok_or(format!("字体'{}'未找到", family))?;
        let font = system.get_font(face.id).ok_or(UiError::OptNone)?;
        Ok(font)
    }

    pub fn set_family_size(&mut self, ui: &mut Ui, text: &mut RichText) -> UiResult<()> {
        let family = text.family.get_or_insert_with(|| ui.context.font.family().to_string());
        let size = text.size.get_or_insert_with(|| ui.context.font.size());
        self.family = family.to_string();
        self.size = *size;
        // self.buffer.set_metrics(ui.context.font.system_mut(), Metrics::new(self.size, text.height));
        self.init(ui)?;
        Ok(())
    }

    pub fn init(&mut self, ui: &mut Ui) -> UiResult<()> {
        let font = self.get_font_by_family(ui.context.font.system_mut(), &self.family)?;
        self.glyph_font = Some(ab_glyph::FontArc::try_from_vec(font.data().to_vec())?);
        self.font = Some(font);
        Ok(())
    }

    pub(crate) fn measure_text(&self, text: &RichText, wrap: bool, max_wrap_width: f32) -> UiResult<Vec<LineChar>> {
        let mut res = vec![];
        let text = text.text.replace("\r\n", "\n");
        for line in text.split("\n") {
            let mut lines = self.measure_line(line, wrap, max_wrap_width)?;
            res.append(&mut lines);
        }
        if let Some(last) = res.last_mut() {
            last.auto_wrap = true;
        }
        Ok(res)
    }

    fn measure_line(&self, line: &str, wrap: bool, max_wrap: f32) -> UiResult<Vec<LineChar>> {
        let mut buffer = UnicodeBuffer::default();
        buffer.set_direction(Direction::LeftToRight);
        buffer.push_str(line);
        buffer.guess_segment_properties();
        let font = self.font.as_ref().ok_or(UiError::OptNone)?;
        let shape_plan = ShapePlan::new(font.rustybuzz(), Direction::LeftToRight, Some(buffer.script()), None, &vec![]);
        let glyph_buffer = rustybuzz::shape_with_plan(font.rustybuzz(), &shape_plan, buffer);
        let font_scale = font.rustybuzz().units_per_em() as f32;
        let mut res = vec![];
        let mut line_char = LineChar::new();
        let glyph_positions = glyph_buffer.glyph_positions();
        for (position, ch) in glyph_positions.iter().zip(line.chars()) {
            let x_advance = position.x_advance as f32 / font_scale + 0.0;
            let cchar = CChar::new(ch, x_advance * self.size);
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

    pub fn measure_char(&self, ch: char) -> UiResult<CChar> {
        let mut buffer = UnicodeBuffer::default();
        buffer.set_direction(Direction::LeftToRight);
        buffer.push_str(ch.to_string().as_str());
        buffer.guess_segment_properties();
        let font = self.font.as_ref().ok_or(UiError::OptNone)?;
        let shape_plan = ShapePlan::new(font.rustybuzz(), Direction::LeftToRight, Some(buffer.script()), None, &vec![]);
        let glyph_buffer = rustybuzz::shape_with_plan(font.rustybuzz(), &shape_plan, buffer);
        let font_scale = font.rustybuzz().units_per_em() as f32;
        let x_advance = glyph_buffer.glyph_positions().get(0).ok_or(UiError::OptNone)?.x_advance as f32 / font_scale + 0.0;
        Ok(CChar::new(ch, x_advance * self.size))
    }

    pub(crate) fn line_height(&self) -> UiResult<f32> {
        let scale = PxScale::from(self.size);
        let scale_font = self.glyph_font.as_ref().ok_or(UiError::OptNone)?.as_scaled(scale);
        let ascent = scale_font.ascent();
        let descent = scale_font.descent();
        let line_gap = scale_font.line_gap();
        Ok(ascent - descent + line_gap)
    }
}

pub(crate) enum FontKind {
    #[cfg(feature = "gpu")]
    WGpu(WGpuFont),
    #[cfg(all(target_os = "linux", not(feature = "gpu")))]
    X11(X11Font),
    #[cfg(all(target_os = "windows", not(feature = "gpu")))]
    Win32(Win32Font),
}

impl FontKind {
    pub fn new() -> FontKind {
        #[cfg(feature = "gpu")]
        return FontKind::WGpu(WGpuFont::new());
        #[cfg(all(target_os = "linux", not(feature = "gpu")))]
        return FontKind::X11(X11Font::new_empty());
        #[cfg(all(target_os = "windows", not(feature = "gpu")))]
        return FontKind::Win32(Win32Font::new());
    }

    pub(crate) fn line_height(&self) -> UiResult<f32> {
        match self {
            #[cfg(feature = "gpu")]
            FontKind::WGpu(font) => font.line_height(),
            #[cfg(all(target_os = "linux", not(feature = "gpu")))]
            FontKind::X11(font) => font.line_height(),
            #[cfg(all(target_os = "windows", not(feature = "gpu")))]
            FontKind::Win32(font) => font.line_height()
        }
    }

    pub(crate) fn set_family_size(&mut self, ui: &mut Ui, text: &mut RichText) -> UiResult<()> {
        match self {
            #[cfg(feature = "gpu")]
            FontKind::WGpu(font) => font.set_family_size(ui, text),
            #[cfg(all(target_os = "linux", not(feature = "gpu")))]
            FontKind::X11(font) => font.set_family_size(ui, text),
            #[cfg(all(target_os = "windows", not(feature = "gpu")))]
            FontKind::Win32(font) => font.set_family_size(ui, text)
        }
    }

    pub fn measure_text(&self, text: &RichText, wrap: bool, max_wrap_width: f32) -> UiResult<Vec<LineChar>> {
        match self {
            #[cfg(feature = "gpu")]
            FontKind::WGpu(font) => font.measure_text(text, wrap, max_wrap_width),
            #[cfg(all(target_os = "linux", not(feature = "gpu")))]
            FontKind::X11(font) => font.measure_text(text, wrap, max_wrap_width),
            #[cfg(all(target_os = "windows", not(feature = "gpu")))]
            FontKind::Win32(font) => font.measure_text(text, wrap, max_wrap_width),
        }
    }
    #[cfg(feature = "gpu")]
    pub fn wgpu_mut(&mut self) -> &mut WGpuFont {
        match self { FontKind::WGpu(font) => font }
    }

    pub fn measure_char(&self, ch: char) -> UiResult<CChar> {
        match self {
            #[cfg(feature = "gpu")]
            FontKind::WGpu(font) => font.measure_char(ch),
            #[cfg(all(target_os = "linux", not(feature = "gpu")))]
            FontKind::X11(font) => font.measure_char(ch),
            #[cfg(all(target_os = "windows", not(feature = "gpu")))]
            FontKind::Win32(font) => font.measure_char(ch),
        }
    }
}