use ab_glyph::{Font as AbFont, PxScale, ScaleFont};
use glyphon::fontdb::Source;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use glyphon::cosmic_text::rustybuzz;
use glyphon::cosmic_text::rustybuzz::Face;
use rustybuzz::UnicodeBuffer;
use crate::text::TextSize;

pub struct Font {
    family: String,
    data: Arc<Vec<u8>>,
    glyph_font: ab_glyph::FontArc,
    font: Arc<glyphon::Font>,
    size: f32,
}

impl Font {
    pub fn default() -> Font {
        let mut font_system = glyphon::FontSystem::new();
        let face = font_system.db().faces().find(|x| x.families[0].0.contains("FangSong")).unwrap(); //FangSong
        let family = face.families[0].0.clone();
        let font = font_system.get_font(face.id).unwrap();
        let font_data = Arc::new(font.data().to_vec());
        let glyph_font = ab_glyph::FontArc::try_from_vec(font_data.to_vec()).unwrap();
        Font {
            family,
            data: font_data,
            glyph_font,
            font,
            size: 14.0,
        }
    }

    pub fn from_file(fp: impl AsRef<Path>) -> Font {
        let data = fs::read(fp).unwrap();
        Font::from_vec(data)
    }

    pub fn from_vec(data: Vec<u8>) -> Font {
        let mut res = Font::default();
        res.data = Arc::new(data);
        res.glyph_font = ab_glyph::FontArc::try_from_vec(res.data.to_vec()).unwrap();
        let mut font_system = glyphon::FontSystem::new();
        let id = font_system.db_mut().load_font_source(Source::Binary(res.data.clone()));
        for face in font_system.db().faces() {
            if face.id.to_string() != id[0].to_string() { continue; }
            res.family = face.families[0].0.clone();
            break;
        }
        let font = font_system.get_font(id[0]).unwrap();
        res.font = font;
        res
    }

    ///设置全局字体大小
    pub fn with_size(mut self, font_size: f32) -> Self {
        self.size = font_size;
        self
    }

    pub(crate) fn text_size(&self, txt: &str, font_size: f32) -> TextSize {
        let mut size = TextSize::new();
        let scale = PxScale::from(font_size);
        let scale_font = self.glyph_font.as_scaled(scale);
        let ascent = scale_font.ascent();
        let descent = scale_font.descent();
        let line_gap = scale_font.line_gap();
        size.line_height = ascent - descent + line_gap;
        size.font_size = font_size;
        let face = Face::from_slice(&self.data, 0).expect("invalid font data");
        let mut buf = UnicodeBuffer::new();
        buf.push_str(txt);
        let glyph_buffer = rustybuzz::shape(&face, &[], buf);
        let positions = glyph_buffer.glyph_positions();
        let upem = face.units_per_em() as f32;
        let scale = font_size / upem;
        for pos in positions {
            size.line_width += pos.x_advance as f32 * scale;
        }
        size
    }

    pub(crate) fn char_width(&self, char: char, font_size: f32) -> f32 {
        let scale = PxScale::from(font_size);
        let scale_font = self.glyph_font.as_scaled(scale);
        let glyph = self.glyph_font.glyph_id(char);
        scale_font.h_advance(glyph)
    }
    pub(crate) fn family(&self) -> glyphon::Family {
        glyphon::Family::Name(&self.family)
    }

    pub(crate) fn font_attr(&self) -> glyphon::Attrs {
        glyphon::Attrs::new().family(self.family())
    }
}