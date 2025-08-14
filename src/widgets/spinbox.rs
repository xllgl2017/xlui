use crate::frame::context::{ContextUpdate, UpdateType};
use crate::frame::App;
use crate::response::{Callback, Response};
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::ui::Ui;
use crate::vertex::Vertex;
use crate::widgets::textedit::TextEdit;
use crate::widgets::Widget;
use crate::NumCastExt;
use std::any::Any;
use std::fmt::Display;
use std::ops::{AddAssign, Range, SubAssign};

pub struct SpinBox<T> {
    pub(crate) id: String,
    edit: TextEdit,
    rect: Rect,
    size_mode: SizeMode,
    value: T,
    gap: T,
    range: Range<T>,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui, T)>>,
    up_rect: Rect,
    up_index: Range<usize>,
    down_rect: Rect,
    down_index: Range<usize>,
    color: Color,
    inactive_color: Color,
    init: bool,
    change: bool,
    contact_ids: Vec<String>,
}

impl<T: Display + NumCastExt + 'static> SpinBox<T> {
    pub fn new(v: T, g: T, r: Range<T>) -> Self {
        let color = Color::rgb(95, 95, 95);
        let inactive_color = Color::rgb(153, 152, 152);
        SpinBox {
            id: crate::gen_unique_id(),
            edit: TextEdit::new(format!("{:.*}", 2, v)),
            rect: Rect::new(),
            size_mode: SizeMode::Auto,
            value: v,
            gap: g,
            range: r,
            callback: None,
            up_rect: Rect::new(),
            up_index: 0..1,
            down_rect: Rect::new(),
            down_index: 0..1,
            color,
            inactive_color,
            init: false,
            change: false,
            contact_ids: vec![],
        }
    }
    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn reset_size(&mut self) {
        match self.size_mode {
            SizeMode::Auto => self.rect.set_size(100.0, 25.0),
            SizeMode::FixWidth => self.rect.set_height(25.0),
            SizeMode::FixHeight => self.rect.set_width(80.0),
            SizeMode::Fix => {}
        }
        let mut edit_rect = self.rect.clone();
        edit_rect.set_x_max(edit_rect.dx().max - 18.0);
        // edit_rect.dx.max = edit_rect.dx.max - 18.0;
        self.edit.set_rect(edit_rect);
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, T)) -> Self {
        self.callback = Some(Callback::create_spinbox(f));
        self
    }

    pub fn set_callback<A: App>(&mut self, f: fn(&mut A, &mut Ui, T)) {
        self.callback = Some(Callback::create_spinbox(f));
    }

    pub fn set_value(&mut self, value: T) {
        self.change = true;
        self.value = value;
    }

    pub fn contact(mut self, id: impl ToString) -> Self {
        self.contact_ids.push(id.to_string());
        self
    }

    fn init(&mut self, ui: &mut Ui) {
        self.init = true;
        self.rect = ui.layout().available_rect().clone_with_size(&self.rect);
        self.reset_size();
        self.edit.redraw(ui);
        let mut rect = self.rect.clone();
        rect.set_width(18.0);
        // ui.layout().alloc_rect(&rect);
        self.up_rect.set_x_min(self.rect.dx().max - 14.0);
        self.up_rect.set_x_max(self.rect.dx().max);
        self.up_rect.set_y_min(self.rect.dy().min + 1.0);
        self.up_rect.set_y_max(self.rect.dy().min + self.rect.height() / 2.0 - 2.0);
        // self.up_rect =
        //     Rect {
        //         dx: Axis { min: self.rect.dx().max - 14.0, max: self.rect.dx().max },
        //         ox: Axis { min: self.rect.dx().max - 14.0, max: self.rect.dx().max },
        //         dy: Axis { min: self.rect.dy().min + 1.0, max: self.rect.dy().min + self.rect.height() / 2.0 - 2.0 },
        //         oy: Axis { min: self.rect.dy().min + 1.0, max: self.rect.dy().min + self.rect.height() / 2.0 - 2.0 },
        //     };
        let vertices = vec![
            Vertex::new([self.up_rect.dx().min + self.up_rect.width() / 2.0, self.up_rect.dy().min], &self.color, &ui.context.size),
            Vertex::new([self.up_rect.dx().min, self.up_rect.dy().max], &self.color, &ui.context.size),
            Vertex::new([self.rect.dx().max, self.up_rect.dy().max], &self.color, &ui.context.size),
        ];
        self.up_index = ui.context.render.triangle.add_triangle(vertices, &ui.device);
        self.down_rect.set_x_min(self.rect.dx().max - 14.0);
        self.down_rect.set_x_max(self.rect.dx().max);
        self.down_rect.set_y_min(self.rect.dy().max - self.rect.height() / 2.0 + 2.0);
        self.down_rect.set_y_max(self.rect.dy().max - 2.0);
        // self.down_rect = Rect {
        //     dx: Axis { min: self.rect.dx().max - 14.0, max: self.rect.dx().max },
        //     ox: Axis { min: self.rect.dx().max - 14.0, max: self.rect.dx().max },
        //     dy: Axis { min: self.rect.dy().max - self.rect.height() / 2.0 + 2.0, max: self.rect.dy().max - 2.0 },
        //     oy: Axis { min: self.rect.dy().max - self.rect.height() / 2.0 + 2.0, max: self.rect.dy().max - 2.0 },
        // };
        self.down_index = ui.context.render.triangle.add_triangle(vec![
            Vertex::new([self.down_rect.dx().min + self.down_rect.width() / 2.0, self.down_rect.dy().max], &self.color, &ui.context.size),
            Vertex::new([self.rect.dx().max - 14.0, self.down_rect.dy().min], &self.color, &ui.context.size),
            Vertex::new([self.rect.dx().max, self.down_rect.dy().min], &self.color, &ui.context.size),
        ], &ui.device);
    }
}


