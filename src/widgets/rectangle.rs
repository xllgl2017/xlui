use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::Response;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::Widget;

pub struct Rectangle {
    id: String,
    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,
    hovered: bool,
}

impl Rectangle {
    pub fn new(rect: Rect, style: ClickStyle) -> Self {
        Rectangle {
            id: crate::gen_unique_id(),
            fill_param: RectParam::new(rect, style),
            fill_index: 0,
            fill_buffer: None,
            hovered: false,
        }
    }

    fn update_rect(&mut self, ui: &mut Ui) {
        let data = self.fill_param.as_draw_param(self.hovered, ui.device.device_input.mouse.pressed);
        ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
        ui.context.window.request_redraw();
    }

    fn init(&mut self, ui: &mut Ui) {
        let data = self.fill_param.as_draw_param(false, false);
        let buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &buffer);
        self.fill_buffer = Some(buffer);
    }
}

impl Widget for Rectangle {
    fn redraw(&mut self, ui: &mut Ui) -> Response {
        if self.fill_buffer.is_none() { self.init(ui); }
        let resp = Response::new(&self.id, &self.fill_param.rect);
        if ui.pass.is_none() { return resp; }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        resp
    }

    fn update(&mut self, ui: &mut Ui) {
        if let Some(ref offset) = ui.canvas_offset {
            self.fill_param.rect.offset(offset.x, offset.y);
            self.update_rect(ui);
            return;
        }
        let hovered = ui.device.device_input.hovered_at(&self.fill_param.rect);
        if self.hovered != hovered {
            self.hovered = hovered;
            self.update_rect(ui);
        }
    }
}