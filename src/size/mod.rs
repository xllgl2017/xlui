use crate::layout::LayoutDirection;
use crate::{Align, Padding, Rect};

pub mod rect;
pub mod padding;
pub mod border;
pub mod pos;
pub mod radius;
pub mod font;
pub mod margin;

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
        winit::dpi::PhysicalSize::new(self.width_u32(), self.height_u32())
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


///```text
///              (fix/min/max width)
/// (x,y)---------------------------(right)
/// |                                 |
/// |                                 |
/// |                                 |
/// |                                 |(fix/min/max height)
/// |                                 |
/// |                                 |
/// |------------------------------(bottom)
/// ```
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
    padding: Padding,
    align: Align,
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
            padding: Padding::same(0.0),
            align: Align::LeftTop,
        }
    }

    pub fn offset_to_rect(&mut self, rect: &Rect) {
        match rect.x_direction() {
            LayoutDirection::Min => self.x = rect.dx().min,
            LayoutDirection::Max => self.x = rect.dx().max - self.width()
        }
        match rect.y_direction() {
            LayoutDirection::Min => self.y = rect.dy().min,
            LayoutDirection::Max => self.y = rect.dy().max - self.height()
        }
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.set_width(width);
        self.set_height(height);
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.set_width(width);
        self
    }

    pub fn set_width(&mut self, width: f32) {
        self.width = width + self.padding.horizontal();
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.set_height(height);
        self
    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height + self.padding.vertical();
    }

    pub(crate) fn is_fix_width(&self) -> bool {
        self.fix_width.is_some()
    }

    pub(crate) fn width(&self) -> f32 {
        if let Some(width) = self.fix_width { return width; };
        if let Some(min_width) = self.min_width && self.width < min_width { return min_width; }
        if let Some(max_width) = self.max_width && self.width > max_width { return max_width; }
        self.width
    }

    pub(crate) fn height(&self) -> f32 {
        if let Some(height) = self.fix_height { return height; };
        if let Some(min_height) = self.min_height && self.height < min_height { return min_height; }
        if let Some(max_height) = self.max_height && self.height > max_height { return max_height; }
        self.height
    }

    pub(crate) fn rect(&self) -> Rect {
        let mut rect = Rect::new();
        rect.set_x_min(self.x());
        rect.set_y_min(self.y());
        rect.set_x_max(self.right());
        rect.set_y_max(self.bottom());
        rect
    }

    pub(crate) fn max_rect(&self) -> Rect {
        let mut rect = Rect::new();
        rect.set_x_min(self.x);
        rect.set_y_min(self.y);
        rect.set_width(self.width());
        rect.set_height(self.height());
        rect
    }

    /// Align
    ///```text
    /// |--------------------------------|
    /// |lt             ct            rt |
    /// |lc             cc            rc |
    /// |lb             cb            rb |
    /// |--------------------------------|
    /// ```
    pub(crate) fn x(&self) -> f32 {
        let mut x = self.x + self.padding.left;
        let fix_width = if let Some(fix_width) = self.fix_width {
            fix_width
        } else { return x };
        match self.align {
            Align::CenterTop | Align::Center | Align::CenterBottom => {
                let free_space = fix_width - self.width;
                x += free_space / 2.0;
            }
            Align::RightTop | Align::RightCenter | Align::RightBottom => {
                let free_space = fix_width - self.width;
                x += free_space;
            }
            _ => {}
        }
        x
    }

    pub(crate) fn x_i32(&self) -> i32 {
        self.x() as i32
    }

    pub(crate) fn y(&self) -> f32 {
        let mut y = self.y + self.padding.top;
        let fix_height = if let Some(fix_height) = self.fix_height {
            fix_height
        } else {
            return y;
        };
        match self.align {
            Align::LeftCenter | Align::Center | Align::RightCenter => {
                let free_space = fix_height - self.height;
                y += free_space / 2.0;
            }
            Align::LeftBottom | Align::CenterBottom | Align::RightBottom => {
                let free_space = fix_height - self.height;
                y += free_space;
            }
            _ => {}
        }
        y
    }

    // pub fn y_i32(&self) -> i32 {
    //     self.y() as i32
    // }

    pub(crate) fn right(&self) -> f32 {
        self.x + self.width() - self.padding.right
    }

    pub(crate) fn right_i32(&self) -> i32 {
        self.right() as i32
    }

    pub(crate) fn bottom(&self) -> f32 {
        self.y + self.height() - self.padding.bottom
    }

    pub(crate) fn bottom_i32(&self) -> i32 {
        self.bottom() as i32
    }

    pub fn set_fix_size(&mut self, w: f32, h: f32) {
        self.set_fix_width(w);
        self.set_fix_height(h);
    }

    pub fn with_fix_height(mut self, height: f32) -> Self {
        self.set_fix_height(height);
        self
    }

    pub fn set_fix_height(&mut self, h: f32) {
        self.fix_height = Some(h);
    }

    pub fn with_fix_width(mut self, width: f32) -> Self {
        self.set_fix_width(width);
        self
    }

    pub fn set_fix_width(&mut self, w: f32) {
        self.fix_width = Some(w);
    }

    pub fn set_max_width(&mut self, width: f32) {
        self.max_width = Some(width);
    }

    // pub fn set_max_height(&mut self, height: f32) {
    //     self.max_height = Some(height);
    // }

    pub fn set_min_width(&mut self, min_width: f32) {
        self.min_width = Some(min_width);
    }

    // pub fn set_min_height(&mut self, min_height: f32) {
    //     self.min_height = Some(min_height);
    // }

    pub fn add_fix_width(&mut self, w: f32) {
        if let Some(fix_width) = self.fix_width {
            self.fix_width = Some(fix_width + w);
        }
    }

    // pub fn add_fix_height(&mut self, h: f32) {
    //     if let Some(fix_height) = self.fix_height {
    //         self.fix_height = Some(fix_height + h);
    //     }
    // }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.set_padding(padding);
        self
    }

    pub fn set_padding(&mut self, padding: Padding) {
        self.padding = padding;
    }

    pub fn padding(&self) -> &Padding {
        &self.padding
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.set_width(w);
        self.set_height(h);
        self
    }

    pub fn with_align(mut self, align: Align) -> Self {
        self.set_align(align);
        self
    }

    pub fn set_align(&mut self, align: Align) {
        self.align = align;
    }

    ///Align
    pub fn an(&mut self, align: Align) -> &mut Self {
        self.set_align(align);
        self
    }

    ///padding
    pub fn pd(&mut self, padding: Padding) -> &mut Self {
        self.set_padding(padding);
        self
    }
}