use crate::frame::context::{Context, ContextUpdate};
use crate::layout::Layout;
use crate::paint::color::Color;
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::style::{BorderStyle, ClickStyle, FillStyle, Shadow};
use crate::ui::{DrawParam, Ui};
use crate::Device;
use crate::frame::App;

pub struct Popup {
    pub(crate) id: String,
    parent_id: String,
    rect: Rect,
    pub(crate) layout: Layout,
    fill_index: usize,
}

impl Popup {
    pub fn new(ui: &mut Ui, rect: Rect, pid: String) -> Popup {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            color: Color::rgba(0, 0, 0, 30),
        };
        let mut fill_param = RectParam::new(rect.clone(), Popup::popup_style())
            .with_shadow(shadow);
        let data = fill_param.as_draw_param(false, false);
        let fill_buffer = ui.ui_manage.context.render.rectangle.create_buffer(&ui.device, data);
        let fill_index = ui.ui_manage.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        Popup {
            id: crate::gen_unique_id(),
            parent_id: pid,
            rect,
            layout: Layout::top_to_bottom(),
            fill_index,
        }
    }

    pub fn popup_style() -> ClickStyle {
        ClickStyle {
            fill: FillStyle {
                inactive: Color::rgb(240, 240, 240),
                hovered: Color::rgb(240, 240, 240),
                clicked: Color::rgb(240, 240, 240),
            },
            border: BorderStyle {
                inactive: Border {
                    width: 1.0,
                    radius: Radius::same(5),
                    color: Color::rgba(144, 209, 255, 255),
                },
                hovered: Border {
                    width: 1.0,
                    radius: Radius::same(5),
                    color: Color::rgba(144, 209, 255, 255),
                },
                clicked: Border {
                    width: 1.0,
                    radius: Radius::same(5),
                    color: Color::rgba(144, 209, 255, 255),
                },
            },
        }
    }

    pub fn click(&mut self, device: &Device, context: &mut Context) {
        if !context.popup_opened(&self.id) { return; }
        for (index, widget) in self.layout.widgets.iter_mut().enumerate() {
            if !device.device_input.click_at(widget.rect()) { continue; }
            context.send_update(self.parent_id.clone(), ContextUpdate::Combo(index));
            break;
        }
    }

    pub fn draw<A: App>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        if !param.context.popup_opened(&self.id) { return; }
        param.context.render.rectangle.render(self.fill_index, pass);
        for widget in self.layout.widgets.iter_mut() {
            widget.draw(param, pass);
        }
    }
}

