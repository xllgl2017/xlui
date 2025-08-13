use crate::frame::context::UpdateType;
use crate::response::Response;
use crate::text::text_buffer::TextBuffer;
use crate::text::TextWrap;
use crate::ui::Ui;
use crate::widgets::Widget;

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

    fn init(&mut self, ui: &mut Ui) {
        self.buffer.rect = ui.layout().available_rect().clone_with_size(&self.buffer.rect);
        self.buffer.reset_size(ui.context);
        self.buffer.draw(ui);
    }
}


impl Widget for Label {
    fn redraw(&mut self, ui: &mut Ui) -> Response {
        if self.buffer.buffer.is_none() { self.init(ui); }
        if ui.pass.is_none() { return Response::new(&self.id, &self.buffer.rect); }
        self.buffer.redraw(ui);
        Response::new(&self.id, &self.buffer.rect)
    }


    fn update(&mut self, ui: &mut Ui) { //处理鼠标键盘时间
        match &ui.update_type {
            // UpdateType::Init => self.init(ui),
            UpdateType::Offset(o) => self.buffer.rect.offset(o.x, o.y),
            _ => {}
        }
        if !self.change { return; }
        self.buffer.set_text(self.buffer.text.clone(), ui);
    }
}