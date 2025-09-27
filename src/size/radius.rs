#[derive(Clone, PartialEq)]
pub struct Radius {
    pub left_bottom: u8,
    pub right_bottom: u8,
    pub right_top: u8,
    pub left_top: u8,
}

impl Radius {
    pub fn same(radius: u8) -> Radius {
        Radius {
            left_bottom: radius,
            right_bottom: radius,
            right_top: radius,
            left_top: radius,
        }
    }

    pub fn with_left_bottom(mut self, radius: u8) -> Radius {
        self.left_bottom = radius;
        self
    }

    pub fn with_right_bottom(mut self, radius: u8) -> Radius {
        self.right_bottom = radius;
        self
    }

    pub fn with_right_top(mut self, radius: u8) -> Radius {
        self.right_top = radius;
        self
    }

    pub fn with_left_top(mut self, radius: u8) -> Radius {
        self.left_top = radius;
        self
    }
}
