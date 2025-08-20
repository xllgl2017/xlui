#[derive(Clone)]
pub struct Padding {
    pub left: f32,
    pub bottom: f32,
    pub right: f32,
    pub top: f32,
}

impl Padding {
    pub(crate) const ZERO: Padding = Padding { left: 0.0, bottom: 0.0, right: 0.0, top: 0.0 };
    pub fn same(padding: f32) -> Padding {
        Padding {
            left: padding,
            bottom: padding,
            right: padding,
            top: padding,
        }
    }

    pub fn vertical(&self) -> f32 { self.top + self.bottom }

    pub fn horizontal(&self) -> f32 { self.right + self.left }
}