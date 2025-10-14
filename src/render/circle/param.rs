#[cfg(feature = "gpu")]
use crate::render::Screen;
#[cfg(feature = "gpu")]
use crate::render::WrcParam;
#[cfg(feature = "gpu")]
use crate::shape::circle::CircleShape;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
#[cfg(feature = "gpu")]
use crate::Size;

#[cfg(feature = "gpu")]
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
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
    #[cfg(feature = "gpu")]
    draw: CircleDrawParam,
    #[cfg(feature = "gpu")]
    pub(crate) circle_shape: CircleShape,
    #[cfg(feature = "gpu")]
    pub(crate) screen: Screen,
}

impl CircleParam {
    pub fn new(rect: Rect, style: ClickStyle) -> Self {
        #[cfg(feature = "gpu")]
        let fill_color = style.dyn_fill(false, false).as_gamma_rgba();
        #[cfg(feature = "gpu")]
        let border = style.dyn_border(false, false);
        #[cfg(feature = "gpu")]
        let center = [rect.dx().center(), rect.dy().center()];
        #[cfg(feature = "gpu")]
        let radius = rect.height() / 2.0;
        #[cfg(feature = "gpu")]
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
            #[cfg(feature = "gpu")]
            draw,
            #[cfg(feature = "gpu")]
            circle_shape: CircleShape::new(),
            #[cfg(feature = "gpu")]
            screen: Screen { size: [1000.0, 800.0] },
        }
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.style = style;
    }
}

#[cfg(feature = "gpu")]
impl WrcParam for CircleParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, _: Size) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered);
        let border = self.style.dyn_border(mouse_down, hovered);
        let center = [self.rect.dx().center(), self.rect.dy().center()];
        let radius = self.rect.height() / 2.0;
        self.draw.center = center;
        self.draw.radius = radius;
        self.draw.border_thickness = border.left_width;
        self.draw.fill_color = fill_color.as_gamma_rgba();
        self.draw.border_color = border.color.as_gamma_rgba();
        self.circle_shape.draw(&self.rect, fill_color, border);
        while (self.circle_shape.indices.len() * 2) % 4 != 0 {
            self.circle_shape.indices.push(0);
        }
        bytemuck::bytes_of(&self.draw)
    }
}
