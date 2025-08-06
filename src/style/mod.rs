use crate::paint::color::Color;
use crate::size::border::Border;
use crate::size::rect::Rect;

pub struct Frame {
    pub rect: Rect,
    pub fill: Color,
    pub shadow: Color,

}
#[derive(Clone)]
pub struct FillStyle {
    pub inactive: Color,
    pub hovered: Color,
    pub clicked: Color,
}

#[derive(Clone)]
pub struct BorderStyle {
    pub inactive: Border,
    pub hovered: Border,
    pub clicked: Border,
}

#[derive(Clone)]
pub struct ClickStyle {
    pub fill: FillStyle,
    pub border: BorderStyle,
}

impl ClickStyle {
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
}

pub struct Shadow {
    pub offset: [f32; 2],
    pub spread: f32,
    pub color: Color,
}

impl Shadow {
    pub fn new() -> Shadow {
        Shadow {
            offset: [0.0; 2],
            spread: 0.0,
            color: Color::TRANSPARENT,
        }
    }
}


pub struct Style {
    pub window: Frame,
    pub widget: WidgetStyle,
}


impl Style {
    pub fn light_style() -> Style {
        Style {
            window: Frame {
                rect: Rect::new(),
                fill: Color::rgb(240, 240, 240),
                shadow: Color::rgb(0, 0, 0),
            },

            widget: WidgetStyle {
                click: ClickStyle {
                    fill: FillStyle {
                        inactive: Color::rgb(230, 230, 230),
                        hovered: Color::rgb(230, 230, 230),
                        clicked: Color::rgb(165, 165, 165),
                    },

                    border: BorderStyle {
                        inactive: Border::new(0.0),
                        hovered: Border::new(1.0).color(Color::rgb(0, 0, 0)),
                        clicked: Border::new(1.0).color(Color::rgb(0, 0, 0)),
                    },
                },
            },
        }
    }
}