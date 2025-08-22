use crate::response::Response;
use crate::ui::Ui;
use crate::widgets::textedit::multi::MultiEdit;
use crate::widgets::textedit::single::SingleEdit;
use crate::widgets::Widget;

pub mod single;
pub mod multi;
pub(crate) mod buffer;
mod select;
mod cursor;

#[derive(PartialEq)]
enum EditKind {
    Single,
    Multi,
}

pub enum TextEdit {
    Single(SingleEdit),
    Multi(MultiEdit),
}

impl TextEdit {
    pub fn single(text: impl ToString) -> TextEdit {
        TextEdit::Single(SingleEdit::new(text.to_string()))
    }

    pub fn multi(text: impl ToString) -> TextEdit {
        TextEdit::Multi(MultiEdit::new(text.to_string()))
    }
}

impl Widget for TextEdit {
    fn redraw(&mut self, ui: &mut Ui) {
        match self {
            TextEdit::Single(v) => v.redraw(ui),
            TextEdit::Multi(v) => v.redraw(ui)
        }
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match self {
            TextEdit::Single(v) => v.update(ui),
            TextEdit::Multi(v) => v.update(ui)
        }
    }
}