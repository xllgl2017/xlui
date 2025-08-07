use crate::frame::context::Context;
use crate::ui::Ui;

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
    fn update(&mut self, ctx: &mut Context); //后续更新调用
}
