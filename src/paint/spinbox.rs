use crate::{Device, Pos};
use crate::frame::context::Context;
use crate::paint::color::Color;
use crate::paint::edit::PaintTextEdit;
use crate::paint::triangle::PaintTriangle;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::vertex::Vertex;
use crate::widgets::textedit::TextEdit;

pub struct PaintSpinBox {
    edit: PaintTextEdit,
    triangle: PaintTriangle,
    value: i32,
    up_rect: Rect,
    down_rect: Rect,
    rect: Rect,
}

impl PaintSpinBox {
    pub fn new(ui: &mut Ui, rect: &Rect, text_edit: &TextEdit) -> PaintSpinBox {
        let mut triangle = PaintTriangle::new(ui);
        let color = Color::rgb(95, 95, 95);
        let inactive_color = Color::rgb(153, 152, 152);
        let up_rect = Rect {
            x: Pos { min: rect.x.max - 14.0, max: rect.x.max },
            y: Pos { min: rect.y.min + 1.0, max: rect.y.min + rect.height() / 2.0 - 2.0 },
        };
        let vertices = vec![
            Vertex::new([up_rect.x.min + up_rect.width() / 2.0, up_rect.y.min], &color, &ui.ui_manage.context.size),
            Vertex::new([up_rect.x.min, up_rect.y.max], &color, &ui.ui_manage.context.size),
            Vertex::new([rect.x.max, up_rect.y.max], &color, &ui.ui_manage.context.size),
        ];
        triangle.add_triangle(vertices, &ui.device);
        let down_rect = Rect {
            x: Pos { min: rect.x.max - 14.0, max: rect.x.max },
            y: Pos { min: rect.y.max - rect.height() / 2.0 + 2.0, max: rect.y.max - 2.0 },
        };
        triangle.add_triangle(vec![
            Vertex::new([down_rect.x.min + down_rect.width() / 2.0, down_rect.y.max], &inactive_color, &ui.ui_manage.context.size),
            Vertex::new([rect.x.max - 14.0, down_rect.y.min], &inactive_color, &ui.ui_manage.context.size),
            Vertex::new([rect.x.max, down_rect.y.min], &inactive_color, &ui.ui_manage.context.size),
        ], &ui.device);
        let mut edit = PaintTextEdit::new(ui, text_edit.rect.clone(), &text_edit.text_buffer);
        text_edit.gen_style(ui, &mut edit);
        edit.fill.prepare(&ui.device, false, false);
        // edit.prepare(&ui.device, &mut ui.ui_manage.context, false, false, false);
        PaintSpinBox {
            edit,
            value: 0,
            up_rect,
            triangle,
            down_rect,
            rect: rect.clone(),
        }
    }

    pub fn prepare(&mut self, device: &Device, context: &Context) {
        self.triangle.prepare(device, context);
    }

    pub fn render(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        self.edit.render(device, context, render_pass);
        self.triangle.render(render_pass);
    }

    pub fn click(&mut self, device: &Device, context: &mut Context) {
        let (x, y) = device.device_input.mouse.lastest();
        if self.up_rect.has_position(x, y) {
            self.value += 1;
            self.edit.set_text(self.value.to_string().as_str(), context);
            context.window.request_redraw();
        } else if self.down_rect.has_position(x, y) {
            self.value -= 1;
            self.edit.set_text(self.value.to_string().as_str(), context);
            context.window.request_redraw();
        }
        self.edit.click(device, context);
    }

    pub fn mouse_move(&mut self, device: &Device, context: &Context) {
        let (x, y) = device.device_input.mouse.lastest();
        if self.up_rect.has_position(x, y) {
            let hovered_color = Color::rgb(95, 95, 95);
            self.triangle.vertices[0..3].iter_mut().for_each(|x| x.color = hovered_color.as_gamma_rgba());
            self.prepare(device, context);
            context.window.request_redraw();
        } else if self.down_rect.has_position(x, y) {
            let hovered_color = Color::rgb(95, 95, 95);
            self.triangle.vertices[3..6].iter_mut().for_each(|x| x.color = hovered_color.as_gamma_rgba());
            self.prepare(device, context);
            context.window.request_redraw();
        }
        self.edit.mouse_move(device, context);
    }

    pub fn mouse_down(&mut self, device: &Device, context: &mut Context) {
        self.edit.mouse_down(device, context);
        if !self.edit.focused {
            self.value = self.edit.text().parse::<i32>().unwrap_or(self.value);
            self.edit.set_text(self.value.to_string().as_str(), context);
        }
    }


    pub fn key_input(&mut self, device: &Device, context: &mut Context, key: winit::keyboard::Key) {
        self.edit.key_input(device, context, key);
    }

    pub fn rect(&self) -> &Rect { &self.rect }
}