use crate::frame::context::Context;
use crate::layout::Layout;
use crate::layout::popup::Popup;
use crate::paint::combobox::PaintComboBox;
use crate::paint::PaintTask;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::text::text_buffer::TextBuffer;
use crate::ui::{Ui, UiM};
use crate::widgets::Widget;

pub struct ComboBox {
    id: String,
    pub(crate) rect: Rect,
    pub(crate) popup: Popup,
    size_mode: SizeMode,
    pub(crate) text_buffer: TextBuffer,
    widgets: fn(&mut Ui),
}

impl ComboBox {
    pub fn new() -> ComboBox {
        ComboBox {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            popup: Popup::new(),
            size_mode: SizeMode::Auto,
            text_buffer: TextBuffer::new("".to_string()),
            widgets: |ui| {},
        }
    }

    fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(context);
        match self.size_mode {
            SizeMode::Auto => self.rect.set_size(100.0, 20.0),
            SizeMode::FixWidth => self.rect.set_height(20.0),
            SizeMode::FixHeight => self.rect.set_width(100.0),
            SizeMode::Fix => {}
        }
        self.text_buffer.rect = self.rect.clone_add_padding(&Padding::same(2.0));
        let mut popup_rect = self.rect.clone_with_size(&self.popup.rect());
        popup_rect.set_width(self.rect.width());
        popup_rect.offset_y(self.rect.height() + 5.0);
        self.popup.set_rect(popup_rect);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> ComboBox {
        self.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
        self
    }

    pub fn with_popup_height(mut self, height: f32) -> Self {
        self.popup.rect_mut().set_height(height);
        self
    }

    pub fn with_widgets(mut self, widgets: fn(&mut Ui)) -> Self {
        self.widgets = widgets;
        self
    }
}


impl Widget for ComboBox {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.rect);
        let popup_layout = self.popup.layout.take().unwrap();
        let previous_layout = ui.current_layout.replace(popup_layout).unwrap();
        (self.widgets)(ui);
        let popup_layout = ui.current_layout.replace(previous_layout).unwrap();
        self.popup.layout.replace(popup_layout);
        let task = PaintComboBox::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::ComboBox(task));
    }

    fn update(&mut self, uim: &mut UiM) {}
}