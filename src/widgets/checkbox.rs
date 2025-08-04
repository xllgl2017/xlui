use crate::frame::context::Context;
use crate::paint::checkbox::PaintCheckBox;
use crate::paint::PaintTask;
use crate::response::checkbox::CheckBoxResponse;
use crate::response::{Callback, DrawnEvent};
use crate::size::rect::Rect;
use crate::text::text_buffer::TextBuffer;
use crate::ui::{Ui, UiM};
use crate::widgets::Widget;

pub struct CheckBox {
    id: String,
    rect: Rect,
    text: TextBuffer,
}

impl CheckBox {
    pub fn new(txt: impl ToString) -> CheckBox {
        CheckBox {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            text: TextBuffer::new(txt.to_string()),
        }
    }

    pub fn reset_size(&mut self, context: &Context) {
        self.text.rect = self.rect.clone();
        self.text.reset_size(context);
        self.text.rect.offset_x(15.0);
        self.rect.set_width(15.0 + self.text.rect.width());
        self.rect.set_height(20.0);
        self.text.rect.set_height(20.0);
    }
}

impl Widget for CheckBox {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.rect);
        let task = PaintCheckBox::new(ui, &self.rect, &self.text);
        ui.add_paint_task(self.id.clone(), PaintTask::CheckBox(task));
        ui.response.insert(self.id.clone(),CheckBoxResponse{
            rect: self.rect.clone(),
            event: DrawnEvent::Click,
            callback: Callback::new(),
        });
    }

    fn update(&mut self, uim: &mut UiM) {
        todo!()
    }
}