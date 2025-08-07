use crate::{Device, Pos};
use crate::frame::context::Context;
use crate::paint::color::Color;
use crate::paint::popup::PaintPopup;
use crate::paint::text::PaintText;
use crate::paint::triangle::PaintTriangle;
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::vertex::Vertex;
use crate::widgets::combobox::ComboBox;

pub struct PaintComboBox {
    popup: PaintPopup,
    text: PaintText,
    triangle: PaintTriangle,
    fill_param: RectParam,
    fill_buffer: wgpu::Buffer,
    fill_index: usize,
    open: bool,
}

impl PaintComboBox {
    pub fn new(ui: &mut Ui, combobox: &mut ComboBox) -> PaintComboBox {
        let popup = PaintPopup::new(ui, &mut combobox.popup);
        let text = PaintText::new(ui, &combobox.text_buffer);

        let mut fill_style = ui.style.widget.click.clone();
        fill_style.fill.inactive = Color::rgb(230, 230, 230);
        fill_style.border.inactive = Border::new(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        let mut fill_param = RectParam::new(combobox.rect.clone(), fill_style);
        let data = fill_param.as_draw_param(false, false);
        let fill_buffer = ui.ui_manage.context.render.rectangle.create_buffer(&ui.device, data);
        let fill_index = ui.ui_manage.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);

        let mut triangle = PaintTriangle::new(ui);
        let down_rect = Rect {
            x: Pos { min: combobox.rect.x.max - 14.0, max: combobox.rect.x.max - 4.0 },
            y: Pos { min: combobox.rect.y.min + 5.0, max: combobox.rect.y.max - 6.0 },
        };
        let color = Color::rgb(95, 95, 95);
        triangle.add_triangle(vec![
            Vertex::new([down_rect.x.min + down_rect.width() / 2.0, down_rect.y.max], &color, &ui.ui_manage.context.size),
            Vertex::new([down_rect.x.min, down_rect.y.min], &color, &ui.ui_manage.context.size),
            Vertex::new([down_rect.x.max, down_rect.y.min], &color, &ui.ui_manage.context.size),
        ], &ui.device);
        // triangle.add_triangle(vec![
        //     Vertex::new([down_rect.x.min + down_rect.width() / 2.0, down_rect.y.min], &color, &ui.ui_manage.context.size),
        //     Vertex::new([down_rect.x.min, down_rect.y.max], &color, &ui.ui_manage.context.size),
        //     Vertex::new([down_rect.x.max, down_rect.y.max], &color, &ui.ui_manage.context.size),
        // ], &ui.device);
        PaintComboBox {
            popup,
            text,
            triangle,
            fill_param,
            fill_buffer,
            fill_index,
            open: false,
        }
    }

    pub fn rect(&self) -> &Rect { &self.fill_param.rect }

    pub fn click(&mut self, device: &Device, context: &Context) {
        let (x, y) = device.device_input.mouse.lastest();
        if self.fill_param.rect.has_position(x, y) { //在显示区域点击
            self.open = !self.open;
        } else if self.popup.rect.has_position(x, y) { //弹窗区域点击

        } else { self.open = false; }
        context.window.request_redraw();
    }

    pub fn resize(&mut self, device: &Device, context: &Context) {
        self.triangle.prepare(device, context);
    }

    pub fn draw(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        context.render.rectangle.render(self.fill_index, render_pass);
        self.triangle.render(render_pass);
        self.text.render(device, context, render_pass);
        if self.open { self.popup.draw(device, context, render_pass); }
    }
}