use crate::ui::{Ui, UiM};

pub mod label;
pub mod button;
pub mod image;
pub mod textedit;
pub mod scroll;
pub mod spinbox;
pub mod slider;
pub mod checkbox;
pub mod radio;
pub mod combobox;

pub trait Widget {
    fn draw(&mut self, ui: &mut Ui); //初次绘制调用
    fn update(&mut self, uim: &mut UiM); //后续更新调用
}

// pub(crate) enum WidgetKind {
//     Label(Label),
//     Button(Button),
//     Image(Image),
//     Custom(Box<dyn Widget>),
//
// }