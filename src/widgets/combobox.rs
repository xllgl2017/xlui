//! ### ComboBox的示例用法
//!```
//! use std::fmt::Display;
//! use xlui::frame::App;
//! use xlui::ui::Ui;
//! use xlui::widgets::combobox::ComboBox;
//! use xlui::widgets::Widget;
//!
//! fn combo_changed<A:App>(_:&mut A,_:&mut Ui,t:&i32){
//!    println!("ComboBox的Item改变了：{}",t);
//! }
//!
//! fn draw<A:App>(ui:&mut Ui){
//!    //这里的data可以是任意实现了Display的类型
//!    let data=vec![1,2,3,4];
//!    let combo=ComboBox::new(data)
//!        //设置打开的弹窗布局的高度
//!        .with_popup_height(150.0)
//!        //连接到Item改变的监听函数
//!        .connect(combo_changed::<A>);
//!    ui.add(combo);
//! }
//! ```

use crate::frame::context::{Context, UpdateType};
use crate::frame::App;
use crate::layout::popup::Popup;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::select::SelectItem;
use crate::widgets::Widget;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

pub struct ComboBox<T> {
    pub(crate) id: String,
    popup_id: String,
    size_mode: SizeMode,
    text_buffer: TextBuffer,
    data: Vec<T>,
    popup_rect: Rect,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, &T)>>,

    fill_render: RenderParam<RectParam>,
    // fill_param: RectParam,
    // fill_id: String,
    // fill_buffer: Option<wgpu::Buffer>,

    selected: Arc<RwLock<Option<String>>>,

    previous_select: Option<String>,
}

impl<T: Display + 'static> ComboBox<T> {
    pub fn new(data: Vec<T>) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(230, 230, 230);
        fill_style.border.inactive = Border::new(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        ComboBox {
            id: crate::gen_unique_id(),
            popup_id: "".to_string(),
            size_mode: SizeMode::Auto,
            text_buffer: TextBuffer::new("".to_string()),
            data,
            popup_rect: Rect::new(),
            callback: None,
            fill_render: RenderParam::new(RectParam::new(Rect::new(), fill_style)),
            // fill_param: RectParam::new(Rect::new(), fill_style),
            // fill_id: "".to_string(),
            // fill_buffer: None,
            previous_select: None,
            selected: Arc::new(RwLock::new(None)),

        }
    }

    fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(context);
        match self.size_mode {
            SizeMode::Auto => self.fill_render.param.rect.set_size(100.0, 20.0),
            SizeMode::FixWidth => self.fill_render.param.rect.set_height(20.0),
            SizeMode::FixHeight => self.fill_render.param.rect.set_width(100.0),
            SizeMode::Fix => {}
        }
        self.text_buffer.rect = self.fill_render.param.rect.clone_add_padding(&Padding::same(2.0));
        self.popup_rect = self.fill_render.param.rect.clone_with_size(&self.popup_rect);
        self.popup_rect.set_width(self.fill_render.param.rect.width());
        self.popup_rect.add_min_y(self.fill_render.param.rect.height() + 5.0);
        self.popup_rect.add_max_y(self.fill_render.param.rect.height() + 5.0);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.fill_render.param.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
        self
    }

    /// 设置popup的高度
    pub fn with_popup_height(mut self, height: f32) -> Self {
        self.popup_rect.set_height(height);
        self
    }

    fn add_item(&self, ui: &mut Ui, item: &T) {
        let mut select = SelectItem::new(item.to_string()).padding(Padding::same(3.0))
            .contact(self.selected.clone());
        select.set_size(ui.layout().available_rect().width(), 25.0);
        ui.add(select);
    }

    fn add_items(&self, ui: &mut Ui) {
        for datum in self.data.iter() {
            self.add_item(ui, datum);
        }
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, &T)) -> Self {
        self.callback = Some(Callback::create_combobox(f));
        self
    }

    fn init(&mut self, ui: &mut Ui) {
        //分配大小
        self.fill_render.param.rect = ui.layout().available_rect().clone_with_size(&self.fill_render.param.rect);
        self.reset_size(&ui.context);
        //下拉框布局
        let popup = Popup::new(ui, self.popup_rect.clone());
        self.popup_id = popup.id.clone();
        popup.show(ui, |ui| self.add_items(ui));
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //背景
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(230, 230, 230);
        fill_style.border.inactive = Border::new(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        self.fill_render.init_rectangle(ui, false, false);
        // let data = self.fill_param.as_draw_param(false, false);
        // let fill_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        // self.fill_id = ui.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        // self.fill_buffer = Some(fill_buffer);
        //文本
        self.text_buffer.draw(ui);
    }

    pub fn parent(&self) -> Arc<RwLock<Option<String>>> {
        self.selected.clone()
    }
}


impl<T: Display + 'static> Widget for ComboBox<T> {
    fn redraw(&mut self, ui: &mut Ui) {
        // if self.fill_buffer.is_none() { self.init(ui); }
        // let resp = Response::new(&self.id, &self.fill_param.rect);
        // if ui.pass.is_none() { return resp; }
        let select = self.selected.read().unwrap();
        if *select != self.previous_select {
            self.previous_select = select.clone();
            if let Some(ref select) = self.previous_select {
                self.text_buffer.set_text(select.to_string());
                if let Some(ref mut callback) = self.callback {
                    let app = ui.app.take().unwrap();
                    let t = self.data.iter().find(|x| &x.to_string() == select).unwrap();
                    callback(app, ui, t);
                    ui.app.replace(app);
                    ui.context.window.request_redraw();
                }
            }

            let popup = &mut ui.popups.as_mut().unwrap()[&self.popup_id];
            popup.open = false;
        }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.text_buffer.redraw(ui);
        // resp
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.fill_render.param.rect) {
                    let popup = &mut ui.popups.as_mut().unwrap()[&self.popup_id];
                    popup.open = !popup.open;
                    ui.update_type = UpdateType::None;
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}