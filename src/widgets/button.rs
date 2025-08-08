//!```
//! use xlui::ui::Ui;
//! use xlui::widgets::button::Button;
//! use xlui::widgets::Widget;
//!
//! # xlui::_run_test(|ui|{
//!    let mut btn=Button::new("hello button");
//!    btn.draw(ui);
//! # });
//! ```

use crate::align::Align;
use crate::frame::context::Context;
use crate::frame::App;
use crate::paint::button::PaintButton;
use crate::paint::PaintTask;
use crate::response::Callback;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;
use std::any::Any;
use crate::widgets::image::Image;

pub struct Button {
    pub(crate) id: String,
    pub(crate) text_buffer: TextBuffer,
    text_algin: Align,
    pub(crate) rect: Rect,
    padding: Padding,
    pub(crate) border: Border,
    size_mode: SizeMode,
    pub(crate) callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context)>>,
    pub(crate) image: Option<Image>,
}


impl Button {
    pub fn new(text: impl ToString) -> Self {
        let padding = Padding::same(2.0);
        let text_buffer = TextBuffer::new(text.to_string());
        Button {
            id: crate::gen_unique_id(),
            text_buffer,
            text_algin: Align::Center,
            rect: Rect::new(),
            padding,
            border: Border::new(1.0),
            size_mode: SizeMode::Auto,
            callback: None,
            image: None,
        }
    }

    pub fn image_and_text(source: &'static str, text: impl ToString) -> Self {
        let mut res = Button::new(text);
        res.image = Some(Image::new(source));
        res
    }

    pub(crate) fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(&context);
        match self.size_mode {
            SizeMode::Auto => {
                let width = self.text_buffer.rect.width() + self.padding.horizontal();
                let height = self.text_buffer.rect.height() + self.padding.vertical();
                self.rect.set_size(width, height);
            }
            SizeMode::FixWidth => self.rect.set_height(self.text_buffer.rect.height()),
            SizeMode::FixHeight => self.rect.set_width(self.text_buffer.rect.width()),
            SizeMode::Fix => {
                self.text_buffer.rect = self.rect.clone_add_padding(&self.padding);
                println!("text {:?}", self.text_buffer.rect);
            }
        }
        if self.image.is_some() {
            self.rect.set_width(self.rect.width() + self.rect.height());
            self.text_buffer.rect = self.rect.clone_add_padding(&self.padding);
            self.text_buffer.rect.offset_x(self.rect.height());
            let mut image_rect = self.rect.clone_add_padding(&self.padding);
            image_rect.offset(self.padding.left, self.padding.top);
            image_rect.set_width(image_rect.height() - self.padding.vertical());
            image_rect.set_height(image_rect.height() - self.padding.vertical());
            self.image.as_mut().unwrap().rect = image_rect;
        } else {
            self.text_buffer.rect = self.rect.clone_add_padding(&self.padding);
        }
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

    pub fn width(mut self, w: f32) -> Self {
        self.set_width(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.set_height(h);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn connect<A: App>(mut self, f: impl FnMut(&mut A, &mut Context) + 'static) -> Self {
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
        let task = PaintButton::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::Button(task));
    }

    fn update(&mut self, ctx: &mut Context) {
        self.text_buffer.update(ctx);
    }
}