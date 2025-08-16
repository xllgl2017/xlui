use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::Response;
use crate::size::rect::Rect;
use crate::style::{ClickStyle, Shadow};
use crate::ui::Ui;
use crate::widgets::Widget;

pub struct Rectangle {
    id: String,
    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,
    hovered: bool,
    changed: bool,
}

impl Rectangle {
    pub fn new(rect: Rect, style: ClickStyle) -> Self {
        Rectangle {
            id: crate::gen_unique_id(),
            fill_param: RectParam::new(rect, style),
            fill_index: 0,
            fill_buffer: None,
            hovered: false,
            changed: false,
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

    pub fn with_shadow(mut self, shadow: Shadow) -> Self {
        self.fill_param = self.fill_param.with_shadow(shadow);
        self
    }

    pub fn style_mut(&mut self) -> &mut ClickStyle {
        self.changed = true;
        &mut self.fill_param.style
    }

    pub fn offset_x(&mut self, v: f32) {
        self.changed = true;
        self.fill_param.shadow.offset[0] = v;
    }

    pub fn offset_y(&mut self, v: f32) {
        self.changed = true;
        self.fill_param.shadow.offset[1] = v;
    }
}

impl Widget for Rectangle {
    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.fill_param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.update_rect(ui);
                }
            }
            UpdateType::Offset(ref o) => {
                if !ui.can_offset { return Response::new(&self.id, &self.fill_param.rect); }
                self.fill_param.rect.offset(o);
                self.update_rect(ui);
            }
            _ => {}
        }
        if self.changed {
            self.changed = false;
            let data = self.fill_param.as_draw_param(false, false);
            ui.device.queue.write_buffer(&self.fill_buffer.as_ref().unwrap(), 0, data);
            ui.context.window.request_redraw();
        }
        Response::new(&self.id, &self.fill_param.rect)
    }
}