//! ### Slider的示例用法
//! ```
//! use xlui::frame::App;
//! use xlui::ui::Ui;
//! use xlui::widgets::spinbox::SpinBox;
//!
//! fn spinbox_changed<A:App>(_:&mut A,_:&mut Ui,v:f32){
//!     println!("SpinBox改变了:{}",v);
//! }
//!
//! fn draw<A:App>(ui:&mut Ui){
//!     //快速创建一个Slider
//!     ui.spinbox(10.0,1.0,0.0..100.0)
//!         //设置回调函数
//!         .set_callback(spinbox_changed::<A>);
//!     let spinbox=SpinBox::new(10.0,1.0,0.0..100.0)
//!         //关联ID为my_slider的控件
//!         .contact("my_slider")
//!         //连接到Slider值监听函数
//!         .connect(spinbox_changed::<A>)
//!         //设置控件ID
//!         .id("my_spinbox");
//!     ui.add(spinbox);
//! }
//! ```
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
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, T)>>,
    up_rect: Rect,
    up_index: Range<usize>,
    down_rect: Rect,
    down_index: Range<usize>,
    color: Color,
    inactive_color: Color,
    init: bool,
    changed: bool,
    contact_ids: Vec<String>,

    press_up: bool,
    press_down: bool,
    press_time: u128,
}

impl<T: PartialOrd + AddAssign + SubAssign + ToString + Copy + Display + NumCastExt + 'static> SpinBox<T> {
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
            changed: false,
            contact_ids: vec![],
            press_up: false,
            press_down: false,
            press_time: 0,
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
        self.changed = true;
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
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        self.edit.update(ui);
        let mut rect = self.rect.clone();
        rect.set_width(18.0);
        // ui.layout().alloc_rect(&rect);
        self.up_rect.set_x_min(self.rect.dx().max - 14.0);
        self.up_rect.set_x_max(self.rect.dx().max);
        self.up_rect.set_y_min(self.rect.dy().min + 1.0);
        self.up_rect.set_y_max(self.rect.dy().min + self.rect.height() / 2.0 - 2.0);
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
        self.down_index = ui.context.render.triangle.add_triangle(vec![
            Vertex::new([self.down_rect.dx().min + self.down_rect.width() / 2.0, self.down_rect.dy().max], &self.color, &ui.context.size),
            Vertex::new([self.rect.dx().max - 14.0, self.down_rect.dy().min], &self.color, &ui.context.size),
            Vertex::new([self.rect.dx().max, self.down_rect.dy().min], &self.color, &ui.context.size),
        ], &ui.device);
    }

    // fn update_value(&mut self, ui: &mut Ui) {
    //     let c = if self.value <= self.range.start {
    //         self.value = self.range.start;
    //         self.inactive_color.as_gamma_rgba()
    //     } else {
    //         self.color.as_gamma_rgba()
    //     };
    //     ui.context.render.triangle.prepare(self.down_index.clone(), &ui.device, ui.context.size.as_gamma_size(), c);
    //     let c = if self.value >= self.range.end {
    //         self.value = self.range.end;
    //         self.inactive_color.as_gamma_rgba()
    //     } else {
    //         self.color.as_gamma_rgba()
    //     };
    //     ui.context.render.triangle.prepare(self.up_index.clone(), &ui.device, ui.context.size.as_gamma_size(), c);
    //     self.edit.update_text(ui, format!("{:.*}", 2, self.value));
    // }

    fn call(&mut self, ui: &mut Ui) {
        if let Some(ref mut callback) = self.callback {
            let app = ui.app.take().unwrap();
            callback(app, ui, self.value);
            ui.app.replace(app);
        }
    }

    fn click_up(&mut self, ui: &mut Ui) {
        let is_end = self.value >= self.range.end;
        if !is_end {
            self.value += self.gap;
            if self.value > self.range.end { self.value = self.range.end }
            self.call(ui);
            ui.send_updates(&self.contact_ids, ContextUpdate::F32(self.value.as_f32()))
        }
        self.changed = true;
        ui.update_type = UpdateType::None;
        ui.context.window.request_redraw();
    }

    fn click_down(&mut self, ui: &mut Ui) {
        let is_start = self.value <= self.range.start;
        if !is_start {
            self.value -= self.gap;
            if self.value < self.range.start { self.value = self.range.start }
            self.call(ui);
            ui.send_updates(&self.contact_ids, ContextUpdate::F32(self.value.as_f32()))
        }
        self.changed = true;
        ui.update_type = UpdateType::None;
        ui.context.window.request_redraw();
    }

    fn listen_input(&mut self, ui: &mut Ui, st: u64) {
        let event = ui.context.event.clone();
        let wid = ui.context.window.id();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(st));
            event.send_event((wid, UpdateType::None)).unwrap();
        });
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.changed && !ui.context.resize { return; }
        self.changed = false;
        let c = if self.value <= self.range.start {
            self.value = self.range.start;
            self.inactive_color.as_gamma_rgba()
        } else {
            self.color.as_gamma_rgba()
        };
        ui.context.render.triangle.prepare(self.down_index.clone(), &ui.device, ui.context.size.as_gamma_size(), c);
        let c = if self.value >= self.range.end {
            self.value = self.range.end;
            self.inactive_color.as_gamma_rgba()
        } else {
            self.color.as_gamma_rgba()
        };
        ui.context.render.triangle.prepare(self.up_index.clone(), &ui.device, ui.context.size.as_gamma_size(), c);
        self.edit.update_text(ui, format!("{:.*}", 2, self.value));
    }
}


