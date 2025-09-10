use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutItem};
use crate::map::Map;
use crate::response::Response;
use crate::size::SizeMode;
use crate::ui::Ui;
use crate::{Offset, Padding, Pos};
use std::mem;
use std::ops::Range;
use crate::widgets::WidgetSize;

pub struct RecycleLayout {
    id: String,
    items: Map<String, LayoutItem>,
    size_mode: SizeMode,
    padding: Padding,
    item_space: f32, //item之间的间隔
    // width: f32,
    // height: f32,
    offset: Offset,
    count: usize,
    display: Range<usize>,
    item_height: f32,
}

impl RecycleLayout {
    pub fn new() -> Self {
        RecycleLayout {
            id: crate::gen_unique_id(),
            items: Map::new(),
            size_mode: SizeMode::Auto,
            padding: Padding::same(0.0),
            item_space: 5.0,
            // width: 0.0,
            // height: 0.0,
            offset: Offset::new(Pos::new()),
            count: 0,
            display: 0..0,
            item_height: 0.0,
        }
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
}

impl Layout for RecycleLayout {
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
                    ui.draw_rect.add_min_y(resp.size.dh + self.item_space)
                    // match self.direction {
                    //     LayoutDirection::Min => ui.draw_rect.add_min_y(resp.height + self.item_space),
                    //     LayoutDirection::Max => ui.draw_rect.add_max_y(-resp.height - self.item_space),
                    // }
                }
                height -= self.item_space;
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

        //
        // match self.size_mode {
        //     SizeMode::Auto => Response::new(&self.id, self.width, self.height),
        //     SizeMode::FixWidth(w) => Response::new(&self.id, w, self.height),
        //     SizeMode::FixHeight(h) => Response::new(&self.id, self.width, h),
        //     SizeMode::Fix(w, h) => Response::new(&self.id, w, h),
        // }
    }
    fn items(&self) -> &Map<String, LayoutItem> {
        &self.items
    }
    fn items_mut(&mut self) -> &mut Map<String, LayoutItem> {
        &mut self.items
    }

    fn set_offset(&mut self, offset: Offset) {
        self.offset = offset;
    }

    fn set_size(&mut self, w: f32, h: f32) {
        self.set_width(w);
        self.set_height(h);
    }
}