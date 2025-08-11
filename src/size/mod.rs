pub mod rect;
pub mod padding;
pub mod border;
pub mod pos;

pub enum SizeMode {
    Auto,
    FixWidth,
    FixHeight,
    Fix,
}

impl SizeMode {
    pub fn fix_width(&mut self) {
        match self {
            SizeMode::Auto => *self = SizeMode::FixWidth,
            SizeMode::FixHeight => *self = SizeMode::Fix,
            _ => {}
        }
    }

    pub fn fix_height(&mut self) {
        match self {
            SizeMode::Auto => *self = SizeMode::FixHeight,
            SizeMode::FixWidth => *self = SizeMode::Fix,
            _ => {}
        }
    }
}

#[derive(Clone, Debug)]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn as_gamma_size(&self) -> [f32; 2] {
        [self.width as f32, self.height as f32]
    }

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