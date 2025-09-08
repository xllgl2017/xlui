/// ### RadioButton的示例用法
/// ```
/// use xlui::frame::App;
/// use xlui::ui::Ui;
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

use crate::frame::context::{ContextUpdate, UpdateType};
use crate::frame::App;
use crate::render::circle::param::CircleParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::rich::RichText;
use crate::text::buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;

pub struct RadioButton {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) value: bool,
    pub(crate) text: TextBuffer,
    pub(crate) callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, bool)>>,
    size_mode: SizeMode,
    outer_render: RenderParam<CircleParam>,
    inner_render: RenderParam<CircleParam>,
    hovered: bool,
    contact_ids: Vec<String>,
    changed: bool,
}

impl RadioButton {
    pub fn new(v: bool, label: impl Into<RichText>) -> RadioButton {
        let mut outer_style = ClickStyle::new();
        outer_style.fill.inactive = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.fill.hovered = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.fill.clicked = Color::TRANSPARENT; //Color::rgb(95, 95, 95);
        outer_style.border.inactive = Border::new(1.0).color(Color::rgb(95, 95, 95));
        outer_style.border.hovered = Border::new(1.0).color(Color::rgb(56, 160, 200));
        outer_style.border.clicked = Border::new(1.0).color(Color::rgb(56, 182, 244));

        let mut inner_style = ClickStyle::new();
        inner_style.fill.inactive = Color::TRANSPARENT;
        inner_style.fill.hovered = Color::rgb(56, 160, 200);
        inner_style.fill.clicked = Color::rgb(56, 182, 244);
        inner_style.border.inactive = Border::new(0.0).color(Color::TRANSPARENT);
        inner_style.border.hovered = Border::new(0.0).color(Color::TRANSPARENT);
        inner_style.border.clicked = Border::new(0.0).color(Color::TRANSPARENT);
        RadioButton {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            value: v,
            text: TextBuffer::new(label),
            callback: None,
            size_mode: SizeMode::Auto,
            outer_render: RenderParam::new(CircleParam::new(Rect::new(), outer_style)),
            inner_render: RenderParam::new(CircleParam::new(Rect::new(), inner_style)),
            hovered: false,
            contact_ids: vec![],
            changed: false,
        }
    }
    fn reset_size(&mut self, ui: &mut Ui) {
        self.rect.set_height(16.0);
        self.text.rect = self.rect.clone();
        self.text.rect.add_min_x(18.0);
        self.text.rect.add_max_x(18.0);
        self.text.init(ui);
        match self.size_mode {
            SizeMode::Auto => self.rect.set_width(18.0 + self.text.rect.width()),
            SizeMode::FixWidth => {}
            SizeMode::FixHeight => self.rect.set_width(18.0 + self.text.rect.width()),
            SizeMode::Fix => {}
        }
    }

    pub fn with_width(mut self, width: f32) -> RadioButton {
        self.rect.set_width(width);
        self.size_mode = SizeMode::FixWidth;
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

    pub fn contact(mut self, id: impl ToString) -> Self {
        self.contact_ids.push(id.to_string());
        self
    }

    fn init(&mut self, ui: &mut Ui) {
        //分配大小
        self.rect = ui.layout().available_rect().clone_with_size(&self.rect);
        self.reset_size(ui);
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //外圆
        self.outer_render.param.rect = self.rect.clone();
        self.outer_render.param.rect.set_width(self.rect.height());
        self.outer_render.init_circle(ui, self.value, self.value);
        //内圆
        self.inner_render.param.rect = self.rect.clone();

        self.inner_render.param.rect.add_min_x(4.0);
        self.inner_render.param.rect.contract_y(4.0);
        self.inner_render.param.rect.set_width(self.inner_render.param.rect.height());
        self.inner_render.init_circle(ui, self.value, self.value);
        //文本
        self.text.init(ui);
    }

    // fn update_radio(&mut self, ui: &mut Ui) {
    //
    //     ui.context.window.request_redraw();
    // }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_bool(&mut self.value);
            self.changed = true;
        }
        if !self.changed && !ui.can_offset { return; }
        if ui.can_offset {
            self.outer_render.param.rect.offset(&ui.offset);
            self.inner_render.param.rect.offset(&ui.offset);
            self.text.rect.offset(&ui.offset);
            self.rect.offset(&ui.offset);
        }
        self.outer_render.update(ui, self.hovered || self.value, ui.device.device_input.mouse.pressed || self.value);
        self.inner_render.update(ui, self.value, ui.device.device_input.mouse.pressed || self.value);
    }
}


impl Widget for RadioButton {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.circle.render(&self.outer_render, pass);
        ui.context.render.circle.render(&self.inner_render, pass);
        self.text.redraw(ui);
    }

    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.rect);
                if hovered != self.hovered {
                    self.hovered = hovered;
                    self.changed = true;
                }
            }
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.rect) {
                    self.value = !self.value;
                    self.changed = true;
                    if let Some(ref mut callback) = self.callback {
                        let app = ui.app.take().unwrap();
                        callback(app, ui, self.value);
                        ui.app.replace(app);
                    }
                    ui.update_type = UpdateType::None;
                    ui.send_updates(&self.contact_ids, ContextUpdate::Bool(self.value));
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }

        Response::new(&self.id, &self.rect)
    }
}