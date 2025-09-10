// pub mod scroll_area;
// pub mod popup;
pub mod horizontal;
pub mod vertical;

use std::any::Any;
use std::ops::{Deref, DerefMut};
use crate::frame::context::UpdateType;
// use crate::layout::horizontal::HorizontalLayout;
use crate::layout::vertical::VerticalLayout;
use crate::map::Map;
use crate::response::Response;
use crate::size::padding::Padding;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetKind};
use crate::Offset;

pub trait Layout: Any {
    fn update(&mut self, ui: &mut Ui) -> Response<'_>;
    fn items(&self) -> &Map<String, LayoutItem>;
    fn items_mut(&mut self) -> &mut Map<String, LayoutItem>;

    // fn redraw(&mut self, ui: &mut Ui);
}

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

    pub fn widget<T: Widget>(&mut self) -> &mut T {
        match self {
            LayoutItem::Layout(_) => panic!("仅可返回widget"),
            LayoutItem::Widget(widget) => widget.as_mut_()
        }
    }

    pub fn width(&self) -> f32 {
        match self {
            LayoutItem::Layout(v) => v.width,
            LayoutItem::Widget(v) => v.width(),
        }
    }

    pub fn height(&self) -> f32 {
        match self {
            LayoutItem::Layout(v) => v.height,
            LayoutItem::Widget(v) => v.height(),
        }
    }
}

pub struct LayoutKind {
    layout: Box<dyn Layout>,
    id: String,
    width: f32,
    height: f32,
}

