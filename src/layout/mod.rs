pub mod scroll_area;
pub mod popup;

use crate::frame::context::UpdateType;
use crate::layout::scroll_area::ScrollArea;
use crate::map::Map;
use crate::{Offset, OffsetDirection};
use crate::size::padding::Padding;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::size::SizeMode;
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
            LayoutKind::Horizontal(v) => {
                if !widget.rect.out_of_rect(&v.max_rect) { v.display.insert(widget.id.clone(), v.widgets.len()); }
                v.widgets.insert(id, widget)
            }
            LayoutKind::Vertical(v) => {
                if !widget.rect.out_of_rect(&v.max_rect) { v.display.insert(widget.id.clone(), v.widgets.len()); }
                v.widgets.insert(id, widget)
            }
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }
    pub fn add_child(&mut self, kind: LayoutKind) {
        let id = kind.id().to_string();
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
                v.width += space;
            }
            LayoutKind::Vertical(v) => {
                v.available_rect.add_min_y(space);
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

    fn _remove_widget(layout: &mut LayoutKind, id: &String, ui: &mut Ui) -> Option<WidgetKind> {
        if !layout.widgets().has_key(id) { return None; }
        let pos = layout.widgets().iter().position(|x| &x.id == id).unwrap();
        let rect = &layout.widgets()[id].rect;
        let offset_y = rect.height() + layout.item_space();
        layout.update_size(-offset_y);
        let offset = Offset::new(Pos::new()).with_y(-offset_y).delete_offset();
        let ut = UpdateType::Offset(offset);
        ui.update_type = ut;
        ui.can_offset = true;
        for i in pos..layout.widgets().len() {
            layout.widgets()[i].update(ui);
        }
        ui.can_offset = false;
        ui.update_type = UpdateType::None;
        layout.widgets().remove(&id)
    }

    pub fn remove_widget(&mut self, ui: &mut Ui, wid: &String) -> Option<WidgetKind> {
        let widget = Self::_remove_widget(self, wid, ui);
        if widget.is_some() { return widget; }
        for child in self.children().iter_mut() {
            let widget = child.remove_widget(ui, wid);
            if widget.is_some() { return widget; }
        }
        None
    }

    pub fn set_rect(&mut self, rect: Rect, padding: &Padding) {
        match self {
            LayoutKind::Horizontal(v) => {
                v.max_rect = rect.with_direction(v.max_rect.direction());
                v.available_rect = v.max_rect.clone_add_padding(&padding);
                match v.max_rect.direction() {
                    LayoutDirection::Min => v.available_rect.set_x_max(f32::INFINITY),
                    LayoutDirection::Max => v.available_rect.set_x_min(-f32::INFINITY),
                }
            }
            LayoutKind::Vertical(v) => {
                v.max_rect = rect;
                v.available_rect = v.max_rect.clone_add_padding(&padding);
                v.available_rect.set_y_max(f32::INFINITY);
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

    fn item_space(&self) -> f32 {
        match self {
            LayoutKind::Horizontal(v) => v.item_space,
            LayoutKind::Vertical(v) => v.item_space,
            LayoutKind::ScrollArea(v) => v.layout.as_ref().unwrap().item_space,
        }
    }

    fn update_size(&mut self, reduce: f32) {
        match self {
            LayoutKind::Horizontal(v) => {
                v.available_rect.add_min_x(reduce);
                v.width += reduce;
            }
            LayoutKind::Vertical(v) => {
                v.available_rect.add_min_y(reduce);
                v.height += reduce;
            }
            LayoutKind::ScrollArea(v) => {
                v.layout.as_mut().unwrap().available_rect.add_min_y(reduce);
                v.layout.as_mut().unwrap().height += reduce;
            }
        }
    }

    pub fn with_size(mut self, width: f32, height: f32, padding: Padding) -> Self {
        match self {
            LayoutKind::Horizontal(ref mut v) => {
                v.max_rect.set_size(width, height);
                v.available_rect = v.max_rect.clone_add_padding(&padding);
            }
            LayoutKind::Vertical(ref mut v) => {
                v.max_rect.set_size(width, height);
                v.available_rect = v.max_rect.clone_add_padding(&padding);
            }
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }

        self
    }

    pub fn max_rect(&self) -> &Rect {
        match self {
            LayoutKind::Horizontal(v) => &v.max_rect,
            LayoutKind::Vertical(v) => &v.max_rect,
            LayoutKind::ScrollArea(_) => panic!("使用ScrollArea::show")
        }
    }

    pub fn id(&self) -> &String {
        match self {
            LayoutKind::Horizontal(v) => &v.id,
            LayoutKind::Vertical(v) => &v.id,
            LayoutKind::ScrollArea(v) => &v.id,
        }
    }

    pub fn set_offset(&mut self, offset: Offset, can: bool) {
        match self {
            LayoutKind::Horizontal(v) => {
                v.offset_changed = can;
                v.widget_offset = offset;
            }
            LayoutKind::Vertical(v) => {
                v.offset_changed = can;
                v.widget_offset = offset;
            }
            LayoutKind::ScrollArea(_) => {}
        }
    }
}

#[derive(Clone, Debug, Copy)]
pub enum LayoutDirection {
    Min,
    Max,
}

pub struct HorizontalLayout {
    id: String,
    children: Map<LayoutKind>,
    widgets: Map<WidgetKind>,
    pub(crate) max_rect: Rect,
    pub(crate) available_rect: Rect,
    pub(crate) width: f32,
    pub(crate) height: f32,
    pub(crate) item_space: f32, //item之间的间隔
    offset_changed: bool,
    widget_offset: Offset,
    display: Map<usize>,
}

impl HorizontalLayout {
    fn new(direction: LayoutDirection) -> HorizontalLayout {
        HorizontalLayout {
            id: crate::gen_unique_id(),
            children: Map::new(),
            widgets: Map::new(),
            max_rect: Rect::new().with_direction(direction.clone()),
            available_rect: Rect::new().with_direction(direction.clone()),
            width: 0.0,
            height: 0.0,
            item_space: 5.0,
            offset_changed: false,
            widget_offset: Offset::new(Pos::new()),
            display: Map::new(),
        }
    }

    pub fn left_to_right() -> HorizontalLayout {
        let layout = HorizontalLayout::new(LayoutDirection::Min);
        layout
    }

    pub fn right_to_left() -> HorizontalLayout {
        let layout = HorizontalLayout::new(LayoutDirection::Max);
        layout
    }

    pub(crate) fn max_rect(mut self, rect: Rect, padding: Padding) -> Self {
        self.max_rect = rect.with_direction(self.max_rect.direction());
        self.available_rect = self.max_rect.clone_add_padding(&padding);
        match self.max_rect.direction() {
            LayoutDirection::Min => self.available_rect.set_x_max(f32::INFINITY),
            LayoutDirection::Max => self.available_rect.set_x_min(-f32::INFINITY),
        }
        self
    }

    pub(crate) fn alloc_rect(&mut self, rect: &Rect) {
        match self.max_rect.direction() {
            LayoutDirection::Min => self.available_rect.add_min_x(rect.width() + self.item_space),
            LayoutDirection::Max => self.available_rect.add_max_x(-rect.width() - self.item_space),
        }
        self.width += rect.width() + if self.width == 0.0 { 0.0 } else { self.item_space };
        println!("alloc rect  {} {} {:?}", self.height, rect.height(), rect);
        if self.height < rect.height() { self.height = rect.height(); }
    }

    pub(crate) fn drawn_rect(&self) -> Rect {
        let mut rect = self.max_rect.clone();
        rect.set_width(if self.width > self.max_rect.width() { self.max_rect.width() } else { self.width });
        rect.set_height(if self.height > self.max_rect.height() { self.max_rect.height() } else { self.height });
        rect
    }
}

impl Layout for HorizontalLayout {
    fn update(&mut self, ui: &mut Ui) {
        for child in self.children.iter_mut() {
            child.update(ui);
        }
        if let UpdateType::Offset(ref o) = ui.update_type {
            if !ui.can_offset { return; }
            self.widget_offset = o.clone();
            match o.direction {
                OffsetDirection::Down => {}
                OffsetDirection::Left => {}
                OffsetDirection::Right => {}
                OffsetDirection::Up => {}
            }
            self.offset_changed = true;
        } else {
            for di in self.display.iter() {
                self.widgets[*di].update(ui);
            }
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        ui.can_offset = self.offset_changed;
        ui.offset = self.widget_offset.clone();
        self.offset_changed = false;
        for di in self.display.iter() {
            self.widgets[*di].redraw(ui);
        }
        for child in self.children.iter_mut() {
            child.redraw(ui);
        }
    }
}

pub struct VerticalLayout {
    id: String,
    children: Map<LayoutKind>,
    widgets: Map<WidgetKind>,
    pub(crate) max_rect: Rect,
    pub(crate) available_rect: Rect,
    width: f32,
    pub(crate) height: f32,
    item_space: f32, //item之间的间隔
    widget_offset: Offset,
    offset_changed: bool,
    display: Map<usize>,
    size_mode: SizeMode,
}

impl VerticalLayout {
    pub fn new() -> VerticalLayout {
        VerticalLayout {
            id: crate::gen_unique_id(),
            children: Map::new(),
            widgets: Map::new(),
            max_rect: Rect::new(),
            available_rect: Rect::new(),
            width: 0.0,
            height: 0.0,
            item_space: 5.0,
            widget_offset: Offset::new(Pos::new()),
            offset_changed: false,
            display: Map::new(),
            size_mode: SizeMode::Auto,
        }
    }

    pub(crate) fn max_rect(mut self, rect: Rect, padding: Padding) -> Self {
        self.max_rect = rect;
        self.available_rect = self.max_rect.clone_add_padding(&padding);
        self.available_rect.set_y_max(f32::INFINITY);
        self
    }

    pub(crate) fn alloc_rect(&mut self, rect: &Rect) {
        self.available_rect.add_min_y(rect.height() + self.item_space);
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
        for child in self.children.iter_mut() {
            child.update(ui);
        }
        if let UpdateType::Offset(ref o) = ui.update_type {
            if !ui.can_offset { return; }
            self.widget_offset = o.clone();
            let mut draw_rect = self.drawn_rect();
            if let SizeMode::Auto = self.size_mode {
                draw_rect.offset(&self.widget_offset);
            }
            match o.direction {
                OffsetDirection::Down => {
                    let ds = self.display.first().cloned().unwrap_or(0);
                    self.display.clear();
                    let mut display_appear = false;
                    for wi in ds..self.widgets.len() {
                        let display = self.widgets[wi].offset(&self.widget_offset, &draw_rect);
                        if !display && !display_appear { continue; }
                        display_appear = true;
                        if display { self.display.insert(self.widgets[wi].id.clone(), wi); } else { break; }
                    }
                    println!("down display: {}-{}", ds, self.display.len());
                    self.offset_changed = true;
                }
                OffsetDirection::Left => {}
                OffsetDirection::Right => {}
                OffsetDirection::Up => {
                    let de = self.display.last().cloned().unwrap_or(self.widgets.len() - 1);
                    self.display.clear();
                    let mut display_appear = false;
                    for i in 0..=de {
                        let wi = de - i;
                        let display = self.widgets[wi].offset(&self.widget_offset, &draw_rect);
                        if !display && !display_appear { continue; }
                        display_appear = true;
                        if display { self.display.insert(self.widgets[wi].id.clone(), wi); } else { break; }
                    }
                    self.display.reverse();
                    self.offset_changed = true;
                    println!("up display: {}-{}", de, self.display.len());
                }
            }
            ui.context.window.request_redraw();
        } else {
            for di in self.display.iter() {
                self.widgets[*di].update(ui);
            }
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        ui.can_offset = self.offset_changed;
        ui.offset = self.widget_offset.clone();
        self.offset_changed = false;
        for di in self.display.iter() {
            self.widgets[*di].redraw(ui);
        }
        // ui.offset = Offset::new(Pos::new());
        for child in self.children.iter_mut() {
            child.redraw(ui);
        }
    }
}