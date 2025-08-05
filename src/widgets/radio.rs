//! ```
//! # use xlui::widgets::radio::RadioButton;
//! use xlui::widgets::Widget;
//! # xlui::_run_test(|ui|{
//! let mut btn=RadioButton::new(false,"radio");
//! btn.draw(ui);
//! #  });

use std::any::Any;
use crate::frame::context::Context;
use crate::paint::PaintTask;
use crate::paint::radio::PaintRadioButton;
use crate::response::{Callback, DrawnEvent};
use crate::response::checkbox::CheckBoxResponse;
use crate::size::rect::Rect;
use crate::text::text_buffer::TextBuffer;
use crate::ui::{Ui, UiM};
use crate::widgets::Widget;

pub struct RadioButton {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) value: bool,
    pub(crate) text: TextBuffer,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM, bool)>>,

}

impl RadioButton {
    pub fn new(v: bool, label: impl ToString) -> RadioButton {
        RadioButton {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            value: v,
            text: TextBuffer::new(label.to_string()),
            callback: None,
        }
    }
    fn reset_size(&mut self, context: &Context) {
        self.rect.set_height(16.0);
        self.text.rect = self.rect.clone();
        self.text.rect.offset_x(16.0);
        self.text.reset_size(context);
        self.rect.set_width(16.0 + self.text.rect.width());
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut UiM, bool)) -> Self {
        self.callback = Some(Callback::create_check(f));
        self
    }
}


impl Widget for RadioButton {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone();
        self.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.rect);
        let task = PaintRadioButton::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::Radio(task));
        ui.response.insert(self.id.clone(), CheckBoxResponse {
            rect: self.rect.clone(),
            event: DrawnEvent::Click,
            callback: Callback::check(self.callback.take()),
            checked: self.value,
        })
    }

    fn update(&mut self, uim: &mut UiM) {}
}