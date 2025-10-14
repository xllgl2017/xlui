#[cfg(feature = "gpu")]
use crate::render::WrcParam;
use crate::size::rect::Rect;
use crate::style::{ClickStyle, FrameStyle, Shadow};
use crate::*;
#[cfg(feature = "gpu")]
use crate::render::Screen;
#[cfg(feature = "gpu")]
use crate::shape::rectangle::RectangleShape;

pub struct RectParam {
    pub(crate) rect: Rect,
    pub(crate) style: ClickStyle,
    pub(crate) shadow: Shadow,
    #[cfg(feature = "gpu")]
    pub(crate) rect_shape: RectangleShape,
    #[cfg(feature = "gpu")]
    pub(crate) screen:Screen
}

impl RectParam {
    pub fn new() -> Self {
        RectParam {
            rect: Rect::new(),
            style: ClickStyle::new(),
            shadow: Shadow::new(),
            #[cfg(feature = "gpu")]
            rect_shape: RectangleShape::new(),
            #[cfg(feature = "gpu")]
            screen: Screen {
                size: [1000.0,800.0],
            },
        }
    }

    pub fn new_frame(rect: Rect, frame: FrameStyle) -> Self {
        let mut style = ClickStyle::new();
        style.fill = FillStyle::same(frame.fill);
        style.border = BorderStyle::same(frame.border);
        let res = Self::new().with_rect(rect).with_style(style);
        res.with_shadow(frame.shadow)
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.rect.set_size(w, h);
        self
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.rect.set_height(h);
        self
    }

    pub fn with_style(mut self, style: ClickStyle) -> Self {
        self.set_style(style);
        self
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.style = style;
    }

    pub fn set_frame(&mut self, frame: FrameStyle) {
        let mut style = ClickStyle::new();
        style.fill = FillStyle::same(frame.fill);
        style.border = BorderStyle::same(frame.border);
        self.style = style;
        self.shadow = frame.shadow;
    }

    pub fn with_shadow(mut self, shadow: Shadow) -> RectParam {
        self.shadow = shadow;
        self
    }
}
#[cfg(feature = "gpu")]
impl WrcParam for RectParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, size: Size) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered);
        let border = self.style.dyn_border(mouse_down, hovered);
        self.rect_shape.reset(&self.rect, fill_color, border);
        self.screen.size=[size.width, size.height];
        bytemuck::bytes_of(&self.screen)
    }
}