use crate::style::color::Color;

#[derive(Clone, PartialEq)]
pub struct Border {
    pub left_width: f32,
    pub right_width: f32,
    pub top_width: f32,
    pub bottom_width: f32,
    pub color: Color,
}

impl Border {
    pub fn same(width: f32) -> Self {
        Border {
            left_width: width,
            right_width: width,
            top_width: width,
            bottom_width: width,
            color: Color::BLACK,
        }
    }

    pub fn set_same(&mut self, width: f32) {
        self.left_width = width;
        self.right_width = width;
        self.top_width = width;
        self.bottom_width = width;
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_bottom(mut self, w: f32) -> Self {
        self.bottom_width = w;
        self
    }

    pub fn width(&self) -> f32 {
        self.top_width
    }
}
