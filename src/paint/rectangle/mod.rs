use crate::frame::context::Render;
use crate::paint::rectangle::param::RectangleParam;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::Device;
use wgpu::util::DeviceExt;

pub(crate) mod param;

pub struct PaintRectangle {
    param_buffer: wgpu::Buffer,
    pub(crate) param: RectangleParam,
    index: usize,
}

impl PaintRectangle {
    pub fn new(ui: &mut Ui, rect: Rect) -> Self {
        let param = RectangleParam {
            rect,
            style: ui.style.widget.click.clone(),
        };
        let param_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Uniform"),
            contents: bytemuck::bytes_of(&param.as_draw_param(false, false)),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let index = ui.ui_manage.context.render.rectangle.create_bind_group(&ui.device, &param_buffer);
        PaintRectangle {
            param_buffer,
            param,
            index,
        }
    }

    pub fn offset(&mut self, target_x: f32) {
        let x = self.param.rect.x.min;
        self.param.rect.offset_x(target_x - x);
    }

    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.param.rect
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.param.style = style;
    }

    pub fn prepare(&mut self, device: &Device, hovered: bool, mouse_down: bool) {
        println!("prepare{} {}", hovered, mouse_down);
        let draw_param = self.param.as_draw_param(hovered, mouse_down);
        device.queue.write_buffer(&self.param_buffer, 0, bytemuck::bytes_of(&draw_param));
    }
    pub fn render(&mut self, render: &Render, render_pass: &mut wgpu::RenderPass) {
        render.rectangle.render(self.index, render_pass);
    }
}