use crate::frame::context::UpdateType;
use crate::render::circle::param::CircleParam;
use crate::render::{RenderKind, RenderParam};
#[cfg(feature = "gpu")]
use crate::render::WrcRender;
use crate::response::Response;
use crate::size::Geometry;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize};

pub struct Circle {
    id: String,
    render: RenderParam,
    changed: bool,
    geometry: Geometry,
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
            changed: false,

        }
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.changed = true;
        self.render.set_style(style);
    }

    pub fn style_mut(&mut self) -> &mut ClickStyle {
        self.changed = true;
        self.render.style_mut()
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.render.rect_mut().offset_to_rect(&ui.draw_rect);
            #[cfg(feature = "gpu")]
            self.render.update(ui, false, false);
        }
        if ui.widget_changed.contains(WidgetChange::Value) {
            #[cfg(feature = "gpu")]
            self.render.update(ui, false, false);
        }
    }

    fn init(&mut self, ui: &mut Ui) {
        #[cfg(feature = "gpu")]
        self.render.init(ui, false, false);
        self.changed = false;
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