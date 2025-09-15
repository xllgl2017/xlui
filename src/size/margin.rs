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

    pub fn left(mut self, v: f32) -> Self {
        self.left = v;
        self
    }

    pub fn right(mut self, v: f32) -> Self {
        self.right = v;
        self
    }

    pub fn top(mut self, v: f32) -> Self {
        self.top = v;
        self
    }

    pub fn bottom(mut self, v: f32) -> Self {
        self.bottom = v;
        self
    }
}