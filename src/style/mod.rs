pub mod color;
mod visual;

use std::fs;
use std::path::Path;
use crate::size::border::Border;
use crate::style::color::Color;
use crate::{Radius, UiError, UiResult};
pub use visual::*;
use crate::map::Map;

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


pub struct Style {
    widgets: Map<String, VisualStyle>,
    frame: FrameStyle,
}

impl Style {
    fn new() -> Style {
        Style {
            widgets: Map::new(),
            frame: FrameStyle {
                fill: Color::rgb(165, 235, 154),
                shadow: Shadow {
                    offset: [10.0, 10.0],
                    spread: 10.0,
                    blur: 5.0,
                    color: Color::rgb(123, 123, 123),
                },
                border: Border::same(1.0),
                radius: Radius::same(2),
            },
        }
    }

    pub fn default() -> Style {
        let mut res = Style::new();
        let mut style = VisualStyle::same((Color::rgb(230, 230, 230), 1.0, 3).into());
        style.inactive.border.set_same(0.0);
        style.pressed.fill = Color::rgb(165, 165, 165);
        res.widgets.insert(".button".to_string(), style);
        res
    }

    #[inline]
    pub fn set_widget_style(&self, visual: &mut Visual, keys: Vec<impl ToString>) {
        for key in keys {
            let key = key.to_string();
            if let Some(widget) = self.widgets.get(&key) {
                visual.enable().set_style(widget.clone());
                return;
            }
        }
    }

    fn read_color(value: &str) -> UiResult<Color> {
        let values = value.replace("rgb(", "").replace(")", "").replace("rgba(", "").replace(" ", "");
        let mut rgba = values.split(",");
        let r = rgba.next().ok_or("invalid red value")?.parse()?;
        let g = rgba.next().ok_or("invalid green value")?.parse()?;
        let b = rgba.next().ok_or("invalid blue value")?.parse()?;
        let a = rgba.next().map(|v| v.parse().unwrap_or(255)).unwrap_or(255);
        Ok(Color::rgba(r, g, b, a))
    }

    fn read_size(value: &str) -> UiResult<f32> {
        let value = value.replace(" ", "").replace("px", "");
        Ok(value.parse()?)
    }

    fn read_border(value: &str) -> UiResult<Border> {
        let value = value.replace(", ", ",");
        let items = value.split(" ").collect::<Vec<_>>();
        let mut border = Border::same(0.0);
        for item in items {
            if item.starts_with("rgb") {
                border.color = Style::read_color(item)?;
            } else if let Ok(width) = item.parse() {
                border.set_same(width);
            } else if item.ends_with("px") || item.parse::<f32>().is_ok() {
                border.set_same(Style::read_size(item)?);
            }
        }
        Ok(border)
    }

    pub fn from_css(fp: impl AsRef<Path>) -> UiResult<Style> {
        let lines = fs::read_to_string(fp.as_ref())?.replace("\r\n", "\n").replace("\n", "");
        let mut index = 0;
        let mut res = Style::new();
        while let Some(rpos) = lines[index..].find("}") {
            let lpos = lines[index..].find("{").ok_or(UiError::CssStyleError)?;
            let name = lines[index..index + lpos].replace(" ", "");
            let key = if name.contains(":") { name.split(":").next().ok_or(UiError::CssStyleError)? } else { &name };
            let visual_style = res.widgets.entry_or_insert_with(key.to_string(), || VisualStyle::new());
            let items = lines[index + lpos + 1..index + rpos].split(";");
            for item in items {
                if item == "" { continue; }
                let mut kvs = item.split(":");
                let key = kvs.next().ok_or(UiError::CssStyleError)?.replace(" ", "");
                let value = kvs.next().ok_or(UiError::CssStyleError)?;

                match key.as_str() {
                    "background-color" => if name.contains(":hover") {
                        visual_style.hovered.fill = Style::read_color(&value)?;
                    } else if name.contains(":active") {
                        visual_style.pressed.fill = Style::read_color(&value)?;
                    } else if name.contains(":disabled") {
                        visual_style.disabled.fill = Style::read_color(&value)?;
                    } else {
                        let color = Style::read_color(&value)?;
                        // visual_style.disabled.fill = color.clone();
                        visual_style.inactive.fill = color.clone();
                        // visual_style.hovered.fill = color.clone();
                        // visual_style.pressed.fill = color;
                    }
                    "border" => if name.contains(":hover") {
                        visual_style.hovered.border = Style::read_border(&value)?;
                    } else if name.contains(":active") {
                        visual_style.pressed.border = Style::read_border(&value)?;
                    } else if name.contains(":disabled") {
                        visual_style.disabled.border = Style::read_border(&value)?;
                    } else {
                        let border = Style::read_border(&value)?;
                        // visual_style.disabled.border = border.clone();
                        visual_style.inactive.border = border.clone();
                        // visual_style.hovered.border = border.clone();
                        // visual_style.pressed.border = border;
                    }
                    _ => {}
                }
            }
            // println!("{:#?}", visual_style);
            index += rpos + 1;
        }
        Ok(res)
    }
}


#[cfg(test)]
mod tests {
    use crate::style::Style;

    #[test]
    fn test_css_style() {
        let style = Style::from_css("res/css/widgets.css").unwrap();
    }
}