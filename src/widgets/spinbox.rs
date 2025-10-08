use crate::frame::context::{ContextUpdate, UpdateType};
use crate::frame::App;
use crate::key::Key;
use crate::render::triangle::param::TriangleParam;
use crate::render::{RenderKind, RenderParam};
#[cfg(feature = "gpu")]
use crate::render::WrcRender;
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle};
use crate::ui::Ui;
use crate::widgets::textedit::TextEdit;
use crate::widgets::{Widget, WidgetChange, WidgetSize};
use crate::window::UserEvent;
use crate::NumCastExt;
use std::fmt::Display;
use std::ops::{AddAssign, Range, SubAssign};
/// ### Slider的示例用法
/// ```
/// use xlui::*;
///
/// fn spinbox_changed<A:App>(_:&mut A,_:&mut Ui,v:f32){
///     println!("SpinBox改变了:{}",v);
/// }
///
/// fn draw<A:App>(ui:&mut Ui){
///     //快速创建一个Slider
///     ui.spinbox(10.0,1.0,0.0..100.0)
///         //设置回调函数
///         .set_callback(spinbox_changed::<A>);
///     let spinbox=SpinBox::new(10.0,1.0,0.0..100.0)
///         //关联ID为my_slider的控件
///         .contact("my_slider")
///         //连接到Slider值监听函数
///         .connect(spinbox_changed::<A>)
///         //设置控件ID
///         .id("my_spinbox");
///     ui.add(spinbox);
/// }
/// ```
pub struct SpinBox<T> {
    pub(crate) id: String,
    edit: TextEdit,
    rect: Rect,
    geometry: Geometry,
    value: T,
    gap: T,
    range: Range<T>,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, T)>>,
    up_render: RenderParam,
    down_render: RenderParam,
    up_rect: Rect,
    down_rect: Rect,
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
        style.border = BorderStyle::same(Border::same(0.0));
        let up_param = TriangleParam::new(Pos::new(), Pos::new(), Pos::new(), style.clone());
        let down_param = TriangleParam::new(Pos::new(), Pos::new(), Pos::new(), style);
        SpinBox {
            id: crate::gen_unique_id(),
            edit: TextEdit::single_edit(format!("{:.*}", 2, v)),
            rect: Rect::new().with_size(100.0, 25.0),
            geometry: Geometry::new().with_size(100.0, 25.0),
            value: v,
            gap: g,
            range: r,
            callback: None,
            up_render: RenderParam::new(RenderKind::Triangle(up_param)),
            down_render: RenderParam::new(RenderKind::Triangle(down_param)),
            up_rect: Rect::new(),
            down_rect: Rect::new(),
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

    pub(crate) fn reset_size(&mut self) {
        self.edit.geometry().set_fix_width(self.geometry.width() - 18.0);
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
        self.reset_size();
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        self.edit.update(ui);
        self.up_rect.set_x_min(self.rect.dx().max - 14.0);
        self.up_rect.set_x_max(self.rect.dx().max);
        self.up_rect.set_y_min(self.rect.dy().min + 1.0);
        self.up_rect.set_y_max(self.rect.dy().min + self.rect.height() / 2.0 - 2.0);
        let mut p0 = Pos::new();
        p0.x = self.up_rect.dx().min + self.up_rect.width() / 2.0;
        p0.y = self.up_rect.dy().min;
        let mut p1 = Pos::new();
        p1.x = self.up_rect.dx().min;
        p1.y = self.up_rect.dy().max;
        let mut p2 = Pos::new();
        p2.x = self.rect.dx().max;
        p2.y = self.up_rect.dy().max;
        self.up_render.set_poses(p0, p1, p2);
        #[cfg(feature = "gpu")]
        self.up_render.init(ui, false, false);
        self.down_rect.set_x_min(self.rect.dx().max - 14.0);
        self.down_rect.set_x_max(self.rect.dx().max);
        self.down_rect.set_y_min(self.rect.dy().max - self.rect.height() / 2.0 + 2.0);
        self.down_rect.set_y_max(self.rect.dy().max - 2.0);
        let mut p0 = Pos::new();
        p0.x = self.down_rect.dx().min + self.down_rect.width() / 2.0;
        p0.y = self.down_rect.dy().max;
        let mut p1 = Pos::new();
        p1.x = self.rect.dx().max - 14.0;
        p1.y = self.down_rect.dy().min;
        let mut p2 = Pos::new();
        p2.x = self.rect.dx().max;
        p2.y = self.down_rect.dy().min;
        self.down_render.set_poses(p0, p1, p2);
        #[cfg(feature = "gpu")]
        self.down_render.init(ui, false, false);
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
        let wid = ui.context.window.id();
        ui.context.user_update = (wid, UpdateType::None);
        let window = ui.context.window.clone();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(st));
            window.request_update_event(UserEvent::ReqUpdate);
        });
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_t(&mut self.value);
            self.changed = true;
        }
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.rect.offset_to_rect(&ui.draw_rect);
            self.up_rect.set_x_min(self.rect.dx().max - 14.0);
            self.up_rect.set_x_max(self.rect.dx().max);
            self.up_rect.set_y_min(self.rect.dy().min + 1.0);
            self.up_rect.set_y_max(self.rect.dy().min + self.rect.height() / 2.0 - 2.0);
            self.up_render.offset_to_rect(&self.up_rect);
            #[cfg(feature = "gpu")]
            self.up_render.update(ui, false, false);

            self.down_rect.set_x_min(self.rect.dx().max - 14.0);
            self.down_rect.set_x_max(self.rect.dx().max);
            self.down_rect.set_y_min(self.rect.dy().max - self.rect.height() / 2.0 + 2.0);
            self.down_rect.set_y_max(self.rect.dy().max - 2.0);
            self.down_render.offset_to_rect(&self.down_rect);
            #[cfg(feature = "gpu")]
            self.down_render.update(ui, false, false);
        }
        if ui.widget_changed.contains(WidgetChange::Value) {
            #[cfg(feature = "gpu")]
            self.down_render.update(ui, self.value <= self.range.start, false);
            #[cfg(feature = "gpu")]
            self.up_render.update(ui, self.value >= self.range.end, false);
            self.edit.update_text(ui, format!("{:.*}", 2, self.value));
        }
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
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        if ui.widget_changed.contains(WidgetChange::Position) {
            let mut edit_rect = self.rect.clone();
            edit_rect.set_x_max(edit_rect.dx().max - 18.0);
            ui.draw_rect = edit_rect;
        }
        self.edit.redraw(ui);
        // #[cfg(feature = "gpu")]
        // let pass = ui.pass.as_mut().unwrap();
        // #[cfg(feature = "gpu")]
        // ui.context.render.triangle.render(&self.down_render, pass);
        // #[cfg(feature = "gpu")]
        // ui.context.render.triangle.render(&self.up_render, pass);
        self.down_render.draw(ui, self.value <= self.range.start, false);
        self.up_render.draw(ui, self.value >= self.range.end, false);
    }
}


impl<T: PartialOrd + AddAssign + SubAssign + ToString + Copy + Display + NumCastExt + 'static> Widget for SpinBox<T> {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MousePress => {
                if ui.device.device_input.pressed_at(self.down_render.rect()) {
                    self.edit.focused = false;
                    self.press_down = true;
                    self.press_time = crate::time_ms();
                    self.listen_input(ui, 500);
                } else if ui.device.device_input.pressed_at(self.up_render.rect()) {
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
                return Response::new(&self.id, WidgetSize::same(self.rect.width(), self.rect.height()));
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
                return Response::new(&self.id, WidgetSize::same(self.rect.width(), self.rect.height()));
            }
            UpdateType::KeyRelease(ref key) => {
                if !self.edit.focused { return Response::new(&self.id, WidgetSize::same(self.rect.width(), self.rect.height())); }
                if let Key::Enter = key {
                    self.edit.focused = false;
                    self.update_from_edit(ui, true);
                } else {
                    self.edit.update(ui);
                }


                return Response::new(&self.id, WidgetSize::same(self.rect.width(), self.rect.height()));
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
            _ => {}
        }
        self.edit.update(ui);
        Response::new(&self.id, WidgetSize::same(self.rect.width(), self.rect.height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }
}