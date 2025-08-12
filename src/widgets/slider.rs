use crate::frame::App;
use crate::radius::Radius;
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

pub struct Slider {
    pub(crate) id: String,
    rect: Rect,
    value: f32,
    range: Range<f32>,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui, f32)>>,

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
        }
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

    fn init(&mut self, ui: &mut Ui) {
        //分配大小
        self.rect = ui.layout().available_rect().clone();
        self.rect.x.min += 8.0;
        self.rect.x.max += 8.0;
        self.rect.set_size(130.0, 16.0);
        // ui.layout().alloc_rect(&self.rect);
        //背景
        self.fill_param.rect = self.rect.clone();
        self.fill_param.rect.y.min += 5.0;
        self.fill_param.rect.y.max -= 5.0;
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
        self.slider_param.rect.x.min -= self.rect.height() / 2.0;
        self.slider_param.rect.set_width(self.rect.height());
        let offset = self.value * self.rect.width() / (self.range.end - self.range.start);
        self.slider_param.rect.offset_x(offset);
        let data = self.slider_param.as_draw_param(false, false);
        let slider_buffer = ui.context.render.circle.create_buffer(&ui.device, data);
        self.slider_index = ui.context.render.circle.create_bind_group(&ui.device, &slider_buffer);
        self.slider_buffer = Some(slider_buffer);
    }
}

impl Widget for Slider {
    fn redraw(&mut self, ui: &mut Ui) -> Response {
        if self.fill_buffer.is_none() { self.init(ui); }
        let resp = Response::new(&self.id, &self.rect);
        if ui.pass.is_none() { return resp; }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        ui.context.render.rectangle.render(self.slided_index, pass);
        ui.context.render.circle.render(self.slider_index, pass);
        resp
    }

    fn update(&mut self, ui: &mut Ui) {
        match self.focused {
            true => self.focused = self.focused && ui.device.device_input.mouse.pressed,
            false => self.focused = ui.device.device_input.pressed_at(&self.slider_param.rect)
        }
        //滑动
        if self.focused && ui.device.device_input.mouse.pressed {
            let ox = ui.device.device_input.mouse.offset_x();
            // let lx = self.fill_param.rect.x.min..self.fill_param.rect.x.max;
            self.slider_param.rect.offset_x_limit(ox, &self.fill_param.rect.x);
            let cl = (self.slider_param.rect.width() / 2.0 + self.slider_param.rect.x.min - self.fill_param.rect.x.min) / self.fill_param.rect.width();
            let cv = (self.range.end - self.range.start) * cl;
            self.value = cv;
            let scale = self.value / (self.range.end - self.range.start);
            self.slided_param.rect.set_width(self.fill_param.rect.width() * scale);
            let data = self.slided_param.as_draw_param(false, false);
            ui.device.queue.write_buffer(self.slided_buffer.as_ref().unwrap(), 0, data);
            let data = self.slider_param.as_draw_param(true, true);
            ui.device.queue.write_buffer(self.slider_buffer.as_ref().unwrap(), 0, data);
            ui.context.window.request_redraw();
            if let Some(ref mut callback) = self.callback {
                let app = ui.app.take().unwrap();
                callback(*app, ui, self.value);
                ui.app.replace(app);
            }
            return;
        }

        let hovered = ui.device.device_input.hovered_at(&self.slider_param.rect);
        if self.hovered != hovered {
            self.hovered = hovered;
            let data = self.slider_param.as_draw_param(self.hovered, ui.device.device_input.mouse.pressed);
            ui.device.queue.write_buffer(self.slider_buffer.as_ref().unwrap(), 0, data);
            ui.context.window.request_redraw();
        }
    }
}