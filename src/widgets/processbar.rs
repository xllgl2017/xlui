use crate::frame::context::UpdateType;
use crate::render::{RenderParam, Visual, VisualStyle, WidgetStyle};
use crate::response::Response;
use crate::shape::Shape;
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::Shadow;
use std::ops::Range;

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
    visual: Visual,
    //当前位置
    process_render: RenderParam,
    value: f32,
    range: Range<f32>,
    geometry: Geometry,
    state: WidgetState,
}

impl ProcessBar {
    pub fn new(v: f32) -> Self {
        let fill_style = VisualStyle::same(WidgetStyle {
            fill: Color::rgb(220, 220, 220),
            border: Border::same(0.0),
            radius: Radius::same(4),
            shadow: Shadow::new(),
        });
        let process_style = VisualStyle::same(WidgetStyle {
            fill: Color::rgb(56, 182, 244),
            border: Border::same(0.0),
            radius: Radius::same(4),
            shadow: Shadow::new(),
        });
        ProcessBar {
            id: crate::gen_unique_id(),
            visual: Visual::new().with_enable().with_style(fill_style).with_size(200.0, 10.0),
            process_render: RenderParam::new(Shape::Rectangle).with_style(process_style).with_size(200.0,10.0),
            value: v,
            range: 0.0..100.0,
            geometry: Geometry::new().with_context_size(200.0, 10.0),
            state: WidgetState::default(),
        }
    }


    fn init(&mut self) {
        let w = self.value * self.visual.rect_mut().width() / (self.range.end - self.range.start);
        self.process_render.rect_mut().set_width(w);
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
            self.visual.rect_mut().offset_to_rect(&ui.draw_rect);
            self.process_render.rect_mut().offset_to_rect(&ui.draw_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            if self.value > self.range.end { self.value = self.range.end; }
            let w = self.value * self.visual.rect_mut().width() / (self.range.end - self.range.start);
            self.process_render.rect_mut().set_width(w);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.visual.draw(ui, self.state.disabled, false, false, false);
        self.process_render.draw(ui, self.state.disabled, false, false);
    }
}


impl Widget for ProcessBar {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init | UpdateType::ReInit => self.init(),
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