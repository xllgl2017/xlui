use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize};
use crate::{Offset, Pos};

pub struct ScrollBar {
    id: String,
    fill_render: RenderParam<RectParam>,
    slider_render: RenderParam<RectParam>,
    context_size: f32,
    focused: bool,
    offset: Offset,
    changed: bool,
}


impl ScrollBar {
    fn new() -> ScrollBar {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(215, 215, 215); //Color::TRANSPARENT; //
        fill_style.fill.hovered = Color::TRANSPARENT; //Color::rgb(215, 215, 215);
        fill_style.fill.clicked = Color::TRANSPARENT; //Color::rgb(215, 215, 215);
        let mut slider_style = ClickStyle::new();
        slider_style.fill.inactive = Color::rgb(56, 182, 244);
        slider_style.fill.hovered = Color::rgb(56, 182, 244);
        slider_style.fill.clicked = Color::rgb(56, 182, 244);
        slider_style.border.inactive = Border::new(0.0).radius(Radius::same(0));
        slider_style.border.hovered = Border::new(0.0).radius(Radius::same(0));
        slider_style.border.clicked = Border::new(0.0).radius(Radius::same(0));
        ScrollBar {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new().with_size(10.0, 20.0), fill_style)),
            slider_render: RenderParam::new(RectParam::new(Rect::new().with_size(10.0, 10.0), slider_style)),
            context_size: 0.0,
            focused: false,
            offset: Offset::new(Pos::new()),
            changed: false,
        }
    }

    pub fn horizontal() -> ScrollBar {
        let mut res = ScrollBar::new();
        res.fill_render.param.rect.set_size(300.0, 5.0);
        res.slider_render.param.rect.set_size(30.0, 5.0);
        res
    }

    pub fn vertical() -> ScrollBar {
        let mut res = ScrollBar::new();
        res.fill_render.param.rect.set_size(5.0, 300.0);
        res.slider_render.param.rect.set_size(5.0, 30.0);
        res
    }

    pub fn set_vbar_value_by_offset(&mut self, offset: f32) -> f32 {
        let oy = self.slider_offset_y(offset);
        let roy = self.slider_render.param.rect.offset_y_limit(self.offset.y + oy, self.fill_render.param.rect.dy());
        // let mut offset = Offset::new(Pos::new()).with_y(self.context_offset_y(-roy));
        // if self.offset < roy {
        //     offset.direction = OffsetDirection::Down
        // } else {
        //     offset.direction = OffsetDirection::Up;
        // }
        // println!("{} {} {} {}", roy, offset.y, self.context_height,self.fill_render.param.rect.height());
        self.offset.y = roy;
        self.changed = true;

        // let ut = UpdateType::Offset(offset);
        // ui.update_type = UpdateType::None;
        // self.slider_render.update(ui, true, true);
        // ui.request_update(ut);
        // offset
        self.context_offset_y(-roy)
    }

    pub fn set_hbar_value_by_offset(&mut self, offset: f32) -> f32 {
        let ox = self.slider_offset_x(offset);
        let rox = self.slider_render.param.rect.offset_x_limit(self.offset.x + ox, self.fill_render.param.rect.dx());
        // println!("{} {}", rox, self.context_offset_x(-rox));
        self.offset.x = rox;
        self.changed = true;
        self.context_offset_x(-rox)
    }

    pub fn offset(&mut self) -> f32 {
        if self.height() > self.width() { //垂直滚动条
            self.context_offset_y(-self.offset.y)
        } else { //水平滚动条
            self.context_offset_x(-self.offset.x)
        }
    }

    // pub fn set_size(&mut self, w: f32, h: f32) {
    //     self.set_width(w);
    //     self.set_height(h);
    // }

    pub fn set_width(&mut self, w: f32) {
        self.fill_render.param.rect.set_width(w);
    }

    pub fn set_height(&mut self, h: f32) {
        self.fill_render.param.rect.set_height(h);
    }

    pub fn width(&self) -> f32 {
        self.fill_render.param.rect.width()
    }

    pub fn height(&self) -> f32 {
        self.fill_render.param.rect.height()
    }

    pub fn set_context_height(&mut self, context_height: f32) {
        self.context_size = context_height;
        let mut slider_height = if self.context_size < self.fill_render.param.rect.height() {
            self.fill_render.param.rect.height()
        } else {
            self.fill_render.param.rect.height() * self.fill_render.param.rect.height() / self.context_size
        };
        if slider_height < 32.0 { slider_height = 32.0; }
        self.slider_render.param.rect.set_height(slider_height);
        self.changed = true;
    }

    pub fn set_context_width(&mut self, context_width: f32) {
        self.context_size = context_width;
        let mut slider_width = if self.context_size < self.fill_render.param.rect.width() {
            self.fill_render.param.rect.width()
        } else {
            self.fill_render.param.rect.width() * self.fill_render.param.rect.width() / self.context_size
        };
        if slider_width < 32.0 { slider_width = 32.0; }
        self.slider_render.param.rect.set_width(slider_width);
        self.changed = true;
    }

    // pub fn set_height(&mut self, height: f32) {
    //     self.fill_render.param.rect.set_height(height);
    // }

    // pub fn scrolling(&self) -> bool {
    //     self.offset.y < (self.fill_render.param.rect.height() - self.slider_render.param.rect.height()) && self.offset != 0.0
    // }

    //计算滑块位移
    fn slider_offset_y(&self, cy: f32) -> f32 {
        let scrollable_content = self.context_size - self.fill_render.param.rect.height();
        let scrollable_slider = self.fill_render.param.rect.height() - self.slider_render.param.rect.height();
        let scroll_ratio = cy / scrollable_content; // 内容偏移占比：
        scroll_ratio * scrollable_slider // 滑块应偏移：
    }

    fn slider_offset_x(&self, cx: f32) -> f32 {
        let scrollable_context = self.context_size - self.fill_render.param.rect.width();
        let scrollable_slider = self.fill_render.param.rect.width() - self.slider_render.param.rect.width();
        let scroll_ratio = cx / scrollable_context;
        scroll_ratio * scrollable_slider
    }

    //计算内容位移
    fn context_offset_y(&self, oy: f32) -> f32 {
        let scrollable_content = self.context_size - self.fill_render.param.rect.height();
        let scrollable_slider = self.fill_render.param.rect.height() - self.slider_render.param.rect.height();
        if scrollable_slider == 0.0 { return 0.0; }
        let scroll_ratio = oy / scrollable_slider; // 内容偏移占比：
        scroll_ratio * scrollable_content // 滑块应偏移：
    }

    fn context_offset_x(&self, ox: f32) -> f32 {
        let scrollable_content = self.context_size - self.fill_render.param.rect.width();
        let scrollable_slider = self.fill_render.param.rect.width() - self.slider_render.param.rect.width();
        if scrollable_slider == 0.0 { return 0.0; }
        let scroll_ratio = ox / scrollable_slider;
        scroll_ratio * scrollable_content
    }

    fn init(&mut self, ui: &mut Ui) {
        //背景
        self.fill_render.init_rectangle(ui, false, false);
        //滑块
        self.slider_render.param.rect = self.fill_render.param.rect.clone_with_size(&self.slider_render.param.rect);
        self.slider_render.init_rectangle(ui, false, false);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.param.rect.offset_to_rect(&ui.draw_rect);
            self.slider_render.param.rect.offset_to_rect(&ui.draw_rect);
            self.fill_render.update(ui, false, false);
            self.slider_render.param.rect.offset(&self.offset);
            self.slider_render.update(ui, false, false);
        }
        if ui.widget_changed.contains(WidgetChange::Value) {
            self.slider_render.update(ui, false, false);
        }
    }
}


