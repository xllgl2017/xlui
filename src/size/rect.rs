use crate::layout::LayoutDirection;
use crate::size::padding::Padding;
use crate::size::pos::{Axis, Pos};
use crate::Offset;

#[derive(Clone, Debug)]
pub struct Rect {
    dx: Axis,
    ox: Axis,
    dy: Axis,
    oy: Axis,
    x_direction: LayoutDirection,
    y_direction: LayoutDirection,
}


impl Rect {
    pub fn new() -> Rect {
        Rect {
            dx: (0.0..0.0).into(),
            ox: (0.0..0.0).into(),
            dy: (0.0..0.0).into(),
            oy: (0.0..0.0).into(),
            x_direction: LayoutDirection::Min,
            y_direction: LayoutDirection::Min,
        }
    }
    pub fn width(&self) -> f32 {
        self.dx.distance()
    }
    pub fn height(&self) -> f32 {
        self.dy.distance()
    }

    pub fn set_width(&mut self, width: f32) {
        self.dx.set_distance(width, &self.x_direction);
        self.ox.set_distance(width, &self.x_direction);
    }

    pub fn set_height(&mut self, height: f32) {
        self.dy.set_distance(height, &self.y_direction);
        self.oy.set_distance(height, &self.y_direction);
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.set_width(width);
        self.set_height(height);
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.set_height(height);
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.set_size(width, height);
        self
    }

    pub fn with_x_direction(mut self, direction: LayoutDirection) -> Self {
        self.x_direction = direction;
        self
    }

    pub fn with_y_direction(mut self, direction: LayoutDirection) -> Self {
        self.y_direction = direction;
        self
    }

    pub fn size(&self) -> (f32, f32) {
        (self.width(), self.height())
    }
    #[cfg(feature = "gpu")]
    pub(crate) fn left_bottom(&self) -> [f32; 2] {
        [self.dx.min, self.dy.max]
    }
    #[cfg(feature = "gpu")]
    pub(crate) fn right_bottom(&self) -> [f32; 2] {
        [self.dx.max, self.dy.max]
    }
    #[cfg(feature = "gpu")]
    pub(crate) fn right_top(&self) -> [f32; 2] {
        [self.dx.max, self.dy.min]
    }
    #[cfg(feature = "gpu")]
    pub(crate) fn left_top(&self) -> [f32; 2] {
        [self.dx.min, self.dy.min]
    }

    pub(crate) fn clone_add_padding(&self, padding: &Padding) -> Rect {
        let mut res = self.clone();
        res.dx.min += padding.left;
        res.dx.max -= padding.right;
        res.ox.min += padding.left;
        res.ox.max += padding.right;
        res.dy.min += padding.top;
        res.dy.max -= padding.bottom;
        res.oy.min += padding.top;
        res.oy.max -= padding.bottom;
        res
    }

    pub(crate) fn contract_y(&mut self, distance: f32) {
        self.oy.contract(distance);
        self.dy.contract(distance);
    }

    pub(crate) fn contract_x(&mut self, distance: f32) {
        self.ox.contract(distance);
        self.dx.contract(distance);
    }

    pub fn contract(&mut self, distance_x: f32, distance_y: f32) {
        self.contract_x(distance_x);
        self.contract_y(distance_y);
    }

    pub fn add_min_x(&mut self, value: f32) {
        self.ox.min += value;
        self.dx.min += value;
    }

    pub fn add_max_x(&mut self, value: f32) {
        self.ox.max += value;
        self.dx.max += value;
    }

    pub fn add_min_y(&mut self, value: f32) {
        self.oy.min += value;
        self.dy.min += value;
    }

    pub fn add_max_y(&mut self, value: f32) {
        self.oy.max += value;
        self.dy.max += value;
    }

    pub fn dx(&self) -> &Axis { &self.dx }

    pub fn dy(&self) -> &Axis { &self.dy }

    pub fn set_x_min(&mut self, x: f32) {
        self.dx.min = x;
        self.ox.min = x;
    }

    pub fn set_x_max(&mut self, x: f32) {
        self.dx.max = x;
        self.ox.max = x;
    }

    pub fn set_y_min(&mut self, y: f32) {
        self.dy.min = y;
        self.oy.min = y;
    }

    pub fn set_y_max(&mut self, y: f32) {
        self.dy.max = y;
        self.oy.max = y;
    }


    pub fn has_position(&self, pos: Pos) -> bool {
        pos.x > self.dx.min && pos.x < self.dx.max && pos.y > self.dy.min && pos.y < self.dy.max
    }

