use crate::frame::context::UpdateType;
use crate::render::circle::param::CircleParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize};

pub struct Circle {
    id: String,
    render: RenderParam<CircleParam>,
    changed: bool,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        let mut rect = Rect::new();
        rect.set_height(r * 2.0);
        rect.set_width(r * 2.0);
        Circle {
            id: crate::gen_unique_id(),
            render: RenderParam::new(CircleParam::new(rect, ClickStyle::new())),
            changed: false,
        }
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id= id.to_string();
        self
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.changed = true;
        self.render.param.style = style;
    }

    pub fn style_mut(&mut self) -> &mut ClickStyle {
        self.changed = true;
        &mut self.render.param.style
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.render.param.rect.offset_to_rect(&ui.draw_rect);
            self.render.update(ui, false, false);
        }
        if ui.widget_changed.contains(WidgetChange::Value) {
            self.render.update(ui, false, false);
        }
    }

    fn init(&mut self, ui: &mut Ui) {
        self.render.init_circle(ui, false, false);
        self.changed = false;
    }
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.circle.render(&self.render, pass);
    }

}

impl Widget for Circle {

    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.init(ui),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.render.param.rect.width(), self.render.param.rect.height()))
    }
}