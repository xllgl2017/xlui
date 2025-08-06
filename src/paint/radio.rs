use crate::frame::context::Context;
use crate::paint::color::Color;
use crate::paint::text::PaintText;
use crate::render::circle::param::CircleParam;
use crate::render::WrcRender;
use crate::response::Response;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::widgets::radio::RadioButton;
use crate::Device;

pub struct PaintRadioButton {
    id: String,
    pub(crate) rect: Rect,
    text: PaintText,
    outer_param: CircleParam,
    outer_buffer: wgpu::Buffer,
    outer_index: usize,
    inner_param: CircleParam,
    inner_buffer: wgpu::Buffer,
    inner_index: usize,
    value: bool,
    hovered: bool,
}

impl PaintRadioButton {
    pub fn new(ui: &mut Ui, radio: &RadioButton) -> Self {
        let mut outer_radio_rect = radio.rect.clone();
        outer_radio_rect.set_width(radio.rect.height());
        let mut outer_style = ui.style.widget.click.clone();
        outer_style.fill.inactive = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.fill.hovered = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.fill.clicked = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.border.inactive = Border::new(1.0).color(Color::rgb(95, 95, 95));
        outer_style.border.hovered = Border::new(1.0).color(Color::rgb(56, 160, 200));
        outer_style.border.clicked = Border::new(1.0).color(Color::rgb(56, 182, 244));

        let mut outer_param = CircleParam::new(outer_radio_rect, outer_style);
        let outer_buffer = ui.ui_manage.context.render.circle.create_buffer(&ui.device, outer_param.as_draw_param(false, false));
        let outer_index = ui.ui_manage.context.render.circle.create_bind_group(&ui.device, &outer_buffer);

        let mut inner_radio_rect = radio.rect.clone();
        inner_radio_rect.x.min += 4.0;
        inner_radio_rect.y.min += 4.0;
        inner_radio_rect.y.max -= 4.0;
        inner_radio_rect.set_width(inner_radio_rect.height());
        let mut inner_style = ui.style.widget.click.clone();
        inner_style.fill.inactive = Color::TRANSPARENT;
        inner_style.fill.hovered = Color::rgb(56, 160, 200);
        inner_style.fill.clicked = Color::rgb(56, 182, 244);
        inner_style.border.inactive = Border::new(0.0).color(Color::TRANSPARENT);
        inner_style.border.hovered = Border::new(0.0).color(Color::TRANSPARENT);
        inner_style.border.clicked = Border::new(0.0).color(Color::TRANSPARENT);

        let mut inner_param = CircleParam::new(inner_radio_rect, inner_style);
        let data = inner_param.as_draw_param(radio.value, radio.value);
        let inner_buffer = ui.ui_manage.context.render.circle.create_buffer(&ui.device, data);
        let data = outer_param.as_draw_param(radio.value, radio.value);
        ui.device.queue.write_buffer(&outer_buffer, 0, data);
        let inner_index = ui.ui_manage.context.render.circle.create_bind_group(&ui.device, &inner_buffer);
        let text = PaintText::new(ui, &radio.text);
        PaintRadioButton {
            id: radio.id.clone(),
            outer_buffer,
            outer_param,
            outer_index,
            rect: radio.rect.clone(),
            text,
            inner_param,
            inner_buffer,
            inner_index,
            value: radio.value,
            hovered: false,
        }
    }

    pub fn mouse_move(&mut self, device: &Device, context: &mut Context) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.rect.has_position(x, y);
        let data = self.inner_param.as_draw_param(self.value || has_pos, self.value);
        device.queue.write_buffer(&self.inner_buffer, 0, data);
        let data = self.outer_param.as_draw_param(self.value || has_pos, self.value);
        device.queue.write_buffer(&self.outer_buffer, 0, data);
        if self.hovered != has_pos {
            context.window.request_redraw();
            self.hovered = has_pos;
        }
    }

    pub fn click(&mut self, device: &Device, context: &mut Context, resp: &mut Response) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.rect.has_position(x, y);
        if !has_pos { return; }
        self.value = !self.value;
        let data = self.outer_param.as_draw_param(self.value, self.value);
        device.queue.write_buffer(&self.outer_buffer, 0, data);
        let data = self.inner_param.as_draw_param(self.value, self.value);
        device.queue.write_buffer(&self.inner_buffer, 0, data);
        context.window.request_redraw();

        resp.checked_mut(&self.id).unwrap().checked = self.value;
    }

    pub fn draw(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        context.render.circle.render(self.outer_index, render_pass);
        context.render.circle.render(self.inner_index, render_pass);
        self.text.render(device, context, render_pass);
    }
}