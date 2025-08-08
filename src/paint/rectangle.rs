use crate::frame::context::Context;
use crate::frame::App;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::Callback;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::{DrawParam, Ui};
use crate::Device;
use std::any::Any;
use wgpu::util::DeviceExt;

pub struct PaintRectangle {
    pub(crate) id: String,
    buffer: wgpu::Buffer,
    pub(crate) param: RectParam,
    index: usize,
    hovered: bool,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context)>>,
}

impl PaintRectangle {
    pub fn new(ui: &mut Ui, rect: Rect) -> Self {
        let mut param = RectParam::new(rect, ui.style.widget.click.clone());
        let param_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Uniform"),
            contents: &param.as_draw_param(false, false),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let index = ui.ui_manage.context.render.rectangle.create_bind_group(&ui.device, &param_buffer);
        PaintRectangle {
            id: crate::gen_unique_id(),
            buffer: param_buffer,
            param,
            index,
            hovered: false,
            callback: None,
        }
    }

    #[deprecated]
    pub fn offset_to_x(&mut self, target_x: f32) {
        let x = self.param.rect.x.min;
        self.param.rect.offset_x(target_x - x);
    }

    pub fn offset(&mut self, device: &Device, ox: f32, oy: f32) -> Vec<(String, Rect)> {
        self.param.rect.offset(ox, oy);
        if ox != 0.0 || oy != 0.0 {
            let data = self.param.as_draw_param(self.hovered, device.device_input.mouse.pressed);
            device.queue.write_buffer(&self.buffer, 0, data);
            return vec![(self.id.clone(), self.param.rect.clone())];
        }
        vec![]
    }

    pub fn rect(&self) -> &Rect { &self.param.rect }

    pub fn rect_mut(&mut self) -> &mut Rect {
        &mut self.param.rect
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.param.style = style;
    }

    pub fn prepare(&mut self, device: &Device, hovered: bool, mouse_down: bool) {
        let draw_param = self.param.as_draw_param(hovered, mouse_down);
        device.queue.write_buffer(&self.buffer, 0, draw_param);
    }
    pub fn render<A>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        param.context.render.rectangle.render(self.index, pass);
    }

    pub fn mouse_move(&mut self, device: &Device, context: &mut Context) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.param.rect.has_position(x, y);
        if self.hovered == has_pos { return; }
        self.hovered = has_pos;
        let data = self.param.as_draw_param(self.hovered, device.device_input.mouse.pressed);
        device.queue.write_buffer(&self.buffer, 0, &data);
        context.window.request_redraw();
    }

    pub fn connect<A: App>(&mut self, f: impl FnMut(&mut A, &mut Context) + 'static) {
        self.callback = Some(Callback::create_click(f));
    }
}