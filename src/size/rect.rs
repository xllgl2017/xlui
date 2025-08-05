use crate::size::padding::Padding;
use crate::Pos;

#[derive(Clone, PartialEq, Debug)]
pub struct Rect {
    pub x: Pos,
    pub y: Pos,
}

impl Rect {
    pub fn new() -> Rect {
        Rect {
            x: Pos {
                min: 0.0,
                max: 0.0,
            },
            y: Pos {
                min: 0.0,
                max: 0.0,
            },
        }
    }
    pub fn width(&self) -> f32 {
        self.x.max - self.x.min
    }
    pub fn height(&self) -> f32 {
        self.y.max - self.y.min
    }

    pub fn set_width(&mut self, width: f32) {
        self.x.max = self.x.min + width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.y.max = self.y.min + height;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.set_width(width);
        self.set_height(height);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.set_size(width, height);
        self
    }

    pub(crate) fn right(&self) -> i32 {
        (self.x.min + self.width()) as i32
    }

    pub(crate) fn bottom(&self) -> i32 {
        (self.y.min + self.height()) as i32
    }

    pub(crate) fn left_bottom(&self) -> [f32; 2] {
        [self.x.min, self.y.max]
    }
    pub(crate) fn right_bottom(&self) -> [f32; 2] {
        [self.x.max, self.y.max]
    }
    pub(crate) fn right_top(&self) -> [f32; 2] {
        [self.x.max, self.y.min]
    }
    pub(crate) fn left_top(&self) -> [f32; 2] {
        [self.x.min, self.y.min]
    }

    #[deprecated]
    pub(crate) fn add_padding(&mut self, padding: &Padding) {
        self.x.min += padding.left;
        self.x.max -= padding.right;
        self.y.min += padding.top;
        self.y.max -= padding.bottom;
    }

    #[deprecated]
    pub(crate) fn clone_reduce_padding(&self, padding: &Padding) -> Rect {
        let mut res = self.clone();
        res.x.min += padding.left;
        res.x.max -= padding.right;
        res.y.min += padding.top;
        res.y.max -= padding.bottom;
        res
    }

    pub(crate) fn clone_add_padding(&self, padding: &Padding) -> Rect {
        let mut res = self.clone();
        res.x.min += padding.left;
        res.x.max -= padding.right;
        res.y.min += padding.top;
        res.y.max -= padding.bottom;
        res
    }


    pub fn offset_x(&mut self, x: f32) {
        self.x.min += x;
        self.x.max += x;
    }
    pub fn offset_y(&mut self, y: f32) {
        self.y.min += y;
        self.y.max += y;
    }

    pub fn offset(&mut self, x: f32, y: f32) {
        self.offset_x(x);
        self.offset_y(y);
    }

    pub fn has_position(&self, x: f32, y: f32) -> bool {
        x > self.x.min && x < self.x.max && y > self.y.min && y < self.y.max
    }

    pub(crate) fn set_min_pos(&mut self, x: f32, y: f32) {
        self.x.min = x;
        self.y.min = y;
    }

    pub(crate) fn clone_with_size(&mut self, other: &Rect) -> Rect {
        let mut res = self.clone();
        res.set_width(other.width());
        res.set_height(other.height());
        res
    }

    pub(crate) fn out_of_rect(&self, other: &Rect) -> bool {
        other.x.min > self.x.max || other.x.max < self.x.min ||
            other.y.min > self.y.max || other.y.max < self.y.min
    }
}