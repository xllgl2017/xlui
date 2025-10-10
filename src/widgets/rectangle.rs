use crate::Border;
use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::Response;
use crate::size::Geometry;
use crate::size::rect::Rect;
use crate::style::{ClickStyle, Shadow};
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize};
/// ### Rectangle的示例用法
/// ```rust
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///     //阴影
///     let shadow = Shadow {
///         offset: [5.0, 8.0],
///         spread: 10.0,//
///         color: Color::rgba(0, 0, 0, 30),
///         blur:1.0,//阴影强调
///     };
///     let rectangle=Rectangle::new(ui.style.borrow().widgets.popup.clone(),300.0,300.0)
///         //设置阴影
///         .with_shadow(shadow);
///     ui.add(rectangle);
/// }
/// ```
pub struct Rectangle {
    id: String,
    fill_render: RenderParam,
    geometry: Geometry,
    hovered: bool,
    changed: bool,
}

impl Rectangle {
    pub fn new(style: ClickStyle, width: f32, height: f32) -> Self {
        let param = RectParam::new().with_size(width, height).with_style(style);
        Rectangle {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RenderKind::Rectangle(param)),
            geometry: Geometry::new().with_size(width, height),
            hovered: false,
            changed: false,
        }
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        *self.fill_render.rect_mut() = rect;
        self
    }

    pub fn set_rect(&mut self, rect: Rect) {
        *self.fill_render.rect_mut() = rect;
    }

    pub fn with_shadow(mut self, shadow: Shadow) -> Self {
        self.fill_render.set_shadow(shadow);
        self
    }

    pub fn style_mut(&mut self) -> &mut ClickStyle {
        self.changed = true;
        self.fill_render.style_mut()
    }

    pub fn set_offset_x(&mut self, v: f32) {
        self.changed = self.fill_render.rect_param_mut().shadow.offset[0] == v;
        self.fill_render.rect_param_mut().shadow.offset[0] = v;
    }

    pub fn set_offset_y(&mut self, v: f32) {
        self.changed = self.fill_render.rect_param_mut().shadow.offset[1] == v;
        self.fill_render.rect_param_mut().shadow.offset[1] = v;
    }

    pub fn set_border(&mut self, nb: Border) {
        let ob = &self.fill_render.style_mut().border.inactive;
        self.changed = ob == &nb;
        self.fill_render.style_mut().border.inactive = nb;
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        // if !self.changed && !ui.can_offset { return; }
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.offset_to_rect(&ui.draw_rect);
            #[cfg(feature = "gpu")]
            self.fill_render.update(ui, self.hovered, ui.device.device_input.mouse.pressed);
        }
        if ui.widget_changed.contains(WidgetChange::Value) {
            #[cfg(feature = "gpu")]
            self.fill_render.update(ui, self.hovered, ui.device.device_input.mouse.pressed);
        }
        // if ui.can_offset { self.fill_render.param.rect.offset(&ui.offset); }

    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.fill_render.draw(ui, self.hovered, ui.device.device_input.mouse.pressed);
    }
}

impl Widget for Rectangle {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            #[cfg(feature = "gpu")]
            UpdateType::Init | UpdateType::ReInit => self.fill_render.init(ui, false, false),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(self.fill_render.rect());
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.changed = true;
                }
            }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }
}