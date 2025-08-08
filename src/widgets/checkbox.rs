use crate::frame::context::Context;
use crate::paint::checkbox::PaintCheckBox;
use crate::paint::PaintTask;
use crate::response::Callback;
use crate::size::rect::Rect;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;
use std::any::Any;
use crate::size::SizeMode;

pub struct CheckBox {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) text: TextBuffer,
    pub(crate) value: bool,
    pub(crate) callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, bool)>>,
    size_mode: SizeMode,
}

impl CheckBox {
    pub fn new(v: bool, label: impl ToString) -> CheckBox {
        CheckBox {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            text: TextBuffer::new(label.to_string()),
            value: v,
            callback: None,
            size_mode: SizeMode::Auto,
        }
    }


    pub fn reset_size(&mut self, context: &Context) {
        self.text.rect = self.rect.clone();
        self.text.reset_size(context);
        self.text.rect.offset_x(15.0);
        match self.size_mode {
            SizeMode::Auto => {
                self.rect.set_width(15.0 + self.text.rect.width());
                self.rect.set_height(20.0);
            }
            SizeMode::FixWidth => self.rect.set_height(20.0),
            SizeMode::FixHeight => self.rect.set_width(15.0 + self.text.rect.width()),
            SizeMode::Fix => {}
        }

        self.text.rect.set_height(self.rect.height());
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Context, bool)) -> Self {
        self.callback = Some(Callback::create_check(f));
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.rect.set_width(width);
        self.size_mode = SizeMode::FixWidth;
        self
    }
}

impl Widget for CheckBox {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.rect);
        let task = PaintCheckBox::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::CheckBox(task));
    }

    fn update(&mut self, ctx: &mut Context) {}
}