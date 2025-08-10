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
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::{Callback, Response};
use crate::style::color::Color;

pub struct ComboBox {
    pub(crate) id: String,
    popup_id: String,
    size_mode: SizeMode,
    text_buffer: TextBuffer,
    data: Vec<String>,
    item_style: ClickStyle,
    popup_rect: Rect,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui, usize)>>,

    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,

}

impl ComboBox {
    pub fn new<T: Display>(data: Vec<T>) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(230, 230, 230);
        fill_style.border.inactive = Border::new(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        let data = data.iter().map(|x| x.to_string()).collect();
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
        // self.popup.set_rect(popup_rect);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.fill_param.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
        self
    }

    /// 设置popup的高度
    pub fn with_popup_height(mut self, height: f32) -> Self {
        self.popup_rect.set_height(height);
        // self.popup.rect_mut().set_height(height);
        self
    }

    fn add_item(&self, ui: &mut Ui, item: &String) {
        // ui.style.widget.click = self.item_style.clone();
        let mut btn = Button::new(item).padding(Padding::same(3.0)).with_style(self.item_style.clone());
        btn.set_size(ui.layout().available_rect().width(), 25.0);
        ui.add(btn);
        // let task = PaintButton::new(ui, &mut btn);
        // popup.layout.widgets.insert(btn.id.clone(), PaintTask::Button(task));
    }

    fn add_items(&self, ui: &mut Ui, popup: &mut Popup) {
        let previous_layout = ui.layout.replace(popup.layout.take().unwrap()).unwrap();
        // let style = ui.style.widget.click.clone();
        for datum in self.data.iter() {
            self.add_item(ui, datum);
        }
        popup.layout = ui.layout.replace(previous_layout);
        // ui.style.widget.click = style;
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, usize)) -> Self {
        self.callback = Some(Callback::create_combobox(f));
        self
    }
}


impl Widget for ComboBox {
    fn draw(&mut self, ui: &mut Ui) -> Response {
        //分配大小
        self.fill_param.rect = ui.layout().available_rect().clone_with_size(&self.fill_param.rect);
        self.reset_size(&ui.context);
        // ui.layout().alloc_rect(&self.fill_param.rect);
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
        self.add_items(ui, &mut popup);
        self.popup_id = popup.id.to_string();
        ui.popups.as_mut().unwrap().insert(popup.id.clone(), popup);
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
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        self.text_buffer.redraw(ui);
    }
}