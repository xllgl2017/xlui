use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::Widget;
use crate::{Offset, OffsetDirection};
use crate::response::Response;
use crate::size::radius::Radius;

pub struct ScrollBar {
    id: String,
    fill_render: RenderParam<RectParam>,
    slider_render: RenderParam<RectParam>,
    context_height: f32,
    focused: bool,
    offset: f32,
    changed: bool,
}


impl ScrollBar {
    pub fn new() -> ScrollBar {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::TRANSPARENT; //Color::rgb(215, 215, 215); //
        fill_style.fill.hovered = Color::TRANSPARENT; //Color::rgb(215, 215, 215);
        fill_style.fill.clicked = Color::TRANSPARENT; //Color::rgb(215, 215, 215);
        let mut slider_style = ClickStyle::new();
        slider_style.fill.inactive = Color::rgb(56, 182, 244);
        slider_style.fill.hovered = Color::rgb(56, 182, 244);
        slider_style.fill.clicked = Color::rgb(56, 182, 244);
        slider_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        slider_style.border.hovered = Border::new(0.0).radius(Radius::same(2));
        slider_style.border.clicked = Border::new(0.0).radius(Radius::same(2));
        ScrollBar {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new().with_size(10.0, 20.0), fill_style)),
            slider_render: RenderParam::new(RectParam::new(Rect::new().with_size(10.0, 10.0), slider_style)),
            context_height: 0.0,
            focused: false,
            offset: 0.0,
            changed: false,
        }
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.fill_render.param.rect = rect;
        self.slider_render.param.rect.set_width(self.fill_render.param.rect.width());
    }

    pub fn set_context_height(&mut self, context_height: f32) {
        self.context_height = context_height;
        let mut slider_height = if self.context_height < self.fill_render.param.rect.height() {
            self.fill_render.param.rect.height()
        } else {
            self.fill_render.param.rect.height() * self.fill_render.param.rect.height() / self.context_height
        };
        if slider_height < 32.0 { slider_height = 32.0; }
        self.slider_render.param.rect.set_height(slider_height);
        self.changed = true;
    }

    pub fn set_height(&mut self, height: f32) {
        self.fill_render.param.rect.set_height(height);
    }

    pub fn scrolling(&self) -> bool {
        self.offset < (self.fill_render.param.rect.height() - self.slider_render.param.rect.height()) && self.offset != 0.0
    }

    //计算滑块位移
    fn slider_offset_y(&self, cy: f32) -> f32 {
        let scrollable_content = self.context_height - self.fill_render.param.rect.height();
        let scrollable_slider = self.fill_render.param.rect.height() - self.slider_render.param.rect.height();
        let scroll_ratio = cy / scrollable_content; // 内容偏移占比：
        scroll_ratio * scrollable_slider // 滑块应偏移：
    }

    //计算内容位移
    fn context_offset_y(&self, oy: f32) -> f32 {
        let scrollable_content = self.context_height - self.fill_render.param.rect.height();
        let scrollable_slider = self.fill_render.param.rect.height() - self.slider_render.param.rect.height();
        if scrollable_slider == 0.0 { return 0.0; }
        let scroll_ratio = oy / scrollable_slider; // 内容偏移占比：
        scroll_ratio * scrollable_content // 滑块应偏移：
    }

    fn init(&mut self, ui: &mut Ui) {
        //背景
        self.fill_render.init_rectangle(ui, false, false);
        //滑块
        self.slider_render.param.rect = self.fill_render.param.rect.clone_with_size(&self.slider_render.param.rect);
        self.slider_render.init_rectangle(ui, false, false);
    }
}


impl Widget for ScrollBar {
    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        if self.context_height > self.fill_render.param.rect.height() {
            ui.context.render.rectangle.render(&self.slider_render, pass);
        }
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            UpdateType::MouseMove => {
                if self.focused && ui.device.device_input.mouse.pressed {
                    let oy = ui.device.device_input.mouse.offset_y();
                    let roy = self.slider_render.param.rect.offset_y_limit(self.offset + oy, self.fill_render.param.rect.dy());
                    let mut offset = Offset::new(ui.device.device_input.mouse.pressed_pos).with_y(self.context_offset_y(-roy));
                    if self.offset < roy {
                        offset.direction = OffsetDirection::Down
                    } else {
                        offset.direction = OffsetDirection::Up;
                    }
                    self.offset = roy;
                    let ut = UpdateType::Offset(offset);
                    ui.update_type = UpdateType::None;
                    self.slider_render.update(ui, true, true);
                    ui.context.window.request_redraw();
                    ui.request_update(ut);
                }
            }
            UpdateType::MousePress => self.focused = ui.device.device_input.pressed_at(&self.slider_render.param.rect),
            UpdateType::Offset(ref o) => {
                let oy = self.slider_offset_y(o.y);
                let roy = self.slider_render.param.rect.offset_y_limit(self.offset + oy, self.fill_render.param.rect.dy());
                let mut offset =Offset::new(o.pos).with_y(self.context_offset_y(-roy));
                if self.offset < roy {
                    offset.direction = OffsetDirection::Down
                } else {
                    offset.direction = OffsetDirection::Up;
                }
                self.offset = roy;

                let ut = UpdateType::Offset(offset);
                ui.update_type = UpdateType::None;
                self.slider_render.update(ui, true, true);
                ui.request_update(ut);
            }
            _ => {
                if self.changed {
                    self.changed = false;
                    self.slider_render.update(ui, false, false);
                }
            }
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}