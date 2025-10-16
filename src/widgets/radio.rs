use crate::frame::context::{ContextUpdate, UpdateType};
use crate::frame::App;
use crate::render::circle::param::CircleParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::buffer::TextBuffer;
use crate::text::rich::RichText;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};

/// ### RadioButton的示例用法
/// ```
/// use xlui::*;
///
/// fn checked<A:App>(_:&mut A,_:&mut Ui,checked:bool){
///     println!("单选框的状态改变: {}",checked);
/// }
///
/// fn draw<A:App>(ui:&mut Ui){
///     //快速创建一个单选框
///     ui.radio(false,"HelloRadio");
///     let radio=RadioButton::new(false,"radio")
///         //设置控件ID
///         .id("my_radio")
///         //连接到回调函数
///         .connect(checked::<A>)
///         //设置控件宽度
///         .with_width(100.0)
///         //与my_checkbox关联
///         .contact("my_checkbox");
///
///     ui.add(radio);
/// }
pub struct RadioButton {
    pub(crate) id: String,
    pub(crate) value: bool,
    pub(crate) text: TextBuffer,
    pub(crate) callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, bool)>>,
    geometry: Geometry,
    outer_render: RenderParam,
    inner_render: RenderParam,
    contact_ids: Vec<String>,
    group_ids: Vec<String>,
    state: WidgetState,
}

impl RadioButton {
    pub fn new(v: bool, label: impl Into<RichText>) -> RadioButton {
        let mut outer_style = ClickStyle::new();
        outer_style.fill.inactive = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.fill.hovered = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.fill.clicked = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.border.inactive = Border::same(1.0).color(Color::rgb(95, 95, 95));
        outer_style.border.hovered = Border::same(1.0).color(Color::rgb(56, 160, 200));
        outer_style.border.clicked = Border::same(1.0).color(Color::rgb(56, 182, 244));

        let mut inner_style = ClickStyle::new();
        inner_style.fill.inactive = Color::TRANSPARENT;
        inner_style.fill.hovered = Color::rgb(56, 160, 200);
        inner_style.fill.clicked = Color::rgb(56, 182, 244);
        inner_style.border.inactive = Border::same(0.0).color(Color::TRANSPARENT);
        inner_style.border.hovered = Border::same(0.0).color(Color::TRANSPARENT);
        inner_style.border.clicked = Border::same(0.0).color(Color::TRANSPARENT);
        let outer_param = CircleParam::new(Rect::new().with_size(16.0, 16.0), outer_style);
        let inner_param = CircleParam::new(Rect::new().with_size(16.0, 16.0), inner_style);
        RadioButton {
            id: crate::gen_unique_id(),
            value: v,
            text: TextBuffer::new(label),
            callback: None,
            geometry: Geometry::new(),
            outer_render: RenderParam::new(RenderKind::Circle(outer_param)),
            inner_render: RenderParam::new(RenderKind::Circle(inner_param)),
            contact_ids: vec![],
            group_ids: vec![],
            state: WidgetState::default(),
        }
    }

    fn reset_size(&mut self, ui: &mut Ui) {
        self.text.geometry.add_fix_width(self.geometry.context_width() - 18.0);
        self.text.init(ui);
        self.geometry.set_context_size(self.text.geometry.padding_width() + 18.0, self.text.geometry.padding_height());
    }

    pub fn with_width(mut self, width: f32) -> RadioButton {
        self.geometry.set_fix_width(width);
        self
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, bool)) -> Self {
        self.callback = Some(Callback::create_check(f));
        self
    }

    pub fn set_callback<A: App>(&mut self, f: fn(&mut A, &mut Ui, bool)) {
        self.callback = Some(Callback::create_check(f));
    }

    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    ///控件值关联，自动更新给定id的控件的值
    pub fn contact(mut self, id: impl ToString) -> Self {
        self.contact_ids.push(id.to_string());
        self
    }

    /// 关联radio组
    pub fn set_group_by_id(&mut self, id: impl ToString) {
        self.group_ids.push(id.to_string());
    }

    fn init(&mut self, ui: &mut Ui) {
        //分配大小
        self.reset_size(ui);
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        #[cfg(feature = "gpu")]
        //外圆
        self.outer_render.init(ui, self.value, self.value);
        //内圆
        self.inner_render.rect_mut().add_min_x(4.0);
        self.inner_render.rect_mut().contract_y(4.0);
        let height = self.inner_render.rect().height();
        self.inner_render.rect_mut().set_width(height);
        #[cfg(feature = "gpu")]
        self.inner_render.init(ui, self.value, self.value);
        //文本
        self.text.init(ui);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_bool(&mut self.value);
            ui.widget_changed |= WidgetChange::Value;
        }
        if self.state.changed { ui.widget_changed |= WidgetChange::Value; }
        self.state.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.outer_render.rect_mut().offset_to_rect(&ui.draw_rect);
            let mut text_rect = ui.draw_rect.clone();
            text_rect.add_min_x(self.outer_render.rect().width() + 2.0);
            self.text.geometry.offset_to_rect(&text_rect);
            let mut inner_rect = ui.draw_rect.clone();
            inner_rect.set_width(self.inner_render.rect().width());
            inner_rect.add_min_x(4.0);
            inner_rect.add_min_y(4.0);
            self.inner_render.offset_to_rect(&inner_rect);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.outer_render.draw(ui, self.state.hovered || self.value, self.value);
        self.inner_render.draw(ui, self.value, self.value);
        self.text.redraw(ui);
    }
}


impl Widget for RadioButton {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.geometry.padding_rect());
                if self.state.on_hovered(hovered) { ui.context.window.request_redraw(); }
            }
            UpdateType::MouseRelease => {
                let clicked = ui.device.device_input.click_at(&self.geometry.padding_rect());
                if self.state.on_clicked(clicked) {
                    self.value = !self.value || !self.group_ids.is_empty();
                    if let Some(ref mut callback) = self.callback {
                        let app = ui.app.take().unwrap();
                        callback(app, ui, self.value);
                        ui.app.replace(app);
                    }
                    ui.update_type = UpdateType::None;
                    ui.send_updates(&self.contact_ids, ContextUpdate::Bool(self.value));
                    ui.send_updates(&self.group_ids, ContextUpdate::Bool(!self.value));
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }

        Response::new(&self.id, WidgetSize::same(self.geometry.margin_width(), self.geometry.margin_height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}