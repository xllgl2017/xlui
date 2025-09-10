use std::mem;
use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutDirection, LayoutItem, LayoutKind};
use crate::map::Map;
use crate::ui::Ui;
use crate::{Offset, OffsetDirection, Padding, Pos, Rect};
use crate::response::Response;
use crate::size::SizeMode;
use crate::widgets::WidgetSize;

pub struct HorizontalLayout {
    id: String,
    items: Map<String, LayoutItem>,
    // pub(crate) max_rect: Rect,
    // pub(crate) available_rect: Rect,
    // width: f32,
    // height: f32,
    item_space: f32, //item之间的间隔
    offset_changed: bool,
    // widget_offset: Offset,
    display: Map<String, usize>,
    size_mode: SizeMode,
    direction: LayoutDirection,
    padding: Padding,
    offset: Offset
}

impl HorizontalLayout {
    fn new(direction: LayoutDirection) -> HorizontalLayout {
        HorizontalLayout {
            id: crate::gen_unique_id(),
            items: Map::new(),
            // widgets: Map::new(),
            // max_rect: Rect::new().with_x_direction(direction.clone()),
            // available_rect: Rect::new().with_x_direction(direction.clone()),
            // width: 0.0,
            // height: 0.0,
            item_space: 5.0,
            offset_changed: false,
            // widget_offset: Offset::new(Pos::new()),
            display: Map::new(),
            size_mode: SizeMode::Auto,
            direction,
            padding: Padding::same(0.0),
            offset: Offset::new(Pos::new())
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

    // pub(crate) fn max_rect(mut self, rect: Rect, padding: Padding) -> Self {
    //     self.max_rect = rect.with_x_direction(self.max_rect.x_direction());
    //     self.available_rect = self.max_rect.clone_add_padding(&padding);
    //     match self.max_rect.x_direction() {
    //         LayoutDirection::Min => self.available_rect.set_x_max(f32::INFINITY),
    //         LayoutDirection::Max => self.available_rect.set_x_min(-f32::INFINITY),
    //     }
    //     self
    // }
    //
    // pub(crate) fn alloc_rect(&mut self, rect: &Rect) {
    //     match self.max_rect.x_direction() {
    //         LayoutDirection::Min => self.available_rect.add_min_x(rect.width() + self.item_space),
    //         LayoutDirection::Max => self.available_rect.add_max_x(-rect.width() - self.item_space),
    //     }
    //     self.width += rect.width() + if self.width == 0.0 { 0.0 } else { self.item_space };
    //     println!("alloc rect  {} {} {:?}", self.height, rect.height(), rect);
    //     if self.height < rect.height() { self.height = rect.height(); }
    // }

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
}

impl Layout for HorizontalLayout {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        let previous_rect = mem::take(&mut ui.draw_rect);
        let mut width = 0.0;
        let mut height = 0.0;
        match ui.update_type {
            UpdateType::Init => {
                for item in self.items.iter() {
                    if height < item.height() { height = item.height(); }
                    width += item.width() + self.item_space;
                }
                width -= self.item_space;
            }
            _ => {
                ui.draw_rect.set_x_min(previous_rect.dx().min + self.padding.left);
                ui.draw_rect.set_x_max(previous_rect.dx().max - self.padding.right);
                ui.draw_rect.set_y_min(previous_rect.dy().min + self.padding.top);
                ui.draw_rect.set_y_max(previous_rect.dy().max - self.padding.bottom);
                ui.draw_rect.set_x_direction(self.direction);
                for item in self.items.iter_mut() {
                    let resp = item.update(ui);
                    if height < resp.size.dh { height = resp.size.dh; }
                    width += resp.size.dw + self.item_space;
                    match self.direction {
                        LayoutDirection::Min => ui.draw_rect.add_min_x(resp.size.dw + self.item_space),
                        LayoutDirection::Max => ui.draw_rect.add_max_x(-resp.size.dw - self.item_space),
                    }
                }
                width -= self.item_space;
            }
        }
        ui.draw_rect = previous_rect;
        let (dw, dh) = self.size_mode.size(width, height);
        Response::new(&self.id,WidgetSize{
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
        //     match o.direction {
        //         OffsetDirection::Down => {}
        //         OffsetDirection::Left => {}
        //         OffsetDirection::Right => {}
        //         OffsetDirection::Up => {}
        //     }
        //     self.offset_changed = true;
        // } else {
        //     for di in self.display.iter() {
        //         self.widgets[*di].update(ui);
        //     }
        // }
    }
    fn items(&self) -> &Map<String, LayoutItem> {
        &self.items
    }
    fn items_mut(&mut self) -> &mut Map<String, LayoutItem> {
        &mut self.items
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

    fn set_offset(&mut self, offset: Offset) {
        self.offset = offset;
    }

    fn set_size(&mut self, w: f32, h: f32) {
       self.set_width(w);
       self.set_height(h);
   }
}