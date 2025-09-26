use std::mem;
use crate::align::Align;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::text::rich::RichText;
use crate::text::TextWrap;
use crate::ui::Ui;
use crate::SAMPLE_COUNT;
use glyphon::Shaping;
use wgpu::MultisampleState;
use crate::text::cchar::{CChar, LineChar};

pub struct TextBuffer {
    pub(crate) text: RichText,
    pub(crate) rect: Rect,
    pub(crate) size_mode: SizeMode,
    pub(crate) buffer: Option<glyphon::Buffer>,
    pub(crate) render: Option<glyphon::TextRenderer>,
    pub(crate) clip_x: f32,
    pub(crate) clip_y: f32,
    pub(crate) change: bool,
    pub(crate) align: Align,
    pub(crate) lines: Vec<LineChar>,
}

impl TextBuffer {
    pub fn new(text: impl Into<RichText>) -> TextBuffer {
        TextBuffer {
            text: text.into(),
            rect: Rect::new(),
            size_mode: SizeMode::Auto,
            buffer: None,
            render: None,
            clip_x: 0.0,
            clip_y: 0.0,
            change: false,
            align: Align::LeftTop,
            lines: vec![],
        }
    }


    fn reset(&mut self) {
        if self.buffer.is_none() { return; }
        self.text.width = 0.0;
        self.lines.clear();
        let buffer = self.buffer.as_ref().unwrap();
        for buffer_line in &buffer.lines {
            let mut line = LineChar::new();
            line.auto_wrap = false;
            for layout in buffer_line.layout_opt().unwrap() {
                for glyph in &layout.glyphs {
                    let cchar = buffer_line.text()[glyph.start..glyph.end].chars().next().unwrap();
                    if self.size_mode.is_fixed_width() && line.width + glyph.w >= self.rect.width() && self.text.wrap.is_wrap() {
                        let mut line = mem::take(&mut line);
                        line.auto_wrap = true;
                        self.lines.push(line);
                    }
                    line.push(CChar::new(cchar, glyph.w));
                    self.text.width += glyph.w;
                }
            }
            self.lines.push(line);
        }
        if let Some(line) = self.lines.last_mut() { line.auto_wrap = true; }
    }

