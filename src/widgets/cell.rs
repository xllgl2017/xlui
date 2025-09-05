use crate::layout::{HorizontalLayout, LayoutKind};
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::ui::Ui;
use crate::{Padding, Rect, Widget};
use crate::frame::context::UpdateType;
use crate::style::ClickStyle;
use crate::widgets::UiDraw;

pub struct Cell {
    pub(crate) id: String,
    fill_render: RenderParam<RectParam>,
    layout: Option<LayoutKind>,
    context: UiDraw,
}

impl Cell {
    pub fn new() -> Self {
        Cell {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new(), ClickStyle::new())),
            layout: None,
            context: Box::new(|_| {}),
        }
    }


    pub fn with_context(mut self, context: UiDraw) -> Cell {
        self.context = context;
        self
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            let current_layout = HorizontalLayout::left_to_right().max_rect(ui.available_rect().clone(), Padding::same(0.0));
            let current_layout = LayoutKind::Horizontal(current_layout);
            let previous_layout = ui.layout.replace(current_layout).unwrap();
            (self.context)(ui);
            self.fill_render.param.rect = ui.layout().drawn_rect();
            self.layout = ui.layout.replace(previous_layout);
        } else {
            self.layout.as_mut().unwrap().update(ui);
        }
        self.fill_render.init_rectangle(ui, false, false);
    }
}

impl Widget for Cell {
    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.layout.as_mut().unwrap().redraw(ui);
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
            UpdateType::IME(_) => {}
            UpdateType::CreateWindow => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}