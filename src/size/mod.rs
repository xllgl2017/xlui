pub mod rect;
pub mod padding;
pub mod border;
pub mod pos;
pub mod radius;
pub mod font;

#[derive(Clone, Copy)]
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
            _ => {}
        }
    }

    pub fn fix_height(&mut self, h: f32) {
        match self {
            SizeMode::Auto => *self = SizeMode::FixHeight(h),
            SizeMode::FixWidth(w) => *self = SizeMode::Fix(*w, h),
            _ => {}
        }
    }


    pub fn is_fixed_width(&self) -> bool {
        match self {
            SizeMode::FixWidth(_) | SizeMode::Fix(_,_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn as_gamma_size(&self) -> [f32; 2] {
        [self.width as f32, self.height as f32]
    }

    #[cfg(feature = "winit")]
    pub fn as_physical_size(&self) -> winit::dpi::PhysicalSize<u32> {
        winit::dpi::PhysicalSize::new(self.width, self.height)
    }
}

impl From<(u32, u32)> for Size {
    fn from(value: (u32, u32)) -> Self {
        Size {
            width: value.0,
            height: value.1,
        }
    }
}