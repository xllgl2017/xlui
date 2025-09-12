/// ### RichText的示例用法
/// ```
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///     //此处需要导入RichTextExt，对已实现Display的类型进行转换
///     let text="Rich"
///         //设置文本颜色
///         .color(Color::BLUE)
///         //设置换行类型
///         .wrap(TextWrap::NoWrap)
///         //设置字号
///         .size(16.0)
///         //设置字体
///         .family("微软雅黑");
///     ui.label(text);
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
    pub(crate) family: Option<String>,
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
            family: None,
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
    ///字体
    pub fn family(mut self, family: impl ToString) -> RichText {
        self.family = Some(family.to_string());
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

    pub(crate) fn font_family(&self) -> glyphon::Attrs<'_> {
        let family = self.family.as_ref().unwrap();
        let glyphon_family = glyphon::Family::Name(&family);
        glyphon::Attrs::new().family(glyphon_family)
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
    fn family(self, family: impl ToString) -> RichText;
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

    fn family(self, family: impl ToString) -> RichText {
        RichText::new(self.to_string()).family(family.to_string())
    }
}
