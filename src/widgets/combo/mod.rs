pub mod check;

use crate::frame::context::UpdateType;
use crate::frame::App;
use crate::layout::popup::Popup;
use crate::render::rectangle::param::RectParam;
use crate::render::triangle::param::TriangleParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::select::SelectItem;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::{Align, FillStyle, Offset};
use std::fmt::Display;
use std::sync::{Arc, RwLock};

/// ### ComboBox的示例用法
///```
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
    text_buffer: TextBuffer,
    data: Vec<T>,
    popup_rect: Rect,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, &T)>>,

    fill_render: RenderParam,
    allow_render: RenderParam,

    selected: Arc<RwLock<Option<String>>>,

    previous_select: Option<String>,
    state: WidgetState,
}

impl<T: Display + 'static> ComboBox<T> {
    pub fn new(data: Vec<T>) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(230, 230, 230);
        fill_style.border.inactive = Border::same(0.0).radius(Radius::same(3));
        fill_style.border.hovered = Border::same(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        fill_style.border.clicked = Border::same(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        let mut allow_style = ClickStyle::new();
        allow_style.fill = FillStyle::same(Color::BLACK);
        let fill_param = RectParam::new().with_size(100.0, 20.0).with_style(fill_style);
        let allow_param = TriangleParam::new((0.0, 0.0).into(), (10.0, 0.0).into(), (5.0, 8.0).into(), allow_style);
        ComboBox {
            id: crate::gen_unique_id(),
            popup_id: "".to_string(),
            text_buffer: TextBuffer::new("".to_string()).with_align(Align::LeftCenter).padding(Padding::same(2.0)),
            data,
            popup_rect: Rect::new().with_size(100.0, 150.0),
            callback: None,
            fill_render: RenderParam::new(RenderKind::Rectangle(fill_param)),
            previous_select: None,
            selected: Arc::new(RwLock::new(None)),
            allow_render: RenderParam::new(RenderKind::Triangle(allow_param)),
            state: WidgetState::default(),
        }
    }

    fn reset_size(&mut self, ui: &mut Ui) {
        self.text_buffer.geometry.set_min_width(100.0);
        self.text_buffer.geometry.set_max_width(100.0);
        self.text_buffer.init(ui);
        self.fill_render.rect_mut().set_size(self.text_buffer.geometry.width(), self.text_buffer.geometry.height());
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.text_buffer.geometry.set_fix_size(width, height);
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
        select.geometry().set_fix_size(self.popup_rect.width() - 10.0, 25.0);
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
        //计算大小
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
        fill_style.border.inactive = Border::same(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        #[cfg(feature = "gpu")]
        self.fill_render.init(ui, false, false);
        #[cfg(feature = "gpu")]
        self.allow_render.init(ui, false, false);
        //文本
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
                if let Some(ref mut callback) = self.callback {
                    let app = ui.app.take().unwrap();
                    let t = self.data.iter().find(|x| &x.to_string() == select).unwrap();
                    callback(app, ui, t);
                    ui.app.replace(app);
                    ui.context.window.request_redraw();
                }
                self.state.changed = true;
            }

            let popup = &mut ui.popups.as_mut().unwrap()[&self.popup_id];
            popup.request_state(false);
            self.state.on_release();
        }
        if self.state.changed { ui.widget_changed |= WidgetChange::Value; }
        self.state.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.rect_mut().offset_to_rect(&ui.draw_rect);
            self.popup_rect.offset_to_rect(&ui.draw_rect);
            self.popup_rect.offset_y(&Offset::new().covered().with_y(self.fill_render.rect_mut().height() + 5.0));
            ui.popups.as_mut().unwrap()[&self.popup_id].set_rect(self.popup_rect.clone());
            let mut allow_rect = ui.draw_rect.clone();
            allow_rect.set_x_min(allow_rect.dx().min + self.fill_render.rect().width() - 15.0);
            allow_rect.add_min_y(5.0);
            self.allow_render.offset_to_rect(&allow_rect);
            let mut text_rect = self.fill_render.rect_mut().clone();
            text_rect.add_min_x(2.0);
            self.text_buffer.geometry.offset_to_rect(&text_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            self.text_buffer.update_buffer(ui);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.fill_render.draw(ui, self.state.hovered || self.state.focused, self.state.focused);
        self.allow_render.draw(ui, false, false);
        self.text_buffer.redraw(ui);
    }

    ///初始化时设置当前item，默认为None
    pub fn with_current_index(mut self, index: usize) -> Self {
        let current = self.data[index].to_string();
        self.text_buffer.set_text(current.clone());
        *self.selected.write().unwrap() = Some(current);
        self
    }
}


impl<T: Display + 'static> Widget for ComboBox<T> {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(self.fill_render.rect());
                if self.state.on_hovered(hovered) { ui.context.window.request_redraw(); }
            }
            UpdateType::MousePress => {
                let pressed = ui.device.device_input.pressed_at(self.fill_render.rect());
                if self.state.on_pressed(pressed) { ui.context.window.request_redraw(); }
            }
            UpdateType::MouseRelease => {
                let clicked = ui.device.device_input.click_at(self.fill_render.rect());
                if self.state.on_clicked(clicked) {
                    self.state.focused = true;
                    let popup = &mut ui.popups.as_mut().unwrap()[&self.popup_id];
                    popup.toggle();
                    ui.update_type = UpdateType::None;
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.fill_render.rect().width(), self.fill_render.rect().height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.text_buffer.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}