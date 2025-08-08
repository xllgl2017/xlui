use ab_glyph::{Font as AbFont, PxScale, ScaleFont};
use glyphon::fontdb::Source;
use std::fs;
use std::sync::Arc;
use crate::text::TextSize;

pub struct Font {
    family: String,
    id: glyphon::fontdb::ID,
    data: Arc<Vec<u8>>,
    glyph_font: ab_glyph::FontArc,
    font: Arc<glyphon::Font>,
}

impl Font {
    fn new() -> Font {
        let mut font_system = glyphon::FontSystem::new();
        let face = font_system.db().faces().next().unwrap();
        let font = font_system.get_font(face.id).unwrap();
        let glyph_font = ab_glyph::FontArc::try_from_vec(font.data().to_vec()).unwrap();
        Font {
            family: "".to_string(),
            id: Default::default(),
            data: Arc::new(vec![]),
            glyph_font,
            font,
        }
    }

    pub fn new_from_file(fp: &str) -> Font {
        let data = fs::read(fp).unwrap();
        Font::from_vec(data)
    }

    pub fn from_vec(data: Vec<u8>) -> Font {
        let mut res = Font::new();
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

    pub fn text_size(&self, txt: &str, font_size: f32) -> TextSize {
        let mut size = TextSize::new();
        let scale = PxScale::from(font_size);
        let scale_font = self.glyph_font.as_scaled(scale);
        txt.chars().for_each(|c| {
            let glyph = self.glyph_font.glyph_id(c);
            size.line_width += scale_font.h_advance(glyph);
        });
        let ascent = scale_font.ascent();
        let descent = scale_font.descent();
        let line_gap = scale_font.line_gap();
        size.line_height = ascent - descent + line_gap;
        size.font_size = font_size;
        size
    }


    pub fn text_width(&self, txt: &str, font_size: f32) -> Vec<f32> {
        let mut res = vec![];
        let scale = PxScale::from(font_size);
        let scale_font = self.glyph_font.as_scaled(scale);
        txt.chars().for_each(|c| {
            let glyph = self.glyph_font.glyph_id(c);
            res.push(scale_font.h_advance(glyph));
        });
        res
    }

    pub fn char_width(&self, char: char, font_size: f32) -> f32 {
        let scale = PxScale::from(font_size);
        let scale_font = self.glyph_font.as_scaled(scale);
        let glyph = self.glyph_font.glyph_id(char);
        scale_font.h_advance(glyph)
    }
    pub fn family(&self) -> glyphon::Family {
        glyphon::Family::Name(&self.family)
    }

    pub fn font_attr(&self) -> glyphon::Attrs {
        glyphon::Attrs::new().family(self.family())
    }
}