    pub(crate) fn clone_with_size(&self, other: &Rect) -> Rect {
        let mut res = self.clone();
        res.set_width(other.width());
        res.set_height(other.height());
        res
    }

    pub fn x_direction(&self) -> LayoutDirection {
        self.x_direction
    }

    pub fn y_direction(&self) -> LayoutDirection { self.y_direction }

    pub fn set_x_direction(&mut self, x_direction: LayoutDirection) {
        self.x_direction = x_direction;
    }

    pub fn set_y_direction(&mut self, y_direction: LayoutDirection) {
        self.y_direction = y_direction;
    }

    #[cfg(all(target_os = "linux", not(feature = "gpu")))]
    pub fn as_x_rect(&self) -> x11::xlib::XRectangle {
        x11::xlib::XRectangle {
            x: self.dx.min as i16,
            y: self.dy.min as i16,
            width: self.width() as u16,
            height: self.width() as u16,
        }
    }
}

//位移数值为总位移
impl Rect {
    pub fn offset_x(&mut self, o: &Offset) {
        if o.covered {
            self.dx += o.x;
            self.ox += o.x;
        } else {
            self.dx = self.ox + o.x;
        }
    }

    pub fn offset_x_limit(&mut self, ox: f32, lx: &Axis) -> f32 {
        if self.ox.min + ox < lx.min {
            let ox = lx.min - self.ox.min;
            self.dx = self.ox + ox;
            ox
        } else if self.ox.max + ox > lx.max {
            let ox = lx.max - self.ox.max;
            self.dx = self.ox + ox;
            ox
        } else {
            self.dx = self.ox + ox;
            ox
        }
    }

    pub fn offset_y(&mut self, o: &Offset) {
        if o.covered {
            self.oy += o.y;
            self.dy += o.y;
        } else {
            self.dy = self.oy + o.y;
        }
    }

    pub fn offset_y_limit(&mut self, oy: f32, ly: &Axis) -> f32 {
        if self.oy.min + oy < ly.min {
            let oy = ly.min - self.oy.min;
            self.dy = self.oy + oy;
            oy
        } else if self.oy.max + oy > ly.max {
            let oy = ly.max - self.oy.max;
            self.dy = self.oy + oy;
            oy
        } else {
            self.dy = self.oy + oy;
            oy
        }
    }

    pub fn offset(&mut self, o: &Offset) {
        self.offset_x(o);
        self.offset_y(o);
    }

    pub fn offset_x_to(&mut self, tx: f32) {
        let ox = tx - self.ox.min;
        self.dx = self.ox + ox;
    }

    pub fn offset_y_to(&mut self, ty: f32) {
        let oy = ty - self.oy.min;
        self.dy = self.oy + oy;
    }

    pub fn offset_to(&mut self, tx: f32, ty: f32) {
        self.offset_x_to(tx);
        self.offset_y_to(ty);
    }

    pub fn offset_to_rect(&mut self, rect: &Rect) -> Offset {
        let mut offset = Offset::new().covered();
        match rect.x_direction {
            LayoutDirection::Min => {
                let ox = rect.dx.min - self.ox.min;
                self.dx = self.ox + ox;
                self.ox += ox;
                offset.x = ox;
            }
            LayoutDirection::Max => {
                let ox = rect.dx.max - self.ox.max;
                self.dx = self.ox + ox;
                self.ox += ox;
                offset.x = ox;
            }
        }

        match rect.y_direction {
            LayoutDirection::Min => {
                let oy = rect.dy.min - self.oy.min;
                self.dy = self.oy + oy;
                self.oy += oy;
                offset.y = oy;
            }
            LayoutDirection::Max => {
                let oy = rect.dy.max - self.oy.max;
                self.dy = self.oy + oy;
                self.oy += oy;
                offset.y = oy;
            }
        }
        offset
    }

    pub fn get_ox(&self) -> f32 {
        self.dx.min - self.ox.min
    }
    pub fn get_oy(&self) -> f32 {
        self.dy.min - self.oy.min
    }

    #[cfg(all(windows, not(feature = "gpu")))]
    pub fn as_win32_rect(&self) -> windows::Win32::Foundation::RECT {
        windows::Win32::Foundation::RECT {
            left: self.dx.min as i32,
            top: self.dy.min as i32,
            right: self.dx.max as i32,
            bottom: self.dy.max as i32,
        }
    }
}

impl PartialEq for Rect {
    fn eq(&self, other: &Self) -> bool {
        self.dx.min == other.dx.min && self.dx.max == other.dx.max
            && self.dy.min == other.dy.min && self.dy.max == other.dy.max
    }
}