use crate::frame::context::UpdateType;
use crate::render::{RenderParam, Visual, VisualStyle, WidgetStyle};
use crate::response::Response;
use crate::shape::Shape;
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::{Offset, Shadow};

pub struct ScrollBar {
    id: String,
    visual: Visual,
    slider_render: RenderParam,
    context_size: f32,
    offset: Offset,
    geometry: Geometry,
    state: WidgetState,
}


impl ScrollBar {
    fn new() -> ScrollBar {
        let mut fill_style = VisualStyle::same(WidgetStyle {
            fill: Color::TRANSPARENT,
            border: Border::same(0.0),
            radius: Radius::same(0),
            shadow: Shadow::new(),
        });
        fill_style.inactive.fill = Color::rgb(215, 215, 215);
        // fill_style.fill.inactive = Color::rgb(215, 215, 215); //Color::TRANSPARENT; //
        // fill_style.fill.hovered = Color::TRANSPARENT; //Color::rgb(215, 215, 215);
        // fill_style.fill.clicked = Color::TRANSPARENT; //Color::rgb(215, 215, 215);
        let slider_style = VisualStyle::same(WidgetStyle {
            fill: Color::rgb(56, 182, 244),
            border: Border::same(0.0),
            radius: Radius::same(0),
            shadow: Shadow::new(),
        });
        // slider_style.fill.inactive = Color::rgb(56, 182, 244);
        // slider_style.fill.hovered = Color::rgb(56, 182, 244);
        // slider_style.fill.clicked = Color::rgb(56, 182, 244);
        // slider_style.border.inactive = Border::same(0.0).radius(Radius::same(0));
        // slider_style.border.hovered = Border::same(0.0).radius(Radius::same(0));
        // slider_style.border.clicked = Border::same(0.0).radius(Radius::same(0));
        // let fill_param = RectParam::new().with_size(10.0, 20.0).with_style(fill_style);
        // let slider_param = RectParam::new().with_size(10.0, 10.0).with_style(slider_style);
        ScrollBar {
            id: crate::gen_unique_id(),
            visual: Visual::new().with_enable().with_style(fill_style),
            // fill_render: RenderParam::new(RenderKind::Rectangle(fill_param)),
            slider_render: RenderParam::new(Shape::rectangle()).with_style(slider_style),
            context_size: 0.0,
            offset: Offset::new(),
            geometry: Geometry::new(),
            state: WidgetState::default(),
        }
    }

    pub fn horizontal() -> ScrollBar {
        let mut res = ScrollBar::new();
        res.geometry.set_context_size(300.0, 5.0);
        res
    }

    pub fn vertical() -> ScrollBar {
        let mut res = ScrollBar::new();
        res.geometry.set_context_size(5.0, 300.0);
        res
    }

    pub fn set_vbar_value_by_offset(&mut self, offset: f32) -> f32 {
        if self.context_size < self.geometry.context_height() { return 0.0; }
        let oy = self.slider_offset_y(offset);
        let roy = self.slider_render.rect_mut().offset_y_limit(self.offset.y + oy, self.visual.rect().dy());
        self.offset.y = roy;
        self.state.changed = true;
        self.context_offset_y(-roy)
    }

    pub fn set_hbar_value_by_offset(&mut self, offset: f32) -> f32 {
        if self.context_size < self.geometry.context_width() { return 0.0; }
        let ox = self.slider_offset_x(offset);
        let rox = self.slider_render.rect_mut().offset_x_limit(self.offset.x + ox, self.visual.rect().dx());
        self.offset.x = rox;
        self.state.changed = true;
        self.context_offset_x(-rox)
    }

    pub fn offset(&mut self) -> f32 {
        if self.geometry.context_height() > self.geometry.context_width() { //垂直滚动条
            self.context_offset_y(-self.offset.y)
        } else { //水平滚动条
            self.context_offset_x(-self.offset.x)
        }
    }

    pub fn set_context_height(&mut self, context_height: f32) {
        self.context_size = context_height;
        let mut slider_height = if self.context_size < self.geometry.context_height() {
            self.geometry.context_height()
        } else {
            self.geometry.context_height() * self.geometry.context_height() / self.context_size
        };
        if slider_height < 32.0 { slider_height = 32.0; }
        self.slider_render.rect_mut().set_size(self.geometry.context_width(), slider_height);
        self.state.changed = true;
    }

