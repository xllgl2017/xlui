use crate::align::Align;
use crate::frame::context::UpdateType;
use crate::frame::App;
use crate::render::image::ImageSource;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::{Callback, Response};
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::Geometry;
use crate::style::ClickStyle;
use crate::text::buffer::TextBuffer;
use crate::text::rich::RichText;
use crate::ui::Ui;
use crate::widgets::image::Image;
use crate::widgets::{Widget, WidgetChange, WidgetSize};

/// ### Button的示例用法
/// ```
/// use xlui::*;
///
/// fn clicked<A:App>(_:&mut A,btn:&mut Button,ui:&mut Ui){
///    //修改图片
///    btn.set_image("logo.jpg" );
///    //修改文本
///    btn.set_text("被点击了");
///    println!("按钮被点击了");
/// }
///
/// fn draw<A:App>(ui:&mut Ui){
///    //快捷创建一个按钮
///    ui.button("点击")
///        //设置点击回调函数
///        .set_callback(clicked::<A>);
///    //控件样式
///    let style=ClickStyle::new();
///    let mut btn=Button::new("hello button")
///        //连接到点击回调函数
///        .connect(clicked::<A>)
///        //设置控件高度
///        .height(30.0)
///        //设置控件宽度
///        .width(30.0)
///        //设置按钮样式
///        .with_style(style)
///        //设置内部padding
///        .padding(Padding::same(5.0));
///    //设置字体大小
///    //btn.set_font_size(14.0);
///    //设置控件宽高
///    btn.set_size(30.0,30.0);
///    ui.add(btn);
///    //图片按钮
///    let image_btn=Button::image_and_text("logo.jpg","点击");
///    ui.add(image_btn);
/// }
/// ```
pub struct Button {
    pub(crate) id: String,
    pub(crate) text_buffer: TextBuffer,
    padding: Padding,
    geometry: Geometry,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Button, &mut Ui)>>,
    inner_callback: Option<Box<dyn FnMut()>>,
    fill_render: RenderParam<RectParam>,
    image: Option<Image>,
    hovered: bool,
    changed: bool,
}


impl Button {
    pub fn new(text: impl Into<RichText>) -> Self {
        let padding = Padding::same(2.0);
        // let mut text_buffer = TextBuffer::new(text);
        // text_buffer.align = Align::Center;
        Button {
            id: crate::gen_unique_id(),
            text_buffer: TextBuffer::new(text).with_align(Align::Center),
            padding,
            geometry: Geometry::new(),
            callback: None,
            inner_callback: None,
            image: None,
            hovered: false,
            changed: false,
            fill_render: RenderParam::new(RectParam::new(Rect::new(), ClickStyle::new())),
        }
    }

    pub fn image_and_text(source: impl Into<ImageSource>, text: impl Into<RichText>) -> Self {
        let mut res = Button::new(text);
        res.image = Some(Image::new(source));
        res
    }

    pub(crate) fn reset_size(&mut self, ui: &mut Ui) {
        self.text_buffer.geometry.add_fix_width(-self.padding.horizontal());
        self.text_buffer.geometry.add_fix_height(-self.padding.vertical());
        // self.text_buffer.size_mode = self.size_mode;
        self.text_buffer.init(ui);
        self.geometry.set_size(self.text_buffer.geometry.width(), self.text_buffer.geometry.height());
        // let (w, h) = self.size_mode.size(self.text_buffer.rect.width(), self.text_buffer.rect.height());
        self.fill_render.param.rect.set_size(self.geometry.width(), self.geometry.height());
        if let Some(ref mut image) = self.image {
            let ih = self.fill_render.param.rect.height() - self.padding.vertical() - 2.0;
            image.set_size(ih, ih);
            self.text_buffer.set_width(self.text_buffer.geometry.width() - ih);
            self.text_buffer.init(ui);
        }
    }


    pub fn set_width(&mut self, width: f32) {
        self.geometry.set_fix_width(width);
        self.text_buffer.geometry.set_fix_width(width);
        // self.size_mode.fix_width(width);
    }

