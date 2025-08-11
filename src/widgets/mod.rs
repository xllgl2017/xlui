use std::any::Any;
use crate::response::Response;
use crate::ui::Ui;
use crate::widgets::button::Button;
use crate::widgets::label::Label;

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
pub mod select;

pub trait Widget: Any {
    fn draw(&mut self, ui: &mut Ui) -> Response; //初次绘制调用
    fn update(&mut self, ui: &mut Ui); //后续更新调用
    fn redraw(&mut self, ui: &mut Ui);
}

pub enum WidgetKind {
    Label(Label),
    Button(Button),
    Custom(Box<dyn Widget>),
}