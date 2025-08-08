use crate::frame::context::Context;
use crate::text::text_buffer::TextBuffer;
use crate::text::TextWrap;
use crate::ui::Ui;
use crate::widgets::Widget;


pub struct Label {
    pub(crate) text_buffer: TextBuffer,
}

impl Label {
    pub fn new(text: impl ToString) -> Label {
        let buffer = TextBuffer::new(text.to_string());
        Label {
            text_buffer: buffer,
        }
    }

    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.text_buffer.set_wrap(wrap);
        self
    }


    pub fn set_text(&mut self, text: String) {
        self.text_buffer.set_text(text);
    }

    pub fn width(mut self, w: f32) -> Self {
        self.text_buffer.set_width(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.text_buffer.set_height(h);
        self
    }

    pub fn size(mut self, s: f32) -> Self {
        self.text_buffer.text_size.font_size = s;
        self
    }
}


impl Widget for Label {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.text_buffer.rect = layout.available_rect.clone_with_size(&self.text_buffer.rect);
        self.text_buffer.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.text_buffer.rect);
        self.text_buffer.draw(ui); //创建绘制任务并计算需绘制的宽高
    }


    fn update(&mut self, ctx: &mut Context) {
        self.text_buffer.update(ctx);
    }
}