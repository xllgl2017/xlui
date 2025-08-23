use crate::response::Response;
use crate::size::rect::Rect;
use crate::ui::Ui;
use std::any::Any;
use crate::Offset;

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
pub(crate) mod item;
pub mod listview;
pub mod processbar;
pub mod triangle;
pub mod circle;

pub trait Widget: Any {
    fn redraw(&mut self, ui: &mut Ui); //绘制调用
    fn update(&mut self, ui: &mut Ui) -> Response; //后续更新调用
}

pub struct WidgetKind {
    pub(crate) widget: Box<dyn Widget>,
    pub(crate) id: String,
    pub(crate) rect: Rect,
}

impl WidgetKind {
    pub fn new(ui: &mut Ui, mut widget: impl Widget) -> Self {
        let resp = widget.update(ui);
        WidgetKind {
            id: resp.id.to_string(),
            rect: resp.rect.clone(),
            widget: Box::new(widget),
        }
    }

    pub fn offset(&mut self, o: &Offset, pr: &Rect) -> bool {
        self.rect.offset(o);
        !self.rect.out_of_rect(pr)
    }
    pub fn update(&mut self, ui: &mut Ui) {
        let resp = self.widget.update(ui);
        if resp.rect != &self.rect {
            self.rect = resp.rect.clone();
        }
    }

    pub fn redraw(&mut self, ui: &mut Ui) {
        self.widget.redraw(ui);
    }
}