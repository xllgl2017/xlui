use crate::frame::context::UpdateType;
use crate::render::{RenderParam, WrcRender};
use crate::render::triangle::param::TriangleParam;
use crate::response::Response;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize};

pub struct Triangle {
    id: String,
    rect: Rect,
    render: RenderParam<TriangleParam>,
    changed: bool,
}


impl Triangle {
    pub fn new() -> Self {
        Triangle {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            render: RenderParam::new(TriangleParam::new(Pos::new(), Pos::new(), Pos::new(), ClickStyle::new())),
            changed: false,
        }
    }

    pub fn with_pos(mut self, p0: Pos, p1: Pos, p2: Pos) -> Self {
        self.set_pos(p0, p1, p2);
        self
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id= id.to_string();
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
        println!("triangle  {:?}", rect);
        self.render.param.p0 = p0;
        self.render.param.p1 = p1;
        self.render.param.p2 = p2;
        self.rect = rect;
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
            let offset = self.rect.offset_to_rect(&ui.draw_rect);
            self.render.param.p0.offset(offset.x, offset.y);
            self.render.param.p1.offset(offset.x, offset.y);
            self.render.param.p2.offset(offset.x, offset.y);
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
}


impl Widget for Triangle {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.triangle.render(&self.render, pass);
    }

    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.init(ui),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.rect.width(), self.rect.height()))
    }
}