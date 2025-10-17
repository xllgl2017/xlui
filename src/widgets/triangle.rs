use crate::frame::context::UpdateType;
use crate::render::{RenderParam, VisualStyle};
use crate::response::Response;
use crate::shape::Shape;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::size::Geometry;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::Color;

pub struct Triangle {
    id: String,
    render: RenderParam,
    geometry: Geometry,
    state: WidgetState,
}


impl Triangle {
    pub fn new() -> Self {
        let mut style = VisualStyle::same((Color::rgb(230, 230, 230), 1.0, 3).into());
        style.inactive.border.set_same(0.0);
        style.pressed.fill = Color::rgb(165, 165, 165);
        Triangle {
            id: crate::gen_unique_id(),
            render: RenderParam::new(Shape::triangle()).with_style(style),
            geometry: Geometry::new(),
            state: WidgetState::default(),
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
        self.render.set_poses(p0, p1, p2);
        self.geometry.set_context_size(rect.width(), rect.height());
        self.geometry.offset_to_rect(&rect);
    }

    pub fn with_style(mut self, style: VisualStyle) -> Self {
        self.render.set_style(style);
        self
    }

    pub fn set_style(&mut self, style: VisualStyle) {
        self.render.set_style(style);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.render.offset_to_rect(&ui.draw_rect);
            #[cfg(feature = "gpu")]
            self.render.update(ui, false, false);
        }
    }

    pub fn style_mut(&mut self) -> &mut VisualStyle {
        self.render.style_mut()
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.render.draw(ui, self.state.disabled, false, false);
    }
}


impl Widget for Triangle {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            #[cfg(feature = "gpu")]
            UpdateType::ReInit => self.render.re_init(),
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