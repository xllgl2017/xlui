pub mod color;

use crate::size::border::Border;
use crate::style::color::Color;
use crate::Radius;


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
///         border:Border::same(1.0).radius(Radius::same(2))
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
///     let mut style=ClickStyle::new();
///     //未活跃的状态
///     style.fill.inactive=Color::GRAY;
///     style.border.inactive=Border::same(1.0).radius(Radius::same(5)).color(Color::RED);
///     //滑动状态
///     style.fill.hovered=Color::GRAY;
///     style.border.hovered=Border::same(1.0).radius(Radius::same(5)).color(Color::RED);
///     //活跃状态
///     style.fill.clicked=Color::GRAY;
///     style.border.clicked=Border::same(1.0).radius(Radius::same(5)).color(Color::RED);
/// }
/// ```
///
///
#[derive(Clone)]
pub struct FillStyle {
    pub inactive: Color,
    pub hovered: Color,
    pub clicked: Color,
}

impl FillStyle {
    pub fn same(c: Color) -> Self {
        FillStyle {
            inactive: c.clone(),
            hovered: c.clone(),
            clicked: c,
        }
    }
}

#[derive(Clone)]
pub struct BorderStyle {
    pub inactive: Border,
    pub hovered: Border,
    pub clicked: Border,
}

impl BorderStyle {
    pub fn same(c: Border) -> Self {
        BorderStyle {
            inactive: c.clone(),
            hovered: c.clone(),
            clicked: c,
        }
    }
}

#[derive(Clone)]
pub struct ClickStyle {
    pub fill: FillStyle,
    pub border: BorderStyle,
}

impl ClickStyle {
    pub fn new() -> ClickStyle {
        ClickStyle {
            fill: FillStyle {
                inactive: Color::rgb(230, 230, 230),
                hovered: Color::rgb(230, 230, 230),
                clicked: Color::rgb(165, 165, 165),
            },

            border: BorderStyle {
                inactive: Border::same(0.0),
                hovered: Border::same(1.0).color(Color::rgb(0, 0, 0)),
                clicked: Border::same(1.0).color(Color::rgb(0, 0, 0)),
            },
        }
    }
    pub fn dyn_fill(&self, clicked: bool, hovered: bool) -> &Color {
        if clicked && hovered { return &self.fill.clicked; }
        if hovered { return &self.fill.hovered; }
        &self.fill.inactive
    }

    pub fn dyn_border(&self, clicked: bool, hovered: bool) -> &Border {
        if clicked && hovered { return &self.border.clicked; }
        if hovered { return &self.border.hovered; }
        &self.border.inactive
    }
}

pub struct WidgetStyle {
    pub click: ClickStyle,
    pub popup: ClickStyle,
}

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

// pub struct Style {
//     pub window: FrameStyle,
//     pub widgets: WidgetStyle,
// }


// impl Style {
//     pub fn light_style() -> Style {
//         Style {
//             window: FrameStyle {
//                 fill: Color::rgb(240, 240, 240),
//                 shadow: Shadow::new(),
//                 border: Border::same(0.0).radius(Radius::same(0)),
//             },
//
//             widgets: WidgetStyle {
//                 click: ClickStyle::new(),
//                 popup: ClickStyle {
//                     fill: FillStyle::same(Color::rgb(240, 240, 240)),
//                     border: BorderStyle::same(Border::same(1.0).radius(Radius::same(5))
//                         .color(Color::rgb(144, 209, 255))),
//                 },
//             },
//         }
//     }
// }