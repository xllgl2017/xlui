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


pub struct CircleParam {
    pub(crate) rect: Rect,
    pub(crate) style: ClickStyle,
    #[cfg(feature = "gpu")]
    pub(crate) circle_shape: CircleShape,
    #[cfg(feature = "gpu")]
    pub(crate) screen: Screen,
}

impl CircleParam {
    pub fn new(rect: Rect, style: ClickStyle) -> Self {
        Self {
            rect,
            style,
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
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, size: Size) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered);
        let border = self.style.dyn_border(mouse_down, hovered);
        self.circle_shape.draw(&self.rect, fill_color, border);
        while (self.circle_shape.indices.len() * 2) % 4 != 0 {
            self.circle_shape.indices.push(0);
        }
        self.screen.size=[size.width, size.height];
        bytemuck::bytes_of(&self.screen)
    }
}
