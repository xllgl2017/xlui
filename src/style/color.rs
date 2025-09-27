/// ### 颜色
/// ```rust
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///     let color=Color::WHITE;//白色
///     let color=Color::rgb(255,0,0);//红色
///     let color=Color::rgba(255,255,255,100);//支持透明
/// }
/// ```
///

#[derive(Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const BLACK: Color = Color { r: 0, g: 0, b: 0, a: 255 };
    pub const WHITE: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const RED: Color = Color { r: 255, g: 0, b: 0, a: 255 };
    pub const GREEN: Color = Color { r: 0, g: 255, b: 0, a: 255 };
    pub const GRAY: Color = Color { r: 255, g: 255, b: 255, a: 255 };
    pub const ORANGE: Color = Color { r: 255, g: 165, b: 0, a: 255 };
    pub const BLUE: Color = Color { r: 0, g: 0, b: 255, a: 255 };
    pub const YELLOW: Color = Color { r: 255, g: 255, b: 0, a: 255 };
    pub const TRANSPARENT: Color = Color { r: 0, g: 0, b: 0, a: 0 };

    pub fn new() -> Color {
        Color { r: 0, g: 0, b: 0, a: 255 }
    }

    pub fn as_gamma_rgb(&self) -> [f32; 3] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0]
    }

    pub fn as_gamma_rgba(&self) -> [f32; 4] {
        [self.r as f32 / 255.0, self.g as f32 / 255.0, self.b as f32 / 255.0, self.a as f32 / 255.0]
    }

    pub fn as_wgpu_color(&self) -> wgpu::Color {
        wgpu::Color {
            r: self.r as f64 / 255.0,
            g: self.g as f64 / 255.0,
            b: self.b as f64 / 255.0,
            a: self.a as f64 / 255.0,
        }
    }

    pub fn as_glyphon_color(&self) -> glyphon::Color {
        glyphon::Color::rgb(self.r, self.g, self.b)
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 255 }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}