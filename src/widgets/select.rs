use crate::align::Align;
use crate::frame::context::UpdateType;
use crate::render::{Visual, VisualStyle, WidgetStyle};
use crate::response::Response;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::radius::Radius;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::text::buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::Shadow;
use std::fmt::Display;
use std::sync::{Arc, RwLock};

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
///     item.geometry().set_context_size(30.0,30.0);
///     ui.add(item);
///
/// }
/// ```
pub struct SelectItem<T> {
    pub(crate) id: String,
    text: TextBuffer,
    value: T,
    parent_selected: Arc<RwLock<Option<String>>>,
    visual: Visual,

    callback: Option<Box<dyn FnMut(&mut Option<T>)>>,
    state: WidgetState,
}

impl<T: Display> SelectItem<T> {
    pub fn new(value: T) -> Self {
        let mut fill_style = VisualStyle::same(WidgetStyle {
            fill: Color::rgba(153, 193, 241, 220),
            border: Border::same(1.0).color(Color::rgba(144, 209, 255, 255)),
            radius: Radius::same(2),
            shadow: Shadow::new(),
        });
        fill_style.inactive.fill = Color::TRANSPARENT;
        fill_style.inactive.border.set_same(0.0);
        SelectItem {
            id: crate::gen_unique_id(),
            text: TextBuffer::new(value.to_string()).with_align(Align::LeftCenter).padding(Padding::same(2.0)),
            value,
            parent_selected: Arc::new(RwLock::new(None)),
            visual: Visual::new().with_enable().with_style(fill_style),
            callback: None,
            state: WidgetState::default(),
        }
    }

    pub(crate) fn reset_size(&mut self, ui: &mut Ui) {
        self.text.init(ui);
        self.visual.rect_mut().set_size(self.text.geometry.padding_width(), self.text.geometry.padding_height());
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.text.geometry.set_fix_size(w, h);
        self
    }

    pub fn connect(mut self, f: impl FnMut(&mut Option<T>) + 'static) -> Self {
        self.callback = Some(Box::new(f));
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.text.geometry.set_padding(padding);
        self
    }

    pub fn contact(mut self, parent: Arc<RwLock<Option<String>>>) -> Self {
        self.parent_selected = parent;
        self
    }

    pub fn align(mut self, align: Align) -> Self {
        self.text.geometry.set_align(align);
        self
    }

    pub fn need_contact(&self) -> Arc<RwLock<Option<String>>> {
        self.parent_selected.clone()
    }

    fn init(&mut self, ui: &mut Ui) {
        self.reset_size(ui);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        self.state.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.visual.rect_mut().offset_to_rect(&ui.draw_rect);
            self.text.geometry.offset_to_rect(&ui.draw_rect);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let current = self.parent_selected.read().unwrap();
        let selected = current.as_ref() == Some(&self.value.to_string());
        self.visual.draw(ui, self.state.disabled, selected || self.state.hovered, selected, false);
        self.text.redraw(ui);
        self.visual.draw(ui, self.state.disabled, selected || self.state.hovered, selected, true);
    }
}

impl<T: PartialEq + Display + 'static> Widget for SelectItem<T> {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            #[cfg(feature = "gpu")]
            UpdateType::ReInit => self.visual.re_init(),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(self.visual.rect());
                if self.state.on_hovered(hovered) { ui.context.window.request_redraw(); }
            }
            UpdateType::MouseRelease => {
                let clicked = ui.device.device_input.click_at(self.visual.rect());
                if self.state.on_clicked(clicked) {
                    let mut selected = self.parent_selected.write().unwrap();
                    *selected = Some(self.value.to_string());
                    ui.update_type = UpdateType::None;
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.text.geometry.margin_width(), self.text.geometry.margin_height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.text.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}