impl<T: PartialOrd + AddAssign + SubAssign + ToString + Copy + Display + NumCastExt + 'static> Widget for SpinBox<T> {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.edit.redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.triangle.render(self.down_index.clone(), pass);
        ui.context.render.triangle.render(self.up_index.clone(), pass);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MousePress => {
                if ui.device.device_input.pressed_at(&self.down_rect) {
                    println!("press down");
                    self.press_down = true;
                    self.press_time = crate::time_ms();
                    self.listen_input(ui, 500);
                } else if ui.device.device_input.pressed_at(&self.up_rect) {
                    println!("press up");
                    self.press_up = true;
                    self.press_time = crate::time_ms();
                    self.listen_input(ui, 500);
                } else {
                    self.press_down = false;
                    self.press_up = false;
                    self.press_time = 0;
                    let focused = self.edit.focused;
                    self.edit.update(ui);
                    if focused && self.edit.focused != focused {
                        let v = self.edit.text().parse::<f32>().unwrap_or(self.value.as_f32());
                        self.value = T::from_num(v);
                        self.changed = true;
                        // self.update_value(ui);
                        ui.send_updates(&self.contact_ids, ContextUpdate::F32(self.value.as_f32()));
                        ui.context.window.request_redraw();
                    }
                }
                return Response::new(&self.id, &self.rect);
            }
            UpdateType::MouseRelease => {
                self.press_up = false;
                self.press_down = false;
                self.press_time = 0;
                if ui.device.device_input.click_at(&self.up_rect) {
                    self.click_up(ui)
                } else if ui.device.device_input.click_at(&self.down_rect) {
                    self.click_down(ui);
                }
                return Response::new(&self.id, &self.rect);
            }
            UpdateType::KeyRelease(_) => {
                if !self.edit.focused { return Response::new(&self.id, &self.rect); }
                self.edit.update(ui);

                return Response::new(&self.id, &self.rect);
            }
            UpdateType::None => {
                if self.press_up && crate::time_ms() - self.press_time >= 500 {
                    self.click_up(ui);
                    self.listen_input(ui, 100);
                } else if self.press_down && crate::time_ms() - self.press_time >= 500 {
                    self.click_down(ui);
                    self.listen_input(ui, 100);
                }
            }
            UpdateType::Offset(ref o) => {
                if !ui.can_offset { return Response::new(&self.id, &self.rect); }
                self.rect.offset(o);
                self.up_rect.offset(o);
                self.down_rect.offset(o);
                ui.context.render.triangle.offset(self.up_index.clone(), o);
                ui.context.render.triangle.offset(self.down_index.clone(), o);
                self.changed = true;
            }
            _ => {}
        }
        self.edit.update(ui);
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_t(&mut self.value);
            self.changed = true;
            // self.update_value(ui);
            ui.context.window.request_redraw();
        }
        Response::new(&self.id, &self.rect)
    }
}