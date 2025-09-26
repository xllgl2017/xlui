use crate::Rect;

pub mod rect;
pub mod padding;
pub mod border;
pub mod pos;
pub mod radius;
pub mod font;
pub mod margin;

#[derive(Clone, Copy)]
#[deprecated]
pub enum SizeMode {
    Auto,
    FixWidth(f32),
    FixHeight(f32),
    Fix(f32, f32),
}

impl SizeMode {
    pub fn fix_width(&mut self, w: f32) {
        match self {
            SizeMode::Auto => *self = SizeMode::FixWidth(w),
            SizeMode::FixHeight(h) => *self = SizeMode::Fix(w, *h),
            SizeMode::FixWidth(fw) => *fw = w,
            SizeMode::Fix(fw, _) => *fw = w,
        }
    }

    pub fn fix_height(&mut self, h: f32) {
        match self {
            SizeMode::Auto => *self = SizeMode::FixHeight(h),
            SizeMode::FixWidth(w) => *self = SizeMode::Fix(*w, h),
            SizeMode::FixHeight(fh) => *fh = h,
            SizeMode::Fix(_, fh) => *fh = h,
        }
    }


    pub fn is_fixed_width(&self) -> bool {
        match self {
            SizeMode::FixWidth(_) | SizeMode::Fix(_, _) => true,
            _ => false,
        }
    }

    pub fn is_auto_width(&self) -> bool {
        match self {
            SizeMode::Auto => true,
            SizeMode::FixHeight(_) => true,
            _ => false,
        }
    }

    pub fn size(&self, w: f32, h: f32) -> (f32, f32) {
        match self {
            SizeMode::Auto => (w, h),
            SizeMode::FixWidth(w) => (*w, h),
            SizeMode::FixHeight(h) => (w, *h),
            SizeMode::Fix(w, h) => (*w, *h)
        }
    }

    pub fn width(&self, w: f32) -> f32 {
        match self {
            SizeMode::Auto => w,
            SizeMode::FixWidth(w) => *w,
            SizeMode::FixHeight(_) => w,
            SizeMode::Fix(w, _) => *w,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn as_gamma_size(&self) -> [f32; 2] {
        [self.width, self.height]
    }

    #[cfg(feature = "winit")]
    pub fn as_physical_size(&self) -> winit::dpi::PhysicalSize<u32> {
        winit::dpi::PhysicalSize::new(self.width, self.height)
    }

    pub fn width_u32(&self) -> u32 {
        self.width as u32
    }

    pub fn height_u32(&self) -> u32 {
        self.height as u32
    }
}

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Self {
        Size {
            width: value.0 as f32,
            height: value.1 as f32,
        }
    }
}

impl From<&wgpu::SurfaceConfiguration> for Size {
    fn from(value: &wgpu::SurfaceConfiguration) -> Self {
        Size {
            width: value.width as f32,
            height: value.height as f32,
        }
    }
}

pub struct Geometry {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    min_width: Option<f32>,
    min_height: Option<f32>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    fix_width: Option<f32>,
    fix_height: Option<f32>,
}

impl Geometry {
    pub fn new() -> Geometry {
        Geometry {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
            min_width: None,
            min_height: None,
            max_width: None,
            max_height: None,
            fix_width: None,
            fix_height: None,
        }
    }

    pub fn set_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.set_width(width);
        self.set_height(height);
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
    }

    pub fn is_fix_width(&self) -> bool {
        self.fix_width.is_some()
    }

    pub fn width(&self) -> f32 {
        if let Some(width) = self.fix_width { return width; };
        if let Some(min_width) = self.min_width && self.width < min_width { return min_width; }
        if let Some(max_height) = self.max_width && self.width > max_height { return max_height; }
        self.width
    }

    pub fn height(&self) -> f32 {
        if let Some(height) = self.max_height { return height; };
        if let Some(min_height) = self.min_height && self.height < min_height { return min_height; }
        if let Some(max_height) = self.max_height && self.height > max_height { return max_height; }
        self.height
    }

    pub fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        rect.set_x_min(self.x);
        rect.set_y_min(self.y);
        rect.set_width(self.width());
        rect.set_height(self.height());
        rect
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn x_i32(&self) -> i32 {
        self.x as i32
    }

    pub fn y(&self) -> f32 {
        self.y
    }

    pub fn y_i32(&self) -> i32 {
        self.y as i32
    }

    pub fn right(&self) -> f32 {
        self.x + self.width
    }

    pub fn right_i32(&self) -> i32 {
        self.x as i32 + self.width as i32
    }

    pub fn bottom(&self) -> f32 {
        self.y + self.height
    }

    pub fn bottom_i32(&self) -> i32 {
        self.y as i32 + self.height as i32
    }

    pub fn set_fix_size(&mut self, w: f32, h: f32) {
        self.set_fix_width(w);
        self.set_fix_height(h);
    }

    pub fn set_fix_height(&mut self, h: f32) {
        self.fix_height = Some(h);
    }

    pub fn set_fix_width(&mut self, w: f32) {
        self.fix_width = Some(w);
    }

    pub fn add_fix_width(&mut self, w: f32) {
        if let Some(fix_width) = self.fix_width {
            self.fix_width = Some(fix_width + w);
        }
    }

    pub fn add_fix_height(&mut self, h: f32) {
        if let Some(fix_height) = self.fix_height {
            self.fix_height = Some(fix_height + h);
        }
    }
}