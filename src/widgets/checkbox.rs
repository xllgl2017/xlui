//! ### CheckBox的示例用法
//! ```
//! use xlui::frame::App;
//! use xlui::ui::Ui;
//! use xlui::widgets::checkbox::CheckBox;
//! use xlui::widgets::Widget;
//!
//! fn checked<A:App>(_:&mut A,_:&mut Ui,check:bool){
//!    println!("复选框状态改变: {}",check);
//! }
//!
//! fn draw<A:App>(ui:&mut Ui){
//!    //快速创建一个复选框
//!    ui.checkbox(false,"Hello CheckBox")
//!        //设置回调函数
//!        .set_callback(checked::<A>);
//!
//!    let mut check=CheckBox::new(false,"hello button")
//!        //连接到回调函数
//!        .connect(checked::<A>)
//!        //设置控件宽度
//!        .with_width(100.0)
//!        //与ID为my_radio的控件关联
//!        .contact("my_radio")
//!        //设置ID
//!        .id("my_checked");
//!    ui.add(check);
//!
//! }
//! ```

use crate::frame::context::{Context, ContextUpdate, UpdateType};
use crate::frame::App;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;

pub struct CheckBox {
    pub(crate) id: String,
    rect: Rect,
    text: TextBuffer,
    check_text: TextBuffer,
    value: bool,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, bool)>>,
    size_mode: SizeMode,

    check_render: RenderParam<RectParam>,
    // check_param: RectParam,
    // check_id: String,
    // check_buffer: Option<wgpu::Buffer>,

    hovered: bool,
    contact_ids: Vec<String>,
}

impl CheckBox {
    pub fn new(v: bool, label: impl ToString) -> CheckBox {
        let mut check_style = ClickStyle::new();
        check_style.fill.inactive = Color::rgb(210, 210, 210);
        check_style.fill.hovered = Color::rgb(210, 210, 210);
        check_style.fill.clicked = Color::rgb(210, 210, 210);
        check_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        check_style.border.hovered = Border::new(1.0).color(Color::BLACK).radius(Radius::same(2));
        check_style.border.clicked = Border::new(1.0).color(Color::BLACK).radius(Radius::same(2));
        CheckBox {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            text: TextBuffer::new(label.to_string()),
            check_text: TextBuffer::new("√".to_string()),
            value: v,
            callback: None,
            size_mode: SizeMode::Auto,
            check_render: RenderParam::new(RectParam::new(Rect::new(), check_style)),
            // check_param: RectParam::new(Rect::new(), check_style),
            // check_id: "".to_string(),
            // check_buffer: None,
            hovered: false,
            contact_ids: vec![],
        }
    }


    pub(crate) fn reset_size(&mut self, context: &Context) {
        self.text.rect = self.rect.clone();
        self.text.reset_size(context);
        self.text.rect.add_min_x(15.0);
        self.text.rect.add_max_x(15.0);
        // self.text.rect.offset_x(15.0);
        match self.size_mode {
            SizeMode::Auto => {
                self.rect.set_width(15.0 + self.text.rect.width());
                self.rect.set_height(20.0);
            }
            SizeMode::FixWidth => self.rect.set_height(20.0),
            SizeMode::FixHeight => self.rect.set_width(15.0 + self.text.rect.width()),
            SizeMode::Fix => {}
        }

        self.text.rect.set_height(self.rect.height());
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, bool)) -> Self {
        self.callback = Some(Callback::create_check(f));
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.rect.set_width(width);
        self.size_mode = SizeMode::FixWidth;
        self
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
        self.rect = ui.layout().available_rect().clone_with_size(&self.rect);
        self.reset_size(&ui.context);
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //复选框
        self.check_render.param.rect = self.rect.clone();
        self.check_render.param.rect.set_width(15.0);
        self.check_render.param.rect.set_height(15.0);
        self.check_render.init_rectangle(ui, false, self.value);
        // let data = self.check_param.as_draw_param(false, self.value);
        // let check_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        // self.check_id = ui.context.render.rectangle.create_bind_group(&ui.device, &check_buffer);
        // self.check_buffer = Some(check_buffer);
        //文本
        self.text.draw(ui);
        self.check_text.reset_size(&ui.context);
        self.check_text.rect = self.check_render.param.rect.clone();
        self.check_text.draw(ui);
    }

    fn update_check(&mut self, ui: &mut Ui) {
        self.check_render.update(ui, self.hovered, ui.device.device_input.mouse.pressed);
        // let data = self.check_param.as_draw_param(self.hovered, ui.device.device_input.mouse.pressed);
        // ui.device.queue.write_buffer(self.check_buffer.as_ref().unwrap(), 0, data);
    }
}

impl Widget for CheckBox {
    fn redraw(&mut self, ui: &mut Ui) {
        // if self.check_buffer.is_none() { self.init(ui); }
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_bool(&mut self.value);
            self.update_check(ui);
        }
        // let resp = Response::new(&self.id, &self.rect);
        // if ui.pass.is_none() { return resp; }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.check_render, pass);
        self.text.redraw(ui);
        if self.value { self.check_text.redraw(ui); }
        // resp
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.update_check(ui);
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MousePress => {}
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.rect) {
                    self.value = !self.value;
                    self.update_check(ui);
                    if let Some(ref mut callback) = self.callback {
                        let app = ui.app.take().unwrap();
                        callback(app, ui, self.value);
                        ui.app.replace(app);
                    }
                    ui.send_updates(&self.contact_ids, ContextUpdate::Bool(self.value));
                    ui.context.window.request_redraw();
                    ui.update_type = UpdateType::None;
                }
            }
            _ => {}
        }
        Response::new(&self.id, &self.rect)
    }
}