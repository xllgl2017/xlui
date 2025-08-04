use crate::frame::context::{Context, Render};
use crate::paint::color::Color;
use crate::paint::rectangle::PaintRectangle;
use crate::radius::Radius;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::Device;
use std::ops::Range;

pub struct PaintSlider {
    fill: PaintRectangle,
    slider: PaintRectangle,
    value: f32,
    value_range: Range<f32>,
    focused: bool,
}

impl PaintSlider {
    pub fn new(ui: &mut Ui, rect: &Rect) -> PaintSlider {
        let mut fill_rect = rect.clone();
        fill_rect.y.min += 5.0;
        fill_rect.y.max -= 5.0;
        let mut fill = PaintRectangle::new(ui, fill_rect);
        let mut fill_style = ui.style.widget.click.clone();
        fill_style.fill.inactive = Color::rgb(56, 182, 244);
        fill_style.fill.hovered = Color::rgb(56, 182, 244);
        fill_style.fill.clicked = Color::rgb(56, 182, 244);
        fill_style.border.inactive = Border::new(0).radius(Radius::same(3));
        fill_style.border.hovered = Border::new(0).radius(Radius::same(3));
        fill_style.border.clicked = Border::new(0).radius(Radius::same(3));
        fill.set_style(fill_style);
        fill.prepare(&ui.device, false, false);
        let mut slider_rect = rect.clone();
        slider_rect.x.min -= rect.height() / 2.0;
        slider_rect.set_width(rect.height());
        let mut slider = PaintRectangle::new(ui, slider_rect);
        let mut slider_style = ui.style.widget.click.clone();
        slider_style.fill.inactive = Color::rgb(56, 182, 244);
        slider_style.fill.hovered = Color::rgb(56, 182, 244);
        slider_style.fill.clicked = Color::rgb(56, 182, 244);
        slider_style.border.inactive = Border::new(0).color(Color::BLACK).radius(Radius::same(8));
        slider_style.border.hovered = Border::new(1).color(Color::BLACK).radius(Radius::same(8));
        slider_style.border.clicked = Border::new(1).color(Color::BLACK).radius(Radius::same(8));
        slider.set_style(slider_style);
        slider.prepare(&ui.device, false, false);
        PaintSlider {
            fill,
            slider,
            value: 0.0,
            value_range: 0.0..100.0,
            focused: false,
        }
    }

    pub fn render(&mut self, render: &Render, render_pass: &mut wgpu::RenderPass) {
        self.fill.render(render, render_pass);
        self.slider.render(render, render_pass);
    }

    pub fn mouse_move(&mut self, device: &Device, context: &Context) {
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