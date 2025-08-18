//! ### Slider的示例用法
//! ```
//!
//! use xlui::frame::App;
//! use xlui::ui::Ui;
//! use xlui::widgets::slider::Slider;
//!
//! fn slider_changed<A:App>(_:&mut A,_:&mut Ui,v:f32){
//!     println!("Slider改变了:{}",v);
//! }
//!
//! fn draw<A:App>(ui:&mut Ui){
//!     //快速创建一个Slider
//!     ui.slider(10.0,0.0..100.0)
//!         //设置回调函数
//!         .set_callback(slider_changed::<A>);
//!     let slider=Slider::new(10.0)
//!         //关联ID为my_spinbox的控件
//!         .contact("my_spinbox")
//!         //连接到Slider值监听函数
//!         .connect(slider_changed::<A>)
//!         //设置控件ID
//!         .id("my_slider")
//!         //设置Slider值的范围
//!         .with_range(0.0..100.0);
//!     ui.add(slider);
//! }
//! ```
use crate::frame::App;
use crate::render::circle::param::CircleParam;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::Widget;
use std::any::Any;
use std::ops::Range;
use crate::frame::context::{ContextUpdate, UpdateType};
use crate::Offset;
use crate::size::pos::Pos;
use crate::size::radius::Radius;

pub struct Slider {
    pub(crate) id: String,
    rect: Rect,
    value: f32,
    range: Range<f32>,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui, f32)>>,
    contact_ids: Vec<String>,

    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,

    slider_param: CircleParam,
    slider_index: usize,
    slider_buffer: Option<wgpu::Buffer>,

    slided_param: RectParam,
    slided_index: usize,
    slided_buffer: Option<wgpu::Buffer>,

    focused: bool,
    hovered: bool,
    offset: f32,
}

impl Slider {
    pub fn new(v: f32) -> Slider {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(220, 220, 220);
        fill_style.fill.hovered = Color::rgb(220, 220, 220);
        fill_style.fill.clicked = Color::rgb(220, 220, 220);
        fill_style.border.inactive = Border::new(0.0).radius(Radius::same(3));
        fill_style.border.hovered = Border::new(0.0).radius(Radius::same(3));
        fill_style.border.clicked = Border::new(0.0).radius(Radius::same(3));

        let mut slider_style = ClickStyle::new();
        slider_style.fill.inactive = Color::rgb(56, 182, 244);
        slider_style.fill.hovered = Color::rgb(56, 182, 244);
        slider_style.fill.clicked = Color::rgb(56, 182, 244);
        slider_style.border.inactive = Border::new(0.0).color(Color::BLACK).radius(Radius::same(8));
        slider_style.border.hovered = Border::new(1.0).color(Color::BLACK).radius(Radius::same(8));
        slider_style.border.clicked = Border::new(1.0).color(Color::BLACK).radius(Radius::same(8));

        let mut slided_style = ClickStyle::new();
        slided_style.fill.inactive = Color::rgb(56, 182, 244);
        slided_style.fill.hovered = Color::rgb(56, 182, 244);
        slided_style.fill.clicked = Color::rgb(56, 182, 244);
        slided_style.border.inactive = Border::new(0.0).radius(Radius::same(3));
        slided_style.border.hovered = Border::new(0.0).radius(Radius::same(3));
        slided_style.border.clicked = Border::new(0.0).radius(Radius::same(3));
        Slider {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            value: v,
            range: 0.0..1.0,
            callback: None,
            contact_ids: vec![],
            fill_param: RectParam::new(Rect::new(), fill_style),
            fill_index: 0,
            fill_buffer: None,
            slider_param: CircleParam::new(Rect::new(), slider_style),
            slider_index: 0,
            slider_buffer: None,
            slided_param: RectParam::new(Rect::new(), slided_style),
            slided_index: 0,
            slided_buffer: None,
            focused: false,
            hovered: false,
            offset: 0.0,
        }
    }

    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, f32)) -> Self {
        self.callback = Some(Callback::create_slider(f));
        self
    }

    pub fn with_range(mut self, range: Range<f32>) -> Self {
        self.range = range;
        self
    }

    pub fn set_callback<A: App>(&mut self, f: fn(&mut A, &mut Ui, f32)) {
        self.callback = Some(Callback::create_slider(f));
    }

    pub fn contact(mut self, id: impl ToString) -> Self {
        self.contact_ids.push(id.to_string());
        self
    }

    fn init(&mut self, ui: &mut Ui) {
        //分配大小
        self.rect = ui.layout().available_rect().clone();
        self.rect.set_size(130.0, 16.0);
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //背景
        self.fill_param.rect = self.rect.clone();
        self.fill_param.rect.contract(8.0, 5.0);
        let data = self.fill_param.as_draw_param(false, false);
        let fill_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        self.fill_buffer = Some(fill_buffer);
        //已滑动背景
        self.slided_param.rect = self.fill_param.rect.clone();
        let scale = self.value / (self.range.end - self.range.start);
        self.slided_param.rect.set_width(self.fill_param.rect.width() * scale);
        let data = self.slided_param.as_draw_param(false, false);
        let slided_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.slided_index = ui.context.render.rectangle.create_bind_group(&ui.device, &slided_buffer);
        self.slided_buffer = Some(slided_buffer);
        //滑块
        self.slider_param.rect = self.rect.clone();
        self.slider_param.rect.add_min_x(-self.rect.height() / 2.0);
        self.slider_param.rect.set_width(self.rect.height());
        self.offset = self.value * self.rect.width() / (self.range.end - self.range.start);
        self.slider_param.rect.offset_x(&Offset::new(Pos::new()).with_x(self.offset));
        let data = self.slider_param.as_draw_param(false, false);
        let slider_buffer = ui.context.render.circle.create_buffer(&ui.device, data);
        self.slider_index = ui.context.render.circle.create_bind_group(&ui.device, &slider_buffer);
        self.slider_buffer = Some(slider_buffer);
    }

    fn update_slider(&mut self, ui: &mut Ui) {
        let scale = self.value / (self.range.end - self.range.start);
        self.slided_param.rect.set_width(self.fill_param.rect.width() * scale);
        let data = self.slided_param.as_draw_param(false, false);
        ui.device.queue.write_buffer(self.slided_buffer.as_ref().unwrap(), 0, data);
        let data = self.slider_param.as_draw_param(self.hovered || self.focused, ui.device.device_input.mouse.pressed);
        ui.device.queue.write_buffer(self.slider_buffer.as_ref().unwrap(), 0, data);
        ui.context.window.request_redraw();
    }
}

