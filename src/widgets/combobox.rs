//!### ComboBox的示例用法
//!
//! ```
//! use xlui::widgets::combobox::ComboBox;
//! use xlui::widgets::Widget;
//! # xlui::_run_test(|ui|{
//!    ComboBox::new().with_popup_height(150.0).with_widgets(|ui|{
//!        ui.label("combo");
//!    }).draw(ui);
//! # });
//! ```

use crate::frame::context::Context;
use crate::layout::popup::Popup;
use crate::paint::color::Color;
use crate::paint::combobox::PaintComboBox;
use crate::paint::PaintTask;
use crate::radius::Radius;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::button::Button;
use crate::widgets::Widget;
use std::fmt::Display;

pub struct ComboBox<T> {
    id: String,
    pub(crate) rect: Rect,
    pub(crate) popup: Popup,
    size_mode: SizeMode,
    pub(crate) text_buffer: TextBuffer,
    data: Vec<T>,
    item_style: ClickStyle,
}

impl<T> ComboBox<T> {
    pub fn new(data: Vec<T>) -> Self {
        ComboBox {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            popup: Popup::new(),
            size_mode: SizeMode::Auto,
            text_buffer: TextBuffer::new("".to_string()),
            data,
            item_style: ClickStyle {
                fill: FillStyle {
                    inactive: Color::TRANSPARENT,
                    hovered: Color::rgba(153, 193, 241, 220),
                    clicked: Color::rgba(153, 193, 241, 220),
                },
                border: BorderStyle {
                    inactive: Border::new(0.0),
                    hovered: Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2)),
                    clicked: Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2)),
                },
            },
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

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
        self
    }

    /// 设置popup的高度
    pub fn with_popup_height(mut self, height: f32) -> Self {
        self.popup.rect_mut().set_height(height);
        self
    }
}

impl<T: Display + PartialEq> ComboBox<T> {
    fn add_item(&self, ui: &mut Ui, item: &T, row: usize) {
        let style = ui.style.widget.click.clone();
        ui.style.widget.click = self.item_style.clone();
        let mut btn = Button::new(item.to_string()).padding(Padding::same(3.0));
        btn.set_size(self.popup.rect().width() - 10.0, 25.0);
        btn.draw(ui);
        ui.style.widget.click = style;
    }

    fn add_items(&self, ui: &mut Ui) {
        for (row, datum) in self.data.iter().enumerate() {
            self.add_item(ui, datum, row);
        }
    }
}


impl<T: Display + PartialEq> Widget for ComboBox<T> {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.rect);
        let mut popup_layout = self.popup.layout.take().unwrap();
        popup_layout.item_space = 2.0;
        let previous_layout = ui.current_layout.replace(popup_layout).unwrap();
        self.add_items(ui);
        let popup_layout = ui.current_layout.replace(previous_layout).unwrap();
        self.popup.layout.replace(popup_layout);
        let task = PaintComboBox::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::ComboBox(task));
    }

    fn update(&mut self, ctx: &mut Context) {}
}