    pub fn set_height(&mut self, height: f32) {
        self.geometry.set_fix_height(height);
        self.text_buffer.geometry.set_fix_height(height);
        // self.size_mode.fix_height(height);
    }


    pub fn set_size(&mut self, width: f32, height: f32) {
        self.set_width(width);
        self.set_height(height);
        // self.geometry.set_fix_size(width, height);
        // self.size_mode = SizeMode::Fix(width, height);
    }

    ///仅作用于draw
    pub fn width(mut self, w: f32) -> Self {
        self.set_width(w);
        self
    }

    ///仅作用于draw
    pub fn align(mut self, align: Align) -> Self {
        self.text_buffer.align = align;
        self
    }
    ///仅作用于draw
    pub fn height(mut self, h: f32) -> Self {
        self.set_height(h);
        self
    }
    ///仅作用于draw
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
    pub fn set_image(&mut self, source: impl Into<ImageSource>) {
        match self.image {
            None => self.image = Some(Image::new(source)),
            Some(ref mut image) => image.set_image(source)
        }
    }


    pub fn set_style(&mut self, style: ClickStyle) {
        self.fill_render.param.style = style;
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            // self.fill_render.param.rect = ui.layout().available_rect().clone_with_size(&self.fill_render.param.rect);
            self.reset_size(ui);
        }
        //按钮矩形
        self.fill_render.init_rectangle(ui, false, false);

        if let Some(ref mut image) = self.image {
            image.update(ui);
            // image.rect = self.image_rect.clone();
            // image.changed = true
        }
        //按钮文本
        // self.text_buffer.draw(ui);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.param.rect.offset_to_rect(&ui.draw_rect);
            self.fill_render.update(ui, self.hovered, ui.device.device_input.mouse.pressed);
            self.text_buffer.geometry.set_pos(ui.draw_rect.dx().min, ui.draw_rect.dy().min);
            // self.text_buffer.rect.offset_to_rect(&ui.draw_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            self.fill_render.update(ui, self.hovered, ui.device.device_input.mouse.pressed);
            self.text_buffer.update_buffer(ui);
        }
        // if !self.changed && !ui.can_offset { return; }
        // self.changed = false;
        // if ui.can_offset {
        //     self.fill_render.param.rect.offset(&ui.offset);
        //     self.text_buffer.rect.offset(&ui.offset);
        // }
        // self.fill_render.update(ui, self.hovered, ui.device.device_input.mouse.pressed);
    }
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        match self.image {
            None => self.text_buffer.redraw(ui),
            Some(ref mut image) => {
                if ui.widget_changed.contains(WidgetChange::Position) {
                    let mut text_rect = ui.draw_rect.clone();
                    text_rect.add_min_x(image.rect.width());
                    self.text_buffer.geometry.set_pos(ui.draw_rect.dx().min + self.padding.left + image.rect.width(), ui.draw_rect.dy().min + self.padding.top);
                    // self.text_buffer.rect.offset_to_rect(&text_rect);
                    self.text_buffer.redraw(ui);
                }
                let mut image_rect = ui.draw_rect.clone_with_size(&image.rect);
                image_rect.add_min_x(self.padding.left + 1.0);
                image_rect.add_min_y(self.padding.top + 1.0);
                ui.draw_rect = image_rect;
                image.redraw(ui);
            }
        }
        if let Some(ref mut image) = self.image { image.redraw(ui); }
    }
}


impl Widget for Button {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        if let Some(ref mut image) = self.image {
            image.update(ui);
        }
        match &mut ui.update_type {
            UpdateType::Draw => self.redraw(ui),
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
                if ui.device.device_input.pressed_at(&self.fill_render.param.rect) {
                    self.hovered = true;
                    self.changed = true;
                    ui.context.window.request_redraw();
                }
                // if !ui.device.device_input.pressed_at(&self.fill_render.param.rect) { return Response::new(&self.id, &self.fill_render.param.rect); }

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
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.fill_render.param.rect.width(), self.fill_render.param.rect.height()))
    }
}