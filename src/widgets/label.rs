//! ### Label的示例用法
//! ```
//! use xlui::ui::Ui;
//! use xlui::widgets::label::Label;
//!
//! fn draw(ui:&mut Ui){
//!     //快速创建一个Label
//!     ui.label("这里是Label");
//!     let label=Label::new("这里是Label")
//!         //设置控件宽度
//!         .width(100.0)
//!         //设置控件高度
//!         .height(100.0);
//!         //设置字体大小
//!         //.size(14.0);
//!     //获取控件ID
//!     let _id=label.get_id();
//!     ui.add(label);
//! }
//! ```

use crate::align::Align;
use crate::frame::context::UpdateType;
use crate::response::Response;
use crate::text::rich::RichText;
use crate::text::text_buffer::TextBuffer;
use crate::text::TextWrap;
use crate::ui::Ui;
use crate::widgets::Widget;

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
    // ///仅作用于draw
    // pub fn size(mut self, s: f32) -> Self {
    //     self.buffer.text_size.font_size = s;
    //     self
    // }

    pub fn text(&self) -> &String {
        &self.buffer.text.text
    }

    pub fn get_id(&self) -> &str {
        &self.id
    }

    fn init(&mut self, ui: &mut Ui) {
        self.buffer.rect = ui.layout().available_rect().clone_with_size(&self.buffer.rect);
        self.buffer.reset_size(ui.context);
        self.buffer.draw(ui);
    }

    fn update_before_draw(&mut self, ui: &mut Ui) {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_str(&mut self.buffer.text.text);
            self.buffer.change = true;
        }
        if !self.buffer.change && !ui.can_offset { return; }
        if ui.can_offset { self.buffer.rect.offset(&ui.offset); }
        self.buffer.update_buffer(ui);
    }
}


impl Widget for Label {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_before_draw(ui);
        self.buffer.redraw(ui);
    }


    fn update(&mut self, ui: &mut Ui) -> Response { //处理鼠标键盘时间
        match &ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => {
                println!("11111111111111111");
                self.buffer.draw(ui)
            }
            // UpdateType::Offset(o) => {
            //     if !ui.can_offset { return Response::new(&self.id, &self.buffer.rect); }
            //     self.buffer.rect.offset(o);
            //     ui.context.window.request_redraw();
            // }
            _ => {}
        }
        Response::new(&self.id, &self.buffer.rect)
    }
}