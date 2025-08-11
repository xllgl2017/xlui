use std::sync::{Arc, RwLock};
use crate::frame::context::Context;
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::Response;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;

pub struct SelectItem<T> {
    id: String,
    text: TextBuffer,
    padding: Padding,
    size_mode: SizeMode,
    value: Option<T>,
    selected: Arc<RwLock<Option<T>>>,

    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,

    callback: Option<Box<dyn FnMut(&mut Option<T>)>>,
    hovered: bool,
}

impl<T> SelectItem<T> {
    pub fn new(text: impl ToString) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::TRANSPARENT;
        fill_style.fill.hovered = Color::rgba(153, 193, 241, 220);
        fill_style.fill.clicked = Color::rgba(153, 193, 241, 220);
        fill_style.border.inactive = Border::new(0.0);
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        SelectItem {
            id: crate::gen_unique_id(),
            text: TextBuffer::new(text.to_string()),
            padding: Padding::same(2.0),
            size_mode: SizeMode::Auto,
            value: None,
            selected: Arc::new(RwLock::new(None)),
            fill_param: RectParam::new(Rect::new(), fill_style),
            fill_index: 0,
            fill_buffer: None,
            callback: None,
            hovered: false,
        }
    }

    pub(crate) fn reset_size(&mut self, context: &Context) {
        self.text.reset_size(&context);
        match self.size_mode {
            SizeMode::Auto => {
                let width = self.text.rect.width() + self.padding.horizontal();
                let height = self.text.rect.height() + self.padding.vertical();
                self.fill_param.rect.set_size(width, height);
            }
            SizeMode::FixWidth => self.fill_param.rect.set_height(self.text.rect.height()),
            SizeMode::FixHeight => self.fill_param.rect.set_width(self.text.rect.width()),
            SizeMode::Fix => {}
        }
        self.text.rect = self.fill_param.rect.clone_add_padding(&self.padding);
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.fill_param.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
    }

    pub(crate) fn set_callback(&mut self, f: impl FnMut(&mut Option<T>) + 'static) {
        self.callback = Some(Box::new(f));
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub(crate) fn parent(mut self, parent: Arc<RwLock<Option<T>>>) -> Self {
        self.selected = parent;
        self
    }
}

impl<T: PartialEq + 'static> Widget for SelectItem<T> {
    fn draw(&mut self, ui: &mut Ui) -> Response {
        self.fill_param.rect = ui.layout().available_rect().clone_with_size(&self.fill_param.rect);
        self.reset_size(&ui.context);
        //背景
        let data = self.fill_param.as_draw_param(self.value.is_some(), self.value.is_some());
        let fill_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        self.fill_buffer = Some(fill_buffer);
        //
        self.text.draw(ui);
        Response {
            id: self.id.clone(),
            rect: self.fill_param.rect.clone(),
        }
    }

    fn update(&mut self, ui: &mut Ui) {
        if let Some(ref offset) = ui.canvas_offset {
            self.fill_param.rect.offset(offset.x, offset.y);
            let data = self.fill_param.as_draw_param(self.value.is_some(), self.value.is_some());
            ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
            self.text.rect.offset(offset.x, offset.y);
            return;
        }
        let selected = self.selected.read().unwrap();
        if *selected != self.value {
            self.value = None;
            let data = self.fill_param.as_draw_param(self.value.is_some(), self.value.is_some());
            ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
        }
        drop(selected);
        let out = self.fill_param.rect.out_of_border(&ui.current_rect);
        let clicked = ui.device.device_input.click_at(&self.fill_param.rect);
        if clicked && !out {
            self.hovered = true;
            if let Some(ref mut callback) = self.callback {
                callback(&mut self.value);
            }
            let data = self.fill_param.as_draw_param(self.value.is_some(), self.value.is_some());
            ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
            ui.context.window.request_redraw();
            return;
        }
        let hovered = ui.device.device_input.hovered_at(&self.fill_param.rect);
        if self.hovered != hovered {
            self.hovered = hovered;
            let data = self.fill_param.as_draw_param(self.hovered || self.value.is_some(), ui.device.device_input.mouse.pressed || self.value.is_some());
            ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
            ui.context.window.request_redraw();
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        self.text.redraw(ui);
    }
}