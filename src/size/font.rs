use ab_glyph::{Font as AbFont, PxScale, ScaleFont};
use glyphon::fontdb::Source;
use std::fs;
use std::path::Path;
use std::sync::Arc;

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

    pub(crate) fn line_height(&self, font_size: f32) -> f32 {
        let scale = PxScale::from(font_size);
        let scale_font = self.glyph_font.as_scaled(scale);
        let ascent = scale_font.ascent();
        let descent = scale_font.descent();
        let line_gap = scale_font.line_gap();
        ascent - descent + line_gap
    }

    pub(crate) fn family(&self) -> &str { &self.family }


    pub fn size(&self) -> f32 {
        self.size
    }
}