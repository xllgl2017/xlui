/// ### Label的示例用法
/// ```
/// use xlui::ui::Ui;
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///     //快速创建一个Label
///     ui.label("这里是Label");
///     let label=Label::new("这里是Label")
///         //设置控件宽度
///         .width(100.0)
///         //设置控件高度
///         .height(100.0);
///         //设置字体大小
///         //.size(14.0);
///     //获取控件ID
///     let _id=label.get_id();
///     ui.add(label);
/// }
/// ```

use crate::align::Align;
use crate::frame::context::UpdateType;
use crate::response::Response;
use crate::text::rich::RichText;
use crate::text::buffer::TextBuffer;
use crate::text::TextWrap;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize};

pub struct Label {
    id: String,
    buffer: TextBuffer,
}

impl Label {
    pub fn new(text: impl Into<RichText>) -> Label {
        let buffer = TextBuffer::new(text);
        Label {
            id: crate::gen_unique_id(),
            buffer,
        }
    }
    ///仅作用于draw
    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.buffer.set_wrap(wrap);
        self
    }
    ///仅作用于draw
    pub fn align(mut self, align: Align) -> Self {
        self.buffer.align = align;
        self
    }


    pub fn set_text(&mut self, text: impl ToString) {
        self.buffer.set_text(text.to_string());
    }
    ///仅作用于draw
    pub fn width(mut self, w: f32) -> Self {
        self.buffer.set_width(w);
        self
    }
    ///仅作用于draw
    pub fn height(mut self, h: f32) -> Self {
        self.buffer.set_height(h);
        self
    }

    pub fn text(&self) -> &String {
        &self.buffer.text.text
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    fn init(&mut self, ui: &mut Ui) {
        // self.buffer.rect = ui.layout().available_rect().clone_with_size(&self.buffer.rect);
        self.buffer.init(ui);
    }

    fn update_before_draw(&mut self, ui: &mut Ui) {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_str(&mut self.buffer.text.text);
            self.buffer.change = true;
        }
        if self.buffer.change {
            ui.widget_changed |= WidgetChange::Value;
        }
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.buffer.rect.offset_to_rect(&ui.draw_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            self.buffer.update_buffer(ui);
        }
        self.buffer.change = false;
        // if ui.widget_changed == WidgetChange::None as u32 { return; }
        // match ui.widget_changed {
        //     WidgetChange::None => {}
        //     WidgetChange::Position => self.update_position(ui),
        //     WidgetChange::Value => self.buffer.update_buffer(ui),
        //     WidgetChange::PositionAndValue => {
        //         self.update_position(ui);
        //         self.buffer.update_buffer(ui);
        //     }
        // }
        // if !self.buffer.change { return; }
        // if ui.can_offset { self.buffer.rect.offset(&ui.offset); }
        // self.buffer.update_buffer(ui);
    }
}


impl Widget for Label {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_before_draw(ui);
        self.buffer.redraw(ui);
    }


    fn update(&mut self, ui: &mut Ui) -> Response<'_> { //处理鼠标键盘时间
        match &ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.buffer.init(ui),
            UpdateType::Draw => self.redraw(ui),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.buffer.rect.width(), self.buffer.rect.height()))
    }
}