impl Widget for ScrollBar {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        if self.context_size > self.fill_render.param.rect.height() && self.height() > self.width() {//垂直
            ui.context.render.rectangle.render(&self.fill_render, pass);
            ui.context.render.rectangle.render(&self.slider_render, pass);
        }
        if self.context_size > self.fill_render.param.rect.width() && self.width() > self.height() {//垂直
            ui.context.render.rectangle.render(&self.fill_render, pass);
            ui.context.render.rectangle.render(&self.slider_render, pass);
        }
    }

    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            UpdateType::MouseMove => {
                if self.focused && ui.device.device_input.mouse.pressed {
                    // println!("bar move {}", self.offset.y);
                    if self.height() > self.width() { //垂直滚动条
                        let oy = ui.device.device_input.mouse.offset_y();
                        let roy = self.slider_render.param.rect.offset_y_limit(self.offset.y + oy, self.fill_render.param.rect.dy());
                        self.offset.y = roy;
                    } else { //水平滚动条
                        let ox = ui.device.device_input.mouse.offset_x();
                        let rox = self.slider_render.param.rect.offset_x_limit(self.offset.x + ox, self.fill_render.param.rect.dx());
                        self.offset.x = rox;
                    }
                    ui.context.window.request_redraw();
                    // let mut offset = Offset::new(ui.device.device_input.mouse.pressed_pos).with_y(self.context_offset_y(-roy));
                    // if self.offset < roy {
                    //     offset.direction = OffsetDirection::Down
                    // } else {
                    //     offset.direction = OffsetDirection::Up;
                    // }

                    self.changed = true;
                    // let ut = UpdateType::Offset(offset);
                    // ui.update_type = UpdateType::None;
                    // self.slider_render.update(ui, true, true);
                    // ui.context.window.request_redraw();
                    // ui.request_update(ut);
                }
            }
            UpdateType::MousePress => self.focused = ui.device.device_input.pressed_at(&self.slider_render.param.rect),
            // UpdateType::Offset(ref o) => {
            //     // let oy = self.slider_offset_y(o.y);
            //     // let roy = self.slider_render.param.rect.offset_y_limit(self.offset + oy, self.fill_render.param.rect.dy());
            //     // let mut offset = Offset::new(o.pos).with_y(self.context_offset_y(-roy));
            //     // if self.offset < roy {
            //     //     offset.direction = OffsetDirection::Down
            //     // } else {
            //     //     offset.direction = OffsetDirection::Up;
            //     // }
            //     // self.offset = roy;
            //     //
            //     // let ut = UpdateType::Offset(offset);
            //     // ui.update_type = UpdateType::None;
            //     // self.slider_render.update(ui, true, true);
            //     // ui.request_update(ut);
            // }
            _ => {
                if self.changed {
                    self.changed = false;
                    self.slider_render.update(ui, false, false);
                }
            }
        }
        Response::new(&self.id, WidgetSize::same(self.fill_render.param.rect.width(), self.fill_render.param.rect.height()))
    }
}