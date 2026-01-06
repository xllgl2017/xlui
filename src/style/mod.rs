pub mod color;

use crate::size::border::Border;
use crate::style::color::Color;
use crate::{Radius, Rect, Ui};
use crate::render::RenderParam;
use crate::shape::Shape;

/// #### 窗口样式
/// 可以用于窗口布局
/// ```rust
///  use xlui::*;
///
/// fn draw(ui:&mut Ui){
///     let style=FrameStyle{
///         fill:Color::rgb(165,235,154),
///         shadow:Shadow{
///             offset:[10.0,10.0],
///             spread:10.0,
///             blur:5.0,
///             color:Color::rgb(123,123,123)
///         },
///         border:Border::same(1.0),
///         radius:Radius::same(2)
///     };
/// }
/// ```
pub struct FrameStyle {
    pub fill: Color,
    pub shadow: Shadow,
    pub border: Border,
    pub radius: Radius,

}

///
/// #### 控件样式
/// 包含控件在未活跃、滑动、活跃的状态
/// ```
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///     let mut style=VisualStyle::new();
///     //未活跃的状态
///     style.inactive.fill=Color::GRAY;
///     style.inactive.border=Border::same(1.0).color(Color::RED);
///     style.inactive.radius=Radius::same(5);
///     //滑动状态
///     style.hovered.fill=Color::GRAY;
///     style.hovered.border=Border::same(1.0).color(Color::RED);
///     style.hovered.radius=Radius::same(5);
///     //活跃状态
///     style.pressed.fill=Color::GRAY;
///     style.pressed.border=Border::same(1.0).color(Color::RED);
///     style.pressed.radius=Radius::same(5);
/// }
/// ```
///
///


#[derive(Clone)]
pub struct Shadow {
    pub offset: [f32; 2],
    pub spread: f32,
    pub blur: f32,
    pub color: Color,
}

impl Shadow {
    pub fn new() -> Shadow {
        Shadow {
            offset: [0.0; 2],
            spread: 0.0,
            blur: 0.0,
            color: Color::TRANSPARENT,
        }
    }
}


#[derive(Clone)]
pub struct WidgetStyle {
    pub fill: Color,
    pub border: Border,
    pub radius: Radius,
    pub shadow: Shadow,
}

impl WidgetStyle {
    pub fn new() -> WidgetStyle {
        WidgetStyle {
            fill: Color::new(),
            border: Border::same(0.0),
            radius: Radius::same(0),
            shadow: Shadow::new(),
        }
    }
}

impl From<(Color, f32, u8)> for WidgetStyle {
    fn from(value: (Color, f32, u8)) -> Self {
        let mut res = WidgetStyle::new();
        res.fill = value.0;
        res.border = Border::same(value.1);
        res.radius = Radius::same(value.2);
        res
    }
}

#[derive(Clone)]
pub struct VisualStyle {
    pub disabled: WidgetStyle,
    pub inactive: WidgetStyle,
    pub hovered: WidgetStyle,
    pub pressed: WidgetStyle,
}

impl VisualStyle {
    pub fn new() -> VisualStyle {
        VisualStyle {
            disabled: WidgetStyle::new(),
            inactive: WidgetStyle::new(),
            hovered: WidgetStyle::new(),
            pressed: WidgetStyle::new(),
        }
    }

    pub fn same(style: WidgetStyle) -> VisualStyle {
        let mut res = VisualStyle::new();
        res.disabled = style.clone();
        res.inactive = style.clone();
        res.hovered = style.clone();
        res.pressed = style;
        res
    }

    pub fn dyn_style(&self, disabled: bool, hovered: bool, pressed: bool) -> &WidgetStyle {
        if disabled { return &self.disabled; }
        if pressed { return &self.pressed; }
        if hovered { return &self.hovered; }
        &self.inactive
    }
}


pub struct Visual {
    render: RenderParam,
    disable: bool,
    foreground: bool,
}

impl Visual {
    pub fn new() -> Visual {
        Visual {
            #[cfg(feature = "gpu")]
            render: RenderParam::new(Shape::rectangle()),
            #[cfg(not(feature = "gpu"))]
            render: RenderParam::new(Shape::Rectangle),
            disable: true,
            foreground: false,
        }
    }

    #[cfg(feature = "gpu")]
    pub fn re_init(&mut self) {
        self.render.re_init();
    }

    pub fn with_enable(mut self) -> Visual {
        self.disable = false;
        self
    }

    pub fn enable(&mut self) -> &mut Visual {
        self.disable = false;
        self
    }

    pub fn with_style(mut self, style: VisualStyle) -> Visual {
        self.render.set_style(style);
        self
    }

    pub fn set_style(&mut self, style: VisualStyle) {
        self.render.set_style(style);
    }


    pub fn draw(&mut self, ui: &mut Ui, disabled: bool, hovered: bool, pressed: bool, foreground: bool) {
        if self.disable || self.foreground != foreground { return; }
        self.render.draw(ui, disabled, hovered, pressed);
    }

    pub fn foreground(&self) -> bool {
        self.foreground
    }

    pub fn disable(&self) -> bool {
        self.disable
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Visual {
        self.render.rect.set_size(w, h);
        self
    }

    pub fn rect(&self) -> &Rect { self.render.rect() }

    pub fn rect_mut(&mut self) -> &mut Rect { self.render.rect_mut() }

    pub fn with_rect(mut self, rect: Rect) -> Visual {
        self.render.rect = rect;
        self
    }

    pub fn offset_to_rect(&mut self, rect: &Rect) {
        self.render.offset_to_rect(rect)
    }

    pub fn style(&self) -> &VisualStyle { &self.render.style }

    pub fn style_mut(&mut self) -> &mut VisualStyle { self.render.style_mut() }
}
