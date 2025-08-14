pub mod scroll_area;
pub mod popup;

use crate::frame::context::UpdateType;
use crate::layout::scroll_area::ScrollArea;
use crate::map::Map;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetKind};

pub trait Layout {
    fn update(&mut self, ui: &mut Ui);
    fn redraw(&mut self, ui: &mut Ui);
}

pub enum LayoutKind {
    Horizontal(HorizontalLayout),
    Vertical(VerticalLayout),
    ScrollArea(ScrollArea),
}

impl LayoutKind {
    pub fn update(&mut self, ui: &mut Ui) {
        match self {
            LayoutKind::Horizontal(v) => v.update(ui),
            LayoutKind::Vertical(v) => v.update(ui),
            LayoutKind::ScrollArea(v) => v.update(ui),
        }
    }

    pub fn redraw(&mut self, ui: &mut Ui) {
        match self {
            LayoutKind::Horizontal(v) => v.redraw(ui),
            LayoutKind::Vertical(v) => v.redraw(ui),
            LayoutKind::ScrollArea(v) => v.redraw(ui),
        }
    }

    pub fn add_widget(&mut self, id: String, widget: WidgetKind) {
        match self {
            LayoutKind::Horizontal(v) => v.widgets.insert(id, widget),
            LayoutKind::Vertical(v) => v.widgets.insert(id, widget),
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }
    pub fn add_child(&mut self, id: String, kind: LayoutKind) {
        match self {
            LayoutKind::Horizontal(v) => {
                v.alloc_rect(&kind.drawn_rect());
                v.children.insert(id, kind)
            }
            LayoutKind::Vertical(v) => {
                v.alloc_rect(&kind.drawn_rect());
                v.children.insert(id, kind)
            }
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }
    pub fn available_rect(&self) -> &Rect {
        match self {
            LayoutKind::Horizontal(v) => &v.available_rect,
            LayoutKind::Vertical(v) => &v.available_rect,
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }

    pub fn alloc_rect(&mut self, rect: &Rect) {
        match self {
            LayoutKind::Horizontal(v) => v.alloc_rect(rect),
            LayoutKind::Vertical(v) => v.alloc_rect(rect),
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }

    pub fn drawn_rect(&self) -> Rect {
        match self {
            LayoutKind::Horizontal(v) => {
                let mut rect = v.max_rect.clone();
                rect.set_width(if v.width > v.max_rect.width() { v.max_rect.width() } else { v.width });
                rect.set_height(if v.height > v.max_rect.height() { v.max_rect.height() } else { v.height });
                rect
            }
            LayoutKind::Vertical(v) => v.drawn_rect(),
            LayoutKind::ScrollArea(v) => v.drawn_rect().clone()
        }
    }
    pub fn add_space(&mut self, space: f32) {
        match self {
            LayoutKind::Horizontal(v) => {
                v.available_rect.add_min_x(space);
                // v.available_rect.dx.min += space;
                v.width += space;
            }
            LayoutKind::Vertical(v) => {
                v.available_rect.add_min_y(space);
                // v.available_rect.dy.min += space;
                v.height += space;
            }
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }

    pub fn get_widget(&mut self, id: &String) -> Option<&mut Box<dyn Widget>> {
        match self {
            LayoutKind::Horizontal(v) => {
                let widget = v.widgets.get_mut(id);
                if widget.is_some() { return Some(&mut widget?.widget); }
                for child in v.children.iter_mut() {
                    let widget = child.get_widget(id);
                    if widget.is_some() { return widget; }
                }
                None
            }
            LayoutKind::Vertical(v) => {
                let widget = v.widgets.get_mut(id);
                if widget.is_some() { return Some(&mut widget?.widget); }
                for child in v.children.iter_mut() {
                    let widget = child.get_widget(id);
                    if widget.is_some() { return widget; }
                }
                None
            }
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }

    pub fn get_layout(&mut self, id: &String) -> Option<&mut LayoutKind> {
        match self {
            LayoutKind::Horizontal(_) | LayoutKind::Vertical(_) => {
                for (k, v) in self.children().entry_mut() {
                    if k == id { return Some(v); }
                    let layout = v.get_layout(id);
                    if layout.is_some() { return layout; }
                }
                None
            }
            LayoutKind::ScrollArea(v) => if &v.id == id { Some(self) } else { None }
        }
    }

    // fn _remove_widget(widgets: &mut Map<WidgetKind>, id: &String) -> Option<WidgetKind> {
    //     let pos = widgets.iter().position(|x| &x.id == id);
    //     if pos.is_none() { return None }
    //     let pos=pos.unwrap();
    // }

    pub fn remove_widget(&mut self, wid: &String) -> Option<WidgetKind> {
        let widget = self.widgets().remove(wid);
        if widget.is_some() { return widget; }
        for child in self.children().iter_mut() {
            let widget = child.remove_widget(wid);
            if widget.is_some() { return widget; }
        }
        None
    }

    pub fn set_rect(&mut self, rect: Rect, padding: &Padding) {
        match self {
            LayoutKind::Horizontal(v) => {
                v.max_rect = rect;
                v.available_rect = v.max_rect.clone_add_padding(&padding);
                v.available_rect.set_x_max(f32::INFINITY);
                // v.available_rect.dx.max = f32::INFINITY;
            }
            LayoutKind::Vertical(v) => {
                v.max_rect = rect;
                v.available_rect = v.max_rect.clone_add_padding(&padding);
                v.available_rect.set_y_max(f32::INFINITY);
                // v.available_rect.dy.max = f32::INFINITY;
            }
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }

    fn widgets(&mut self) -> &mut Map<WidgetKind> {
        match self {
            LayoutKind::Horizontal(v) => &mut v.widgets,
            LayoutKind::Vertical(v) => &mut v.widgets,
            LayoutKind::ScrollArea(v) => {
                &mut v.layout.as_mut().unwrap().widgets
            }
        }
    }

    fn children(&mut self) -> &mut Map<LayoutKind> {
        match self {
            LayoutKind::Horizontal(v) => &mut v.children,
            LayoutKind::Vertical(v) => &mut v.children,
            LayoutKind::ScrollArea(v) => &mut v.layout.as_mut().unwrap().children,
        }
    }
}


fn update_or_redraw(widgets: &mut Map<WidgetKind>, children: &mut Map<LayoutKind>, ui: &mut Ui, update: bool) {
    match update {
        true => {
            for widget in widgets.iter_mut() {
                widget.update(ui)
            }
            for child in children.iter_mut() {
                child.update(ui);
            }
        }
        false => {
            for widget in widgets.iter_mut() {
                widget.redraw(ui);
            }
            for child in children.iter_mut() {
                child.redraw(ui);
            }
        }
    }
}

pub struct HorizontalLayout {
    children: Map<LayoutKind>,
    widgets: Map<WidgetKind>,
    display: Map<usize>,
    max_rect: Rect,
    available_rect: Rect,
    width: f32,
    height: f32,
    item_space: f32, //item之间的间隔
}

impl HorizontalLayout {
    pub fn new() -> HorizontalLayout {
        HorizontalLayout {
            children: Map::new(),
            widgets: Map::new(),
            display: Map::new(),
            max_rect: Rect::new(),
            available_rect: Rect::new(),
            width: 0.0,
            height: 0.0,
            item_space: 5.0,
        }
    }

    pub(crate) fn max_rect(mut self, rect: Rect, padding: Padding) -> Self {
        self.max_rect = rect;
        self.available_rect = self.max_rect.clone_add_padding(&padding);
        self.available_rect.set_x_max(f32::INFINITY);
        // self.available_rect.dx.max = f32::INFINITY;
        self
    }

    pub(crate) fn alloc_rect(&mut self, rect: &Rect) {
        self.available_rect.add_min_x(rect.width() + self.item_space);
        // self.available_rect.dx.min += rect.width() + self.item_space;
        self.width += rect.width() + if self.width == 0.0 { 0.0 } else { self.item_space };
        println!("alloc rect  {} {}", self.height, rect.height());
        if self.height < rect.height() { self.height = rect.height(); }
    }
}

impl Layout for HorizontalLayout {
    fn update(&mut self, ui: &mut Ui) {
        update_or_redraw(&mut self.widgets, &mut self.children, ui, true);
    }

    fn redraw(&mut self, ui: &mut Ui) {
        update_or_redraw(&mut self.widgets, &mut self.children, ui, false);
    }
}

pub struct VerticalLayout {
    children: Map<LayoutKind>,
    widgets: Map<WidgetKind>,
    max_rect: Rect,
    available_rect: Rect,
    width: f32,
    height: f32,
    item_space: f32, //item之间的间隔
}

impl VerticalLayout {
    pub fn new() -> VerticalLayout {
        VerticalLayout {
            children: Map::new(),
            widgets: Map::new(),
            max_rect: Rect::new(),
            available_rect: Rect::new(),
            width: 0.0,
            height: 0.0,
            item_space: 5.0,
        }
    }

    pub(crate) fn with_size(mut self, width: f32, height: f32, padding: Padding) -> Self {
        self.max_rect.set_size(width, height);
        self.available_rect = self.max_rect.clone_add_padding(&padding);
        self
    }

    pub(crate) fn max_rect(mut self, rect: Rect, padding: Padding) -> Self {
        self.max_rect = rect;
        self.available_rect = self.max_rect.clone_add_padding(&padding);
        self.available_rect.set_y_max(f32::INFINITY);
        // self.available_rect.dy.max = f32::INFINITY;
        self
    }

    pub(crate) fn alloc_rect(&mut self, rect: &Rect) {
        self.available_rect.add_min_y(rect.height() + self.item_space);
        // self.available_rect.dy.min += rect.height() + self.item_space;
        if self.width < rect.width() { self.width = rect.width() + self.item_space; }
        self.height += rect.height() + if self.height == 0.0 { 0.0 } else { self.item_space };
    }

    pub(crate) fn drawn_rect(&self) -> Rect {
        let mut rect = self.max_rect.clone();
        rect.set_width(if self.width > self.max_rect.width() { self.max_rect.width() } else { self.width });
        rect.set_height(if self.height > self.max_rect.height() { self.max_rect.height() } else { self.height });
        rect
    }
}


impl Layout for VerticalLayout {
    fn update(&mut self, ui: &mut Ui) {
        update_or_redraw(&mut self.widgets, &mut self.children, ui, true);
        match &ui.update_type {
            UpdateType::Offset(o) => self.available_rect.offset(o.x, o.y),
            _ => {}
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        update_or_redraw(&mut self.widgets, &mut self.children, ui, false);
    }
}