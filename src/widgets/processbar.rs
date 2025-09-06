use std::ops::Range;
use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::style::color::Color;
use crate::ui::Ui;
use crate::widgets::Widget;

pub struct ProcessBar {
    id: String,
    //背景
    fill_render: RenderParam<RectParam>,
    //当前位置
    process_render: RenderParam<RectParam>,
    //
    value: f32,
    range: Range<f32>,
    change: bool,
}

impl ProcessBar {
    pub fn new(v: f32) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(220, 220, 220);
        fill_style.fill.hovered = Color::rgb(220, 220, 220);
        fill_style.fill.clicked = Color::rgb(220, 220, 220);
        fill_style.border.inactive = Border::new(0.0).radius(Radius::same(4));
        fill_style.border.hovered = Border::new(0.0).radius(Radius::same(1));
        fill_style.border.clicked = Border::new(0.0).radius(Radius::same(1));
        let mut process_style = ClickStyle::new();
        process_style.fill.inactive = Color::rgb(56, 182, 244);
        process_style.fill.hovered = Color::rgb(56, 182, 244);
        process_style.fill.clicked = Color::rgb(56, 182, 244);
        process_style.border.inactive = Border::new(0.0).radius(Radius::same(4));
        process_style.border.hovered = Border::new(0.0).radius(Radius::same(1));
        process_style.border.clicked = Border::new(0.0).radius(Radius::same(1));
        ProcessBar {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new().with_size(200.0, 10.0), fill_style)),
            process_render: RenderParam::new(RectParam::new(Rect::new(), process_style)),
            value: v,
            range: 0.0..100.0,
            change: false,
        }
    }


    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.fill_render.param.rect = ui.available_rect().clone_with_size(&self.fill_render.param.rect);
            self.process_render.param.rect = self.fill_render.param.rect.clone();
        }
        let w = self.value * self.fill_render.param.rect.width() / (self.range.end - self.range.start);
        self.process_render.param.rect.set_width(w);
        //
        self.fill_render.init_rectangle(ui, false, false);
        //
        self.process_render.init_rectangle(ui, false, false);
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn with_range(mut self, r: Range<f32>) -> Self {
        self.range = r;
        self
    }

    pub fn set_value(&mut self, value: f32) {
        self.value = value;
        self.change = true;
    }

    pub fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.change && !ui.can_offset { return; }
        self.change = false;
        if ui.can_offset {
            self.fill_render.param.rect.offset(&ui.offset);
            self.process_render.param.rect.offset(&ui.offset);
        }
        let w = self.value * self.fill_render.param.rect.width() / (self.range.end - self.range.start);
        self.process_render.param.rect.set_width(w);
        self.process_render.update(ui, false, false);
        self.fill_render.update(ui, false, false);
    }
}


impl Widget for ProcessBar {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        ui.context.render.rectangle.render(&self.process_render, pass);
    }

    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_f32(&mut self.value);
            self.change = true;
        }
        match ui.update_type {
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            _ => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}