impl LayoutKind {
    pub fn new(layout: impl Layout + 'static) -> Self {
        LayoutKind {
            layout: Box::new(layout),
            id: crate::gen_unique_id(),
            width: 0.0,
            height: 0.0,
        }
    }
    pub fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        let resp = self.layout.update(ui);
        if resp.width != self.width || resp.height != self.height {
            self.id = resp.id.to_string();
            self.width = resp.width;
            self.height = resp.height;
        }
        resp
    }

    pub fn layout_mut(&mut self) -> &mut Box<dyn Layout> {
        &mut self.layout
    }

    pub fn as_mut_<T: Layout>(&mut self) -> Option<&mut T> {
        let layout = self.layout.deref_mut() as &mut dyn Any;
        layout.downcast_mut()
    }

    pub fn as_<T: Layout>(&self) -> Option<&T> {
        let layout = self.layout.deref() as &dyn Any;
        layout.downcast_ref()
    }


    // pub fn redraw(&mut self, ui: &mut Ui) {
    //     match self {
    //         LayoutKind::Horizontal(v) => v.redraw(ui),
    //         LayoutKind::Vertical(v) => v.redraw(ui),
    //         LayoutKind::ScrollArea(v) => v.redraw(ui),
    //     }
    // }

    // pub fn add_widget(&mut self, id: String, widget: WidgetKind) {
    //     match self {
    //         LayoutKind::Horizontal(v) => {
    //             if !widget.rect.out_of_rect(&v.max_rect) { v.display.insert(widget.id.clone(), v.widgets.len()); }
    //             v.widgets.insert(id, widget)
    //         }
    //         LayoutKind::Vertical(v) => {
    //             if !widget.rect.out_of_rect(&v.max_rect) { v.display.insert(widget.id.clone(), v.widgets.len()); }
    //             v.widgets.insert(id, widget)
    //         }
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    // }
    // pub fn add_child(&mut self, kind: LayoutKind) {
    //     let id = kind.id().to_string();
    //     match self {
    //         LayoutKind::Horizontal(v) => {
    //             v.alloc_rect(&kind.drawn_rect());
    //             v.children.insert(id, kind)
    //         }
    //         LayoutKind::Vertical(v) => {
    //             v.alloc_rect(&kind.drawn_rect());
    //             v.children.insert(id, kind)
    //         }
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    // }
    // pub fn available_rect(&self) -> Rect {
    //     match self {
    //         LayoutKind::Horizontal(v) => {
    //             let mut max_rect = v.available_rect.clone();
    //             match v.max_rect.x_direction() {
    //                 LayoutDirection::Min => max_rect.set_x_max(v.max_rect.dx().max),
    //                 LayoutDirection::Max => max_rect.set_x_min(v.max_rect.dx().min),
    //             }
    //             max_rect
    //         }
    //         LayoutKind::Vertical(v) => {
    //             let mut max_rect = v.available_rect.clone();
    //             match v.max_rect.y_direction() {
    //                 LayoutDirection::Min => max_rect.set_y_max(v.max_rect.dy().max),
    //                 LayoutDirection::Max => max_rect.set_y_min(v.max_rect.dy().min),
    //             }
    //             max_rect
    //         }
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    // }
    //
    // pub fn alloc_rect(&mut self, rect: &Rect) {
    //     match self {
    //         LayoutKind::Horizontal(v) => v.alloc_rect(rect),
    //         LayoutKind::Vertical(v) => v.alloc_rect(rect),
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    // }

    // pub fn drawn_rect(&self) -> Rect {
    //     match self {
    //         LayoutKind::Horizontal(v) => {
    //             let mut rect = v.max_rect.clone();
    //             rect.set_width(if v.width > v.max_rect.width() { v.max_rect.width() } else { v.width });
    //             rect.set_height(if v.height > v.max_rect.height() { v.max_rect.height() } else { v.height });
    //             rect
    //         }
    //         LayoutKind::Vertical(v) => v.drawn_rect(),
    //         LayoutKind::ScrollArea(v) => v.drawn_rect().clone()
    //     }
    // }
    // pub fn add_space(&mut self, space: f32) {
    //     match self {
    //         LayoutKind::Horizontal(v) => {
    //             v.available_rect.add_min_x(space);
    //             v.width += space;
    //         }
    //         LayoutKind::Vertical(v) => {
    //             v.available_rect.add_min_y(space);
    //             v.height += space;
    //         }
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    // }

    // pub fn get_widget(&mut self, id: &String) -> Option<&mut Box<dyn Widget>> {
    //     match self {
    //         LayoutKind::Horizontal(v) => {
    //             let widget = v.widgets.get_mut(id);
    //             if widget.is_some() { return Some(&mut widget?.widget); }
    //             for child in v.children.iter_mut() {
    //                 let widget = child.get_widget(id);
    //                 if widget.is_some() { return widget; }
    //             }
    //             None
    //         }
    //         LayoutKind::Vertical(v) => {
    //             let widget = v.widgets.get_mut(id);
    //             if widget.is_some() { return Some(&mut widget?.widget); }
    //             for child in v.children.iter_mut() {
    //                 let widget = child.get_widget(id);
    //                 if widget.is_some() { return widget; }
    //             }
    //             None
    //         }
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    // }

    // pub fn get_layout(&mut self, id: &String) -> Option<&mut LayoutKind> {
    //     match self {
    //         LayoutKind::Horizontal(_) | LayoutKind::Vertical(_) => {
    //             for (k, v) in self.children().entry_mut() {
    //                 if k == id { return Some(v); }
    //                 let layout = v.get_layout(id);
    //                 if layout.is_some() { return layout; }
    //             }
    //             None
    //         }
    //         LayoutKind::ScrollArea(v) => if &v.id == id { Some(self) } else { None }
    //     }
    // }

    // fn _remove_widget(layout: &mut LayoutKind, id: &String, ui: &mut Ui) -> Option<WidgetKind> {
    //     if !layout.widgets().has_key(id) { return None; }
    //     let pos = layout.widgets().iter().position(|x| &x.id == id).unwrap();
    //     let rect = &layout.widgets()[id].rect;
    //     let offset_y = rect.height() + layout.item_space();
    //     layout.update_size(-offset_y);
    //     let offset = Offset::new(Pos::new()).with_y(-offset_y).delete_offset();
    //     let ut = UpdateType::Offset(offset);
    //     ui.update_type = ut;
    //     ui.can_offset = true;
    //     for i in pos..layout.widgets().len() {
    //         layout.widgets()[i].update(ui);
    //     }
    //     ui.can_offset = false;
    //     ui.update_type = UpdateType::None;
    //     layout.widgets().remove(&id)
    // }

    // pub fn remove_widget(&mut self, ui: &mut Ui, wid: &String) -> Option<WidgetKind> {
    //     let widget = Self::_remove_widget(self, wid, ui);
    //     if widget.is_some() { return widget; }
    //     for child in self.children().iter_mut() {
    //         let widget = child.remove_widget(ui, wid);
    //         if widget.is_some() { return widget; }
    //     }
    //     None
    // }

    // pub fn set_rect(&mut self, rect: Rect, padding: &Padding) {
    //     match self {
    //         LayoutKind::Horizontal(v) => {
    //             v.max_rect = rect.with_x_direction(v.max_rect.x_direction());
    //             v.available_rect = v.max_rect.clone_add_padding(&padding);
    //             match v.max_rect.x_direction() {
    //                 LayoutDirection::Min => v.available_rect.set_x_max(f32::INFINITY),
    //                 LayoutDirection::Max => v.available_rect.set_x_min(-f32::INFINITY),
    //             }
    //         }
    //         LayoutKind::Vertical(v) => {
    //             v.max_rect = rect;
    //             v.available_rect = v.max_rect.clone_add_padding(&padding);
    //             match v.max_rect.y_direction() {
    //                 LayoutDirection::Min => v.available_rect.set_y_max(f32::INFINITY),
    //                 LayoutDirection::Max => v.available_rect.set_y_min(-f32::INFINITY),
    //             }
    //         }
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    // }

    // fn widgets(&mut self) -> &mut Map<String, WidgetKind> {
    //     match self {
    //         LayoutKind::Horizontal(v) => &mut v.widgets,
    //         LayoutKind::Vertical(v) => &mut v.widgets,
    //         LayoutKind::ScrollArea(v) => {
    //             &mut v.layout.as_mut().unwrap().widgets
    //         }
    //     }
    // }

    // fn children(&mut self) -> &mut Map<String, LayoutKind> {
    //     match self {
    //         LayoutKind::Horizontal(v) => &mut v.children,
    //         LayoutKind::Vertical(v) => &mut v.children,
    //         LayoutKind::ScrollArea(v) => &mut v.layout.as_mut().unwrap().children,
    //     }
    // }

    // fn item_space(&self) -> f32 {
    //     match self {
    //         LayoutKind::Horizontal(v) => v.item_space,
    //         LayoutKind::Vertical(v) => v.item_space,
    //         LayoutKind::ScrollArea(v) => v.layout.as_ref().unwrap().item_space,
    //     }
    // }

    // fn update_size(&mut self, reduce: f32) {
    //     match self {
    //         LayoutKind::Horizontal(v) => {
    //             v.available_rect.add_min_x(reduce);
    //             v.width += reduce;
    //         }
    //         LayoutKind::Vertical(v) => {
    //             v.available_rect.add_min_y(reduce);
    //             v.height += reduce;
    //         }
    //         LayoutKind::ScrollArea(v) => {
    //             v.layout.as_mut().unwrap().available_rect.add_min_y(reduce);
    //             v.layout.as_mut().unwrap().height += reduce;
    //         }
    //     }
    // }
    //
    // pub fn with_size(mut self, width: f32, height: f32, padding: Padding) -> Self {
    //     match self {
    //         LayoutKind::Horizontal(ref mut v) => {
    //             v.max_rect.set_size(width, height);
    //             v.available_rect = v.max_rect.clone_add_padding(&padding);
    //         }
    //         LayoutKind::Vertical(ref mut v) => {
    //             v.max_rect.set_size(width, height);
    //             v.available_rect = v.max_rect.clone_add_padding(&padding);
    //         }
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    //
    //     self
    // }
    //
    // pub fn max_rect(&self) -> &Rect {
    //     match self {
    //         LayoutKind::Horizontal(v) => &v.max_rect,
    //         LayoutKind::Vertical(v) => &v.max_rect,
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    // }
    //
    // pub fn id(&self) -> &String {
    //     match self {
    //         LayoutKind::Horizontal(v) => &v.id,
    //         LayoutKind::Vertical(v) => &v.id,
    //         LayoutKind::ScrollArea(v) => &v.id,
    //     }
    // }
    //
    // pub fn set_offset(&mut self, offset: Offset, can: bool) {
    //     match self {
    //         LayoutKind::Horizontal(v) => {
    //             v.offset_changed = can;
    //             v.widget_offset = offset;
    //         }
    //         LayoutKind::Vertical(v) => {
    //             v.offset_changed = can;
    //             v.widget_offset = offset;
    //         }
    //         LayoutKind::ScrollArea(_) => {}
    //     }
    // }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}

// impl From<HorizontalLayout> for LayoutKind {
//     fn from(value: HorizontalLayout) -> Self {
//         LayoutKind::Horizontal(value)
//     }
// }
//
// impl From<VerticalLayout> for LayoutKind {
//     fn from(value: VerticalLayout) -> Self {
//         LayoutKind::Vertical(value)
//     }
// }

#[derive(Clone, Debug, Copy, Default)]
pub enum LayoutDirection {
    #[default]
    Min,
    Max,
}



