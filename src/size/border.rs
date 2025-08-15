use crate::size::radius::Radius;
use crate::style::color::Color;

#[derive(Clone)]
pub struct Border {
    pub width: f32,
    pub radius: Radius,
    pub color: Color,
}

impl Border {
    pub fn new(width: f32) -> Self {
        Border {
            width,
            radius: Radius::same(3),
            color: Color::rgb(0, 0, 0),
        }
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
