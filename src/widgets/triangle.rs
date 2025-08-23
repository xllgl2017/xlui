use crate::frame::context::UpdateType;
use crate::render::{RenderParam, WrcRender};
use crate::render::triangle::param::TriangleParam;
use crate::response::Response;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::Widget;

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

    pub fn set_pos(&mut self, p0: Pos, p1: Pos, p2: Pos) {
        let mut x_min = p0.x;
        let mut x_max = p0.x;
        let mut y_min = p1.x;
        let mut y_max = p1.x;
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
        self.render.param.p0 = p0;
        self.render.param.p1 = p1;
        self.render.param.p2 = p2;
        self.rect = rect;
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.render.param.style = style;
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.rect = ui.available_rect().clone_with_size(&self.rect);
        }
        self.render.init_triangle(ui, false, false);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.changed && !ui.can_offset { return; }
        self.changed = false;
        if ui.can_offset {
            self.render.param.offset(&ui.offset);
            self.rect.offset(&ui.offset);
        }
        self.render.update(ui, false, false);
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

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::None => {}
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            UpdateType::MouseMove => {}
            UpdateType::MousePress => {}
            UpdateType::MouseRelease => {}
            UpdateType::MouseWheel => {}
            UpdateType::KeyRelease(_) => {}
            UpdateType::Offset(_) => {}
            UpdateType::Drop => {}
        }
        Response::new(&self.id, &self.rect)
    }
}