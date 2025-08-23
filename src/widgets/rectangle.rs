//! ### Rectangle的示例用法
//! ```
//! use xlui::layout::popup::Popup;
//! use xlui::style::color::Color;
//! use xlui::style::Shadow;
//! use xlui::ui::Ui;
//! use xlui::widgets::rectangle::Rectangle;
//!
//! fn draw(ui:&mut Ui){
//!     //阴影
//!     let shadow = Shadow {
//!         offset: [5.0, 8.0],
//!         spread: 10.0,
//!         color: Color::rgba(0, 0, 0, 30),
//!     };
//!     //获取当前可用矩形
//!     let mut rect =ui.available_rect().clone();
//!     rect.set_size(300.0,300.0);
//!     rect.add_min_x(10.0);
//!     rect.add_max_x(10.0);
//!     rect.add_min_y(10.0);
//!     rect.add_max_y(10.0);
//!     let rectangle=Rectangle::new(rect, Popup::popup_style())
//!         //设置阴影
//!         .with_shadow(shadow);
//!     ui.add(rectangle);
//! }
//! ```

use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::rect::Rect;
use crate::style::{ClickStyle, Shadow};
use crate::ui::Ui;
use crate::widgets::Widget;

pub struct Rectangle {
    id: String,
    fill_render: RenderParam<RectParam>,
    hovered: bool,
    changed: bool,
}

impl Rectangle {
    pub fn new(rect: Rect, style: ClickStyle) -> Self {
        Rectangle {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(rect, style)),
            hovered: false,
            changed: false,
        }
    }

    fn init(&mut self, ui: &mut Ui) {
        self.fill_render.init_rectangle(ui, false, false);
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.fill_render.param.rect = rect;
        self
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.fill_render.param.rect = rect;
    }

    pub fn with_shadow(mut self, shadow: Shadow) -> Self {
        self.fill_render.param = self.fill_render.param.with_shadow(shadow);
        self
    }

    pub fn style_mut(&mut self) -> &mut ClickStyle {
        self.changed = true;
        &mut self.fill_render.param.style
    }

    pub fn offset_x(&mut self, v: f32) {
        self.changed = true;
        self.fill_render.param.shadow.offset[0] = v;
    }

    pub fn offset_y(&mut self, v: f32) {
        self.changed = true;
        self.fill_render.param.shadow.offset[1] = v;
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.changed && !ui.can_offset { return; }
        self.changed = false;
        if ui.can_offset { self.fill_render.param.rect.offset(&ui.offset); }
        self.fill_render.update(ui, self.hovered, ui.device.device_input.mouse.pressed);
    }
}

impl Widget for Rectangle {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.fill_render.param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.changed = true;
                }
            }
            // UpdateType::Offset(ref o) => {
            //     if !ui.can_offset { return Response::new(&self.id, &self.fill_render.param.rect); }
            //     self.fill_render.param.rect.offset(o);
            //     self.changed = true;
            // }
            _ => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}