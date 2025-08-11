use crate::size::padding::Padding;
use crate::size::pos::Axis;
use std::ops::Range;

#[derive(Clone, PartialEq, Debug)]
pub struct Rect {
    pub x: Axis,
    pub y: Axis,
}


impl Rect {
    pub fn new() -> Rect {
        Rect {
            x: (0.0..0.0).into(),
            y: (0.0..0.0).into(),
        }
    }
    pub fn width(&self) -> f32 {
        self.x.distance()
    }
    pub fn height(&self) -> f32 {
        self.y.distance()
    }

    pub fn set_width(&mut self, width: f32) {
        self.x.set_distance(width);
    }

    pub fn set_height(&mut self, height: f32) {
        self.y.set_distance(height);
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.set_width(width);
        self.set_height(height);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.set_size(width, height);
        self
    }

    pub fn size(&self) -> (f32, f32) {
        (self.width(), self.height())
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

    pub(crate) fn clone_add_padding(&self, padding: &Padding) -> Rect {
        let mut res = self.clone();
        res.x.min += padding.left;
        res.x.max -= padding.right;
        res.y.min += padding.top;
        res.y.max -= padding.bottom;
        res
    }


    pub fn offset_x(&mut self, x: f32) {
        self.x += x;
    }

    pub fn offset_x_limit(&mut self, ox: f32, lx: &Axis) -> f32 {
        if self.x.min + ox < lx.min {
            let ox = lx.min - self.x.min;
            self.x += ox;
            ox
        } else if self.x.max + ox > lx.max {
            let ox = lx.max - self.x.max;
            self.x += ox;
            ox
        } else {
            self.x += ox;
            ox
        }
    }

    pub fn offset_y(&mut self, oy: f32) {
        self.y += oy;
    }

    pub fn offset_y_limit(&mut self, oy: f32, ly: Range<f32>) -> f32 {
        if self.y.min + oy < ly.start {
            let oy = ly.start - self.y.min;
            self.y += oy;
            oy
        } else if self.y.max + oy > ly.end {
            let oy = ly.end - self.y.max;
            self.y += oy;
            oy
        } else {
            self.y += oy;
            oy
        }
    }

    pub fn offset(&mut self, x: f32, y: f32) {
        self.offset_x(x);
        self.offset_y(y);
    }

    pub fn offset_x_to(&mut self, tx: f32) {
        let ox = tx - self.x.min;
        self.offset_x(ox)
    }

    pub fn offset_y_to(&mut self, ty: f32) {
        let oy = ty - self.y.min;
        self.offset_y(oy)
    }

    pub fn offset_to(&mut self, tx: f32, ty: f32) {
        self.offset_x_to(tx);
        self.offset_y_to(ty);
    }


    pub fn has_position(&self, x: f32, y: f32) -> bool {
        x > self.x.min && x < self.x.max && y > self.y.min && y < self.y.max
    }

    pub(crate) fn clone_with_size(&self, other: &Rect) -> Rect {
        let mut res = self.clone();
        res.set_width(other.width());
        res.set_height(other.height());
        res
    }

    pub(crate) fn out_of_rect(&self, other: &Rect) -> bool {
        other.x.min > self.x.max || other.x.max < self.x.min ||
            other.y.min > self.y.max || other.y.max < self.y.min
    }

    //处于边界或出于边界
    pub(crate) fn out_of_border(&self, other: &Rect) -> bool {
        self.x.min < other.x.min || self.x.max > other.x.max ||
            self.y.min < other.y.min || self.y.max > other.y.max
    }
}