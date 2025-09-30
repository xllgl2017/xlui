use crate::frame::context::UpdateType;
use crate::render::{RenderParam, WrcRender};
use crate::render::triangle::param::TriangleParam;
use crate::response::Response;
use crate::size::Geometry;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize};

pub struct Triangle {
    id: String,
    render: RenderParam<TriangleParam>,
    changed: bool,
    geometry: Geometry,
}


impl Triangle {
    pub fn new() -> Self {
        Triangle {
            id: crate::gen_unique_id(),
            render: RenderParam::new(TriangleParam::new(Pos::new(), Pos::new(), Pos::new(), ClickStyle::new())),
            changed: false,
            geometry: Geometry::new(),
        }
    }

    pub fn with_pos(mut self, p0: Pos, p1: Pos, p2: Pos) -> Self {
        self.set_pos(p0, p1, p2);
        self
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn set_pos(&mut self, p0: Pos, p1: Pos, p2: Pos) {
        let mut x_min = p0.x;
        let mut x_max = p0.x;
        let mut y_min = p0.y;
        let mut y_max = p0.y;
        if p1.x < x_min { x_min = p1.x; }
        if p2.x < x_min { x_min = p2.x; }
        if p1.x > x_max { x_max = p1.x; }
        if p2.x > x_max { x_max = p2.x; }
        if p1.y < y_min { y_min = p1.y; }
        if p2.y < y_min { y_min = p2.y; }
        if p1.y > y_max { y_max = p1.y; }
        if p2.y > y_max { y_max = p2.y; }
        let mut rect = Rect::new();
        rect.set_x_min(x_min);
        rect.set_x_max(x_max);
        rect.set_y_min(y_min);
        rect.set_y_max(y_max);
        self.render.param.set_poses(p0, p1, p2);
        self.geometry.set_size(rect.width(), rect.height());
        self.geometry.offset_to_rect(&rect);
    }

    pub fn with_style(mut self, style: ClickStyle) -> Self {
        self.render.param.style = style;
        self
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.render.param.style = style;
    }

    fn init(&mut self, ui: &mut Ui) {
        self.render.init_triangle(ui, false, false);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.render.param.offset_to_rect(&ui.draw_rect);
            self.render.update(ui, false, false);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            self.render.update(ui, false, false);
        }
    }

    pub fn style_mut(&mut self) -> &mut ClickStyle {
        self.changed = true;
        &mut self.render.param.style
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.triangle.render(&self.render, pass);
    }
}


impl Widget for Triangle {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.init(ui),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }
}