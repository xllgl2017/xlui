use crate::frame::context::{Context, ContextUpdate};
use crate::frame::App;
use crate::paint::color::Color;
use crate::paint::text::PaintText;
use crate::paint::triangle::PaintTriangle;
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::Callback;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::ui::{DrawParam, Ui};
use crate::vertex::Vertex;
use crate::widgets::combobox::ComboBox;
use crate::{Device, Pos};
use std::any::Any;

pub struct PaintComboBox {
    id: String,
    text: PaintText,
    triangle: PaintTriangle,
    fill_param: RectParam,
    fill_buffer: wgpu::Buffer,
    fill_index: usize,
    popup_open: bool,
    popup_id: String,
    data: Vec<String>,
    pub(crate) callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, usize)>>,
}

impl PaintComboBox {
    pub fn new(ui: &mut Ui, combobox: &mut ComboBox, popup_id: String) -> PaintComboBox {
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
            // popup,
            id: combobox.id.clone(),
            text,
            triangle,
            fill_param,
            fill_buffer,
            fill_index,
            popup_open: false,
            popup_id,
            data: combobox.data.clone(),
            callback: combobox.callback.take(),
        }
    }

    pub fn rect(&self) -> &Rect { &self.fill_param.rect }

    pub fn item_click(&mut self, row: usize) {}

    pub fn click(&mut self, device: &Device, context: &mut Context) {
        let (x, y) = device.device_input.mouse.lastest();
        if self.fill_param.rect.has_position(x, y) { //在显示区域点击
            self.popup_open = !self.popup_open;
            context.send_update(self.popup_id.clone(), ContextUpdate::Popup(self.popup_open));
            context.window.request_redraw();
        }

    }

    pub fn resize(&mut self, device: &Device, context: &Context) {
        self.triangle.prepare(device, context);
    }

    pub fn mouse_move<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {

        // self.popup.mouse_move(device, context, app);
    }

    pub fn draw<A: App>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        if let Some(update) = param.context.updates.remove(&self.id) {
            self.popup_open = false;
            let index = update.combo();
            self.text.set_text(param.context, self.data[index].as_str());
            param.context.send_update(self.popup_id.clone(), ContextUpdate::Popup(false));
            if let Some(ref mut callback) = self.callback {
                callback(param.app, param.context, index);
            }
        }
        param.context.render.rectangle.render(self.fill_index, pass);
        self.triangle.render(pass);
        self.text.render(param, pass);
        // if self.open { self.popup.draw(device, context, render_pass); }
    }

    pub fn connect<A: App>(&mut self, f: fn(&mut A, &mut Context, usize)) {
        self.callback = Some(Callback::create_combobox(f));
    }
}