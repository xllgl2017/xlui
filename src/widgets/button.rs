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

use crate::frame::context::Context;
use crate::frame::App;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::{Callback, Response};
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::image::Image;
use crate::widgets::Widget;
use std::any::Any;

pub struct Button {
    pub(crate) id: String,
    pub(crate) text_buffer: TextBuffer,
    padding: Padding,
    size_mode: SizeMode,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui)>>,
    fill_index: usize,
    fill_param: RectParam,
    fill_buffer: Option<wgpu::Buffer>,
    image: Option<Image>,
    image_rect: Rect,
    hovered: bool,

}


impl Button {
    pub fn new(text: impl ToString) -> Self {
        let padding = Padding::same(2.0);
        let text_buffer = TextBuffer::new(text.to_string());
        Button {
            id: crate::gen_unique_id(),
            text_buffer,
            padding,
            size_mode: SizeMode::Auto,
            callback: None,
            fill_index: 0,
            fill_param: RectParam::new(Rect::new(), ClickStyle::new()),
            fill_buffer: None,
            image: None,
            image_rect: Rect::new(),
            hovered: false,
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
                self.fill_param.rect.set_size(width, height);
            }
            SizeMode::FixWidth => self.fill_param.rect.set_height(self.text_buffer.rect.height()),
            SizeMode::FixHeight => self.fill_param.rect.set_width(self.text_buffer.rect.width()),
            SizeMode::Fix => {
                self.text_buffer.rect = self.fill_param.rect.clone_add_padding(&self.padding);
                println!("text {:?}", self.text_buffer.rect);
            }
        }
        if self.image.is_some() {
            self.fill_param.rect.set_width(self.fill_param.rect.width() + self.fill_param.rect.height());
            self.text_buffer.rect = self.fill_param.rect.clone_add_padding(&self.padding);
            self.text_buffer.rect.offset_x(self.fill_param.rect.height());
            self.image_rect = self.fill_param.rect.clone_add_padding(&self.padding);
            self.image_rect.offset(self.padding.left, self.padding.top);
            self.image_rect.set_width(self.image_rect.height() - self.padding.vertical());
            self.image_rect.set_height(self.image_rect.height() - self.padding.vertical());
        } else {
            self.text_buffer.rect = self.fill_param.rect.clone_add_padding(&self.padding);
        }
    }


    pub fn set_width(&mut self, width: f32) {
        self.fill_param.rect.set_width(width);
        self.size_mode.fix_width();
    }

    pub fn set_height(&mut self, height: f32) {
        self.fill_param.rect.set_height(height);
        self.size_mode.fix_height();
    }


    pub fn set_size(&mut self, width: f32, height: f32) {
        self.fill_param.rect.set_size(width, height);
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

    pub fn connect<A: App>(mut self, f: impl FnMut(&mut A, &mut Ui) + 'static) -> Self {
        self.callback = Some(Callback::create_click(f));
        self
    }

    pub fn set_callback<A: App>(&mut self, f: impl FnMut(&mut A, &mut Ui) + 'static) {
        self.callback = Some(Callback::create_click(f));
    }

    pub fn with_style(mut self, style: ClickStyle) -> Self {
        self.fill_param.style = style;
        self
    }
}


impl Widget for Button {
    fn draw(&mut self, ui: &mut Ui) -> Response {
        self.fill_param.rect = ui.layout().available_rect().clone_with_size(&self.fill_param.rect);
        self.reset_size(&ui.context);
        //按钮矩形
        let data = self.fill_param.as_draw_param(false, false);
        let buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &buffer);
        self.fill_buffer = Some(buffer);
        //
        if let Some(ref mut image) = self.image {
            image.draw(ui);
            image.rect = self.image_rect.clone();
        }
        //按钮文本
        self.text_buffer.draw(ui);
        Response {
            id: self.id.clone(),
            rect: self.fill_param.rect.clone(),
        }
    }

    fn update(&mut self, ui: &mut Ui) {
        if let Some(ref mut image) = self.image {
            image.update(ui);
        }
        if let Some(ref offset) = ui.canvas_offset {
            self.fill_param.rect.offset(offset.x, offset.y);
            let data = self.fill_param.as_draw_param(false, false);
            ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
            self.text_buffer.rect.offset(offset.x, offset.y);
        }
        // let (x, y) = ui.device.device_input.mouse.lastest();
        let has_pos = ui.device.device_input.hovered_at(&self.fill_param.rect);
        if self.hovered != has_pos {
            self.hovered = has_pos;
            let data = self.fill_param.as_draw_param(self.hovered, ui.device.device_input.mouse.pressed);
            ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
            ui.context.window.request_redraw();
        }
        if ui.device.device_input.click_at(&self.fill_param.rect) {
            if let Some(ref mut callback) = self.callback {
                let app = ui.app.take().unwrap();
                callback(*app, ui);
                ui.app.replace(app);
                ui.context.window.request_redraw();
            }
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        if let Some(ref mut image) = self.image {
            image.redraw(ui);
        }
        self.text_buffer.redraw(ui);
    }
}