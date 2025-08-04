use crate::align::Align;
use crate::frame::context::Context;
use crate::paint::button::PaintButton;
use crate::paint::PaintTask;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::text::text_buffer::TextBuffer;
use crate::ui::{Ui, UiM};
use std::any::Any;
use crate::response::button::ButtonResponse;
use crate::response::{Callback, DrawnEvent};
use crate::widgets::Widget;

pub struct Button {
    pub(crate) id: String,
    text_buffer: TextBuffer,
    text_algin: Align,
    pub(crate) rect: Rect,
    padding: Padding,
    pub(crate) border: Border,
    size_mode: SizeMode,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM)>>,
}


impl Button {
    pub fn new(text: String) -> Self {
        let padding = Padding::same(2.0);
        let text_buffer = TextBuffer::new(text);
        Button {
            id: crate::gen_unique_id(),
            text_buffer,
            text_algin: Align::Center,
            rect: Rect::new(),
            padding,
            border: Border::new(1),
            size_mode: SizeMode::Auto,
            callback: None,
        }
    }

    pub(crate) fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(&context);
        match self.size_mode {
            SizeMode::Auto => {
                let width = self.text_buffer.rect.width() + self.padding.left + self.padding.right;
                let height = self.text_buffer.rect.height() + self.padding.top + self.padding.bottom;
                self.rect.set_size(width, height);
            }
            SizeMode::FixWidth => self.rect.set_height(self.text_buffer.rect.height()),
            SizeMode::FixHeight => self.rect.set_width(self.text_buffer.rect.width()),
            SizeMode::Fix => {
                self.text_buffer.rect = self.rect.clone_add_padding(&self.padding);
                println!("{:?}", self.text_buffer.rect);
            }
        }
        self.text_buffer.rect = self.rect.clone_add_padding(&self.padding);
    }


    pub fn set_width(&mut self, width: f32) {
        self.rect.set_width(width);
        self.size_mode.fix_width();
    }

    pub fn set_height(&mut self, height: f32) {
        self.rect.set_height(height);
        self.size_mode.fix_height();
    }


    pub fn set_size(&mut self, width: f32, height: f32) {
        self.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.text_buffer.text_size.font_size = font_size;
    }

    // pub(crate) fn id(&self) -> &String { &self.id }

    pub fn width(mut self, w: f32) -> Self {
        self.set_width(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.set_height(h);
        self
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut UiM)) -> Self {
        self.callback = Some(Callback::create_click(f));
        self
    }
}


impl Widget for Button {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();

        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.rect);
        //按钮矩形
        let task = PaintButton::new(ui, self, &self.text_buffer);
        ui.add_paint_task(self.id.clone(), PaintTask::Button(task));
        ui.response.insert(self.id.clone(), ButtonResponse {
            rect: self.rect.clone(),
            event: DrawnEvent::Click,
            callback: Callback::click(self.callback.take()),
        });
    }

    fn update(&mut self, uim: &mut UiM) {
        self.text_buffer.update(uim);
    }
}