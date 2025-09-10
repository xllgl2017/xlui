use std::mem;
use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutDirection, LayoutItem};
use crate::map::Map;
use crate::{Offset, Padding, Pos};
use crate::response::Response;
use crate::size::SizeMode;
use crate::ui::Ui;
use crate::widgets::WidgetSize;

pub struct VerticalLayout {
    id: String,
    items: Map<String, LayoutItem>,
    display: Map<String, usize>,
    // pub(crate) max_rect: Rect,
    // pub(crate) available_rect: Rect,
    // width: f32,
    // height: f32,
    item_space: f32, //item之间的间隔
    // widget_offset: Offset,
    // offset_changed: bool,
    offset: Offset,
    size_mode: SizeMode,
    padding: Padding,
    direction: LayoutDirection,
}

impl VerticalLayout {
    fn new(direction: LayoutDirection) -> VerticalLayout {
        VerticalLayout {
            id: crate::gen_unique_id(),
            items: Map::new(),
            // max_rect: Rect::new().with_y_direction(direction),
            // available_rect: Rect::new().with_y_direction(direction),
            // width: 0.0,
            // height: 0.0,
            item_space: 5.0,
            // widget_offset: Offset::new(Pos::new()),
            // offset_changed: false,
            display: Map::new(),
            size_mode: SizeMode::Auto,
            padding: Padding::same(0.0),
            direction,
            offset: Offset::new(Pos::new()),
        }
    }

    pub fn top_to_bottom() -> VerticalLayout {
        VerticalLayout::new(LayoutDirection::Min)
    }

    pub fn bottom_to_top() -> VerticalLayout {
        VerticalLayout::new(LayoutDirection::Max)
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.with_width(w).with_height(h)
    }


    pub fn with_width(mut self, w: f32) -> Self {
        self.set_width(w);
        self
    }

    pub fn set_width(&mut self, w: f32) {
        self.size_mode.fix_width(w);
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.set_height(h);
        self
    }

    pub fn set_height(&mut self, h: f32) {
        self.size_mode.fix_height(h);
    }

    pub fn with_space(mut self, s: f32) -> Self {
        self.item_space = s;
        self
    }

    pub fn with_padding(mut self, p: Padding) -> Self {
        self.set_padding(p);
        self
    }

    pub fn set_padding(&mut self, p: Padding) {
        self.padding = p;
    }

    pub(crate) fn padding(&self) -> &Padding {
        &self.padding
    }

    pub fn item_space(&self) -> f32 {
        self.item_space
    }

    // pub fn width(&self) -> f32 {
    //     self.width
    // }
    //
    // pub fn height(&self) -> f32 {
    //     self.height
    // }

    // pub(crate) fn max_rect(mut self, rect: Rect, padding: Padding) -> Self {
    //     self.max_rect = rect;
    //     self.available_rect = self.max_rect.clone_add_padding(&padding);
    //     self.available_rect.set_y_max(f32::INFINITY);
    //     self
    // }
    //
    // pub(crate) fn alloc_rect(&mut self, rect: &Rect) {
    //     match self.max_rect.y_direction() {
    //         LayoutDirection::Min => self.available_rect.add_min_y(rect.height() + self.item_space),
    //         LayoutDirection::Max => self.available_rect.add_max_y(-rect.height() - self.item_space),
    //     }
    //
    //
    //     if self.width < rect.width() { self.width = rect.width() + self.item_space; }
    //     self.height += rect.height() + if self.height == 0.0 { 0.0 } else { self.item_space };
    // }

    // pub(crate) fn drawn_rect(&self) -> Rect {
    //     let mut rect = self.max_rect.clone();
    //     rect.set_width(if self.width > self.max_rect.width() { self.max_rect.width() } else { self.width });
    //     rect.set_height(if self.height > self.max_rect.height() { self.max_rect.height() } else { self.height });
    //     rect
    // }

}

