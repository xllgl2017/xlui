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
use crate::render::triangle::param::TriangleParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle};
use crate::ui::Ui;
use crate::widgets::Widget;
use crate::NumCastExt;
use std::fmt::Display;
use std::ops::{AddAssign, Range, SubAssign};
use crate::widgets::textedit::single::SingleEdit;

pub struct SpinBox<T> {
    pub(crate) id: String,
    edit: SingleEdit,
    rect: Rect,
    size_mode: SizeMode,
    value: T,
    gap: T,
    range: Range<T>,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, T)>>,
    up_render: RenderParam<TriangleParam>,
    down_render: RenderParam<TriangleParam>,
    up_rect: Rect,
    down_rect: Rect,
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
        let mut style = ClickStyle::new();
        style.fill.inactive = color;
        style.fill.hovered = inactive_color;
        style.border = BorderStyle::same(Border::new(0.0));
        SpinBox {
            id: crate::gen_unique_id(),
            edit: SingleEdit::new(format!("{:.*}", 2, v)),
            rect: Rect::new(),
            size_mode: SizeMode::Auto,
            value: v,
            gap: g,
            range: r,
            callback: None,
            up_render: RenderParam::new(TriangleParam::new([0.0; 2], [0.0; 2], [0.0; 2], style.clone())),
            down_render: RenderParam::new(TriangleParam::new([0.0; 2], [0.0; 2], [0.0; 2], style)),
            up_rect: Rect::new(),
            down_rect: Rect::new(),
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
        self.up_rect.set_x_min(self.rect.dx().max - 14.0);
        self.up_rect.set_x_max(self.rect.dx().max);
        self.up_rect.set_y_min(self.rect.dy().min + 1.0);
        self.up_rect.set_y_max(self.rect.dy().min + self.rect.height() / 2.0 - 2.0);
        self.up_render.param.p0 = [self.up_rect.dx().min + self.up_rect.width() / 2.0, self.up_rect.dy().min];
        self.up_render.param.p1 = [self.up_rect.dx().min, self.up_rect.dy().max];
        self.up_render.param.p2 = [self.rect.dx().max, self.up_rect.dy().max];
        self.up_render.init_triangle(ui, false, false);
        self.down_rect.set_x_min(self.rect.dx().max - 14.0);
        self.down_rect.set_x_max(self.rect.dx().max);
        self.down_rect.set_y_min(self.rect.dy().max - self.rect.height() / 2.0 + 2.0);
        self.down_rect.set_y_max(self.rect.dy().max - 2.0);
        self.down_render.param.p0 = [self.down_rect.dx().min + self.down_rect.width() / 2.0, self.down_rect.dy().max];
        self.down_render.param.p1 = [self.rect.dx().max - 14.0, self.down_rect.dy().min];
        self.down_render.param.p2 = [self.rect.dx().max, self.down_rect.dy().min];
        self.down_render.init_triangle(ui, false, false);
    }

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
        self.down_render.update(ui, self.value <= self.range.start, false);
        self.up_render.update(ui, self.value >= self.range.end, false);
        self.edit.update_text(ui, format!("{:.*}", 2, self.value));
    }

    fn update_from_edit(&mut self, ui: &mut Ui, focused: bool) {
        if focused && self.edit.focused != focused {
            let v = self.edit.text().parse::<f32>().unwrap_or(self.value.as_f32());
            self.value = T::from_num(v);
            self.changed = true;
            ui.send_updates(&self.contact_ids, ContextUpdate::F32(self.value.as_f32()));
            ui.context.window.request_redraw();
        }
    }
}


impl<T: PartialOrd + AddAssign + SubAssign + ToString + Copy + Display + NumCastExt + 'static> Widget for SpinBox<T> {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.edit.redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.triangle.render(&self.down_render, pass);
        ui.context.render.triangle.render(&self.up_render, pass);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MousePress => {
                if ui.device.device_input.pressed_at(&self.down_rect) {
                    println!("press down");
                    self.edit.focused = false;
                    self.press_down = true;
                    self.press_time = crate::time_ms();
                    self.listen_input(ui, 500);
                } else if ui.device.device_input.pressed_at(&self.up_rect) {
                    println!("press up");
                    self.edit.focused = false;
                    self.press_up = true;
                    self.press_time = crate::time_ms();
                    self.listen_input(ui, 500);
                } else {
                    self.press_down = false;
                    self.press_up = false;
                    self.press_time = 0;
                    let focused = self.edit.focused;
                    self.edit.update(ui);
                    self.update_from_edit(ui, focused);
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
            UpdateType::KeyRelease(ref key) => {
                if !self.edit.focused { return Response::new(&self.id, &self.rect); }
                if let Some(winit::keyboard::Key::Named(winit::keyboard::NamedKey::Enter)) = key.as_ref() {
                    self.edit.focused = false;
                    self.update_from_edit(ui, true);
                } else {
                    self.edit.update(ui);
                }


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
                self.up_render.param.offset(o);
                self.down_render.param.offset(o);
                self.changed = true;
            }
            UpdateType::Drop => {}
            _ => {}
        }
        self.edit.update(ui);
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_t(&mut self.value);
            self.changed = true;
            ui.context.window.request_redraw();
        }
        Response::new(&self.id, &self.rect)
    }
}