//! ```
//! # use xlui::widgets::radio::RadioButton;
//! use xlui::widgets::Widget;
//! # xlui::_run_test(|ui|{
//! let mut btn=RadioButton::new(false,"radio");
//! btn.draw(ui);
//! #  });

use crate::frame::context::Context;
use crate::paint::radio::PaintRadioButton;
use crate::paint::PaintTask;
use crate::response::Callback;
use crate::size::rect::Rect;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;
use std::any::Any;
use crate::size::SizeMode;

pub struct RadioButton {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) value: bool,
    pub(crate) text: TextBuffer,
    pub(crate) callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, bool)>>,
    size_mode: SizeMode,
}

impl RadioButton {
    pub fn new(v: bool, label: impl ToString) -> RadioButton {
        RadioButton {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            value: v,
            text: TextBuffer::new(label.to_string()),
            callback: None,
            size_mode: SizeMode::Auto,
        }
    }
    fn reset_size(&mut self, context: &Context) {
        self.rect.set_height(16.0);
        self.text.rect = self.rect.clone();
        self.text.rect.offset_x(18.0);
        self.text.reset_size(context);
        match self.size_mode {
            SizeMode::Auto => self.rect.set_width(18.0 + self.text.rect.width()),
            SizeMode::FixWidth => {}
            SizeMode::FixHeight => self.rect.set_width(18.0 + self.text.rect.width()),
            SizeMode::Fix => {}
        }
    }

    pub fn with_width(mut self, width: f32) -> RadioButton {
        self.rect.set_width(width);
        self.size_mode = SizeMode::FixWidth;
        self
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Context, bool)) -> Self {
        self.callback = Some(Callback::create_check(f));
        self
    }
}


impl Widget for RadioButton {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.rect);
        let task = PaintRadioButton::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::Radio(task));
    }

    fn update(&mut self, ctx: &mut Context) {}
}