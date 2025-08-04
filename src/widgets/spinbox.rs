use crate::frame::context::Context;
use crate::paint::spinbox::PaintSpinBox;
use crate::paint::PaintTask;
use crate::response::button::ButtonResponse;
use crate::response::DrawnEvent;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::ui::{Ui, UiM};
use crate::widgets::textedit::TextEdit;
use crate::widgets::Widget;

pub struct SpinBox {
    id: String,
    edit: TextEdit,
    rect: Rect,
    size_mode: SizeMode,
    value: i32,
}

impl SpinBox {
    pub fn new(value: i32) -> SpinBox {
        SpinBox {
            id: crate::gen_unique_id(),
            edit: TextEdit::new(value.to_string()),
            rect: Rect::new(),
            size_mode: SizeMode::Auto,
            value,

        }
    }
    pub fn reset_size(&mut self, context: &Context) {
        self.edit.reset_size(context);
        match self.size_mode {
            SizeMode::Auto => self.rect.set_size(100.0, 25.0),
            SizeMode::FixWidth => self.rect.set_height(25.0),
            SizeMode::FixHeight => self.rect.set_width(80.0),
            SizeMode::Fix => {}
        }
        self.edit.rect = self.rect.clone();
        self.edit.rect.x.max = self.edit.rect.x.max - 18.0;
        self.edit.text_buffer.rect = self.rect.clone_add_padding(&Padding::same(5.0));
    }
}


impl Widget for SpinBox {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.rect);
        let task = PaintSpinBox::new(ui, &self.rect, &self.edit);
        ui.add_paint_task(self.id.clone(), PaintTask::SpinBox(task));
        ui.response.insert(self.id.clone(), ButtonResponse::new(self.rect.clone()).event(DrawnEvent::Click));
    }

    fn update(&mut self, uim: &mut UiM) {
        todo!()
    }
}