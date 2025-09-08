/// ### RichText的示例用法
/// ```
/// use xlui::{RichText, RichTextExt, TextWrap};
/// use xlui::style::color::Color;
/// use xlui::ui::Ui;
///
/// fn draw(ui:&mut Ui){
///     //此处需要导入RichTextExt，对已实现Display的类型进行转换
///     ui.label("Rich".color(Color::BLUE).size(16.0).wrap(TextWrap::NoWrap));
///     ui.label(RichText::new("Rich").color(Color::YELLOW));
/// }
/// ```


use crate::style::color::Color;
use crate::text::TextWrap;
use std::fmt::Display;

pub struct RichText {
    pub(crate) text: String,
    pub(crate) size: Option<f32>,
    pub(crate) color: Color,
    ///字符高度
    pub(crate) height: f32,
    ///Text的总宽度
    pub(crate) width: f32,
    pub(crate) wrap: TextWrap,
}

impl RichText {
    pub fn new(text: impl ToString) -> RichText {
        RichText {
            text: text.to_string(),
            size: None,
            color: Color::BLACK,
            height: 0.0,
            width: 0.0,
            wrap: TextWrap::NoWrap,
        }
    }

    ///设置换行方式，默认为TextWrap::NoWrap
    pub fn wrap(mut self, wrap: TextWrap) -> RichText {
        self.wrap = wrap;
        self
    }
    ///字体大小，如果没有提供就会使用全局字体大小,WindowAttribute::font
    pub fn size(mut self, size: f32) -> RichText {
        self.size = Some(size);
        self
    }
    ///设置字体颜色
    pub fn color(mut self, color: Color) -> RichText {
        self.color = color;
        self
    }

    pub(crate) fn font_size(&self) -> f32 {
        self.size.unwrap()
    }
}

impl<T: Display> From<T> for RichText {
    fn from(value: T) -> Self {
        RichText::new(value)
    }
}

pub trait RichTextExt {
    fn color(self, color: Color) -> RichText;
    fn size(self, size: f32) -> RichText;
    fn wrap(self, wrap: TextWrap) -> RichText;
}


impl<T: Display> RichTextExt for T {
    fn color(self, color: Color) -> RichText {
        RichText::new(self.to_string()).color(color)
    }

    fn size(self, size: f32) -> RichText {
        RichText::new(self.to_string()).size(size)
    }

    fn wrap(self, wrap: TextWrap) -> RichText {
        RichText::new(self.to_string()).wrap(wrap)
    }
}
