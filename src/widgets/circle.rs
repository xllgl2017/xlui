use crate::frame::context::UpdateType;
use crate::render::{RenderParam, VisualStyle};
use crate::response::Response;
use crate::shape::Shape;
use crate::size::Geometry;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::Color;

pub struct Circle {
    id: String,
    render: RenderParam,
    geometry: Geometry,
    state: WidgetState,
}

impl Circle {
    pub fn new(r: f32) -> Self {
        let mut style = VisualStyle::same((Color::rgb(230, 230, 230), 1.0, 3).into());
        style.inactive.border.set_same(0.0);
        style.pressed.fill = Color::rgb(165, 165, 165);

        // let mut rect = Rect::new();
        // rect.set_height(r * 2.0);
        // rect.set_width(r * 2.0);
        Circle {
            id: crate::gen_unique_id(),
            geometry: Geometry::new().with_context_size(r * 2.0, r * 2.0),
            render: RenderParam::new(Shape::Circle).with_style(style).with_size(r * 2.0, r * 2.0),
            state: WidgetState::default(),

        }
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn set_style(&mut self, style: VisualStyle) {
        self.render.set_style(style);
    }

    pub fn style_mut(&mut self) -> &mut VisualStyle {
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
        self.render.draw(ui, self.state.disabled, false, false);
    }
}

impl Widget for Circle {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            #[cfg(feature = "gpu")]
            UpdateType::ReInit => self.render.re_init(),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.margin_width(), self.geometry.margin_width()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}