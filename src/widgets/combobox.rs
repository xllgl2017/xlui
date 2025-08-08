//!### ComboBox的示例用法
//!
//! ```
//! use xlui::widgets::combobox::ComboBox;
//! use xlui::widgets::Widget;
//! # xlui::_run_test(|ui|{
//!    ComboBox::new(&vec![1,2,3,4]).with_popup_height(150.0).draw(ui);
//! # });
//! ```

use std::any::Any;
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
use crate::paint::button::PaintButton;
use crate::response::Callback;

pub struct ComboBox {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    size_mode: SizeMode,
    pub(crate) text_buffer: TextBuffer,
    pub(crate) data: Vec<String>,
    item_style: ClickStyle,
    popup_rect: Rect,
    pub(crate) callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, usize)>>,
}

impl ComboBox {
    pub fn new<T: Display>(data: &Vec<T>) -> Self {
        let data = data.iter().map(|x| x.to_string()).collect();
        ComboBox {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
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
            popup_rect: Rect::new(),
            callback: None,
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
        self.popup_rect = self.rect.clone_with_size(&self.popup_rect);
        self.popup_rect.set_width(self.rect.width());
        self.popup_rect.offset_y(self.rect.height() + 5.0);
        // self.popup.set_rect(popup_rect);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
        self
    }

    /// 设置popup的高度
    pub fn with_popup_height(mut self, height: f32) -> Self {
        self.popup_rect.set_height(height);
        // self.popup.rect_mut().set_height(height);
        self
    }

    fn add_item(&self, ui: &mut Ui, popup: &mut Popup, item: &String) {
        ui.style.widget.click = self.item_style.clone();
        let mut btn = Button::new(item).padding(Padding::same(3.0));
        btn.rect = popup.layout.available_rect.clone_with_size(&btn.rect);
        btn.reset_size(&ui.ui_manage.context);
        btn.set_size(popup.layout.available_rect.width(), 25.0);
        popup.layout.alloc_rect(&btn.rect);
        let task = PaintButton::new(ui, &mut btn);
        popup.layout.widgets.insert(btn.id.clone(), PaintTask::Button(task));
    }

    fn add_items(&self, ui: &mut Ui, popup: &mut Popup) {
        let style = ui.style.widget.click.clone();
        for (row, datum) in self.data.iter().enumerate() {
            self.add_item(ui, popup, datum);
        }
        ui.style.widget.click = style;
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Context, usize)) -> Self {
        self.callback = Some(Callback::create_combobox(f));
        self
    }
}


impl Widget for ComboBox {
    fn draw(&mut self, ui: &mut Ui) {
        self.id=crate::gen_unique_id();
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.reset_size(&ui.ui_manage.context);
        layout.alloc_rect(&self.rect);

        let mut popup = Popup::new(ui, self.popup_rect.clone(),self.id.clone());
        popup.layout.available_rect = self.popup_rect.clone_add_padding(&Padding::same(2.0));
        popup.layout.item_space = 2.0;
        self.add_items(ui, &mut popup);
        let popup_id = popup.id.clone();
        ui.ui_manage.popups.insert(popup.id.clone(), popup);
        let task = PaintComboBox::new(ui, self, popup_id);
        ui.add_paint_task(self.id.clone(), PaintTask::ComboBox(task));
    }

    fn update(&mut self, ctx: &mut Context) {}
}