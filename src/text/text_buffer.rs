use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::text::{TextSize, TextWrap};
use crate::frame::context::Context;
use crate::paint::color::Color;
use crate::paint::PaintTask;
use crate::paint::text::PaintText;
use crate::ui::{Ui, UiM};

pub struct TextBuffer {
    pub(crate) id: String,
    pub(crate) text: String,
    pub(crate) rect: Rect,
    pub(crate) color: Color,
    pub(crate) text_wrap: TextWrap,
    pub(crate) text_size: TextSize,
    pub(crate) size_mode: SizeMode,
}

impl TextBuffer {
    pub fn new(text: String) -> TextBuffer {
        TextBuffer {
            id: crate::gen_unique_id(),
            text,
            rect: Rect::new(),
            color: Color::BLACK,
            text_wrap: TextWrap::NoWrap,
            text_size: TextSize::new(),
            size_mode: SizeMode::Auto,
        }
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
        let task = PaintTask::Text(PaintText::new(ui, self));
        ui.add_paint_task(self.id.clone(), task);
    }


    pub(crate) fn update(&mut self, uim: &mut UiM) {
        uim.update_text_task(self);
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
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