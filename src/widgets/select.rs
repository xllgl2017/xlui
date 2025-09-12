use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize};
use std::fmt::Display;
use std::sync::{Arc, RwLock};
use crate::align::Align;

/// ### SelectItem的示例用法
/// ```
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///     //快速创建一个SelectItem
///     let contact=ui.select_value(1).need_contact();
///     let mut item=SelectItem::new(2)
///         //关联选择
///         .contact(contact.clone())
///         //设置控件内部padding
///         .padding(Padding::same(5.0));
///
///         //设置控件大小
///     item.set_size(30.0,30.0);
///     ui.add(item);
///
/// }
/// ```
pub struct SelectItem<T> {
    pub(crate) id: String,
    text: TextBuffer,
    padding: Padding,
    size_mode: SizeMode,
    value: T,
    parent_selected: Arc<RwLock<Option<String>>>,
    fill_render: RenderParam<RectParam>,

    callback: Option<Box<dyn FnMut(&mut Option<T>)>>,
    hovered: bool,
    selected: bool,
    changed: bool,
}

impl<T: Display> SelectItem<T> {
    pub fn new(value: T) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::TRANSPARENT;
        fill_style.fill.hovered = Color::rgba(153, 193, 241, 220);
        fill_style.fill.clicked = Color::rgba(153, 193, 241, 220);
        fill_style.border.inactive = Border::new(0.0);
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        SelectItem {
            id: crate::gen_unique_id(),
            text: TextBuffer::new(value.to_string()).with_align(Align::Center),
            padding: Padding::same(2.0),
            size_mode: SizeMode::Auto,
            value,
            parent_selected: Arc::new(RwLock::new(None)),
            fill_render: RenderParam::new(RectParam::new(Rect::new(), fill_style)),
            callback: None,
            hovered: false,
            selected: false,
            changed: false,
        }
    }

    pub(crate) fn reset_size(&mut self, ui: &mut Ui) {
        self.text.size_mode = self.size_mode.clone();
        self.text.init(ui);
        let (w, h) = self.size_mode.size(self.text.rect.width() + self.padding.horizontal(), self.text.rect.height() + self.padding.vertical());
        self.fill_render.param.rect.set_size(w, h);
        // match self.size_mode {
        //     SizeMode::Auto => {
        //         let width = self.text.rect.width() + self.padding.horizontal();
        //         let height = self.text.rect.height() + self.padding.vertical();
        //         self.fill_render.param.rect.set_size(width, height);
        //     }
        //     SizeMode::FixWidth => self.fill_render.param.rect.set_height(self.text.rect.height()),
        //     SizeMode::FixHeight => self.fill_render.param.rect.set_width(self.text.rect.width()),
        //     SizeMode::Fix => {}
        // }
        // self.text.rect = self.fill_render.param.rect.clone_add_padding(&self.padding);
    }

    pub fn set_size(&mut self, width: f32, height: f32) {
        // self.fill_render.param.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix(width, height);
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.set_size(w, h);
        self
    }

    pub fn connect(mut self, f: impl FnMut(&mut Option<T>) + 'static) -> Self {
        self.callback = Some(Box::new(f));
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn contact(mut self, parent: Arc<RwLock<Option<String>>>) -> Self {
        self.parent_selected = parent;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.text.align = align;
        self
    }

    pub fn need_contact(&self) -> Arc<RwLock<Option<String>>> {
        self.parent_selected.clone()
    }

    fn init(&mut self, ui: &mut Ui) {
        // self.fill_render.param.rect = ui.layout().available_rect().clone_with_size(&self.fill_render.param.rect);
        self.reset_size(ui);
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //背景
        let current = self.parent_selected.read().unwrap();
        let selected = current.as_ref() == Some(&self.value.to_string());
        self.fill_render.init_rectangle(ui, selected, selected);
        //文本
        // self.text.init(ui);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        let current = self.parent_selected.read().unwrap();
        let selected = current.as_ref() == Some(&self.value.to_string());
        if !selected && self.selected {
            self.selected = false;
            self.changed = true
        } else if selected && !self.selected {
            self.selected = true;
            self.changed = true;
        }
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.param.rect.offset_to_rect(&ui.draw_rect);
            self.text.rect.offset_to_rect(&ui.draw_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            let current = self.parent_selected.read().unwrap();
            let selected = current.as_ref() == Some(&self.value.to_string());
            self.fill_render.update(ui, selected || self.hovered, selected || ui.device.device_input.mouse.pressed);
        }


        // if !self.changed && !ui.can_offset { return; }

        // if ui.can_offset {
        //     self.fill_render.param.rect.offset(&ui.offset);
        //     self.text.rect.offset(&ui.offset);
        // }
    }
}

impl<T: PartialEq + Display + 'static> Widget for SelectItem<T> {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.text.redraw(ui);
    }

    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.fill_render.param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.changed = true;
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MousePress => {}
            UpdateType::MouseRelease => {
                let clicked = ui.device.device_input.click_at(&self.fill_render.param.rect);
                if clicked {
                    self.selected = true;
                    self.changed = true;
                    let mut selected = self.parent_selected.write().unwrap();
                    *selected = Some(self.value.to_string());
                    ui.update_type = UpdateType::None;
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MouseWheel => {}
            #[cfg(feature = "winit")]
            UpdateType::KeyRelease(_) => {}
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.fill_render.param.rect.width(), self.fill_render.param.rect.height()))
    }
}