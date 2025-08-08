use crate::frame::context::Context;
use crate::frame::App;
use crate::paint::rectangle::PaintRectangle;
use crate::paint::text::PaintText;
use crate::response::Callback;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::ui::{DrawParam, Ui};
use crate::widgets::button::Button;
use crate::Device;
use std::any::Any;

pub struct PaintButton {
    id: String,
    fill: PaintRectangle,
    text: PaintText,
    mouse_down: bool,
    hovered: bool,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context)>>,
}

impl PaintButton {
    pub fn new(ui: &mut Ui, btn: &mut Button) -> PaintButton {
        let rectangle_rect = btn.rect.clone_add_padding(&Padding::same(btn.border.width));

        let fill = PaintRectangle::new(ui, rectangle_rect);
        let text = PaintText::new(ui, &btn.text_buffer);
        PaintButton {
            id: btn.id.clone(),
            fill,
            text,
            mouse_down: false,
            hovered: false,
            callback: btn.callback.take(),
        }
    }

    pub fn mouse_move(&mut self, device: &Device, context: &Context) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.fill.param.rect.has_position(x, y);
        if has_pos != self.hovered {
            self.fill.prepare(device, has_pos, device.device_input.mouse.pressed);
            context.window.request_redraw();
        } else if self.hovered && device.device_input.mouse.pressed != self.mouse_down {
            self.fill.prepare(device, has_pos, device.device_input.mouse.pressed);
            context.window.request_redraw();
        }


        // if has_pos != self.hovered || device.device_input.mouse.pressed != self.mouse_down {
        //     // println!("{} {}", has_pos, device.device_input.mouse.pressed);
        //
        // }
        self.hovered = has_pos;
        self.mouse_down = device.device_input.mouse.pressed;
    }

    pub fn click<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        let (lx, ly) = device.device_input.mouse.pressed_pos;
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.fill.param.rect.has_position(lx, ly) && self.fill.param.rect.has_position(x, y);
        if let Some(ref mut callback) = self.callback && has_pos { //按下和释放均在本控件上才会触发
            callback(app, context);
        }
    }


    pub fn render<A>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        self.fill.render(param, pass);
        self.text.render(param, pass);
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

    pub fn connect<A: App>(&mut self, f: impl FnMut(&mut A, &mut Context) + 'static) {
        self.callback = Some(Callback::create_click(f));
    }
}