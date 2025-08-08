use crate::frame::context::Context;
use crate::frame::App;
use crate::paint::color::Color;
use crate::paint::edit::PaintTextEdit;
use crate::paint::triangle::PaintTriangle;
use crate::response::Callback;
use crate::size::rect::Rect;
use crate::ui::{DrawParam, Ui};
use crate::vertex::Vertex;
use crate::widgets::spinbox::SpinBox;
use crate::{Device, Pos};
use std::any::Any;
use std::ops::Range;

pub struct PaintSpinBox {
    id: String,
    edit: PaintTextEdit,
    triangle: PaintTriangle,
    value: i32,
    up_rect: Rect,
    up_index: Range<usize>,
    down_rect: Rect,
    down_index: Range<usize>,
    rect: Rect,
    range: Range<i32>,
    color: Color,
    inactive_color: Color,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, i32)>>,
}

impl PaintSpinBox {
    pub fn new(ui: &mut Ui, spinbox: &mut SpinBox) -> PaintSpinBox {
        let mut triangle = PaintTriangle::new(ui);
        let color = Color::rgb(95, 95, 95);
        let inactive_color = Color::rgb(153, 152, 152);
        let up_rect = Rect {
            x: Pos { min: spinbox.rect.x.max - 14.0, max: spinbox.rect.x.max },
            y: Pos { min: spinbox.rect.y.min + 1.0, max: spinbox.rect.y.min + spinbox.rect.height() / 2.0 - 2.0 },
        };
        let vertices = vec![
            Vertex::new([up_rect.x.min + up_rect.width() / 2.0, up_rect.y.min], &color, &ui.ui_manage.context.size),
            Vertex::new([up_rect.x.min, up_rect.y.max], &color, &ui.ui_manage.context.size),
            Vertex::new([spinbox.rect.x.max, up_rect.y.max], &color, &ui.ui_manage.context.size),
        ];
        let up_index = triangle.add_triangle(vertices, &ui.device);
        let down_rect = Rect {
            x: Pos { min: spinbox.rect.x.max - 14.0, max: spinbox.rect.x.max },
            y: Pos { min: spinbox.rect.y.max - spinbox.rect.height() / 2.0 + 2.0, max: spinbox.rect.y.max - 2.0 },
        };
        let down_index = triangle.add_triangle(vec![
            Vertex::new([down_rect.x.min + down_rect.width() / 2.0, down_rect.y.max], &color, &ui.ui_manage.context.size),
            Vertex::new([spinbox.rect.x.max - 14.0, down_rect.y.min], &color, &ui.ui_manage.context.size),
            Vertex::new([spinbox.rect.x.max, down_rect.y.min], &color, &ui.ui_manage.context.size),
        ], &ui.device);
        let mut edit = PaintTextEdit::new(ui, &mut spinbox.edit);
        spinbox.edit.gen_style(ui, &mut edit);
        edit.fill.prepare(&ui.device, false, false);
        // edit.prepare(&ui.device, &mut ui.ui_manage.context, false, false, false);
        PaintSpinBox {
            id: spinbox.id.clone(),
            edit,
            value: spinbox.value,
            up_rect,
            triangle,
            down_rect,
            down_index,
            rect: spinbox.rect.clone(),
            range: spinbox.range.clone(),
            color,
            up_index,
            inactive_color,
            callback: spinbox.callback.take(),
        }
    }

    pub fn prepare(&mut self, device: &Device, context: &Context) {
        self.triangle.prepare(device, context);
    }

