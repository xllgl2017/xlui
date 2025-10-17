use crate::align::Align;
use crate::frame::context::UpdateType;
use crate::frame::App;
use crate::render::image::ImageSource;
use crate::render::{Visual, VisualStyle};
use crate::response::{Callback, Response};
use crate::size::padding::Padding;
use crate::size::Geometry;
use crate::text::buffer::TextBuffer;
use crate::text::rich::RichText;
use crate::ui::Ui;
use crate::widgets::image::Image;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::Color;

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
///    let style=VisualStyle::new();
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
///    btn.geometry().set_context_size(30.0,30.0);
///    ui.add(btn);
///    //图片按钮
///    let image_btn=Button::image_and_text("logo.jpg","点击");
///    ui.add(image_btn);
/// }
/// ```
pub struct Button {
    pub(crate) id: String,
    pub(crate) text_buffer: TextBuffer,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Button, &mut Ui)>>,
    inner_callback: Option<Box<dyn FnMut()>>,
    visual: Visual,
    image: Option<Image>,
    state: WidgetState,
    geometry: Geometry,
}


impl Button {
    pub fn new(text: impl Into<RichText>) -> Self {
        let mut style = VisualStyle::same((Color::rgb(230, 230, 230), 1.0, 3).into());
        style.inactive.border.set_same(0.0);
        style.pressed.fill = Color::rgb(165, 165, 165);

        Button {
            id: crate::gen_unique_id(),
            text_buffer: TextBuffer::new(text),
            callback: None,
            inner_callback: None,
            image: None,
            visual: Visual::new().with_enable().with_style(style),
            state: WidgetState::default(),
            geometry: Geometry::new().with_padding(Padding::same(2.0)).with_align(Align::Center),
        }
    }

    pub fn image_and_text(source: impl Into<ImageSource>, text: impl Into<RichText>) -> Self {
        let mut res = Button::new(text);
        res.image = Some(Image::new(source));
        res
    }


    ///仅作用于draw
    pub fn width(mut self, w: f32) -> Self {
        self.geometry.set_fix_width(w);
        self
    }
    ///仅作用于draw
    pub fn align(mut self, align: Align) -> Self {
        self.geometry.set_align(align);
        self
    }
    ///仅作用于draw
    pub fn height(mut self, h: f32) -> Self {
        self.geometry.set_fix_height(h);
        self
    }
    ///仅作用于draw
    pub fn padding(mut self, padding: Padding) -> Self {
        self.geometry.set_padding(padding);
        self
    }

    pub fn enable(&mut self) {
        self.state.disabled = false;
    }

    pub fn disable(&mut self) {
        self.state.disabled = true;
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

    pub fn with_style(mut self, style: impl Into<VisualStyle>) -> Self {
        self.visual.set_style(style.into());
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


    pub fn set_style(&mut self, style: VisualStyle) {
        self.visual.set_style(style);
    }

    pub(crate) fn reset_size(&mut self, ui: &mut Ui) {
        self.text_buffer.init(ui);
        match self.image {
            None => self.geometry.set_context_size(self.text_buffer.geometry.margin_width(), self.text_buffer.geometry.margin_height()),
            Some(ref mut image) => {
                let ih = self.geometry.context_height();
                image.geometry().set_fix_size(ih, ih);
                let width = self.text_buffer.geometry.margin_width() + 2.0 + ih;
                let height = self.text_buffer.geometry.margin_height();
                self.geometry.set_context_size(width, height);
                self.text_buffer.geometry.set_fix_size(self.geometry.context_width() - image.geometry().context_width(), self.geometry.context_height());
                self.text_buffer.geometry.set_align(Align::Center);
            }
        }
        self.visual.rect_mut().set_size(self.geometry.padding_width(), self.geometry.padding_height());
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.reset_size(ui);
        }
        if let Some(ref mut image) = self.image {
            image.update(ui);
        }
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.state.changed { ui.widget_changed |= WidgetChange::Value; }
        self.state.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.visual.rect_mut().offset_to_rect(&ui.draw_rect);
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.text_buffer.geometry.offset_to_rect(&self.geometry.context_rect());
        }
        if ui.widget_changed.contains(WidgetChange::Value) {
            self.text_buffer.update_buffer(ui);
        }
    }
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.visual.draw(ui, self.state.disabled, self.state.hovered, self.state.pressed, false);
        match self.image {
            None => self.text_buffer.redraw(ui),
            Some(ref mut image) => {
                if ui.widget_changed.contains(WidgetChange::Position) {
                    let mut text_rect = ui.draw_rect.clone();
                    text_rect.add_min_x(image.geometry().margin_width() + 2.0);
                    println!("{} {} {}", self.text_buffer.geometry.context_width(), self.text_buffer.geometry.context_left(), self.text_buffer.geometry.padding_left());
                    self.text_buffer.geometry.offset_to_rect(&text_rect);
                    self.text_buffer.redraw(ui);
                }
                let mut image_rect = ui.draw_rect.clone_with_size(&image.geometry().context_rect());
                image_rect.add_min_x(self.geometry.padding().left);
                image_rect.add_min_y(self.geometry.padding().top);
                ui.draw_rect = image_rect;
                image.redraw(ui);
            }
        }
        if let Some(ref mut image) = self.image { image.redraw(ui); }
        self.visual.draw(ui, self.state.disabled, self.state.hovered, self.state.pressed, true);
    }
}


impl Widget for Button {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match &mut ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(self.visual.rect());
                if self.state.on_hovered(hovered) { ui.context.window.request_redraw(); }
            }
            UpdateType::MousePress => {
                let pressed = ui.device.device_input.pressed_at(self.visual.rect());
                if self.state.on_pressed(pressed) { ui.context.window.request_redraw(); }
            }
            UpdateType::MouseRelease => {
                let clicked = ui.device.device_input.click_at(self.visual.rect());
                if self.state.on_clicked(clicked) {
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
        Response::new(&self.id, WidgetSize::same(self.visual.rect().width(), self.visual.rect().height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.text_buffer.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}