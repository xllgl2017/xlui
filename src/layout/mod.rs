pub mod scroll_area;
pub mod popup;

use crate::layout::scroll_area::ScrollArea;
use crate::map::Map;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::widgets::Widget;

pub trait Layout {
    fn update(&mut self, ui: &mut Ui);
    fn redraw(&mut self, ui: &mut Ui);
}

pub(crate) enum LayoutKind {
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

    pub fn add_widget(&mut self, id: String, widget: Box<dyn Widget>) {
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

    // pub fn context_height(&self) -> f32 {
    //     match self {
    //         LayoutKind::Horizontal(v) => v.height,
    //         LayoutKind::Vertical(v) => v.height,
    //         LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
    //     }
    // }

    pub fn drawn_rect(&self) -> Rect {
        match self {
            LayoutKind::Horizontal(v) => {
                let mut rect = v.max_rect.clone();
                rect.set_width(if v.width > v.max_rect.width() { v.max_rect.width() } else { v.width });
                rect.set_height(if v.height > v.max_rect.height() { v.max_rect.height() } else { v.height });
                rect
            }
            LayoutKind::Vertical(v) => {
                let mut rect = v.max_rect.clone();
                rect.set_width(if v.width > v.max_rect.width() { v.max_rect.width() } else { v.width });
                rect.set_height(if v.height > v.max_rect.height() { v.max_rect.height() } else { v.height });
                rect
            }
            LayoutKind::ScrollArea(v) => v.rect.clone(),
        }
    }
    pub fn add_space(&mut self, space: f32) {
        match self {
            LayoutKind::Horizontal(v) => {
                v.available_rect.x.min += space;
                v.width += space;
            }
            LayoutKind::Vertical(v) => {
                v.available_rect.y.min += space;
                v.height += space;
            }
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }

    pub fn get_widget(&mut self, id: &String) -> Option<&mut Box<dyn Widget>> {
        match self {
            LayoutKind::Horizontal(v) => v.widgets.get_mut(id),
            LayoutKind::Vertical(v) => v.widgets.get_mut(id),
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }
}

pub struct HorizontalLayout {
    children: Map<LayoutKind>,
    widgets: Map<Box<dyn Widget>>,
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
        self
    }

    pub(crate) fn alloc_rect(&mut self, rect: &Rect) {
        self.available_rect.x.min += rect.width() + self.item_space;
        self.width += rect.width() + if self.width == 0.0 { 0.0 } else { self.item_space };
        if self.height < rect.height() { self.height = rect.height(); }
    }
}

impl Layout for HorizontalLayout {
    fn update(&mut self, ui: &mut Ui) {
        for child in self.children.iter_mut() {
            child.update(ui);
        }
        for widget in self.widgets.iter_mut() {
            widget.update(ui)
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        for child in self.children.iter_mut() {
            child.redraw(ui);
        }
        for widget in self.widgets.iter_mut() {
            widget.redraw(ui)
        }
    }
}

pub struct VerticalLayout {
    children: Map<LayoutKind>,
    widgets: Map<Box<dyn Widget>>,
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
        self
    }

    pub(crate) fn alloc_rect(&mut self, rect: &Rect) {
        self.available_rect.y.min += rect.height() + self.item_space;
        if self.width < rect.width() { self.width = rect.width() + self.item_space; }
        self.height += rect.height() + if self.height == 0.0 { 0.0 } else { self.item_space };
    }
}


impl Layout for VerticalLayout {
    fn update(&mut self, ui: &mut Ui) {
        for child in self.children.iter_mut() {
            child.update(ui);
        }
        for widget in self.widgets.iter_mut() {
            widget.update(ui)
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        for child in self.children.iter_mut() {
            child.redraw(ui);
        }
        for widget in self.widgets.iter_mut() {
            widget.redraw(ui)
        }
    }
}