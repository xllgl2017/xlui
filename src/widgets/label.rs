use crate::align::Align;
use crate::frame::context::UpdateType;
use crate::style::Visual;
use crate::response::Response;
use crate::size::Geometry;
use crate::text::buffer::TextBuffer;
use crate::text::rich::RichText;
use crate::text::TextWrap;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
/// ### Label的示例用法
/// ```
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///     //快速创建一个Label
///     ui.label("这里是Label");
///     let label=Label::new(
///         //设置字体大小
///         "这里是Label".size(14.0)
///         )
///         //设置控件宽度
///         .width(100.0)
///         //设置控件高度
///         .height(100.0);
///     //获取控件ID
///     let _id=label.id();
///     ui.add(label);
/// }
/// ```
pub struct Label {
    id: String,
    buffer: TextBuffer,
    state: WidgetState,
    visual: Visual,
}

impl Label {
    pub fn new(text: impl Into<RichText>) -> Label {
        let buffer = TextBuffer::new(text);
        Label {
            id: crate::gen_unique_id(),
            buffer,
            state: WidgetState::default(),
            visual: Visual::new(),
        }
    }
    ///仅作用于draw
    pub fn wrap(mut self, wrap: TextWrap) -> Self {
        self.buffer.set_wrap(wrap);
        self
    }

    ///仅作用于draw
    pub fn align(mut self, align: Align) -> Self {
        self.buffer.geometry.set_align(align);
        self
    }

    ///设置文本
    pub fn set_text(&mut self, text: impl ToString) {
        self.buffer.set_text(text.to_string());
    }

    ///仅作用于draw
    pub fn width(mut self, w: f32) -> Self {
        self.buffer.geometry.set_fix_width(w);
        self
    }

    pub fn max_width(mut self, w: f32) -> Self {
        self.buffer.geometry.set_max_width(w);
        self
    }
    ///仅作用于draw
    pub fn height(mut self, h: f32) -> Self {
        self.buffer.geometry.set_fix_height(h);
        self
    }

    pub fn text(&self) -> &String {
        &self.buffer.text.text
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    fn init(&mut self, ui: &mut Ui) {
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
            self.buffer.geometry.offset_to_rect(&ui.draw_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            self.buffer.update_buffer(ui);
        }
        self.buffer.change = false;
    }
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_before_draw(ui);
        self.buffer.redraw(ui);
    }
}


impl Widget for Label {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> { //处理鼠标键盘时间
        self.visual.draw(ui, self.state.disabled, self.state.hovered, self.state.pressed, false);
        match &ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.buffer.init(ui),
            UpdateType::Draw => self.redraw(ui),
            _ => self.state.handle_event(ui, &self.buffer.geometry, self.visual.disable())
        }
        self.visual.draw(ui, self.state.disabled, self.state.hovered, self.state.pressed, true);
        Response::new(&self.id, WidgetSize::same(self.buffer.geometry.margin_width(), self.buffer.geometry.margin_height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.buffer.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}