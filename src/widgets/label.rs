use crate::text::text_buffer::TextBuffer;
use crate::text::TextWrap;
use crate::ui::Ui;
use crate::widgets::Widget;
use glyphon::Shaping;
use crate::response::Response;

pub struct Label {
    id: String,
    change: bool,
    buffer: TextBuffer,
}

impl Label {
    pub fn new(text: impl ToString) -> Label {
        let buffer = TextBuffer::new(text.to_string());
        Label {
            id: crate::gen_unique_id(),
            change: false,
            buffer,
        }
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.buffer.set_wrap(wrap);
        self
    }


    pub fn set_text(&mut self, text: String) {
        self.buffer.text = text;
        self.change = true;
    }

    pub fn width(mut self, w: f32) -> Self {
        self.buffer.set_width(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.buffer.set_height(h);
        self
    }

    pub fn size(mut self, s: f32) -> Self {
        self.buffer.text_size.font_size = s;
        self
    }

    pub fn text(&self) -> &String {
        &self.buffer.text
    }
}


impl Widget for Label {
    fn draw(&mut self, ui: &mut Ui) -> Response {
        self.buffer.rect = ui.layout().available_rect().clone_with_size(&self.buffer.rect);
        self.buffer.reset_size(ui.context);
        // ui.layout().alloc_rect(&self.buffer.rect);
        self.buffer.draw(ui);
        Response {
            id: self.id.clone(),
            rect: self.buffer.rect.clone(),
        }
    }


    fn update(&mut self, ui: &mut Ui) { //处理鼠标键盘时间
        if let Some(update) = ui.context.updates.remove(&self.id) {
            self.buffer.buffer.as_mut().unwrap().set_text(&mut ui.context.render.text.font_system, update.text().as_str(), &ui.context.font.font_attr(), Shaping::Advanced);
        }
        if let Some(ref offset) = ui.canvas_offset {
            self.buffer.rect.offset(offset.x, offset.y);
        }
        if !self.change { return; }
        self.buffer.buffer.as_mut().unwrap().set_text(
            &mut ui.context.render.text.font_system, &self.buffer.text,
            &ui.context.font.font_attr(), Shaping::Advanced);
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.buffer.redraw(ui);
    }
}