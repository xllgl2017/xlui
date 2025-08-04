use crate::frame::context::Context;
use crate::paint::rectangle::PaintRectangle;
use crate::paint::text::PaintText;
use crate::size::padding::Padding;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::button::Button;
use crate::Device;
use crate::size::rect::Rect;

pub struct PaintButton {
    id: String,
    fill: PaintRectangle,
    text: PaintText,
    mouse_down: bool,
    hovered: bool,
}

impl PaintButton {
    pub fn new(ui: &mut Ui, btn: &Button, buffer: &TextBuffer) -> PaintButton {
        let rectangle_rect = btn.rect.clone_add_padding(&Padding::same(btn.border.width as f32));

        let fill = PaintRectangle::new(ui, rectangle_rect);
        let text = PaintText::new(ui, buffer);
        PaintButton {
            id: btn.id.clone(),
            fill,
            text,
            mouse_down: false,
            hovered: false,
        }
    }

    pub fn mouse_move(&mut self, device: &Device, context: &Context) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.fill.param.rect.has_position(x, y);
        if has_pos != self.hovered || device.device_input.mouse.pressed != self.mouse_down {
            println!("{} {}", has_pos, device.device_input.mouse.pressed);
            self.fill.prepare(device, has_pos, device.device_input.mouse.pressed);
            context.window.request_redraw();
        }
        self.hovered = has_pos;
        self.mouse_down = device.device_input.mouse.pressed;
    }


    pub fn render(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        self.fill.render(&context.render, render_pass);
        self.text.render(device, context, render_pass);
    }

    pub fn offset(&mut self, device: &Device, ox: f32, oy: f32) -> Vec<(String, Rect)> {
        if ox != 0.0 || oy != 0.0 {
            self.fill.param.rect.offset(ox, oy);
            self.text.rect.offset(ox, oy);
            self.fill.prepare(device, self.hovered, self.hovered);
            vec![(self.id.clone(), self.fill.param.rect.clone())]
        } else {
            vec![]
        }
    }
    pub fn rect(&self) -> &Rect { &self.fill.param.rect }
}