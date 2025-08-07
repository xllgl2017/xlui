use crate::Device;
use crate::frame::context::Context;
use crate::layout::Layout;
use crate::layout::popup::Popup;
use crate::paint::color::Color;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::rect::Rect;
use crate::style::Shadow;
use crate::ui::Ui;

pub struct PaintPopup {
    pub(crate) rect: Rect,
    fill_index: usize,
    layout: Layout,
}


impl PaintPopup {
    pub fn new(ui: &mut Ui, popup: &mut Popup) -> PaintPopup {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            color: Color::rgba(0, 0, 0, 30),
        };
        let mut fill_param = RectParam::new(popup.rect().clone(), Popup::popup_style())
            .with_shadow(shadow);
        let data = fill_param.as_draw_param(false, false);
        let fill_buffer = ui.ui_manage.context.render.rectangle.create_buffer(&ui.device, data);
        let fill_index = ui.ui_manage.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        PaintPopup {
            rect: popup.rect().clone(),
            fill_index,
            layout: popup.layout.take().unwrap(),
        }
    }


    pub fn draw(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        context.render.rectangle.render(self.fill_index, render_pass);
        for widget in self.layout.widgets.iter_mut() {
            widget.draw(device, context, render_pass);
        }
    }
}