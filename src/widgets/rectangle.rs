use crate::frame::context::UpdateType;
use crate::render::{Visual, VisualStyle};
use crate::response::Response;
use crate::size::Geometry;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
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
///     let style = VisualStyle::same(WidgetStyle {
///             fill: Color::rgb(240, 240, 240),
///             border: Border::same(1.0),
///             radius: Radius::same(5),
///             shadow,
///         });
///     let rectangle=Rectangle::new(style,300.0,300.0);
///     ui.add(rectangle);
/// }
/// ```
pub struct Rectangle {
    id: String,
    visual: Visual,
    geometry: Geometry,
    state: WidgetState,
}

impl Rectangle {
    pub fn new(style: VisualStyle, width: f32, height: f32) -> Self {
        // let param = RectParam::new().with_size(width, height).with_style(style);
        Rectangle {
            id: crate::gen_unique_id(),
            visual: Visual::new().with_enable().with_style(style).with_size(width, height),
            geometry: Geometry::new().with_context_size(width, height),
            state: WidgetState::default(),
        }
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    // pub fn with_rect(mut self, rect: Rect) -> Self {
    //     *self.fill_render.rect_mut() = rect;
    //     self
    // }

    // pub fn set_rect(&mut self, rect: Rect) {
    //     *self.fill_render.rect_mut() = rect;
    // }

    // pub fn with_shadow(mut self, shadow: Shadow) -> Self {
    //     self.fill_render.set_shadow(shadow);
    //     self
    // }

    pub fn style_mut(&mut self) -> &mut VisualStyle {
        self.state.changed = true;
        self.visual.enable().style_mut()
    }

    // pub fn set_offset_x(&mut self, v: f32) {
    //     self.state.changed = self.fill_render.rect_param_mut().shadow.offset[0] == v;
    //     self.fill_render.rect_param_mut().shadow.offset[0] = v;
    // }

    // pub fn set_offset_y(&mut self, v: f32) {
    //     self.state.changed = self.fill_render.rect_param_mut().shadow.offset[1] == v;
    //     self.fill_render.rect_param_mut().shadow.offset[1] = v;
    // }

    // pub fn set_border(&mut self, nb: Border) {
    //     let ob = &self.fill_render.style_mut().border.inactive;
    //     self.state.changed = ob == &nb;
    //     self.fill_render.style_mut().border.inactive = nb;
    // }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.visual.rect_mut().offset_to_rect(&ui.draw_rect);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.visual.draw(ui, self.state.disabled, self.state.hovered, self.state.pressed, false);
    }
}

impl Widget for Rectangle {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            #[cfg(feature = "gpu")]
            UpdateType::ReInit => self.visual.re_init(),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(self.visual.rect());
                if self.state.on_pressed(hovered) { ui.context.window.request_redraw(); }
            }
            _ => {}
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