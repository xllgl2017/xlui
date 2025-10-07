use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::RenderParam;
#[cfg(feature = "gpu")]
use crate::render::WrcRender;
use crate::response::Response;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::Shadow;
use crate::ui::Ui;
use crate::widgets::{WidgetChange, WidgetSize};
use crate::{ScrollWidget, Widget};
use crate::size::Geometry;

pub struct Popup {
    pub(crate) id: String,
    scroll_area: ScrollWidget,
    fill_render: RenderParam<RectParam>,
    open: bool,
    requests: Vec<bool>,
    changed: bool,
    geometry: Geometry,
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
        let mut fill_render = RenderParam::new(fill_param);
        #[cfg(feature = "gpu")]
        fill_render.init_rectangle(ui, false, false);
        let area = ScrollWidget::vertical().with_size(width, height).padding(Padding::same(5.0));
        Popup {
            id: crate::gen_unique_id(),
            scroll_area: area,
            fill_render,
            open: false,
            requests: vec![],
            changed: false,
            geometry: Geometry::new().with_size(width, height).with_padding(Padding::same(5.0)),
        }
    }

    pub fn show(mut self, ui: &mut Ui, context: impl FnMut(&mut Ui)) {
        #[cfg(feature = "gpu")]
        self.fill_render.init_rectangle(ui, false, false);
        self.scroll_area.draw(ui, context);
        self.scroll_area.update(ui);
        ui.popups.as_mut().unwrap().insert(self.id.clone(), self);
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.fill_render.param.rect = rect;
        self.changed = true;
    }

    pub fn rect(&self) -> &Rect {
        &self.fill_render.param.rect
    }

    pub fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Value) {
            #[cfg(feature = "gpu")]
            self.fill_render.update(ui, false, false);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        if !self.open { return; }
        #[cfg(feature = "gpu")]
        let pass = ui.pass.as_mut().unwrap();
        #[cfg(feature = "gpu")]
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.fill_render.param.draw(ui, false, false);
        let previous_rect = ui.draw_rect.clone();
        ui.draw_rect = self.fill_render.param.rect.clone();
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
                    if !ui.device.device_input.click_at(&self.fill_render.param.rect) {
                        self.requests.push(false);
                        ui.context.window.request_redraw();
                    }
                }
                if ui.device.device_input.hovered_at(&self.fill_render.param.rect) { ui.update_type = UpdateType::None; }
            }
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }
}
