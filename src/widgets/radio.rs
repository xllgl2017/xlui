//! ```
//! # use xlui::widgets::radio::RadioButton;
//! use xlui::widgets::Widget;
//! # xlui::_run_test(|ui|{
//! let mut btn=RadioButton::new(false,"radio");
//! btn.redraw(ui);
//! #  });

use crate::frame::context::{Context, ContextUpdate, UpdateType};
use crate::render::circle::param::CircleParam;
use crate::render::WrcRender;
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;
use std::any::Any;
use crate::frame::App;

pub struct RadioButton {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) value: bool,
    pub(crate) text: TextBuffer,
    pub(crate) callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui, bool)>>,
    size_mode: SizeMode,

    outer_param: CircleParam,
    outer_index: usize,
    outer_buffer: Option<wgpu::Buffer>,

    inner_param: CircleParam,
    inner_index: usize,
    inner_buffer: Option<wgpu::Buffer>,

    hovered: bool,
    contact_ids: Vec<String>,
}

impl RadioButton {
    pub fn new(v: bool, label: impl ToString) -> RadioButton {
        let mut outer_style = ClickStyle::new();
        outer_style.fill.inactive = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.fill.hovered = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.fill.clicked = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.border.inactive = Border::new(1.0).color(Color::rgb(95, 95, 95));
        outer_style.border.hovered = Border::new(1.0).color(Color::rgb(56, 160, 200));
        outer_style.border.clicked = Border::new(1.0).color(Color::rgb(56, 182, 244));

        let mut inner_style = ClickStyle::new();
        inner_style.fill.inactive = Color::TRANSPARENT;
        inner_style.fill.hovered = Color::rgb(56, 160, 200);
        inner_style.fill.clicked = Color::rgb(56, 182, 244);
        inner_style.border.inactive = Border::new(0.0).color(Color::TRANSPARENT);
        inner_style.border.hovered = Border::new(0.0).color(Color::TRANSPARENT);
        inner_style.border.clicked = Border::new(0.0).color(Color::TRANSPARENT);
        RadioButton {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            value: v,
            text: TextBuffer::new(label.to_string()),
            callback: None,
            size_mode: SizeMode::Auto,
            outer_param: CircleParam::new(Rect::new(), outer_style),
            outer_index: 0,
            outer_buffer: None,
            inner_param: CircleParam::new(Rect::new(), inner_style),
            inner_index: 0,
            inner_buffer: None,
            hovered: false,
            contact_ids: vec![],
        }
    }
    fn reset_size(&mut self, context: &Context) {
        self.rect.set_height(16.0);
        self.text.rect = self.rect.clone();
        self.text.rect.offset_x(18.0);
        self.text.reset_size(context);
        match self.size_mode {
            SizeMode::Auto => self.rect.set_width(18.0 + self.text.rect.width()),
            SizeMode::FixWidth => {}
            SizeMode::FixHeight => self.rect.set_width(18.0 + self.text.rect.width()),
            SizeMode::Fix => {}
        }
    }

    pub fn with_width(mut self, width: f32) -> RadioButton {
        self.rect.set_width(width);
        self.size_mode = SizeMode::FixWidth;
        self
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, bool)) -> Self {
        self.callback = Some(Callback::create_check(f));
        self
    }

    pub fn set_callback<A: App>(&mut self, f: fn(&mut A, &mut Ui, bool)) {
        self.callback = Some(Callback::create_check(f));
    }

    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn contact(mut self, id: impl ToString) -> Self {
        self.contact_ids.push(id.to_string());
        self
    }

    fn init(&mut self, ui: &mut Ui) {
        //分配大小
        self.rect = ui.layout().available_rect().clone_with_size(&self.rect);
        self.reset_size(&ui.context);
        //外圆
        self.outer_param.rect = self.rect.clone();
        self.outer_param.rect.set_width(self.rect.height());
        let data = self.outer_param.as_draw_param(self.value, self.value);
        let outer_buffer = ui.context.render.circle.create_buffer(&ui.device, data);
        self.outer_index = ui.context.render.circle.create_bind_group(&ui.device, &outer_buffer);
        self.outer_buffer = Some(outer_buffer);
        //内圆
        self.inner_param.rect = self.rect.clone();

        // self.inner_param.rect.dx.min += 4.0;
        self.inner_param.rect.add_min_x(4.0);
        self.inner_param.rect.contract_y(4.0);
        // self.inner_param.rect.dy.min += 4.0;
        // self.inner_param.rect.dy.max -= 4.0;
        self.inner_param.rect.set_width(self.inner_param.rect.height());
        let data = self.inner_param.as_draw_param(self.value, self.value);
        let inner_buffer = ui.context.render.circle.create_buffer(&ui.device, data);
        self.inner_index = ui.context.render.circle.create_bind_group(&ui.device, &inner_buffer);
        self.inner_buffer = Some(inner_buffer);
        //文本
        self.text.draw(ui);
    }

    fn update_radio(&mut self, ui: &mut Ui) {
        let data = self.outer_param.as_draw_param(self.hovered || self.value, ui.device.device_input.mouse.pressed || self.value);
        ui.device.queue.write_buffer(self.outer_buffer.as_ref().unwrap(), 0, data);
        let data = self.inner_param.as_draw_param(self.value, ui.device.device_input.mouse.pressed || self.value);
        ui.device.queue.write_buffer(self.inner_buffer.as_ref().unwrap(), 0, data);
        ui.context.window.request_redraw();
    }
}


impl Widget for RadioButton {
    fn redraw(&mut self, ui: &mut Ui) -> Response {
        if self.outer_buffer.is_none() { self.init(ui); }
        let resp = Response::new(&self.id, &self.rect);
        if ui.pass.is_none() { return resp; }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.circle.render(self.outer_index, pass);
        ui.context.render.circle.render(self.inner_index, pass);
        self.text.redraw(ui);
        resp
    }

    fn update(&mut self, ui: &mut Ui) {
        match ui.update_type {
            // UpdateType::Init => self.init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.rect);
                if hovered != self.hovered {
                    self.hovered = hovered;
                    self.update_radio(ui);
                }
            }
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.rect) {
                    self.value = !self.value;
                    self.update_radio(ui);
                    // let data = self.outer_param.as_draw_param(self.value, self.value);
                    // ui.device.queue.write_buffer(self.outer_buffer.as_ref().unwrap(), 0, data);
                    // let data = self.inner_param.as_draw_param(self.value, self.value);
                    // ui.device.queue.write_buffer(self.inner_buffer.as_ref().unwrap(), 0, data);
                    if let Some(ref mut callback) = self.callback {
                        let app = ui.app.take().unwrap();
                        callback(*app, ui, self.value);
                        ui.app.replace(app);
                    }
                    ui.update_type = UpdateType::None;
                    ui.send_updates(&self.contact_ids, ContextUpdate::Bool(self.value));
                    // ui.context.window.request_redraw();
                    return;
                }
            }
            _ => {}
        }
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_bool(&mut self.value);
            self.update_radio(ui);
        }
    }
}