    pub fn render<A>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        self.edit.render(param, pass);
        self.triangle.render(pass);
    }

    pub fn click<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        let (lx, ly) = device.device_input.mouse.pressed_pos;
        let (x, y) = device.device_input.mouse.lastest();
        if self.up_rect.has_position(x, y) && self.up_rect.has_position(lx, ly) {
            let is_end = self.value >= self.range.end;
            let is_start = self.value == self.range.start;
            if !is_end {
                self.value += 1;
                self.edit.set_text(self.value.to_string().as_str(), context);
                if let Some(ref mut callback) = self.callback {
                    callback(app, context, self.value);
                }
            }
            let mut is_change = is_end || self.value == self.range.end;
            self.triangle.vertices[self.up_index.clone()].iter_mut().for_each(|x| {
                x.color = if self.value == self.range.end { self.inactive_color.as_gamma_rgba() } else { self.color.as_gamma_rgba() }
            });
            if is_start {
                self.triangle.vertices[self.down_index.clone()].iter_mut().for_each(|x| {
                    x.color = self.color.as_gamma_rgba();
                });
                is_change = true;
            }
            if is_change { self.triangle.prepare(device, context); }

            context.window.request_redraw();
        } else if self.down_rect.has_position(x, y) && self.down_rect.has_position(lx, ly) {
            let is_start = self.value == self.range.start;
            let is_end = self.value >= self.range.end;
            if !is_start {
                self.value -= 1;
                self.edit.set_text(self.value.to_string().as_str(), context);
                if let Some(ref mut callback) = self.callback {
                    callback(app, context, self.value);
                }
            }
            let mut is_change = is_start || self.value == self.range.start;
            if is_end {
                self.triangle.vertices[self.up_index.clone()].iter_mut().for_each(|x| {
                    x.color = self.color.as_gamma_rgba();
                });
                is_change = true;
            }
            self.triangle.vertices[self.down_index.clone()].iter_mut().for_each(|x| {
                x.color = if self.value == self.range.start { self.inactive_color.as_gamma_rgba() } else { self.color.as_gamma_rgba() };
            });

            if is_change { self.triangle.prepare(device, context); }
            context.window.request_redraw();
        }
        // resp.spinbox_mut(&self.id).unwrap().value = self.value;
    }

    pub fn mouse_move(&mut self, device: &Device, context: &Context) {
        // let (x, y) = device.device_input.mouse.lastest();
        // if self.up_rect.has_position(x, y) {
        //     // let hovered_color = Color::rgb(95, 95, 95);
        //     // self.triangle.vertices[0..3].iter_mut().for_each(|x| x.color = hovered_color.as_gamma_rgba());
        //     self.prepare(device, context);
        //     context.window.request_redraw();
        // } else if self.down_rect.has_position(x, y) {
        //     let hovered_color = Color::rgb(95, 95, 95);
        //     self.triangle.vertices[3..6].iter_mut().for_each(|x| x.color = hovered_color.as_gamma_rgba());
        //     self.prepare(device, context);
        //     context.window.request_redraw();
        // }
        self.edit.mouse_move(device, context);
    }

    pub fn mouse_down<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        if self.edit.focused {
            self.value = self.edit.text().parse::<i32>().unwrap_or(self.value);
            if self.value > self.range.end {
                self.value = self.range.end;
            };
            if self.value < self.range.start {
                self.value = self.range.start;
            }
            self.triangle.vertices[self.up_index.clone()].iter_mut().for_each(|x| {
                x.color = if self.value == self.range.end { self.inactive_color.as_gamma_rgba() } else { self.color.as_gamma_rgba() };
            });
            self.triangle.vertices[self.down_index.clone()].iter_mut().for_each(|x| {
                x.color = if self.value == self.range.start { self.inactive_color.as_gamma_rgba() } else { self.color.as_gamma_rgba() };
            });
            self.triangle.prepare(device, context);
            self.edit.set_text(self.value.to_string().as_str(), context);
            if let Some(ref mut callback) = self.callback {
                callback(app, context, self.value);
            }
        }
        self.edit.mouse_down(device, context);
    }


    pub fn key_input<A: App>(&mut self, device: &Device, context: &mut Context, key: winit::keyboard::Key, app: &mut A) {
        self.edit.key_input(device, context, key, app);
    }

    pub fn rect(&self) -> &Rect { &self.rect }

    pub fn connect<A: App>(&mut self, f: fn(&mut A, &mut Context, i32)) {
        self.callback = Some(Callback::create_spinbox(f));
    }
}