use crate::render::WrcParam;
use crate::{Size, Ui};
use crate::size::rect::Rect;
use crate::style::ClickStyle;

#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "gpu", derive(bytemuck::Pod, bytemuck::Zeroable), )]
pub struct CircleDrawParam {
    center: [f32; 2],          // ⬅️ 圆心坐标（像素坐标）
    radius: f32,               // ⬅️ 圆半径
    border_thickness: f32,     // ⬅️ 边框宽度
    fill_color: [f32; 4],      // ⬅️ 填充颜色
    border_color: [f32; 4],    // ⬅️ 边框颜色
}

pub struct CircleParam {
    pub(crate) rect: Rect,
    pub(crate) style: ClickStyle,
    draw: CircleDrawParam,
}

impl CircleParam {
    pub fn new(rect: Rect, style: ClickStyle) -> Self {
        let fill_color = style.dyn_fill(false, false).as_gamma_rgba();
        let border = style.dyn_border(false, false);
        let center = [rect.dx().center(), rect.dy().center()];
        let radius = rect.height() / 2.0;
        let draw = CircleDrawParam {
            center,
            radius,
            border_thickness: border.left_width,
            fill_color,
            border_color: border.color.as_gamma_rgba(),
        };
        Self {
            rect,
            style,
            draw,
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, hovered: bool, press: bool) {
        let fill = self.style.dyn_fill(press, hovered);
        let border = self.style.dyn_border(press, hovered);
        ui.context.window.win32().paint_circle(ui.hdc.unwrap(), &self.rect, fill, border);
    }
}

impl WrcParam for CircleParam {
    #[cfg(feature = "gpu")]
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, _: Size) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered).as_gamma_rgba();
        let border = self.style.dyn_border(mouse_down, hovered);
        let center = [self.rect.dx().center(), self.rect.dy().center()];
        let radius = self.rect.height() / 2.0;
        self.draw.center = center;
        self.draw.radius = radius;
        self.draw.border_thickness = border.left_width;
        self.draw.fill_color = fill_color;
        self.draw.border_color = border.color.as_gamma_rgba();
        bytemuck::bytes_of(&self.draw)
    }
}
