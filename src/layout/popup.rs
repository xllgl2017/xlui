use crate::frame::context::UpdateType;
use crate::style::{Visual, VisualStyle, WidgetStyle};
use crate::response::Response;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::Geometry;
use crate::widgets::{WidgetSize, WidgetState};
use crate::*;

pub struct Popup {
    pub(crate) id: String,
    scroll_area: ScrollWidget,
    visual: Visual,
    open: bool,
    requests: Vec<bool>,
    geometry: Geometry,
    state: WidgetState,
}

impl Popup {
    pub fn new(width: f32, height: f32) -> Popup {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            blur: 1.0,
            color: Color::rgba(0, 0, 0, 30),
        };
        let style = VisualStyle::same(WidgetStyle {
            fill: Color::rgb(230, 230, 230),
            border: Border::same(1.0).color(Color::rgb(144, 209, 255)),
            radius: Radius::same(5),
            shadow,
        });
        let area = ScrollWidget::vertical().with_size(width, height).padding(Padding::same(5.0));
        Popup {
            id: gen_unique_id(),
            scroll_area: area,
            visual: Visual::new().with_enable().with_style(style),
            open: false,
            requests: vec![],
            geometry: Geometry::new().with_context_size(width, height).with_padding(Padding::same(5.0)),
            state: WidgetState::default(),
        }
    }

    pub fn show(mut self, ui: &mut Ui, context: impl FnMut(&mut Ui)) {
        self.scroll_area.draw(ui, context);
        self.scroll_area.update(ui);
        ui.popups.as_mut().unwrap().insert(self.id.clone(), self);
    }

    pub fn set_rect(&mut self, rect: Rect) {
        *self.visual.rect_mut() = rect;
    }

    pub fn rect(&self) -> &Rect {
        self.visual.rect()
    }

    fn redraw(&mut self, ui: &mut Ui) {
        if !self.open { return; }
        self.visual.draw(ui, self.state.disabled, false, false, false);
        let previous_rect = ui.draw_rect.clone();
        ui.draw_rect = self.visual.rect().clone();
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
                    if !ui.device.device_input.click_at(self.visual.rect()) {
                        self.requests.push(false);
                        ui.context.window.request_redraw();
                    }
                }
                if ui.device.device_input.hovered_at(self.visual.rect()) { ui.update_type = UpdateType::None; }
            }
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