impl Widget for Slider {
    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        ui.context.render.rectangle.render(self.slided_index, pass);
        ui.context.render.circle.render(self.slider_index, pass);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_f32(&mut self.value);
            self.slider_param.rect = self.rect.clone();
            self.slider_param.rect.add_min_x(-self.rect.height() / 2.0);
            self.slider_param.rect.set_width(self.rect.height());
            let offset = self.value * self.rect.width() / (self.range.end - self.range.start);
            let mut lx = self.fill_param.rect.dx().clone();
            lx.extend(self.slider_param.rect.width() / 2.0);
            self.offset = self.slider_param.rect.offset_x_limit(offset, &lx);
            self.update_slider(ui);
        }
        match ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MouseMove => { //滑动
                if self.focused && ui.device.device_input.mouse.pressed {
                    let ox = ui.device.device_input.mouse.offset_x();
                    let mut lx = self.fill_param.rect.dx().clone();
                    lx.extend(self.slider_param.rect.width() / 2.0);
                    let rox = self.slider_param.rect.offset_x_limit(self.offset + ox, &lx);
                    self.offset = rox;
                    let cl = (self.slider_param.rect.width() / 2.0 + self.slider_param.rect.dx().min - self.fill_param.rect.dx().min) / self.fill_param.rect.width();
                    let cv = (self.range.end - self.range.start) * cl;
                    self.value = cv;
                    self.update_slider(ui);
                    if let Some(ref mut callback) = self.callback {
                        let app = ui.app.take().unwrap();
                        callback(*app, ui, self.value);
                        ui.app.replace(app);
                    }
                    ui.send_updates(&self.contact_ids, ContextUpdate::F32(self.value));
                    ui.update_type = UpdateType::None;
                    return Response::new(&self.id, &self.rect);
                }
                let hovered = ui.device.device_input.hovered_at(&self.slider_param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    let data = self.slider_param.as_draw_param(self.hovered, ui.device.device_input.mouse.pressed);
                    ui.device.queue.write_buffer(self.slider_buffer.as_ref().unwrap(), 0, data);
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MousePress => self.focused = ui.device.device_input.pressed_at(&self.slider_param.rect),
            _ => {}
        }
        Response::new(&self.id, &self.rect)
    }
}