    pub fn init(&mut self, ui: &mut Ui) {
        if self.text.size.is_none() { self.text.size = Some(ui.context.font.size()) }
        if self.text.family.is_none() { self.text.family = Some(ui.context.font.family().to_string()) }
        self.text.height = ui.context.font.line_height(self.text.font_size());
        let mut buffer = glyphon::Buffer::new(&mut ui.context.render.text.font_system, glyphon::Metrics::new(self.text.font_size(), self.text.height));
        if let SizeMode::Fix(w, h) = self.size_mode { buffer.set_size(&mut ui.context.render.text.font_system, Some(w), Some(h)) }
        if let SizeMode::FixWidth(w) = self.size_mode { buffer.set_size(&mut ui.context.render.text.font_system, Some(w), None) }
        if let SizeMode::FixHeight(h) = self.size_mode { buffer.set_size(&mut ui.context.render.text.font_system, None, Some(h)) }
        buffer.set_wrap(&mut ui.context.render.text.font_system, self.text.wrap.as_gamma());
        buffer.set_text(&mut ui.context.render.text.font_system, &self.text.text, &self.text.font_family(), Shaping::Advanced);
        let render = glyphon::TextRenderer::new(&mut ui.context.render.text.atlas, &ui.device.device, MultisampleState {
            count: SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }, None);
        self.render = Some(render);
        self.buffer = Some(buffer);
        self.reset();
        let (w, h) = self.size_mode.size(self.text.width, self.text.height);
        self.rect.set_size(w, h);
        // match self.size_mode {
        //     SizeMode::Auto => self.rect.set_size(self.text.width, self.text.height),
        //     SizeMode::FixWidth(w) => self.rect.set_size(w, self.text.height),
        //     SizeMode::FixHeight(h) => self.rect.set_size(self.text.width, h),
        //     _ => {}
        // }
        // if let SizeMode::Auto = self.size_mode { return; }
        let ox = self.rect.width() - self.text.width;
        let oy = self.rect.height() - self.text.height;
        match self.align {
            //固定宽度
            Align::LeftCenter => {
                self.clip_x = 0.0;
                self.clip_y = oy / 2.0;
            }
            Align::Center => {
                self.clip_x = ox / 2.0;
                self.clip_y = oy / 2.0;
            }
            Align::RightCenter => {
                self.clip_x = ox;
                self.clip_y = oy / 2.0;
            }
            //固定高度
            Align::CenterTop => {
                self.clip_x = ox / 2.0;
                self.clip_y = 0.0;
            }
            Align::CenterBottom => {
                self.clip_x = ox / 2.0;
                self.clip_y = oy
            }
            //宽高固定
            Align::LeftTop => {
                self.clip_x = 0.0;
                self.clip_y = 0.0;
            }
            Align::LeftBottom => {
                self.clip_x = 0.0;
                self.clip_y = oy;
            }
            Align::RightTop => {
                self.clip_x = ox;
                self.clip_y = 0.0;
            }
            Align::RightBottom => {
                self.clip_x = ox;
                self.clip_y = oy;
            }
        }
    }

    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        let bounds = glyphon::TextBounds {
            left: self.rect.dx().min as i32,
            top: 0,
            right: self.rect.dx().max as i32,
            bottom: self.rect.dy().max as i32,
        };
        let area = glyphon::TextArea {
            buffer: self.buffer.as_ref().unwrap(),
            left: self.rect.dx().min + self.clip_x,
            top: self.rect.dy().min + self.clip_y,
            scale: 1.0,
            bounds,
            default_color: self.text.color.as_glyphon_color(),
            custom_glyphs: &[],
        };
        self.render.as_mut().unwrap().prepare(
            &ui.device.device, &ui.device.queue,
            &mut ui.context.render.text.font_system,
            &mut ui.context.render.text.atlas,
            &ui.context.viewport, vec![area],
            &mut ui.context.render.text.cache).unwrap();
        let pass = ui.pass.as_mut().unwrap();
        self.render.as_mut().unwrap().render(&mut ui.context.render.text.atlas, &ui.context.viewport, pass).unwrap()
    }

    pub fn set_text(&mut self, text: String) {
        self.change = self.text.text != text;
        println!("{} {}", self.change, text);
        self.text.text = text;
    }

    pub fn with_align(mut self, align: Align) -> Self {
        self.align = align;
        self
    }

    pub fn update_buffer_text(&mut self, ui: &mut Ui, text: &str) {
        match self.buffer {
            None => self.set_text(text.to_string()),
            Some(_) => {
                self.change = self.text.text != text;
                self.text.text = text.to_string();
                self.update_buffer(ui);
                // buffer.set_text(&mut ui.context.render.text.font_system,
                //                 text, &self.text.font_family(), Shaping::Advanced);
                // self.reset();
            }
        }
    }

    pub fn update_if_not(&mut self, ui: &mut Ui, text: &str, reset: bool) {
        self.buffer.as_mut().unwrap().set_text(&mut ui.context.render.text.font_system, text, &self.text.font_family(), Shaping::Advanced);
        if reset { self.reset(); }
    }

    pub fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.change { return; }
        self.change = false;
        self.buffer.as_mut().unwrap().set_text(
            &mut ui.context.render.text.font_system, self.text.text.as_str(),
            &self.text.font_family(), Shaping::Advanced);
        self.reset();
        if self.size_mode.is_auto_width() { self.rect.set_width(self.lines[0].width); }
    }

    pub fn set_wrap(&mut self, wrap: TextWrap) {
        self.text.wrap = wrap;
        self.reset();
    }

    pub fn set_width(&mut self, width: f32) {
        self.rect.set_width(width);
        self.size_mode.fix_width(width);
    }

    pub fn set_height(&mut self, height: f32) {
        self.rect.set_height(height);
        self.size_mode.fix_height(height);
    }

    pub fn height(mut self, height: f32) -> Self {
        self.set_height(height);
        self
    }
}