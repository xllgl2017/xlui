use crate::frame::context::Context;
use crate::frame::App;
use crate::paint::color::Color;
use crate::paint::rectangle::PaintRectangle;
use crate::paint::text::PaintText;
use crate::radius::Radius;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::checkbox::CheckBox;
use crate::Device;
use std::any::Any;
use crate::response::Callback;

pub struct PaintCheckBox {
    id: String,
    check: PaintRectangle,
    text: PaintText,
    checked_text: PaintText,
    checked: bool,
    rect: Rect,
    hovered: bool,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, bool)>>,
}

impl PaintCheckBox {
    pub fn new(ui: &mut Ui, checkbox: &mut CheckBox) -> PaintCheckBox {
        let mut check_rect = checkbox.rect.clone();
        check_rect.set_width(15.0);
        check_rect.set_height(15.0);
        let mut check = PaintRectangle::new(ui, check_rect.clone());
        let mut check_style = ui.style.widget.click.clone();
        check_style.fill.inactive = Color::rgb(210, 210, 210);
        check_style.fill.hovered = Color::rgb(210, 210, 210);
        check_style.fill.clicked = Color::rgb(210, 210, 210);
        check_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        check_style.border.hovered = Border::new(1.0).color(Color::BLACK).radius(Radius::same(2));
        check_style.border.clicked = Border::new(1.0).color(Color::BLACK).radius(Radius::same(2));
        check.set_style(check_style);
        check.prepare(&ui.device, false, false);
        let text = PaintText::new(ui, &mut checkbox.text);
        let mut text_buffer = TextBuffer::new("√".to_string());
        text_buffer.text_size.font_size = 12.0;
        text_buffer.reset_size(&ui.ui_manage.context);
        text_buffer.rect = check_rect;
        text_buffer.rect.y.min += 2.0;
        let checked_text = PaintText::new(ui, &text_buffer);
        PaintCheckBox {
            id: checkbox.id.clone(),
            check,
            text,
            checked: checkbox.value,
            checked_text,
            rect: checkbox.rect.clone(),
            hovered: false,
            callback: checkbox.callback.take(),
        }
    }

    pub fn mouse_move(&mut self, device: &Device, context: &Context) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.rect.has_position(x, y);
        self.check.prepare(device, has_pos, device.device_input.mouse.pressed);
        if self.hovered != has_pos {
            self.hovered = has_pos;
            context.window.request_redraw();
        }
    }

    pub fn mouse_click<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        let (lx, ly) = device.device_input.mouse.pressed_pos;
        let (x, y) = device.device_input.mouse.lastest();
        if !self.rect.has_position(x, y) || !self.rect.has_position(lx, ly) { return; }
        self.checked = !self.checked;
        if let Some(ref mut callback) = self.callback {
            callback(app, context, self.checked)
        }
        context.window.request_redraw();

    }

    pub fn render(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        self.check.render(&context.render, render_pass);
        self.text.render(device, context, render_pass);
        if self.checked { self.checked_text.render(device, context, render_pass); }
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn connect<A: 'static>(&mut self, f: fn(&mut A, &mut Context, bool)) {
        self.callback = Some(Callback::create_check(f));
    }
}