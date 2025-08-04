use crate::Device;
use crate::frame::context::{Context, Render};
use crate::paint::color::Color;
use crate::paint::rectangle::param::RectangleParam;
use crate::radius::Radius;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::ui::Ui;

pub struct PaintScrollBar {
    inner_buffer: wgpu::Buffer,
    outer_index: usize,
    inner_index: usize,
    outer_param: RectangleParam,
    inner_param: RectangleParam,
    hovered: bool,
    focused: bool,
    pub(crate) offset_y: f32,
}

impl PaintScrollBar {
    pub fn new(ui: &mut Ui, rect: &Rect) -> Self {
        let mut outer_style = ui.style.widget.click.clone();
        outer_style.fill.inactive = Color::rgb(215, 215, 215);
        outer_style.fill.hovered = Color::rgb(215, 215, 215);
        outer_style.fill.clicked = Color::rgb(215, 215, 215);
        let outer_param = RectangleParam {
            rect: rect.clone(),
            style: outer_style,
        };
        let mut rect = rect.clone();
        rect.set_height(30.0);
        let mut inner_style = ui.style.widget.click.clone();
        inner_style.fill.inactive = Color::rgb(56, 182, 244);
        inner_style.fill.hovered = Color::rgb(56, 182, 244);
        inner_style.fill.clicked = Color::rgb(56, 182, 244);
        inner_style.border.inactive = Border::new(0).radius(Radius::same(2));
        inner_style.border.hovered = Border::new(0).radius(Radius::same(2));
        inner_style.border.clicked = Border::new(0).radius(Radius::same(2));
        let inner_param = RectangleParam {
            rect,
            style: inner_style,
        };

        let inner_buffer = ui.ui_manage.context.render.rectangle.create_buffer(&ui.device, &inner_param);
        let outer_buffer = ui.ui_manage.context.render.rectangle.create_buffer(&ui.device, &outer_param);
        let outer_index = ui.ui_manage.context.render.rectangle.create_bind_group(&ui.device, &outer_buffer);
        let inner_index = ui.ui_manage.context.render.rectangle.create_bind_group(&ui.device, &inner_buffer);

        PaintScrollBar {
            inner_buffer,
            outer_index,
            inner_index,
            outer_param,
            inner_param,
            hovered: false,
            focused: false,
            offset_y: 0.0,
        }
    }

    pub(crate) fn offset_y(&mut self, device: &Device, oy: f32) {
        self.offset_y = oy;
        self.inner_param.rect.offset_y(oy);
        if self.inner_param.rect.y.min < self.outer_param.rect.y.min {
            let oy = self.inner_param.rect.y.min - self.outer_param.rect.y.min;
            self.offset_y -= oy;
            self.inner_param.rect.offset_y(-oy);
        }
        if self.inner_param.rect.y.max > self.outer_param.rect.y.max {
            let oy = self.inner_param.rect.y.max - self.outer_param.rect.y.max;
            self.offset_y -= oy;
            self.inner_param.rect.offset_y(-oy);
        }
        println!("{}", self.offset_y);
        let draw_param = self.inner_param.as_draw_param(true, device.device_input.mouse.pressed);
        device.queue.write_buffer(&self.inner_buffer, 0, bytemuck::bytes_of(&draw_param));
    }

    pub fn mouse_move(&mut self, device: &Device, context: &Context) {
        self.offset_y = 0.0;
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.inner_param.rect.has_position(x, y);
        match (has_pos || self.focused) && device.device_input.mouse.pressed {
            true => {
                if !device.device_input.mouse.pressed || !self.focused { return; } //非滚动
                let oy = device.device_input.mouse.offset_y();
                if oy == 0.0 { return; }
                self.offset_y(device, oy);
                context.window.request_redraw();
            }
            false => {
                if self.hovered != has_pos {
                    let draw_param = self.inner_param.as_draw_param(has_pos, device.device_input.mouse.pressed);
                    device.queue.write_buffer(&self.inner_buffer, 0, bytemuck::bytes_of(&draw_param));
                    self.hovered = has_pos;
                    context.window.request_redraw();
                }
            }
        }
    }

    pub fn mouse_down(&mut self, device: &Device) {
        let (x, y) = device.device_input.mouse.lastest();
        let focus = self.inner_param.rect.has_position(x, y);
        self.focused = focus;
    }
    pub fn render(&mut self, render: &Render, render_pass: &mut wgpu::RenderPass) {
        render.rectangle.render(self.outer_index, render_pass);
        render.rectangle.render(self.inner_index, render_pass);
    }

    pub fn rect(&self) -> &Rect {
        &self.outer_param.rect
    }
}