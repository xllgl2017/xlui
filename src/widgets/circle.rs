use crate::frame::context::UpdateType;
use crate::render::circle::param::CircleParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::Widget;

pub struct Circle {
    id: String,
    render: RenderParam<CircleParam>,
    changed: bool,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        let mut rect = Rect::new();
        rect.set_x_max(rect.dx().min + r * 2.0);
        rect.set_y_max(rect.dx().max);
        Circle {
            id: crate::gen_unique_id(),
            render: RenderParam::new(CircleParam::new(rect, ClickStyle::new())),
            changed: false,
        }
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
        if !self.changed && !ui.can_offset { return; }
        self.changed = false;
        if ui.can_offset { self.render.param.rect.offset(&ui.offset); }
        self.render.update(ui, false, false);
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.render.param.rect = ui.available_rect().clone_with_size(&self.render.param.rect);
        }
        self.render.init_circle(ui, false, false);
        self.changed = false;
    }
}

impl Widget for Circle {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.circle.render(&self.render, pass);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            _ => {}
        }
        Response::new(&self.id, &self.render.param.rect)
    }
}