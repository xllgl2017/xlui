use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutDirection, LayoutItem, LayoutOffset};
use crate::map::Map;
use crate::style::Visual;
use crate::response::Response;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::style::FrameStyle;
use crate::ui::Ui;
use crate::widgets::space::Space;
use crate::widgets::WidgetSize;
use crate::{Margin, Offset, Padding, Widget};
use std::mem;
use std::ops::Range;

///### 垂直布局的使用
///```rust
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///    //快速创建一个上到下的水平布局
///    ui.vertical(|ui|{
///        //添加布局内容
///    });
///    //创建一个从下到上的布局
///    let layout=VerticalLayout::bottom_to_top()
///        //设置两个item之间的间隔
///        .with_space(10.0)
///        //设置当前布局的高度
///        .with_height(100.0)
///        //设置当前布局的宽度
///        .with_width(100.0)
///        //设置布局内的边距
///        .with_padding(Padding::same(1.0))
///        //设置背景颜色
///        .with_fill(Color::GREEN);
///    ui.add_layout(layout,|ui|{
///        //添加布局内容
///    });
///
/// }
/// ```
///
///
///

pub struct VerticalLayout {
    id: String,
    items: Map<String, LayoutItem>,
    item_space: f32, //item之间的间隔
    geometry: Geometry,
    direction: LayoutDirection,
    visual: Visual,
    display: Range<usize>,
    offset: LayoutOffset,
}

impl VerticalLayout {
    fn new(direction: LayoutDirection) -> VerticalLayout {
        VerticalLayout {
            id: crate::gen_unique_id(),
            items: Map::new(),
            item_space: 5.0,
            geometry: Geometry::new(),
            direction,
            visual: Visual::new(),
            display: 0..0,
            offset: LayoutOffset::new(),
        }
    }

    pub fn top_to_bottom() -> VerticalLayout {
        VerticalLayout::new(LayoutDirection::Min)
    }

    pub fn bottom_to_top() -> VerticalLayout {
        VerticalLayout::new(LayoutDirection::Max)
    }

    //设置布局的大小
    pub fn with_size(self, w: f32, h: f32) -> Self {
        self.with_width(w).with_height(h)
    }


    pub fn with_width(mut self, w: f32) -> Self {
        self.set_width(w);
        self
    }

    ///布局的背景填充
    pub fn with_fill(mut self, color: Color) -> Self {
        self.visual.enable();
        self.visual.style_mut().inactive.fill = color;
        self
    }

    ///设置背景的样式
    pub fn set_style(&mut self, style: FrameStyle) {
        self.visual.enable();
        self.visual.style_mut().inactive.fill = style.fill;
        self.visual.style_mut().inactive.border = style.border;
        self.visual.style_mut().inactive.shadow = style.shadow;
        self.visual.style_mut().inactive.radius = style.radius;
    }

    ///设置布局宽度
    pub fn set_width(&mut self, w: f32) {
        self.geometry.set_fix_width(w);
    }

    ///设置布局的高度
    pub fn with_height(mut self, h: f32) -> Self {
        self.set_height(h);
        self
    }

    ///设置布局的高度
    pub fn set_height(&mut self, h: f32) {
        self.geometry.set_fix_height(h);
    }

    ///设置每个item之间的间隔
    pub fn with_space(mut self, s: f32) -> Self {
        self.item_space = s;
        self
    }

    pub fn with_padding(mut self, p: Padding) -> Self {
        self.set_padding(p);
        self
    }

    pub fn set_padding(&mut self, p: Padding) {
        self.geometry.set_padding(p);
    }

    pub fn set_margin(&mut self, m: Margin) {
        self.geometry.set_margin(m);
    }

    pub fn item_space(&self) -> f32 {
        self.item_space
    }

    fn reset_display(&mut self) {
        let context_rect = self.geometry.context_rect();
        let mut sum_height = 0.0;
        for (index, item) in self.items.iter().enumerate() {
            let item_min_y = context_rect.dy().min + self.offset.current.y + sum_height;
            let item_max_y = context_rect.dy().min + self.offset.current.y + sum_height + item.height() + self.item_space;
            if item_min_y <= context_rect.dy().min && item_max_y > context_rect.dy().min {
                self.display.start = index;
                self.offset.context.y = item_min_y - context_rect.dy().min;
            }
            if item_min_y < context_rect.dy().max && item_max_y >= context_rect.dy().max {
                self.display.end = index;
                break;
            }
            sum_height = sum_height + item.height() + self.item_space;
        }
        if self.display.end == 0 && self.items.len() != 0 { self.display.end = self.items.len() - 1; }
        self.offset.offsetting = false;
    }
}

impl Layout for VerticalLayout {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        self.geometry.offset_to_rect(&ui.draw_rect);
        let mut width = 0.0;
        let mut height = 0.0;
        match ui.update_type {
            UpdateType::Init => {
                for item in self.items.iter() {
                    if width < item.width() { width = item.width(); }
                    height += item.height() + self.item_space;
                }
                self.geometry.set_context_size(width, height);
                self.visual.rect_mut().set_size(self.geometry.padding_width(), self.geometry.padding_height());
            }
            _ => {
                self.geometry.set_margin_size(ui.draw_rect.width(), ui.draw_rect.height());
                if let UpdateType::Draw = ui.update_type {
                    self.visual.rect_mut().offset_to_rect(&ui.draw_rect);
                    self.visual.draw(ui, false, false, false, false);
                }
                if self.offset.offsetting { self.reset_display(); }
                let mut context_rect = self.geometry.context_rect().with_y_direction(self.direction);
                context_rect.offset(&self.offset.context);
                let previous_rect = mem::replace(&mut ui.draw_rect, context_rect);
                for i in self.display.start..=self.display.end {
                    let resp = self.items[i].update(ui);
                    if width < resp.size.dw { width = resp.size.dw; }
                    height += resp.size.dh + self.item_space;
                    match self.direction {
                        LayoutDirection::Min => ui.draw_rect.add_min_y(resp.size.dh + self.item_space),
                        LayoutDirection::Max => ui.draw_rect.add_max_y(-resp.size.dh - self.item_space),
                    }
                }
                height -= self.item_space;
                ui.draw_rect = previous_rect;
            }
        }
        self.geometry.set_context_size(width, height);
        Response::new(&self.id, WidgetSize {
            dw: self.geometry.margin_width(),
            dh: self.geometry.margin_height(),
            rw: width,
            rh: height,
        })
    }
    fn items(&self) -> &Map<String, LayoutItem> {
        &self.items
    }

    fn items_mut(&mut self) -> &mut Map<String, LayoutItem> {
        &mut self.items
    }

    fn add_item(&mut self, mut item: LayoutItem) {
        if let Some(space) = item.widget_mut::<Space>() {
            space.geometry().set_context_width(0.0);
            item.set_width(0.0);
        }
        self.items.insert(item.id().to_string(), item);
    }

    fn set_offset(&mut self, offset: Offset) {
        self.offset.next_offset(offset);
    }

    fn set_size(&mut self, w: f32, h: f32) {
        self.set_width(w);
        self.set_height(h);
    }
}