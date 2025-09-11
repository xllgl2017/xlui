use crate::frame::context::UpdateType;
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
use crate::text::buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::select::SelectItem;
use crate::widgets::{Widget, WidgetChange, WidgetSize};
use std::fmt::Display;
use std::sync::{Arc, RwLock};
use crate::{Offset, Pos};
use crate::align::Align;

/// ### ComboBox的示例用法
///```
/// use std::fmt::Display;
/// use xlui::frame::App;
/// use xlui::ui::Ui;
/// use xlui::*;
///
/// fn combo_changed<A:App>(_:&mut A,_:&mut Ui,t:&i32){
///    println!("ComboBox的Item改变了：{}",t);
/// }
///
/// fn draw<A:App>(ui:&mut Ui){
///    //这里的data可以是任意实现了Display的类型
///    let data=vec![1,2,3,4];
///    let combo=ComboBox::new(data)
///        //设置打开的弹窗布局的高度
///        .with_popup_height(150.0)
///        //连接到Item改变的监听函数
///        .connect(combo_changed::<A>);
///    ui.add(combo);
/// }
/// ```
pub struct ComboBox<T> {
    pub(crate) id: String,
    popup_id: String,
    size_mode: SizeMode,
    text_buffer: TextBuffer,
    data: Vec<T>,
    popup_rect: Rect,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, &T)>>,

    fill_render: RenderParam<RectParam>,

    selected: Arc<RwLock<Option<String>>>,

    previous_select: Option<String>,
    changed: bool,
}

impl<T: Display + 'static> ComboBox<T> {
    pub fn new(data: Vec<T>) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(230, 230, 230);
        fill_style.border.inactive = Border::new(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        ComboBox {
            id: crate::gen_unique_id(),
            popup_id: "".to_string(),
            size_mode: SizeMode::Fix(100.0, 20.0),
            text_buffer: TextBuffer::new("123456".to_string()).with_align(Align::LeftCenter),
            data,
            popup_rect: Rect::new().with_size(100.0, 150.0),
            callback: None,
            fill_render: RenderParam::new(RectParam::new(Rect::new().with_size(100.0, 20.0), fill_style)),
            previous_select: None,
            selected: Arc::new(RwLock::new(None)),

            changed: false,
        }
    }

    fn reset_size(&mut self, ui: &mut Ui) {
        self.text_buffer.size_mode = self.size_mode.clone();
        self.text_buffer.init(ui);
        let (w, h) = self.size_mode.size(self.fill_render.param.rect.width(), self.fill_render.param.rect.height());
        // match self.size_mode {
        //     SizeMode::Auto => self.fill_render.param.rect.set_size(100.0, 20.0),
        //     SizeMode::FixWidth => self.fill_render.param.rect.set_height(20.0),
        //     SizeMode::FixHeight => self.fill_render.param.rect.set_width(100.0),
        //     SizeMode::Fix => {}
        // }
        self.fill_render.param.rect.set_size(w, h);
        // self.text_buffer.rect = self.fill_render.param.rect.clone_add_padding(&Padding::same(2.0));
        // self.popup_rect = self.fill_render.param.rect.clone_with_size(&self.popup_rect);
        // self.popup_rect.set_width(self.fill_render.param.rect.width());
        // self.popup_rect.add_min_y(self.fill_render.param.rect.height() + 5.0);
        // self.popup_rect.add_max_y(self.fill_render.param.rect.height() + 5.0);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        // self.fill_render.param.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix(width, height);
        self
    }

    // 设置popup的高度
    pub fn with_popup_height(mut self, height: f32) -> Self {
        self.popup_rect.set_height(height);
        self
    }

    fn add_item(&self, ui: &mut Ui, item: &T) {
        let mut select = SelectItem::new(item.to_string()).padding(Padding::same(3.0))
            .contact(self.selected.clone()).align(Align::LeftCenter);
        select.set_size(self.popup_rect.width() - 10.0, 25.0);
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
        // self.fill_render.param.rect = ui.layout().available_rect().clone_with_size(&self.fill_render.param.rect);
        self.reset_size(ui);
        //下拉框布局
        let popup = Popup::new(ui, self.popup_rect.width(), self.popup_rect.height());
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
        //文本
        // self.text_buffer.rect = self.fill_render.param.rect.clone_add_padding(&Padding::same(2.0));
        // self.text_buffer.size_mode = SizeMode::Fix;
        self.text_buffer.init(ui);
    }

    pub fn parent(&self) -> Arc<RwLock<Option<String>>> {
        self.selected.clone()
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        let select = self.selected.read().unwrap();
        if *select != self.previous_select {
            self.previous_select = select.clone();
            if let Some(ref select) = self.previous_select {
                self.text_buffer.update_buffer_text(ui, select);
                // self.text_buffer.set_text(select.to_string());
                if let Some(ref mut callback) = self.callback {
                    let app = ui.app.take().unwrap();
                    let t = self.data.iter().find(|x| &x.to_string() == select).unwrap();
                    callback(app, ui, t);
                    ui.app.replace(app);
                    ui.context.window.request_redraw();
                }
                self.changed = true;
            }

            let popup = &mut ui.popups.as_mut().unwrap()[&self.popup_id];
            popup.open = false;
        }
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.param.rect.offset_to_rect(&ui.draw_rect);
            self.fill_render.update(ui, false, false);
            self.popup_rect.offset_to_rect(&ui.draw_rect);
            self.popup_rect.offset_y(&Offset::new(Pos::new()).delete_offset().with_y(self.fill_render.param.rect.height() + 5.0));
            ui.popups.as_mut().unwrap()[&self.popup_id].set_rect(self.popup_rect.clone());
            let mut text_rect = self.fill_render.param.rect.clone();
            text_rect.add_min_x(2.0);
            self.text_buffer.rect = text_rect;
            // self.text_buffer.rect.add_min_x(2.0);
            // println!("{:?}", self.text_buffer.rect);
            // self.popup_rect = self.fill_render.param.rect.clone_with_size(&self.popup_rect);
            // self.popup_rect.set_width(self.fill_render.param.rect.width());
            // self.popup_rect.add_min_y(self.fill_render.param.rect.height() + 5.0);
            // self.popup_rect.add_max_y(self.fill_render.param.rect.height() + 5.0);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            self.text_buffer.update_buffer(ui);
        }
        // if !self.changed && !ui.can_offset { return; }

    }
}


impl<T: Display + 'static> Widget for ComboBox<T> {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.text_buffer.redraw(ui);
    }

    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
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
        Response::new(&self.id, WidgetSize::same(self.fill_render.param.rect.width(), self.fill_render.param.rect.height()))
    }
}