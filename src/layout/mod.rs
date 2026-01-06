pub mod popup;
pub mod horizontal;
pub mod vertical;
pub mod recycle;

use crate::map::Map;
use crate::response::Response;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetKind, WidgetSize};
use crate::Offset;
use std::any::Any;
use std::ops::{Deref, DerefMut};

pub trait Layout: Any {
    ///更新布局
    fn update(&mut self, ui: &mut Ui) -> Response<'_>;
    ///获取布局下所有的item
    fn items(&self) -> &Map<String, LayoutItem>;
    ///获取布局下所有的可变item
    fn items_mut(&mut self) -> &mut Map<String, LayoutItem>;
    ///添加item到布局内
    fn add_item(&mut self, item: LayoutItem);
    ///设置布局的位移
    fn set_offset(&mut self, offset: Offset);
    ///设置布局的大小
    fn set_size(&mut self, w: f32, h: f32);
}

///布局Item，包含布局和控件
pub enum LayoutItem {
    Layout(LayoutKind),
    Widget(WidgetKind),
}

impl LayoutItem {
    pub fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match self {
            LayoutItem::Layout(layout) => layout.update(ui),
            LayoutItem::Widget(widget) => widget.update(ui),
        }
    }

    pub fn widget_mut<T: Widget>(&mut self) -> Option<&mut T> {
        match self {
            LayoutItem::Layout(_) => None,
            LayoutItem::Widget(widget) => widget.as_mut_()
        }
    }

    pub fn widget<T: Widget>(&self) -> Option<&T> {
        match self {
            LayoutItem::Layout(_) => None,
            LayoutItem::Widget(widget) => widget.as_()
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            LayoutItem::Layout(v) => v.size.dw,
            LayoutItem::Widget(v) => v.width(),
        }
    }

    pub fn set_width(&mut self, width: f32) {
        match self {
            LayoutItem::Layout(v) => v.size.dw=width,
            LayoutItem::Widget(v) => v.set_width(width)
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            LayoutItem::Layout(v) => v.size.dh,
            LayoutItem::Widget(v) => v.height(),
        }
    }

    pub fn set_height(&mut self,height: f32) {
        match self {
            LayoutItem::Layout(v) => v.size.dh=height,
            LayoutItem::Widget(v) => v.set_height(height),
        }
    }

    pub fn id(&self) -> &str {
        match self {
            LayoutItem::Layout(layout) => &layout.id,
            LayoutItem::Widget(widget) => widget.id()
        }
    }

    pub fn is_space(&self) -> bool {
        match self {
            LayoutItem::Layout(_) => false,
            LayoutItem::Widget(widget) => widget.is_space()
        }
    }
}

pub struct LayoutKind {
    layout: Box<dyn Layout>,
    id: String,
    size: WidgetSize,
}

impl LayoutKind {
    pub fn new(layout: impl Layout + 'static) -> Self {
        LayoutKind {
            layout: Box::new(layout),
            id: crate::gen_unique_id(),
            size: WidgetSize {
                dw: 0.0,
                dh: 0.0,
                rw: 0.0,
                rh: 0.0,
            },
        }
    }
    pub fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        let resp = self.layout.update(ui);
        if resp.size != self.size {
            self.id = resp.id.to_string();
            self.size = resp.size.clone();
        }
        resp
    }

    pub fn as_mut_<T: Layout>(&mut self) -> Option<&mut T> {
        let layout = self.layout.deref_mut() as &mut dyn Any;
        layout.downcast_mut()
    }

    pub fn as_<T: Layout>(&self) -> Option<&T> {
        let layout = self.layout.deref() as &dyn Any;
        layout.downcast_ref()
    }

    pub fn set_offset(&mut self, offset: Offset) {
        self.layout.set_offset(offset);
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        self.layout.set_size(w, h);
    }

    pub fn add_item(&mut self, item: LayoutItem) {
        self.layout.add_item(item);
    }

    pub fn get_item_mut(&mut self, id: &String) -> Option<&mut LayoutItem> {
        self.layout.items_mut().get_mut(id)
    }

    pub fn items(&self) -> &Map<String, LayoutItem> {
        self.layout.items()
    }

    pub fn get_widget<W: Widget>(&mut self, id: &String) -> Option<&mut W> {
        for (wid, item) in self.layout.items_mut().entry_mut() {
            match item {
                LayoutItem::Layout(layout) => {
                    let widget = layout.get_widget(id);
                    if widget.is_some() { return widget; }
                }
                LayoutItem::Widget(widget) => if wid == id { return widget.as_mut_() }
            }
        }
        None
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Clone, Debug, Copy)]
pub enum LayoutDirection {
    Min,
    Max,
}

enum OffsetDirection {
    TopDown,
    DownTop,
}

struct LayoutOffset {
    previous: Offset,
    current: Offset,
    context: Offset,
    direction: OffsetDirection,
    offsetting: bool,
}

impl LayoutOffset {
    fn new() -> LayoutOffset {
        LayoutOffset {
            previous: Offset::new(),
            current: Offset::new(),
            context: Offset::new(),
            direction: OffsetDirection::TopDown,
            offsetting: true,
        }
    }

    fn next_offset(&mut self, offset: Offset) {
        self.previous = self.current.clone();
        self.current = offset;
        if self.current.y > self.previous.y { self.direction = OffsetDirection::TopDown; }
        if self.current.y < self.previous.y { self.direction = OffsetDirection::DownTop; }
        self.offsetting = true;
    }
}