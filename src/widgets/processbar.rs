use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::Response;
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use std::ops::Range;
use crate::size::Geometry;

///#### ProcessBar的示例用法
///```rust
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///    let mut bar=ProcessBar::new(1.0)
///        //设置控件id
///        .with_id("simple_id")
///        //设置值范围
///        .with_range(0.0..100.0);
///    //更新值
///    bar.set_value(10.0);
/// }
/// ```

pub struct ProcessBar {
    id: String,
    //背景
    fill_render: RenderParam,
    //当前位置
    process_render: RenderParam,
    value: f32,
    range: Range<f32>,
    geometry: Geometry,
    state: WidgetState,
}

impl ProcessBar {
    pub fn new(v: f32) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(220, 220, 220);
        fill_style.fill.hovered = Color::rgb(220, 220, 220);
        fill_style.fill.clicked = Color::rgb(220, 220, 220);
        fill_style.border.inactive = Border::same(0.0).radius(Radius::same(4));
        fill_style.border.hovered = Border::same(0.0).radius(Radius::same(1));
        fill_style.border.clicked = Border::same(0.0).radius(Radius::same(1));
        let mut process_style = ClickStyle::new();
        process_style.fill.inactive = Color::rgb(56, 182, 244);
        process_style.fill.hovered = Color::rgb(56, 182, 244);
        process_style.fill.clicked = Color::rgb(56, 182, 244);
        process_style.border.inactive = Border::same(0.0).radius(Radius::same(4));
        process_style.border.hovered = Border::same(0.0).radius(Radius::same(1));
        process_style.border.clicked = Border::same(0.0).radius(Radius::same(1));
        let fill_param = RectParam::new().with_size(200.0, 10.0).with_style(fill_style);
        let process_param = RectParam::new().with_size(200.0, 10.0).with_style(process_style);
        ProcessBar {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RenderKind::Rectangle(fill_param)),
            process_render: RenderParam::new(RenderKind::Rectangle(process_param)),
            value: v,
            range: 0.0..100.0,
            geometry: Geometry::new().with_context_size(200.0, 10.0),
            state: WidgetState::default(),
        }
    }


    fn init(&mut self, ui: &mut Ui) {
        let w = self.value * self.fill_render.rect_mut().width() / (self.range.end - self.range.start);
        self.process_render.rect_mut().set_width(w);
        #[cfg(feature = "gpu")]
        self.fill_render.init(ui, false, false);
        #[cfg(feature = "gpu")]
        self.process_render.init(ui, false, false);
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
        self.state.changed = true;
    }

    pub(crate) fn update_buffer(&mut self, ui: &mut Ui) {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_f32(&mut self.value);
            ui.widget_changed |= WidgetChange::Value;
        }
        if self.state.changed { ui.widget_changed |= WidgetChange::Value; }
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.fill_render.rect_mut().offset_to_rect(&ui.draw_rect);
            self.process_render.rect_mut().offset_to_rect(&ui.draw_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            if self.value > self.range.end { self.value = self.range.end; }
            let w = self.value * self.fill_render.rect_mut().width() / (self.range.end - self.range.start);
            self.process_render.rect_mut().set_width(w);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.fill_render.draw(ui, false, false);
        self.process_render.draw(ui, false, false);
    }
}


impl Widget for ProcessBar {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.margin_width(), self.geometry.margin_height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}