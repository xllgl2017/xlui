use crate::frame::context::UpdateType;
use crate::render::circle::param::CircleParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::Response;
use crate::size::Geometry;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};

pub struct Circle {
    id: String,
    render: RenderParam,
    geometry: Geometry,
    state: WidgetState,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        let mut rect = Rect::new();
        rect.set_height(r * 2.0);
        rect.set_width(r * 2.0);
        Circle {
            id: crate::gen_unique_id(),
            geometry: Geometry::new().with_size(rect.width(), rect.height()),
            render: RenderParam::new(RenderKind::Circle(CircleParam::new(rect, ClickStyle::new()))),
            state: WidgetState::default(),

        }
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.render.set_style(style);
    }

    pub fn style_mut(&mut self) -> &mut ClickStyle {
        self.render.style_mut()
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.render.rect_mut().offset_to_rect(&ui.draw_rect);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.render.draw(ui, false, false);
    }
}

impl Widget for Circle {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            #[cfg(feature = "gpu")]
            UpdateType::Init | UpdateType::ReInit => self.render.init(ui, false, false),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}