impl<T: PartialOrd + AddAssign + SubAssign + ToString + Copy + Display + NumCastExt + 'static> Widget for SpinBox<T> {
    fn redraw(&mut self, ui: &mut Ui) -> Response {
        if !self.init { self.init(ui); }
        if self.change {
            self.edit.update_text(ui, format!("{:.*}", 2, self.value));
        }
        let resp = Response::new(&self.id, &self.rect);
        if ui.pass.is_none() { return resp; }
        self.edit.redraw(ui);
        if ui.context.resize {
            let c = if self.value == self.range.start { self.inactive_color.as_gamma_rgba() } else { self.color.as_gamma_rgba() };
            ui.context.render.triangle.prepare(self.down_index.clone(), &ui.device, ui.context.size.as_gamma_size(), c);
            let c = if self.value == self.range.end { self.inactive_color.as_gamma_rgba() } else { self.color.as_gamma_rgba() };
            ui.context.render.triangle.prepare(self.up_index.clone(), &ui.device, ui.context.size.as_gamma_size(), c);
        }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.triangle.render(self.down_index.clone(), pass);
        ui.context.render.triangle.render(self.up_index.clone(), pass);
        resp
    }

    fn update(&mut self, ui: &mut Ui) {
        self.edit.update(ui);
        match ui.update_type {
            // UpdateType::Init => self.init(ui),
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.up_rect) {
                    let is_end = self.value >= self.range.end;
                    let is_start = self.value == self.range.start;
                    if !is_end {
                        self.value += self.gap;
                        self.edit.update_text(ui, format!("{:.*}", 2, self.value));
                        if let Some(ref mut callback) = self.callback {
                            let app = ui.app.take().unwrap();
                            callback(*app, ui, self.value);
                            ui.app.replace(app);
                        }
                        ui.send_updates(&self.contact_ids, ContextUpdate::F32(self.value.as_f32()))
                    }
                    let c = if self.value == self.range.end { self.inactive_color.as_gamma_rgba() } else { self.color.as_gamma_rgba() };
                    ui.context.render.triangle.prepare(self.up_index.clone(), &ui.device, ui.context.size.as_gamma_size(), c);
                    if is_start {
                        ui.context.render.triangle.prepare(self.down_index.clone(), &ui.device, ui.context.size.as_gamma_size(), self.color.as_gamma_rgba());
                    }
                    ui.update_type = UpdateType::None;
                    ui.context.window.request_redraw();
                } else if ui.device.device_input.click_at(&self.down_rect) {
                    let is_start = self.value == self.range.start;
                    let is_end = self.value >= self.range.end;
                    if !is_start {
                        self.value -= self.gap;
                        self.edit.update_text(ui, format!("{:.*}", 2, self.value));
                        if let Some(ref mut callback) = self.callback {
                            let app = ui.app.take().unwrap();
                            callback(*app, ui, self.value);
                            ui.app.replace(app);
                        }
                        ui.send_updates(&self.contact_ids, ContextUpdate::F32(self.value.as_f32()))
                    }
                    if is_end {
                        ui.context.render.triangle.prepare(self.up_index.clone(), &ui.device, ui.context.size.as_gamma_size(), self.color.as_gamma_rgba());
                    }
                    let c = if self.value == self.range.start { self.inactive_color.as_gamma_rgba() } else { self.color.as_gamma_rgba() };
                    ui.context.render.triangle.prepare(self.down_index.clone(), &ui.device, ui.context.size.as_gamma_size(), c);
                    ui.update_type = UpdateType::None;
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_t(&mut self.value);
            self.edit.update_text(ui, format!("{:.*}", 2, self.value));
            ui.context.window.request_redraw();
        }
    }
}