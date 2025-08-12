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
}

pub struct WidgetKind {
    pub(crate) widget: Box<dyn Widget>,
    pub(crate) id: String,
    pub(crate) rect: Rect,
}

impl WidgetKind {
    pub fn new(ui: &mut Ui, mut widget: impl Widget) -> Self {
        let resp = widget.redraw(ui);
        WidgetKind {
            id: resp.id.to_string(),
            rect: resp.rect.clone(),
            widget: Box::new(widget),
        }
    }
    pub fn update(&mut self, ui: &mut Ui) {
        self.widget.update(ui);
    }

    pub fn redraw(&mut self, ui: &mut Ui) {
        self.widget.redraw(ui);
    }
}