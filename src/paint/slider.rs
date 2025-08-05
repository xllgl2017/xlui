use crate::frame::context::{Context, Render};
use crate::paint::color::Color;
use crate::paint::rectangle::PaintRectangle;
use crate::radius::Radius;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::Device;
use std::ops::Range;
use crate::response::Response;
use crate::widgets::slider::Slider;

pub struct PaintSlider {
    id: String,
    fill: PaintRectangle,
    slider: PaintRectangle,
    value: f32,
    value_range: Range<f32>,
    focused: bool,
}

impl PaintSlider {
    pub fn new(ui: &mut Ui, slider: &Slider) -> PaintSlider {
        let mut fill_rect = slider.rect.clone();
        fill_rect.y.min += 5.0;
        fill_rect.y.max -= 5.0;
        let mut fill = PaintRectangle::new(ui, fill_rect);
        let mut fill_style = ui.style.widget.click.clone();
        fill_style.fill.inactive = Color::rgb(56, 182, 244);
        fill_style.fill.hovered = Color::rgb(56, 182, 244);
        fill_style.fill.clicked = Color::rgb(56, 182, 244);
        fill_style.border.inactive = Border::new(0.0).radius(Radius::same(3));
        fill_style.border.hovered = Border::new(0.0).radius(Radius::same(3));
        fill_style.border.clicked = Border::new(0.0).radius(Radius::same(3));
        fill.set_style(fill_style);
        fill.prepare(&ui.device, false, false);
        let mut slider_rect = slider.rect.clone();
        slider_rect.x.min -= slider.rect.height() / 2.0;
        slider_rect.set_width(slider.rect.height());
        let offset = slider.value * slider.rect.width() / (slider.range.end - slider.range.start);
        slider_rect.offset_x(offset);
        let mut slider_rectangle = PaintRectangle::new(ui, slider_rect);
        let mut slider_style = ui.style.widget.click.clone();
        slider_style.fill.inactive = Color::rgb(56, 182, 244);
        slider_style.fill.hovered = Color::rgb(56, 182, 244);
        slider_style.fill.clicked = Color::rgb(56, 182, 244);
        slider_style.border.inactive = Border::new(0.0).color(Color::BLACK).radius(Radius::same(8));
        slider_style.border.hovered = Border::new(1.0).color(Color::BLACK).radius(Radius::same(8));
        slider_style.border.clicked = Border::new(1.0).color(Color::BLACK).radius(Radius::same(8));
        slider_rectangle.set_style(slider_style);
        slider_rectangle.prepare(&ui.device, false, false);
        PaintSlider {
            id:slider.id.clone(),
            fill,
            slider: slider_rectangle,
            value: slider.value,
            value_range: slider.range.clone(),
            focused: false,
        }
    }

    pub fn render(&mut self, render: &Render, render_pass: &mut wgpu::RenderPass) {
        self.fill.render(render, render_pass);
        self.slider.render(render, render_pass);
    }

    pub fn mouse_move(&mut self, device: &Device, context: &Context, resp: &mut Response) {
        let (x, y) = device.device_input.mouse.lastest();
        let slider_rect = &mut self.slider.param.rect;
        let fill_rect = &mut self.fill.param.rect;
        let has_pos = slider_rect.has_position(x, y);
        if (has_pos || self.focused) && device.device_input.mouse.pressed {
            slider_rect.offset_x(device.device_input.mouse.offset_x());
            if slider_rect.x.max > fill_rect.x.max + slider_rect.height() / 2.0 {
                slider_rect.offset_x(fill_rect.x.max + slider_rect.height() / 2.0 - slider_rect.x.max);
            }
            if slider_rect.x.min < fill_rect.x.min - slider_rect.height() / 2.0 {
                slider_rect.offset_x(fill_rect.x.min - slider_rect.height() / 2.0 - slider_rect.x.min);
            }
            let cl = (slider_rect.width() / 2.0 + slider_rect.x.min - fill_rect.x.min) / fill_rect.width();
            let cv = (self.value_range.end - self.value_range.start) * cl;
            if self.value != cv { context.window.request_redraw(); }
            resp.slider_mut(&self.id).unwrap().value = cv;
            self.value = cv;
        }
        self.slider.prepare(device, has_pos, device.device_input.mouse.pressed);
    }

    pub fn mouse_down(&mut self, device: &Device) {
        let (x, y) = device.device_input.mouse.lastest();
        self.focused = self.slider.param.rect.has_position(x, y);
    }

    pub fn rect(&self) -> &Rect { &self.fill.param.rect }
}