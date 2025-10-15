use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::Response;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::style::Shadow;
use crate::ui::Ui;
use crate::widgets::{WidgetSize, WidgetState};
use crate::{ScrollWidget, Widget};

pub struct Popup {
    pub(crate) id: String,
    scroll_area: ScrollWidget,
    fill_render: RenderParam,
    open: bool,
    requests: Vec<bool>,
    geometry: Geometry,
    state: WidgetState,
}

impl Popup {
    pub fn new(ui: &mut Ui, width: f32, height: f32) -> Popup {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            blur: 1.0,
            color: Color::rgba(0, 0, 0, 30),
        };

        let fill_param = RectParam::new().with_style(ui.style.borrow().widgets.popup.clone())
            .with_shadow(shadow);
        let mut fill_render = RenderParam::new(RenderKind::Rectangle(fill_param));
        #[cfg(feature = "gpu")]
        fill_render.init(ui, false, false);
        let area = ScrollWidget::vertical().with_size(width, height).padding(Padding::same(5.0));
        Popup {
            id: crate::gen_unique_id(),
            scroll_area: area,
            fill_render,
            open: false,
            requests: vec![],
            geometry: Geometry::new().with_size(width, height).with_padding(Padding::same(5.0)),
            state: WidgetState::default(),
        }
    }

    pub fn show(mut self, ui: &mut Ui, context: impl FnMut(&mut Ui)) {
        #[cfg(feature = "gpu")]
        self.fill_render.init(ui, false, false);
        self.scroll_area.draw(ui, context);
        self.scroll_area.update(ui);
        ui.popups.as_mut().unwrap().insert(self.id.clone(), self);
    }

    pub fn set_rect(&mut self, rect: Rect) {
        *self.fill_render.rect_mut() = rect;
    }

    pub fn rect(&self) -> &Rect {
        self.fill_render.rect()
    }

    fn redraw(&mut self, ui: &mut Ui) {
        if !self.open { return; }
        self.fill_render.draw(ui, false, false);
        let previous_rect = ui.draw_rect.clone();
        ui.draw_rect = self.fill_render.rect().clone();
        self.scroll_area.redraw(ui);
        ui.draw_rect = previous_rect;
    }

    pub fn opened(&mut self) -> bool {
        if self.requests.len() == 0 { return self.open; }
        let open = self.requests.iter().find(|x| **x == true);
        self.open = open.is_some();
        self.requests.clear();
        self.open
    }

    pub fn request_state(&mut self, state: bool) {
        self.requests.push(state);
    }

    pub fn toggle(&mut self) {
        self.requests.push(!self.open);
    }
}

impl Widget for Popup {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init | UpdateType::ReInit => { self.scroll_area.update(ui); }
            _ => if self.open {
                self.scroll_area.update(ui);
                if let UpdateType::MouseRelease = ui.update_type {
                    if !ui.device.device_input.click_at(&self.fill_render.rect()) {
                        self.requests.push(false);
                        ui.context.window.request_redraw();
                    }
                }
                if ui.device.device_input.hovered_at(&self.fill_render.rect()) { ui.update_type = UpdateType::None; }
            }
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
