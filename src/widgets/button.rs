//!### Button的示例用法
//! ```
//! use xlui::frame::App;
//! use xlui::size::padding::Padding;
//! use xlui::style::ClickStyle;
//! use xlui::ui::Ui;
//! use xlui::widgets::button::Button;
//! use xlui::widgets::Widget;
//!
//! fn clicked<A:App>(_:&mut A,btn:&mut Button,ui:&mut Ui){
//!    //修改图片
//!    btn.set_image("logo.jpg",ui);
//!    //修改文本
//!    btn.set_text("被点击了");
//!    println!("按钮被点击了");
//! }
//!
//! fn draw<A:App>(ui:&mut Ui){
//!    //快捷创建一个按钮
//!    ui.button("点击")
//!        //设置点击回调函数
//!        .set_callback(clicked::<A>);
//!    //控件样式
//!    let style=ClickStyle::new();
//!    let mut btn=Button::new("hello button")
//!        //连接到点击回调函数
//!        .connect(clicked::<A>)
//!        //设置控件高度
//!        .height(30.0)
//!        //设置控件宽度
//!        .width(30.0)
//!        //设置按钮样式
//!        .with_style(style)
//!        //设置内部padding
//!        .padding(Padding::same(5.0));
//!    //设置字体大小
//!    btn.set_font_size(14.0);
//!    //设置控件宽高
//!    btn.set_size(30.0,30.0);
//!    ui.add(btn);
//!    //图片按钮
//!    let image_btn=Button::image_and_text("logo.jpg","点击");
//!    ui.add(image_btn);
//! }
//! ```

use crate::frame::context::{Context, UpdateType};
use crate::frame::App;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::{Callback, Response};
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::image::Image;
use crate::widgets::Widget;

pub struct Button {
    pub(crate) id: String,
    pub(crate) text_buffer: TextBuffer,
    padding: Padding,
    size_mode: SizeMode,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Button, &mut Ui)>>,
    inner_callback: Option<Box<dyn FnMut()>>,
    fill_render: RenderParam<RectParam>,
    // fill_id: String,
    // fill_param: RectParam,
    // fill_buffer: Option<wgpu::Buffer>,
    image: Option<Image>,
    image_rect: Rect,
    hovered: bool,
    changed: bool,
}


impl Button {
    pub fn new(text: impl ToString) -> Self {
        let padding = Padding::same(2.0);
        let text_buffer = TextBuffer::new(text.to_string());
        Button {
            id: crate::gen_unique_id(),
            text_buffer,
            padding,
            size_mode: SizeMode::Auto,
            callback: None,
            inner_callback: None,
            // fill_id: "".to_string(),
            // fill_param: RectParam::new(Rect::new(), ClickStyle::new()),
            // fill_buffer: None,
            image: None,
            image_rect: Rect::new(),
            hovered: false,
            changed: false,
            fill_render: RenderParam::new(RectParam::new(Rect::new(), ClickStyle::new())),
        }
    }

    pub fn image_and_text(source: &'static str, text: impl ToString) -> Self {
        let mut res = Button::new(text);
        res.image = Some(Image::new(source));
        res
    }

