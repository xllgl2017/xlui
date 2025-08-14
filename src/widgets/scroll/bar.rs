use crate::frame::context::UpdateType;
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::Widget;
use crate::Offset;
use crate::response::Response;

pub struct ScrollBar {
    id: String,
    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,
    slider_param: RectParam,
    slider_index: usize,
    slider_buffer: Option<wgpu::Buffer>,
    context_height: f32,
    focused: bool,
    offset: f32,
}


impl ScrollBar {
    pub fn new() -> ScrollBar {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(215, 215, 215); //Color::TRANSPARENT; //
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
            fill_param: RectParam::new(Rect::new().with_size(10.0, 20.0), fill_style),
            fill_index: 0,
            fill_buffer: None,
            slider_param: RectParam::new(Rect::new().with_size(10.0, 10.0), slider_style),
            slider_index: 0,
            slider_buffer: None,
            context_height: 0.0,
            focused: false,
            offset: 0.0,
        }
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.set_rect(rect);
        self
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.fill_param.rect = rect;
        self.slider_param.rect.set_width(self.fill_param.rect.width());
    }

    pub fn set_context_height(&mut self, context_height: f32) {
        self.context_height = context_height;
        let mut slider_height = if self.context_height < self.fill_param.rect.height() {
            self.fill_param.rect.height()
        } else {
            self.fill_param.rect.height() * self.fill_param.rect.height() / self.context_height
        };
        if slider_height < 32.0 { slider_height = 32.0; }
        self.slider_param.rect.set_height(slider_height);
    }

    pub fn context_height(mut self, context_height: f32) -> Self {
        self.set_context_height(context_height);
        self
    }

    pub fn set_height(&mut self, height: f32) {
        self.fill_param.rect.set_height(height);
    }

    //计算滑块位移
    fn slider_offset_y(&self, cy: f32) -> f32 {
        let scrollable_content = self.context_height - self.fill_param.rect.height();
        let scrollable_slider = self.fill_param.rect.height() - self.slider_param.rect.height();
        let scroll_ratio = cy / scrollable_content; // 内容偏移占比：
        scroll_ratio * scrollable_slider // 滑块应偏移：
    }

    //计算内容位移
    fn context_offset_y(&self, oy: f32) -> f32 {
        let scrollable_content = self.context_height - self.fill_param.rect.height();
        let scrollable_slider = self.fill_param.rect.height() - self.slider_param.rect.height();
        if scrollable_slider == 0.0 { return 0.0; }
        let scroll_ratio = oy / scrollable_slider; // 内容偏移占比：
        scroll_ratio * scrollable_content // 滑块应偏移：
    }

    fn init(&mut self, ui: &mut Ui) {
        //背景
        let data = self.fill_param.as_draw_param(false, false);
        let fill_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        self.fill_buffer = Some(fill_buffer);
        //滑块
        self.slider_param.rect = self.fill_param.rect.clone_with_size(&self.slider_param.rect);
        let data = self.slider_param.as_draw_param(false, false);
        let slider_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.slider_index = ui.context.render.rectangle.create_bind_group(&ui.device, &slider_buffer);
        self.slider_buffer = Some(slider_buffer);
    }
}


impl Widget for ScrollBar {
    fn redraw(&mut self, ui: &mut Ui) -> Response {
        if self.fill_buffer.is_none() { self.init(ui); }
        let resp = Response::new(&self.id, &self.fill_param.rect);
        if ui.pass.is_none() { return resp; }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        if self.context_height > self.fill_param.rect.height() {
            ui.context.render.rectangle.render(self.slider_index, pass);
        }
        resp
    }

    fn update(&mut self, ui: &mut Ui) {
        match ui.update_type {
            UpdateType::MouseMove => {
                if self.focused && ui.device.device_input.mouse.pressed {
                    let oy = ui.device.device_input.mouse.offset_y();
                    println!("{} {} {:?}", self.offset, self.slider_param.rect.dy().max + self.offset, self.fill_param.rect.dy());
                    // let ly = self.fill_param.rect.dy.min..self.fill_param.rect.dy.max;
                    let roy = self.slider_param.rect.offset_y_limit(self.offset + oy, self.fill_param.rect.dy());
                    self.offset = roy;
                    ui.update_type = UpdateType::Offset(Offset::new_y(self.context_offset_y(-roy)));
                    let data = self.slider_param.as_draw_param(true, true);
                    ui.device.queue.write_buffer(self.slider_buffer.as_ref().unwrap(), 0, data);
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MousePress => self.focused = ui.device.device_input.pressed_at(&self.slider_param.rect),
            UpdateType::Offset(ref o) => {
                let oy = self.slider_offset_y(o.y);
                // let ly = self.fill_param.rect.dy.min..self.fill_param.rect.dy.max;
                let roy = self.slider_param.rect.offset_y_limit(self.offset + oy, self.fill_param.rect.dy());
                self.offset = roy;
                ui.update_type = UpdateType::Offset(Offset::new_y(self.context_offset_y(-roy)));
                let data = self.slider_param.as_draw_param(true, true);
                ui.device.queue.write_buffer(self.slider_buffer.as_ref().unwrap(), 0, data);
                ui.context.window.request_redraw();
            }
            _ => {}
        }
    }
}