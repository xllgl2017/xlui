use std::fmt::Display;
use std::sync::{Arc, RwLock};
use crate::frame::context::{Context, UpdateType};
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
    pub(crate) id: String,
    text: TextBuffer,
    padding: Padding,
    size_mode: SizeMode,
    value: T,
    parent_selected: Arc<RwLock<Option<String>>>,

    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,

    callback: Option<Box<dyn FnMut(&mut Option<T>)>>,
    hovered: bool,
    selected: bool,

}

impl<T: Display> SelectItem<T> {
    pub fn new(value: T) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::TRANSPARENT;
        fill_style.fill.hovered = Color::rgba(153, 193, 241, 220);
        fill_style.fill.clicked = Color::rgba(153, 193, 241, 220);
        fill_style.border.inactive = Border::new(0.0);
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        SelectItem {
            id: crate::gen_unique_id(),
            text: TextBuffer::new(value.to_string()),
            padding: Padding::same(2.0),
            size_mode: SizeMode::Auto,
            value,
            parent_selected: Arc::new(RwLock::new(None)),
            fill_param: RectParam::new(Rect::new(), fill_style),
            fill_index: 0,
            fill_buffer: None,
            callback: None,
            hovered: false,
            selected: false,
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

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.set_size(w, h);
        self
    }

    pub(crate) fn set_callback(&mut self, f: impl FnMut(&mut Option<T>) + 'static) {
        self.callback = Some(Box::new(f));
    }

    pub fn connect(mut self, f: impl FnMut(&mut Option<T>) + 'static) -> Self {
        self.callback = Some(Box::new(f));
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn contact(mut self, parent: Arc<RwLock<Option<String>>>) -> Self {
        self.parent_selected = parent;
        self
    }

    pub fn need_contact(&self) -> Arc<RwLock<Option<String>>> {
        self.parent_selected.clone()
    }

    fn init(&mut self, ui: &mut Ui) {
        self.fill_param.rect = ui.layout().available_rect().clone_with_size(&self.fill_param.rect);
        self.reset_size(&ui.context);
        //背景
        let current = self.parent_selected.read().unwrap();
        let selected = current.as_ref() == Some(&self.value.to_string());
        let data = self.fill_param.as_draw_param(selected, selected);
        let fill_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        self.fill_buffer = Some(fill_buffer);
        //
        self.text.draw(ui);
    }
}

impl<T: PartialEq + Display + 'static> Widget for SelectItem<T> {
    fn redraw(&mut self, ui: &mut Ui) -> Response {
        if self.fill_buffer.is_none() { self.init(ui); }
        let current = self.parent_selected.read().unwrap();
        let selected = current.as_ref() == Some(&self.value.to_string());
        if !selected && self.selected {
            self.selected = false;
            let data = self.fill_param.as_draw_param(false, false);
            ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
        }
        drop(current);
        let resp = Response::new(&self.id, &self.fill_param.rect);
        if ui.pass.is_none() { return resp; }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        self.text.redraw(ui);
        resp
    }

    fn update(&mut self, ui: &mut Ui) {
        match ui.update_type {
            UpdateType::None => {}
            // UpdateType::Init => self.init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.fill_param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    let current = self.parent_selected.read().unwrap();
                    let selected = current.as_ref() == Some(&self.value.to_string());
                    let data = self.fill_param.as_draw_param(self.hovered || self.selected, ui.device.device_input.mouse.pressed || selected);
                    ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MousePress => {}
            UpdateType::MouseRelease => {
                let out = self.fill_param.rect.out_of_border(&ui.current_rect) && false;
                let clicked = ui.device.device_input.click_at(&self.fill_param.rect);
                if clicked && !out {
                    self.selected = true;
                    let mut selected = self.parent_selected.write().unwrap();
                    *selected = Some(self.value.to_string());
                    let data = self.fill_param.as_draw_param(true, true);
                    ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
                    ui.update_type=UpdateType::None;
                    ui.context.window.request_redraw();
                    return;
                }
            }
            UpdateType::MouseWheel => {}
            UpdateType::KeyRelease(_) => {}
            UpdateType::Offset(ref o) => {
                let current = self.parent_selected.read().unwrap();
                let selected = current.as_ref() == Some(&self.value.to_string());
                self.fill_param.rect.offset(o.x, o.y);
                let data = self.fill_param.as_draw_param(selected, selected);
                ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
                self.text.rect.offset(o.x, o.y);
                ui.context.window.request_redraw();
                return;
            }
        }
        // if let Some(ref offset) = ui.canvas_offset {
        //
        // }

        // if !selected && self.selected {
        //     self.selected = false;
        //     let data = self.fill_param.as_draw_param(false, false);
        //     ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
        //     ui.context.window.request_redraw();
        // }
        // drop(current);


    }
}