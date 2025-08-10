//!### ComboBox的示例用法
//!
//! ```
//! use xlui::widgets::combobox::ComboBox;
//! use xlui::widgets::Widget;
//! # xlui::_run_test(|ui|{
//!    ComboBox::new(vec![1,2,3,4]).with_popup_height(150.0).draw(ui);
//! # });
//! ```

use std::any::Any;
use std::cell::RefCell;
use crate::frame::context::Context;
use crate::layout::popup::Popup;
use crate::radius::Radius;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::button::Button;
use crate::widgets::Widget;
use std::fmt::Display;
use std::rc::Rc;
use glyphon::Shaping;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::{Callback, Response};
use crate::style::color::Color;

pub struct ComboBox<T> {
    pub(crate) id: String,
    popup_id: String,
    size_mode: SizeMode,
    text_buffer: TextBuffer,
    data: Vec<T>,
    item_style: ClickStyle,
    popup_rect: Rect,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui, &T)>>,

    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,

    previous_select: usize,
    selected: Rc<RefCell<usize>>,
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
            item_style: ClickStyle {
                fill: FillStyle {
                    inactive: Color::TRANSPARENT,
                    hovered: Color::rgba(153, 193, 241, 220),
                    clicked: Color::rgba(153, 193, 241, 220),
                },
                border: BorderStyle {
                    inactive: Border::new(0.0),
                    hovered: Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2)),
                    clicked: Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2)),
                },
            },
            popup_rect: Rect::new(),
            callback: None,
            fill_param: RectParam::new(Rect::new(), fill_style),
            fill_index: 0,
            fill_buffer: None,
            previous_select: 0,
            selected: Rc::new(RefCell::new(0)),
        }
    }

    fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(context);
        match self.size_mode {
            SizeMode::Auto => self.fill_param.rect.set_size(100.0, 20.0),
            SizeMode::FixWidth => self.fill_param.rect.set_height(20.0),
            SizeMode::FixHeight => self.fill_param.rect.set_width(100.0),
            SizeMode::Fix => {}
        }
        self.text_buffer.rect = self.fill_param.rect.clone_add_padding(&Padding::same(2.0));
        self.popup_rect = self.fill_param.rect.clone_with_size(&self.popup_rect);
        self.popup_rect.set_width(self.fill_param.rect.width());
        self.popup_rect.offset_y(self.fill_param.rect.height() + 5.0);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.fill_param.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
        self
    }

    /// 设置popup的高度
    pub fn with_popup_height(mut self, height: f32) -> Self {
        self.popup_rect.set_height(height);
        self
    }

    fn add_item(&self, ui: &mut Ui, item: &T, row: usize) {
        let mut btn = Button::new(item).padding(Padding::same(3.0)).with_style(self.item_style.clone());
        btn.set_size(ui.layout().available_rect().width(), 25.0);
        let state = self.selected.clone();
        btn.set_callback2(move || {
            println!("{}", row);
            *state.borrow_mut() = row;
        });
        ui.add(btn);
    }

    fn add_items(&self, ui: &mut Ui) {
        for (index, datum) in self.data.iter().enumerate() {
            self.add_item(ui, datum, index);
        }
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, &T)) -> Self {
        self.callback = Some(Callback::create_combobox(f));
        self
    }
}


impl<T: Display + 'static> Widget for ComboBox<T> {
    fn draw(&mut self, ui: &mut Ui) -> Response {
        //分配大小
        self.fill_param.rect = ui.layout().available_rect().clone_with_size(&self.fill_param.rect);
        self.reset_size(&ui.context);
        //背景
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(230, 230, 230);
        fill_style.border.inactive = Border::new(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        let data = self.fill_param.as_draw_param(false, false);
        let fill_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        self.fill_buffer = Some(fill_buffer);
        //文本
        self.text_buffer.draw(ui);

        //下拉框布局
        let mut popup = Popup::new(ui, self.popup_rect.clone());
        self.popup_id = popup.id.clone();
        popup.show(ui, |ui| self.add_items(ui));
        Response {
            id: self.id.clone(),
            rect: self.fill_param.rect.clone(),
        }
    }

    fn update(&mut self, ui: &mut Ui) {
        if ui.device.device_input.click_at(&self.fill_param.rect) {
            let popup = &mut ui.popups.as_mut().unwrap()[&self.popup_id];
            popup.open = !popup.open;
            ui.context.window.request_redraw();
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        if *self.selected.borrow() != self.previous_select && ui.popups.as_mut().unwrap()[&self.popup_id].open {
            self.previous_select = *self.selected.borrow();
            self.text_buffer.set_text(self.data[*self.selected.borrow()].to_string());
            self.text_buffer.buffer.as_mut().unwrap().set_text(
                &mut ui.context.render.text.font_system,
                self.data[*self.selected.borrow()].to_string().as_str(),
                &ui.context.font.font_attr(),
                Shaping::Advanced,
            );
            let popup = &mut ui.popups.as_mut().unwrap()[&self.popup_id];
            popup.open = false;
            if let Some(ref mut callback) = self.callback {
                let app = ui.app.take().unwrap();
                callback(*app, ui, &self.data[*self.selected.borrow()]);
                ui.app.replace(app);
                ui.context.window.request_redraw();
            }
        }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        self.text_buffer.redraw(ui);
    }
}