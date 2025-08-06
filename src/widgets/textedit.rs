use std::any::Any;
use crate::frame::context::Context;
use crate::paint::color::Color;
use crate::paint::edit::PaintTextEdit;
use crate::paint::PaintTask;
use crate::radius::Radius;
use crate::response::{Callback, DrawnEvent};
use crate::response::textedit::TextEditResponse;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::text::text_buffer::TextBuffer;
use crate::ui::{Ui, UiM};
use crate::widgets::Widget;

pub struct TextEdit {
    pub(crate) id: String,
    pub(crate) text_buffer: TextBuffer,
    pub(crate) rect: Rect,
    size_mode: SizeMode,
    border: Border,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM, &str)>>,
}

impl TextEdit {
    pub fn new(context: String) -> TextEdit {
        TextEdit {
            id: crate::gen_unique_id(),
            text_buffer: TextBuffer::new(context),
            rect: Rect::new(),
            size_mode: SizeMode::Auto,
            border: Border::new(1.0).color(Color::BLUE).radius(Radius::same(2)),
            callback: None,
        }
    }

    pub(crate) fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(context); //计算行高
        match self.size_mode {
            SizeMode::Auto => self.rect.set_size(200.0, 25.0),
            SizeMode::FixWidth => self.rect.set_height(25.0),
            SizeMode::FixHeight => self.rect.set_width(200.0),
            SizeMode::Fix => {}
        }
        let mut rect = self.rect.clone_add_padding(&Padding::same(3.0));
        rect.x.min += 5.0;
        self.text_buffer.rect = rect;
    }

    pub(crate) fn gen_style(&self, ui: &mut Ui, task: &mut PaintTextEdit) {
        let mut fill_style = ui.style.widget.click.clone();
        fill_style.fill.inactive = Color::WHITE;
        fill_style.fill.hovered = Color::WHITE;
        fill_style.fill.clicked = Color::WHITE;
        fill_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = fill_style.border.hovered.clone();
        task.fill_style(fill_style);
        let mut select_style = ui.style.widget.click.clone();
        select_style.fill.inactive = Color::rgba(144, 209, 255, 100);
        select_style.fill.hovered = Color::rgba(144, 209, 255, 100);
        select_style.fill.clicked = Color::rgba(144, 209, 255, 100);
        select_style.border.inactive = Border::new(0.0).radius(Radius::same(0));
        select_style.border.hovered = Border::new(0.0).radius(Radius::same(0));
        select_style.border.clicked = Border::new(0.0).radius(Radius::same(0));
        task.select_style(select_style);
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut UiM, &str)) -> Self {
        self.callback = Some(Callback::create_textedit(f));
        self
    }

    pub fn width_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }
}


impl Widget for TextEdit {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.reset_size(&ui.ui_manage.context);

        layout.alloc_rect(&self.rect);
        let mut task = PaintTextEdit::new(ui, self);
        self.gen_style(ui, &mut task);
        task.fill.prepare(&ui.device, false, false);
        ui.add_paint_task(self.id.clone(), PaintTask::TextEdit(task));
        ui.response.insert(self.id.clone(), TextEditResponse {
            rect: self.rect.clone(),
            event: DrawnEvent::Click,
            callback: Callback::textedit(self.callback.take()),
            value: self.text_buffer.text.clone(),
        });
    }

    fn update(&mut self, uim: &mut UiM) {
        self.text_buffer.update(uim);
    }
}