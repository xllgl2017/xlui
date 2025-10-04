use crate::size::radius::Radius;
use crate::style::color::Color;

#[derive(Clone, PartialEq)]
pub struct Border {
    pub left_width: f32,
    pub right_width: f32,
    pub top_width: f32,
    pub bottom_width: f32,
    pub radius: Radius,
    pub color: Color,
}

impl Border {
    pub fn same(width: f32) -> Self {
        Border {
            left_width: width,
            right_width: width,
            top_width: width,
            bottom_width: width,
            radius: Radius::same(3),
            color: Color::rgb(0, 0, 0),
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

    pub fn radius(mut self, radius: Radius) -> Self {
        self.radius = radius;
        self
    }
}
