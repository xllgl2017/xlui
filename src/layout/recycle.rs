use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutItem};
use crate::map::Map;
use crate::response::Response;
use crate::ui::Ui;
use crate::widgets::WidgetSize;
use crate::{Offset, Padding, Pos};
use std::ops::Range;

pub(crate) struct RecycleLayout {
    id: String,
    items: Map<String, LayoutItem>,
    padding: Padding,
    item_space: f32, //item之间的间隔
    offset: Offset, //y偏移
    total_count: usize, //总item数
    draw_count: usize,
    display: Range<usize>, //显示范围
    item_height: f32, //每一个item的高度
    size: WidgetSize,
}

impl RecycleLayout {
    pub fn new() -> Self {
        RecycleLayout {
            id: crate::gen_unique_id(),
            items: Map::new(),
            padding: Padding::same(0.0),
            item_space: 5.0,
            offset: Offset::new(Pos::new()),
            total_count: 0,
            draw_count: 10,
            display: 0..0,
            item_height: 38.0,
            size: WidgetSize::same(0.0, 0.0),
        }
    }

    pub fn with_size(self, w: f32, h: f32) -> Self {
        self.with_width(w).with_height(h)
    }

    pub fn with_width(mut self, w: f32) -> Self {
        self.set_width(w);
        self
    }

    pub fn with_item_height(mut self, h: f32) -> Self {
        self.item_height = h;
        self.draw_count = (self.size.dh * 1.5 / (self.item_height + self.item_space)).ceil() as usize;
        self
    }

    pub fn set_width(&mut self, w: f32) {
        self.size.dw = w;
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.set_height(h);
        self
    }

    pub fn set_height(&mut self, h: f32) {
        self.size.dh = h;
        self.draw_count = (self.size.dh * 1.5 / (self.item_height + self.item_space)).ceil() as usize;
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

    pub fn item_space(&self) -> f32 {
        self.item_space
    }

    pub fn update_display(&mut self) {
        let item_total_h = self.item_height + self.item_space;

        let mut start = (-self.offset.y / item_total_h).floor().max(0.0) as usize;
        let mut end = ((-self.offset.y + self.size.dh) / item_total_h).ceil().max(0.0) as usize;

        if start > self.total_count {
            start = self.total_count;
        }
        if end > self.total_count {
            end = self.total_count;
        }

        self.display = start..end;
        // println!("recycle display: {:#?} {} {} {} {}", self.display, self.item_height, self.item_space, self.offset.y, self.size.dh);
    }

    pub fn display_range(&self) -> &Range<usize> {
        &self.display
    }

    pub fn add_item_empty(&mut self) {
        self.total_count += 1;
        self.size.rh += self.item_height + self.item_space;
    }

    pub fn remove_item(&mut self) {
        self.total_count -= 1;
        self.size.rh -= self.item_height + self.item_space;
        if self.total_count < self.draw_count {
            self.items.remove_map_by_index(0);
        }
        self.update_display();
    }

    pub fn draw_count(&self) -> usize {
        self.draw_count
    }

    pub fn size(&self) -> &WidgetSize {
        &self.size
    }
}

impl Layout for RecycleLayout {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        let previous_rect = ui.draw_rect.clone();
        match ui.update_type {
            UpdateType::Init => {
                self.update_display();
            }
            _ => {
                ui.draw_rect.set_x_min(previous_rect.dx().min + self.padding.left);
                ui.draw_rect.set_x_max(previous_rect.dx().max - self.padding.right);
                ui.draw_rect.set_y_min(previous_rect.dy().min + self.padding.top);
                ui.draw_rect.set_y_max(previous_rect.dy().max - self.padding.bottom);

                let item_total_h = self.item_height + self.item_space;
                let first_offset = -(-self.offset.y - self.display.start as f32 * item_total_h);

                ui.draw_rect.set_y_min(previous_rect.dy().min + first_offset);
                let mut start = self.display.start;
                for item in self.items.iter_mut() {
                    let resp = item.update(ui);
                    ui.draw_rect.add_min_y(resp.size.dh + self.item_space);
                    start += 1;
                    if start >= self.total_count { break; }
                }
            }
        }
        ui.draw_rect = previous_rect;
        Response::new(&self.id, self.size.clone())
    }
    fn items(&self) -> &Map<String, LayoutItem> {
        &self.items
    }
    fn items_mut(&mut self) -> &mut Map<String, LayoutItem> {
        &mut self.items
    }

    fn add_item(&mut self, item: LayoutItem) {
        self.total_count += 1;
        self.size.rh += self.item_height + self.item_space;
        if self.items.len() < self.draw_count {
            self.items.insert(item.id().to_string(), item);
        }
    }

    fn set_offset(&mut self, offset: Offset) {
        self.offset = offset;
        self.update_display();
    }

    fn set_size(&mut self, w: f32, h: f32) {
        self.set_width(w);
        self.set_height(h);
    }
}