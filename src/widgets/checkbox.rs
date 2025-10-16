use crate::frame::context::{ContextUpdate, UpdateType};
use crate::frame::App;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::{Callback, InnerCallB, Response};
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::buffer::TextBuffer;
use crate::text::rich::RichText;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};

/// ### CheckBox的示例用法
/// ```
/// use xlui::*;
///
/// fn checked<A:App>(_:&mut A,_:&mut Ui,check:bool){
///    println!("复选框状态改变: {}",check);
/// }
///
/// fn draw<A:App>(ui:&mut Ui){
///    //快速创建一个复选框
///    ui.checkbox(false,"Hello CheckBox")
///        //设置回调函数
///        .set_callback(checked::<A>);
///
///    let mut check=CheckBox::new(false,"hello button")
///        //连接到回调函数
///        .connect(checked::<A>)
///        //设置控件宽度
///        .with_width(100.0)
///        //与ID为my_radio的控件关联
///        .contact("my_radio")
///        //设置ID
///        .id("my_checked");
///    ui.add(check);
///
/// }
/// ```

pub struct CheckBox {
    pub(crate) id: String,
    text: TextBuffer,
    check_text: TextBuffer,
    value: bool,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, bool)>>,
    inner_callback: Option<InnerCallB>,
    geometry: Geometry,
    check_render: RenderParam,
    contact_ids: Vec<String>,
    fill: Option<RenderParam>,
    state: WidgetState,
}

impl CheckBox {
    pub fn new(v: bool, label: impl Into<RichText>) -> CheckBox {
        let mut check_style = ClickStyle::new();
        check_style.fill.inactive = Color::rgb(210, 210, 210);
        check_style.fill.hovered = Color::rgb(210, 210, 210);
        check_style.fill.clicked = Color::rgb(210, 210, 210);
        check_style.border.inactive = Border::same(0.0).radius(Radius::same(2));
        check_style.border.hovered = Border::same(1.0).color(Color::BLACK).radius(Radius::same(2));
        check_style.border.clicked = Border::same(1.0).color(Color::BLACK).radius(Radius::same(2));
        let param = RectParam::new().with_rect(Rect::new().with_size(15.0, 15.0)).with_style(check_style);
        CheckBox {
            id: crate::gen_unique_id(),
            text: TextBuffer::new(label),
            check_text: TextBuffer::new(RichText::new("√").size(14.0)),
            value: v,
            callback: None,
            inner_callback: None,
            geometry: Geometry::new(),
            check_render: RenderParam::new(RenderKind::Rectangle(param)),
            contact_ids: vec![],
            fill: None,
            state: WidgetState::default(),
        }
    }


    pub(crate) fn reset_size(&mut self, ui: &mut Ui) {
        self.text.init(ui);
        self.geometry.set_context_size(self.text.geometry.margin_width() + 15.0, self.text.geometry.margin_height());
    }

    pub(crate) fn connect_inner(mut self, callback: impl FnMut() + 'static) -> Self {
        self.inner_callback = Some(Box::new(callback));
        self
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, bool)) -> Self {
        self.callback = Some(Callback::create_check(f));
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.geometry.set_fix_width(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.geometry.set_fix_height(height);
        self
    }

    pub fn with_size(self, w: f32, h: f32) -> Self {
        self.with_height(h).with_width(w)
    }

    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn contact(mut self, id: impl ToString) -> Self {
        self.contact_ids.push(id.to_string());
        self
    }

    pub fn set_callback<A: App>(&mut self, f: fn(&mut A, &mut Ui, bool)) {
        self.callback = Some(Callback::create_check(f));
    }

    fn init(&mut self, ui: &mut Ui) {
        //分配大小
        self.reset_size(ui);
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //背景
        if let Some(ref mut fill) = self.fill {
            fill.rect_mut().set_size(self.geometry.padding_width(), self.geometry.padding_height());
            #[cfg(feature = "gpu")]
            fill.init(ui, false, false);
        }
        #[cfg(feature = "gpu")]
        //复选框
        self.check_render.init(ui, false, self.value);
        //文本
        self.check_text.init(ui);
        self.check_text.geometry.offset_to_rect(self.check_render.rect());
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_bool(&mut self.value);
            ui.widget_changed |= WidgetChange::Value;
        }
        if self.state.changed { ui.widget_changed |= WidgetChange::Value; }
        self.state.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            if let Some(ref mut fill) = self.fill {
                fill.offset_to_rect(&ui.draw_rect);
            }
            self.geometry.offset_to_rect(&ui.draw_rect);
            let mut rect = self.geometry.context_rect();
            self.check_render.offset_to_rect(&rect);
            self.check_text.geometry.offset_to_rect(&rect);
            rect.add_min_x(self.check_render.rect().width() + 2.0);
            self.text.geometry.offset_to_rect(&rect);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        if let Some(ref mut fill) = self.fill {
            fill.draw(ui, self.state.hovered || self.value, self.value);
        }
        self.check_render.draw(ui, self.state.hovered, self.state.pressed);
        self.text.redraw(ui);
        if self.value { self.check_text.redraw(ui); }
    }

    pub fn style_mut(&mut self) -> &mut RenderParam {
        self.fill.get_or_insert_with(|| RenderParam::new(RenderKind::Rectangle(RectParam::new())))
    }

    pub fn geometry_mut(&mut self) -> &mut Geometry {
        &mut self.geometry
    }
}

impl Widget for CheckBox {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.geometry.padding_rect());
                if self.state.on_hovered(hovered) { ui.context.window.request_redraw(); }
            }
            UpdateType::MouseRelease => {
                let clicked = ui.device.device_input.click_at(&self.geometry.padding_rect());
                if self.state.on_clicked(clicked) {
                    self.value = !self.value;
                    if let Some(ref mut callback) = self.callback {
                        let app = ui.app.take().unwrap();
                        callback(app, ui, self.value);
                        ui.app.replace(app);
                    }
                    if let Some(ref mut callback) = self.inner_callback {
                        callback();
                    }
                    ui.send_updates(&self.contact_ids, ContextUpdate::Bool(self.value));
                    ui.context.window.request_redraw();
                    ui.update_type = UpdateType::None;
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