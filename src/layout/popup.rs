use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::{ScrollWidget, Widget};
use crate::response::Response;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle, Shadow};
use crate::ui::Ui;
use crate::widgets::{WidgetChange, WidgetSize};

pub struct Popup {
    pub(crate) id: String,
    scroll_area: ScrollWidget,
    fill_render: RenderParam<RectParam>,
    size: WidgetSize,
    pub(crate) open: bool,
    changed: bool,
}

impl Popup {
    pub fn new(ui: &mut Ui, width: f32, height: f32) -> Popup {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            color: Color::rgba(0, 0, 0, 30),
        };

        let fill_param = RectParam::new(Rect::new(), Popup::popup_style())
            .with_shadow(shadow);
        let mut fill_render = RenderParam::new(fill_param);
        fill_render.init_rectangle(ui, false, false);
        let area = ScrollWidget::vertical().with_size(width, height).padding(Padding::same(5.0));
        Popup {
            id: crate::gen_unique_id(),
            scroll_area: area,
            fill_render,
            size: WidgetSize::same(width, height),
            open: false,
            changed: false,
        }
    }

    pub fn show(mut self, ui: &mut Ui, context: impl FnMut(&mut Ui)) {
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

    pub fn popup_style() -> ClickStyle {
        ClickStyle {
            fill: FillStyle {
                inactive: Color::rgb(240, 240, 240),
                hovered: Color::rgb(240, 240, 240),
                clicked: Color::rgb(240, 240, 240),
            },
            border: BorderStyle {
                inactive: Border {
                    width: 1.0,
                    radius: Radius::same(5),
                    color: Color::rgba(144, 209, 255, 255),
                },
                hovered: Border {
                    width: 1.0,
                    radius: Radius::same(5),
                    color: Color::rgba(144, 209, 255, 255),
                },
                clicked: Border {
                    width: 1.0,
                    radius: Radius::same(5),
                    color: Color::rgba(144, 209, 255, 255),
                },
            },
        }
    }

    pub fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Value) {
            self.fill_render.update(ui, false, false);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        if !self.open { return; }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        let previous_rect = ui.draw_rect.clone();
        ui.draw_rect = self.fill_render.param.rect.clone();
        self.scroll_area.redraw(ui);
        ui.draw_rect = previous_rect;
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
                        self.open = false;
                        ui.context.window.request_redraw();
                    }
                }
                if ui.device.device_input.hovered_at(&self.fill_render.param.rect) { ui.update_type = UpdateType::None; }
            }
        }
        Response::new(&self.id, self.size.clone())
    }
}
