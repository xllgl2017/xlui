use crate::layout::LayoutDirection;
use crate::{Align, Margin, Padding, Rect};

pub mod rect;
pub mod padding;
pub mod border;
pub mod pos;
pub mod radius;
pub mod font;
pub mod margin;

#[derive(Clone, Debug, Copy, PartialEq)]
#[cfg_attr(feature = "gpu", repr(C))]
#[cfg_attr(feature = "gpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
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
#[cfg(feature = "gpu")]
impl From<&wgpu::SurfaceConfiguration> for Size {
    fn from(value: &wgpu::SurfaceConfiguration) -> Self {
        Size {
            width: value.width as f32,
            height: value.height as f32,
        }
    }
}


/// #### 布局的几何信息，包含x、y、大小、填充，边缘
/// ```text
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
    ///所在区域的最小值，内容起始需+padding+margin
    x: f32,
    y: f32,
    ///这里的不含padding和margin
    width: f32,
    height: f32,
    min_width: Option<f32>,
    min_height: Option<f32>,
    max_width: Option<f32>,
    max_height: Option<f32>,
    fix_width: Option<f32>,
    fix_height: Option<f32>,
    padding: Padding,
    margin: Margin,
    /// #### 对齐
    ///```text
    /// |--------------------------------|
    /// |lt             ct            rt |
    /// |lc             cc            rc |
    /// |lb             cb            rb |
    /// |--------------------------------|
    /// ```
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
            padding: Padding::ZERO,
            margin: Margin::ZERO,
            align: Align::LeftTop,
        }
    }

    pub fn offset_to_rect(&mut self, rect: &Rect) {
        match rect.x_direction() {
            LayoutDirection::Min => self.x = rect.dx().min,
            LayoutDirection::Max => self.x = rect.dx().max - self.margin_width()
        }
        match rect.y_direction() {
            LayoutDirection::Min => self.y = rect.dy().min,
            LayoutDirection::Max => self.y = rect.dy().max - self.margin_height()
        }
    }

    pub fn set_margin_size(&mut self, w: f32, h: f32) {
        self.set_margin_width(w);
        self.set_margin_height(h);
    }

    pub fn set_margin_width(&mut self, w: f32) {
        self.set_context_width(w - self.padding.horizontal() - self.margin.horizontal())
    }

    pub fn set_margin_height(&mut self, h: f32) {
        self.set_context_height(h - self.padding.vertical() - self.margin.vertical())
    }

    pub fn with_context_size(mut self, w: f32, h: f32) -> Self {
        self.set_context_size(w, h);
        self
    }

    pub fn set_context_size(&mut self, width: f32, height: f32) {
        self.set_context_width(width);
        self.set_context_height(height);
    }

    pub fn with_context_width(mut self, width: f32) -> Self {
        self.set_context_width(width);
        self
    }

    pub fn set_context_width(&mut self, width: f32) {
        self.width = width;
    }

    pub fn with_context_height(mut self, height: f32) -> Self {
        self.set_context_height(height);
        self
    }

    pub fn set_context_height(&mut self, height: f32) {
        self.height = height;
    }

    pub(crate) fn is_fix_width(&self) -> bool {
        self.fix_width.is_some()
    }

    ///返回item的宽度，不含padding和margin
    pub(crate) fn context_width(&self) -> f32 {
        if let Some(width) = self.fix_width { return width - self.padding.horizontal() - self.margin.horizontal(); };
        if let Some(min_width) = self.min_width && self.width < min_width { return min_width; }
        if let Some(max_width) = self.max_width && self.width > max_width + self.padding.horizontal() + self.margin.horizontal() {
            return max_width - self.padding.horizontal() - self.margin.horizontal();
        }
        self.width
    }

    ///返回item的宽度，含padding和margin
    pub(crate) fn margin_width(&self) -> f32 {
        self.context_width() + self.padding.horizontal() + self.margin.horizontal()
    }

    ///返回item的宽度，含padding,不含margin
    pub(crate) fn padding_width(&self) -> f32 {
        self.context_width() + self.padding.horizontal()
    }

    ///返回item的高度，不含padding和margin
    pub(crate) fn context_height(&self) -> f32 {
        if let Some(height) = self.fix_height { return height - self.padding.vertical() - self.margin.vertical(); };
        if let Some(min_height) = self.min_height && self.height < min_height { return min_height; }
        if let Some(max_height) = self.max_height && self.height > max_height - self.padding.vertical() - self.margin.vertical() {
            return max_height - self.padding.vertical() - self.margin.vertical();
        }
        self.height
    }

    ///返回item的高度，含padding和margin
    pub(crate) fn margin_height(&self) -> f32 {
        self.context_height() + self.padding.vertical() + self.margin.vertical()
    }

    ///返回item的高度，含padding,不含margin
    pub(crate) fn padding_height(&self) -> f32 {
        self.context_height() + self.padding.vertical()
    }

    ///返回内容的rect
    pub(crate) fn context_rect(&self) -> Rect {
        let mut rect = Rect::new();
        rect.set_x_min(self.context_left());
        rect.set_x_max(self.context_right());
        rect.set_y_min(self.context_top());
        rect.set_y_max(self.context_bottom());
        rect
    }

    ///返回含padding的rect
    pub(crate) fn padding_rect(&self) -> Rect {
        let mut rect = Rect::new();
        rect.set_x_min(self.padding_left());
        rect.set_x_max(self.padding_right());
        rect.set_y_min(self.padding_top());
        rect.set_y_max(self.padding_bottom());
        rect
    }

    pub(crate) fn context_left(&self) -> f32 {
        let mut x = self.x + self.padding.left + self.margin.left;
        let fix_width = match self.fix_width {
            None => return x,
            Some(w) => w - self.padding.horizontal() - self.margin.horizontal(),
        };
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

    pub(crate) fn margin_left(&self) -> f32 {
        self.x
    }

    pub(crate) fn padding_left(&self) -> f32 {
        self.x + self.margin.left
    }


    #[cfg(feature = "gpu")]
    pub(crate) fn x_i32(&self) -> i32 {
        self.context_left() as i32
    }

    pub(crate) fn context_top(&self) -> f32 {
        let mut y = self.y + self.padding.top + self.margin.top;
        let fix_height = match self.fix_height {
            None => return y,
            Some(h) => h - self.padding.vertical() - self.margin.vertical(),
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

    pub(crate) fn padding_top(&self) -> f32 {
        self.y + self.margin.top
    }

    pub(crate) fn margin_top(&self) -> f32 {
        self.y
    }

    pub(crate) fn context_right(&self) -> f32 {
        self.padding_right() - self.padding.right
    }

    fn padding_right(&self) -> f32 {
        self.margin_right() - self.margin.right
    }

    fn margin_right(&self) -> f32 {
        self.x + self.margin_width()
    }

    pub(crate) fn context_bottom(&self) -> f32 {
        self.padding_bottom() - self.padding.bottom
    }

    fn padding_bottom(&self) -> f32 {
        self.margin_bottom() - self.margin.bottom
    }

    fn margin_bottom(&self) -> f32 {
        self.y + self.margin_height()
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

    pub fn set_min_width(&mut self, min_width: f32) {
        self.min_width = Some(min_width);
    }

    pub fn add_fix_width(&mut self, w: f32) {
        if let Some(fix_width) = self.fix_width {
            self.fix_width = Some(fix_width + w);
        }
    }

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

    pub fn margin(&self) -> &Margin {
        &self.margin
    }

    pub fn with_align(mut self, align: Align) -> Self {
        self.set_align(align);
        self
    }

    pub fn set_align(&mut self, align: Align) {
        self.align = align;
    }

    pub fn set_margin(&mut self, margin: Margin) {
        self.margin = margin;
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

    pub fn mn(&mut self, margin: Margin) -> &mut Self {
        self.set_margin(margin);
        self
    }
}