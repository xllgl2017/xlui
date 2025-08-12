use crate::response::Response;
use crate::size::rect::Rect;
use crate::ui::Ui;
use std::any::Any;

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
pub mod rectangle;
pub mod item;
pub mod listview;

pub trait Widget: Any {
    fn redraw(&mut self, ui: &mut Ui) -> Response; //绘制调用
    fn update(&mut self, ui: &mut Ui); //后续更新调用
    // fn redraw(&mut self, ui: &mut Ui);
}

pub struct WidgetKind {
    widget: Box<dyn Widget>,
    rect: Rect,
}

impl WidgetKind {

}