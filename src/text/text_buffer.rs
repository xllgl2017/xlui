use crate::frame::context::Context;
use crate::size::pos::Axis;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::text::{TextSize, TextWrap};
use crate::ui::Ui;
use crate::SAMPLE_COUNT;
use glyphon::Shaping;
use wgpu::MultisampleState;

pub struct TextBuffer {
    pub(crate) text: String,
    pub(crate) rect: Rect,
    pub(crate) color: Color,
    pub(crate) text_wrap: TextWrap,
    pub(crate) text_size: TextSize,
    pub(crate) size_mode: SizeMode,
    pub(crate) buffer: Option<glyphon::Buffer>,
    pub(crate) render: Option<glyphon::TextRenderer>,
    pub(crate) clip_x: Axis,
    pub(crate) change: bool,
}

impl TextBuffer {
    pub fn new(text: String) -> TextBuffer {
        TextBuffer {
            text,
            rect: Rect::new(),
            color: Color::BLACK,
            text_wrap: TextWrap::NoWrap,
            text_size: TextSize::new(),
            size_mode: SizeMode::Auto,
            buffer: None,
            render: None,
            clip_x: (0.0..0.0).into(),
            change: false,
        }
    }

    pub fn with_wrap(mut self, wrap: TextWrap) -> Self {
        self.text_wrap = wrap;
        self
    }

    pub fn reset_size(&mut self, context: &Context) {
        self.text_size = context.font.text_size(&self.text, self.text_size.font_size);
        match self.size_mode {
            SizeMode::Auto => self.rect.set_size(self.text_size.line_width, self.text_size.line_height),
            SizeMode::FixWidth => self.rect.set_height(self.text_size.line_height),
            SizeMode::FixHeight => self.rect.set_width(self.text_size.line_width),
            _ => {}
        }
    }

    pub(crate) fn draw(&mut self, ui: &mut Ui) {
        let mut buffer = glyphon::Buffer::new(&mut ui.context.render.text.font_system, glyphon::Metrics::new(self.text_size.font_size, self.text_size.line_height));
        buffer.set_wrap(&mut ui.context.render.text.font_system, glyphon::Wrap::Glyph);
        buffer.set_text(&mut ui.context.render.text.font_system, &self.text, &ui.context.font.font_attr(), Shaping::Advanced);
        let render = glyphon::TextRenderer::new(&mut ui.context.render.text.atlas, &ui.device.device, MultisampleState {
            count: SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }, None);
        self.buffer = Some(buffer);
        self.render = Some(render);
    }


    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        // self.update_buffer(ui);
        let bounds = glyphon::TextBounds {
            left: self.rect.dx().min as i32,
            top: 0,
            right: self.rect.dx().max as i32,
            bottom: self.rect.dy().max as i32,
        };
        let area = glyphon::TextArea {
            buffer: self.buffer.as_ref().unwrap(),
            left: self.rect.dx().min - self.clip_x.min,
            top: self.rect.dy().min,
            scale: 1.0,
            bounds,
            default_color: self.color.as_glyphon_color(),
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
        self.text = text;
        self.change = true;
    }

    pub fn update_buffer_text(&mut self, ui: &mut Ui, text: &str) {
        match self.buffer {
            None => self.set_text(text.to_string()),
            Some(ref mut buffer) => buffer.set_text(
                &mut ui.context.render.text.font_system,
                text, &ui.context.font.font_attr(), Shaping::Advanced)
        }
    }

    pub fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.change { return; }
        self.change = false;
        self.buffer.as_mut().unwrap().set_text(
            &mut ui.context.render.text.font_system, self.text.as_str(),
            &ui.context.font.font_attr(), Shaping::Advanced);
    }

    pub fn set_wrap(&mut self, wrap: TextWrap) {
        self.text_wrap = wrap;
    }

    pub fn set_width(&mut self, width: f32) {
        self.rect.set_width(width);
        self.size_mode.fix_width();
    }

    pub fn set_height(&mut self, height: f32) {
        self.rect.set_height(height);
        self.size_mode.fix_height();
    }
}