    pub(crate) fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(&context);
        match self.size_mode {
            SizeMode::Auto => {
                let width = self.text_buffer.rect.width() + self.padding.horizontal();
                let height = self.text_buffer.rect.height() + self.padding.vertical();
                self.fill_render.param.rect.set_size(width, height);
            }
            SizeMode::FixWidth => self.fill_render.param.rect.set_height(self.text_buffer.rect.height()),
            SizeMode::FixHeight => self.fill_render.param.rect.set_width(self.text_buffer.rect.width()),
            SizeMode::Fix => {
                self.text_buffer.rect = self.fill_render.param.rect.clone_add_padding(&self.padding);
                println!("text {:?}", self.text_buffer.rect);
            }
        }
        if self.image.is_some() {
            self.fill_render.param.rect.set_width(self.fill_render.param.rect.width() + self.fill_render.param.rect.height());
            self.text_buffer.rect = self.fill_render.param.rect.clone_add_padding(&self.padding);
            self.text_buffer.rect.add_min_x(self.fill_render.param.rect.height());
            self.text_buffer.rect.add_max_x(self.fill_render.param.rect.height());
            self.image_rect = self.fill_render.param.rect.clone_add_padding(&self.padding);
            self.image_rect.add_min_x(self.padding.left);
            self.image_rect.add_max_x(self.padding.left);
            self.image_rect.add_min_y(self.padding.top);
            self.image_rect.add_max_y(self.padding.top);
            self.image_rect.set_width(self.image_rect.height() - self.padding.vertical());
            self.image_rect.set_height(self.image_rect.height() - self.padding.vertical());
        } else {
            self.text_buffer.rect = self.fill_render.param.rect.clone_add_padding(&self.padding);
        }
    }


    pub fn set_width(&mut self, width: f32) {
        self.fill_render.param.rect.set_width(width);
        self.size_mode.fix_width();
    }

    pub fn set_height(&mut self, height: f32) {
        self.fill_render.param.rect.set_height(height);
        self.size_mode.fix_height();
    }


    pub fn set_size(&mut self, width: f32, height: f32) {
        self.fill_render.param.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.text_buffer.text_size.font_size = font_size;
    }

    pub fn width(mut self, w: f32) -> Self {
        self.set_width(w);
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.set_height(h);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn connect<A: App>(mut self, f: impl FnMut(&mut A, &mut Button, &mut Ui) + 'static) -> Self {
        self.callback = Some(Callback::create_click(f));
        self
    }

    pub fn set_callback<A: App>(&mut self, f: impl FnMut(&mut A, &mut Button, &mut Ui) + 'static) {
        self.callback = Some(Callback::create_click(f));
    }

    pub(crate) fn set_inner_callback(&mut self, f: impl FnMut() + 'static) {
        self.inner_callback = Some(Box::new(f));
    }

    pub fn with_style(mut self, style: ClickStyle) -> Self {
        self.fill_render.param.style = style;
        self
    }

    pub fn set_text(&mut self, text: impl ToString) {
        self.text_buffer.set_text(text.to_string());
    }
    pub fn set_image(&mut self, source: &'static str, ui: &mut Ui) {
        match self.image {
            None => self.image = Some(Image::new(source)),
            Some(ref mut image) => {
                ui.context.render.image.insert_image(&ui.device, source.to_string(), source);
                image.set_image(source)
            }
        }
    }


    pub fn set_style(&mut self, style: ClickStyle) {
        self.fill_render.param.style = style;
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.fill_render.param.rect = ui.layout().available_rect().clone_with_size(&self.fill_render.param.rect);
            self.reset_size(&ui.context);
        }
        //按钮矩形
        self.fill_render.init_rectangle(ui, false, false);
        // let data = self.fill_render.param.as_draw_param(false, false);
        // let buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        // self.fill_id = ui.context.render.rectangle.create_bind_group(&ui.device, &buffer);
        // self.fill_buffer = Some(buffer);
        //
        if let Some(ref mut image) = self.image {
            image.update(ui);
            image.rect = self.image_rect.clone();
        }
        //按钮文本
        self.text_buffer.draw(ui);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        println!("{} {}", self.hovered, ui.device.device_input.mouse.pressed);
        self.fill_render.update(ui, self.hovered, ui.device.device_input.mouse.pressed);
        // let data = self.fill_param.as_draw_param(self.hovered, ui.device.device_input.mouse.pressed);
        // ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
    }
}


impl Widget for Button {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        if let Some(ref mut image) = self.image { image.redraw(ui); }
        self.text_buffer.redraw(ui);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        if let Some(ref mut image) = self.image {
            image.update(ui);
        }
        match &mut ui.update_type {
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            UpdateType::MouseMove => {
                let has_pos = ui.device.device_input.hovered_at(&self.fill_render.param.rect);
                if self.hovered != has_pos {
                    self.hovered = has_pos;
                    self.changed = true;
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MousePress => {
                if !ui.device.device_input.pressed_at(&self.fill_render.param.rect) { return Response::new(&self.id, &self.fill_render.param.rect); }
                self.hovered = true;
                self.changed = true;
                ui.context.window.request_redraw();
            }
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.fill_render.param.rect) {
                    self.changed = true;
                    let callback = self.callback.take();
                    if let Some(mut callback) = callback {
                        let app = ui.app.take().unwrap();
                        callback(app, self, ui);
                        ui.app.replace(app);
                        self.callback.replace(callback);
                    }
                    if let Some(ref mut callback) = self.inner_callback {
                        callback();
                    }
                    ui.update_type = UpdateType::None;
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::Offset(o) => {
                if !ui.can_offset { return Response::new(&self.id, &self.fill_render.param.rect); }
                self.fill_render.param.rect.offset(o);
                self.text_buffer.rect.offset(o);
                self.changed = true;
                ui.context.window.request_redraw();
            }
            _ => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}