impl Layout for VerticalLayout {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        let previous_rect = mem::take(&mut ui.draw_rect);
        let mut width = 0.0;
        let mut height = 0.0;
        match ui.update_type {
            UpdateType::Init => {
                for item in self.items.iter() {
                    if width < item.width() { width = item.width(); }
                    height += item.height() + self.item_space;
                }
            }
            _ => {
                ui.draw_rect.set_x_min(previous_rect.dx().min + self.padding.left);
                ui.draw_rect.set_x_max(previous_rect.dx().max - self.padding.right);
                ui.draw_rect.set_y_min(previous_rect.dy().min + self.padding.top);
                ui.draw_rect.set_y_max(previous_rect.dy().max - self.padding.bottom);
                ui.draw_rect.set_y_direction(self.direction);
                ui.draw_rect.set_x_min(ui.draw_rect.dx().min + self.offset.x);
                ui.draw_rect.set_y_min(ui.draw_rect.dy().min + self.offset.y);
                for item in self.items.iter_mut() {
                    // if self.height + item.height() < self.offset.y.abs() {
                    //     self.height += item.height() + self.item_space;
                    //     continue;
                    // }
                    let resp = item.update(ui);
                    if width < resp.size.dw { width = resp.size.dw; }
                    height += resp.size.dh + self.item_space;
                    match self.direction {
                        LayoutDirection::Min => ui.draw_rect.add_min_y(resp.size.dh + self.item_space),
                        LayoutDirection::Max => ui.draw_rect.add_max_y(-resp.size.dh - self.item_space),
                    }
                }
                height -= self.item_space;
            }
        }
        ui.draw_rect = previous_rect;

        let (dw, dh) = self.size_mode.size(width, height);
        Response::new(&self.id, WidgetSize {
            dw,
            dh,
            rw: width,
            rh: height,
        })

        // match self.size_mode {
        //     SizeMode::Auto => Response::new(&self.id, self.width, self.height),
        //     SizeMode::FixWidth(w) => Response::new(&self.id, w, self.height),
        //     SizeMode::FixHeight(h) => Response::new(&self.id, self.width, h),
        //     SizeMode::Fix(w, h) => Response::new(&self.id, w, h),
        // }

        // for child in self.items.iter_mut() {
        //     child.update(ui);
        // }
        // if let UpdateType::Offset(ref o) = ui.update_type {
        //     if !ui.can_offset { return; }
        //     self.widget_offset = o.clone();
        //     let mut draw_rect = self.drawn_rect();
        //     if let SizeMode::Auto = self.size_mode {
        //         draw_rect.offset(&self.widget_offset);
        //     }
        //     match o.direction {
        //         OffsetDirection::Down => {
        //             let ds = self.display.first().cloned().unwrap_or(0);
        //             self.display.clear();
        //             let mut display_appear = false;
        //             for wi in ds..self.widgets.len() {
        //                 let display = self.widgets[wi].offset(&self.widget_offset, &draw_rect);
        //                 if !display && !display_appear { continue; }
        //                 display_appear = true;
        //                 if display { self.display.insert(self.widgets[wi].id.clone(), wi); } else { break; }
        //             }
        //             println!("down display: {}-{}", ds, self.display.len());
        //             self.offset_changed = true;
        //         }
        //         OffsetDirection::Left => {}
        //         OffsetDirection::Right => {}
        //         OffsetDirection::Up => {
        //             let de = self.display.last().cloned().unwrap_or(self.widgets.len() - 1);
        //             self.display.clear();
        //             let mut display_appear = false;
        //             for i in 0..=de {
        //                 let wi = de - i;
        //                 let display = self.widgets[wi].offset(&self.widget_offset, &draw_rect);
        //                 if !display && !display_appear { continue; }
        //                 display_appear = true;
        //                 if display { self.display.insert(self.widgets[wi].id.clone(), wi); } else { break; }
        //             }
        //             self.display.reverse();
        //             self.offset_changed = true;
        //             println!("up display: {}-{}", de, self.display.len());
        //         }
        //     }
        //     ui.context.window.request_redraw();
        // } else {
        //     for di in self.display.iter() {
        //         self.widgets[*di].update(ui);
        //     }
        // }
    }
    fn items(&self) -> &Map<String, LayoutItem> {
        &self.items
    }

    // fn add_item(&mut self, id: String, item: LayoutItem) {
    //     self.items.insert(id, item);
    // }
    //
    // fn get_item(&self, id: &String) -> Option<&LayoutItem> {
    //     self.items.get(id)
    // }

    fn items_mut(&mut self) -> &mut Map<String, LayoutItem> {
        &mut self.items
    }

    fn add_item(&mut self, item: LayoutItem) {
        self.items.insert(item.id().to_string(), item);
    }

    fn set_offset(&mut self, offset: Offset) {
        self.offset = offset;
    }

    fn set_size(&mut self, w: f32, h: f32) {
        self.set_width(w);
        self.set_height(h);
    }

    // fn redraw(&mut self, ui: &mut Ui) {
    //     ui.can_offset = self.offset_changed;
    //     ui.offset = self.widget_offset.clone();
    //     self.offset_changed = false;
    //     for di in self.display.iter() {
    //         self.widgets[*di].redraw(ui);
    //     }
    //     for child in self.items.iter_mut() {
    //         child.redraw(ui);
    //     }
    // }
}