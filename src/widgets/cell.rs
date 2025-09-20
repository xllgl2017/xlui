use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::{HorizontalLayout, LayoutKind, Rect, Widget};
use crate::widgets::{UiDraw, WidgetSize};

pub struct Cell {
    id: String,
    fill_render: RenderParam<RectParam>,
    layout: Option<LayoutKind>,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new(), ClickStyle::new())),
            layout: None,
        }
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.fill_render.param.rect.set_size(w, h);
        self
    }


    fn show(&mut self, ui: &mut Ui, context: UiDraw) {
        let current_layout = HorizontalLayout::left_to_right().with_space(0.0)
            .with_size(self.fill_render.param.rect.width(), self.fill_render.param.rect.height());
        let current_layout = LayoutKind::new(current_layout);
        let previous_layout = ui.layout.replace(current_layout).unwrap();
        context(ui);
        self.layout = ui.layout.replace(previous_layout);
        self.fill_render.init_rectangle(ui, false, false);
    }

    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.layout.as_mut().unwrap().redraw(ui);
    }
}

impl Widget for Cell {
    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::ReInit => self.fill_render.init_triangle(ui, false, false),
            _ => {}
        }
        self.layout.as_mut().unwrap().update(ui);
        Response::new(&self.id, WidgetSize::same(self.fill_render.param.rect.width(), self.fill_render.param.rect.height()))
    }
}