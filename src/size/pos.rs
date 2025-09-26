use std::ops::{Add, AddAssign, Range, SubAssign};
use crate::layout::LayoutDirection;

#[derive(Clone, PartialEq, Debug, Copy)]
pub struct Pos {
    pub x: f32,
    pub y: f32,
}

impl Pos {
    pub fn new() -> Pos {
        Pos {
            x: 0.0,
            y: 0.0,
        }
    }
    pub fn offset_x(&mut self, ox: f32) {
        self.x += ox;
    }

    pub fn offset_y(&mut self, oy: f32) {
        self.y += oy;
    }

    pub fn offset(&mut self, ox: f32, oy: f32) {
        self.offset_x(ox);
        self.offset_y(oy);
    }

    pub fn clear(&mut self) {
        self.x = 0.0;
        self.y = 0.0;
    }
}

impl From<(f32, f32)> for Pos {
    fn from(p: (f32, f32)) -> Pos {
        Pos {
            x: p.0,
            y: p.1,
        }
    }
}

impl From<(i32, i32)> for Pos {
    fn from(p: (i32, i32)) -> Pos {
        Pos {
            x: p.0 as f32,
            y: p.1 as f32,
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Axis {
    pub min: f32,
    pub(crate) max: f32,
}

impl Axis {
    pub fn distance(&self) -> f32 {
        self.max - self.min
    }

    pub fn set_distance(&mut self, distance: f32, dir: &LayoutDirection) {
        match dir {
            LayoutDirection::Min => self.max = self.min + distance,
            LayoutDirection::Max => self.min = self.max - distance
        }
    }

    pub fn center(&self) -> f32 {
        (self.min + self.max) / 2.0
    }

    //min-distance max+distance
    pub fn extend(&mut self, distance: f32) {
        self.min -= distance;
        self.max += distance;
    }

    pub fn contract(&mut self, distance: f32) {
        self.min += distance;
        self.max -= distance;
    }
}

impl AddAssign<f32> for Axis {
    fn add_assign(&mut self, rhs: f32) {
        self.min += rhs;
        self.max += rhs;
    }
}

impl SubAssign<f32> for Axis {
    fn sub_assign(&mut self, rhs: f32) {
        self.min -= rhs;
        self.max -= rhs;
    }
}

impl From<Range<f32>> for Axis {
    fn from(range: Range<f32>) -> Self {
        Axis {
            min: range.start,
            max: range.end,
        }
    }
}

impl Add<f32> for Axis {
    type Output = Axis;

    fn add(mut self, rhs: f32) -> Self::Output {
        self.min += rhs;
        self.max += rhs;
        self
    }
}