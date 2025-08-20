use crate::render::WrcParam;
use crate::size::rect::Rect;
use crate::style::ClickStyle;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CircleDrawParam {
    center: [f32; 2],
    radius: f32,
    border_thickness: f32,
    fill_color: [f32; 4],
    border_color: [f32; 4],
}

pub struct CircleParam {
    pub(crate) rect: Rect,
    style: ClickStyle,
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
            border_thickness: border.width,
            fill_color,
            border_color: border.color.as_gamma_rgba(),
        };
        Self {
            rect,
            style,
            draw,
        }
    }
}

impl WrcParam for CircleParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered).as_gamma_rgba();
        let border = self.style.dyn_border(mouse_down, hovered);
        let center = [self.rect.dx().center(), self.rect.dy().center()];
        let radius = self.rect.height() / 2.0;
        self.draw.center = center;
        self.draw.radius = radius;
        self.draw.border_thickness = border.width;
        self.draw.fill_color = fill_color;
        self.draw.border_color = border.color.as_gamma_rgba();
        bytemuck::bytes_of(&self.draw)
    }
}