    pub fn set_context_width(&mut self, context_width: f32) {
        self.context_size = context_width;
        let mut slider_width = if self.context_size < self.geometry.context_width() {
            self.geometry.context_width()
        } else {
            self.geometry.context_width() * self.geometry.context_width() / self.context_size
        };
        if slider_width < 32.0 { slider_width = 32.0; }
        self.slider_render.rect_mut().set_size(slider_width, self.geometry.context_height());
        self.state.changed = true;
    }

    //计算滑块位移
    fn slider_offset_y(&self, cy: f32) -> f32 {
        let scrollable_content = self.context_size - self.geometry.context_height();
        let scrollable_slider = self.geometry.context_height() - self.slider_render.rect().height();
        let scroll_ratio = cy / scrollable_content; // 内容偏移占比：
        scroll_ratio * scrollable_slider // 滑块应偏移：
    }

    fn slider_offset_x(&self, cx: f32) -> f32 {
        let scrollable_context = self.context_size - self.geometry.context_width();
        let scrollable_slider = self.geometry.context_width() - self.slider_render.rect().width();
        let scroll_ratio = cx / scrollable_context;
        scroll_ratio * scrollable_slider
    }

    //计算内容位移
    fn context_offset_y(&self, oy: f32) -> f32 {
        let scrollable_content = self.context_size - self.geometry.context_height();
        let scrollable_slider = self.geometry.context_height() - self.slider_render.rect().height();
        if scrollable_slider == 0.0 { return 0.0; }
        let scroll_ratio = oy / scrollable_slider; // 内容偏移占比：
        scroll_ratio * scrollable_content // 滑块应偏移：
    }

    fn context_offset_x(&self, ox: f32) -> f32 {
        let scrollable_content = self.context_size - self.geometry.context_width();
        let scrollable_slider = self.geometry.context_width() - self.slider_render.rect().width();
        if scrollable_slider == 0.0 { return 0.0; }
        let scroll_ratio = ox / scrollable_slider;
        scroll_ratio * scrollable_content
    }

    fn init(&mut self) {
        //背景
        self.visual.rect_mut().set_size(self.geometry.padding_width(), self.geometry.padding_height());
        //滑块
        // self.slider_render.rect_mut().set_size()
        // *self.slider_render.rect_mut() = self.visual.rect().clone_with_size(self.slider_render.rect());
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.visual.rect_mut().offset_to_rect(&ui.draw_rect);
            self.slider_render.offset_to_rect(&ui.draw_rect);
            self.slider_render.rect_mut().offset(&self.offset);
        }
    }
    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.visual.draw(ui, self.state.disabled, false, false, false);
        self.slider_render.draw(ui, false, false, false);
        // if self.context_size > self.visual.rect().height() && self.geometry.context_height() > self.geometry.context_width() { //垂直
        //     self.visual.draw(ui, self.state.disabled, false, false, false);
        //     self.slider_render.draw(ui, false, false, false);
        // }
        // if self.context_size > self.visual.rect().width() && self.geometry.context_width() > self.geometry.context_height() { //垂直
        //     self.visual.draw(ui, self.state.disabled, false, false, false);
        //     self.slider_render.draw(ui, false, false, false);
        // }
    }
}


impl Widget for ScrollBar {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Init | UpdateType::ReInit => self.init(),
            UpdateType::MouseMove => {
                if self.state.hovered_moving() {
                    if self.geometry.context_height() > self.geometry.context_width() { //垂直滚动条
                        let oy = ui.device.device_input.mouse.offset_y();
                        let roy = self.slider_render.rect_mut().offset_y_limit(self.offset.y + oy, self.visual.rect().dy());
                        self.offset.y = roy;
                    } else { //水平滚动条
                        let ox = ui.device.device_input.mouse.offset_x();
                        let rox = self.slider_render.rect_mut().offset_x_limit(self.offset.x + ox, self.visual.rect().dx());
                        self.offset.x = rox;
                    }
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MousePress => {
                let pressed = ui.device.device_input.pressed_at(self.visual.rect());
                self.state.on_pressed(pressed);
            }
            UpdateType::MouseRelease => { self.state.on_release(); }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.visual.rect().width(), self.visual.rect().height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}