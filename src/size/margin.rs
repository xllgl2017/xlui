pub struct Margin {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}


impl Margin {
    pub const ZERO: Margin = Margin { left: 0.0, top: 0.0, right: 0.0, bottom: 0.0 };

    pub fn same(v: f32) -> Self {
        Margin {
            left: v,
            top: v,
            right: v,
            bottom: v,

        }
    }

    pub fn vertical(&self) -> f32 {
        self.left + self.right
    }

    pub fn horizontal(&self) -> f32 {
        self.top + self.bottom
    }
}