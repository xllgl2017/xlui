use crate::frame::context::Context;
use crate::paint::color::Color;
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::ui::{DrawParam, Ui};
use crate::Device;

pub struct PaintScrollBar {
    inner_buffer: wgpu::Buffer,
    outer_index: usize,
    inner_index: usize,
    outer_param: RectParam,
    inner_param: RectParam,
    hovered: bool,
    focused: bool,
    pub(crate) offset_y: f32,
    height: f32,
}

impl PaintScrollBar {
    pub fn new(ui: &mut Ui, rect: &Rect, height: f32) -> Self {
        let mut outer_style = ui.style.widget.click.clone();
        outer_style.fill.inactive = Color::rgb(215, 215, 215);
        outer_style.fill.hovered = Color::rgb(215, 215, 215);
        outer_style.fill.clicked = Color::rgb(215, 215, 215);
        let mut outer_param = RectParam::new(rect.clone(), outer_style);
        let mut rect = rect.clone();
        let mut inner_height = if height < rect.height() { rect.height() } else { rect.height() * rect.height() / height };
        if inner_height < 32.0 { inner_height = 32.0; }
        rect.set_height(inner_height);
        let mut inner_style = ui.style.widget.click.clone();
        inner_style.fill.inactive = Color::rgb(56, 182, 244);
        inner_style.fill.hovered = Color::rgb(56, 182, 244);
        inner_style.fill.clicked = Color::rgb(56, 182, 244);
        inner_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        inner_style.border.hovered = Border::new(0.0).radius(Radius::same(2));
        inner_style.border.clicked = Border::new(0.0).radius(Radius::same(2));
        let mut inner_param = RectParam::new(rect, inner_style);

        let inner_buffer = ui.ui_manage.context.render.rectangle.create_buffer(&ui.device, inner_param.as_draw_param(false, false));
        let outer_buffer = ui.ui_manage.context.render.rectangle.create_buffer(&ui.device, outer_param.as_draw_param(false, false));
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
            height,
        }
    }

    fn slider_offset_y(&self, cy: f32) -> f32 {
        let scrollable_content = self.height - self.outer_param.rect.height();
        let scrollable_slider = self.outer_param.rect.height() - self.inner_param.rect.height();
        let scroll_ratio = cy / scrollable_content; // 内容偏移占比：
        scroll_ratio * scrollable_slider // 滑块应偏移：
    }

    fn context_offset_y(&self, oy: f32) -> f32 {
        let scrollable_content = self.height - self.outer_param.rect.height();
        let scrollable_slider = self.outer_param.rect.height() - self.inner_param.rect.height();
        if scrollable_slider == 0.0 { return 0.0; }
        // println!("{} {}", scrollable_content, scrollable_slider);
        let scroll_ratio = oy / scrollable_slider; // 内容偏移占比：
        scroll_ratio * scrollable_content // 滑块应偏移：
    }

    pub(crate) fn offset_y(&mut self, device: &Device, oy: f32, ct: bool) {
        let oy = if ct {
            self.offset_y = oy;
            let oy = self.slider_offset_y(oy);
            if oy == 0.0 { self.offset_y = 0.0; }
            oy
        } else {
            self.offset_y = self.context_offset_y(oy);
            if self.offset_y == 0.0 { 0.0 } else { oy }
        };
        self.inner_param.rect.offset_y(oy);
        if self.inner_param.rect.y.min < self.outer_param.rect.y.min {
            let oy = self.inner_param.rect.y.min - self.outer_param.rect.y.min;
            self.offset_y -= self.context_offset_y(oy);
            self.inner_param.rect.offset_y(-oy);
        }
        if self.inner_param.rect.y.max > self.outer_param.rect.y.max {
            let oy = self.inner_param.rect.y.max - self.outer_param.rect.y.max;
            self.offset_y -= self.context_offset_y(oy);
            self.inner_param.rect.offset_y(-oy);
        }
        let draw_param = self.inner_param.as_draw_param(true, device.device_input.mouse.pressed);
        device.queue.write_buffer(&self.inner_buffer, 0, draw_param);
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
                self.offset_y(device, oy, false);
                context.window.request_redraw();
            }
            false => {
                if self.hovered != has_pos {
                    let draw_param = self.inner_param.as_draw_param(has_pos, device.device_input.mouse.pressed);
                    device.queue.write_buffer(&self.inner_buffer, 0, draw_param);
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
    pub fn render<A>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        param.context.render.rectangle.render(self.outer_index, pass);
        param.context.render.rectangle.render(self.inner_index, pass);
    }

    pub fn rect(&self) -> &Rect {
        &self.outer_param.rect
    }
}