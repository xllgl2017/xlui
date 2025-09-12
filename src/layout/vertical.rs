use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutDirection, LayoutItem};
use crate::map::Map;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::ui::Ui;
use crate::widgets::space::Space;
use crate::widgets::WidgetSize;
use crate::{Border, Offset, Padding, Pos, Radius, Rect};
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
pub struct VerticalLayout {
    id: String,
    items: Map<String, LayoutItem>,
    item_space: f32, //item之间的间隔
    offset: Offset,
    size_mode: SizeMode,
    padding: Padding,
    direction: LayoutDirection,
    fill_render: Option<RenderParam<RectParam>>,
}

impl VerticalLayout {
    fn new(direction: LayoutDirection) -> VerticalLayout {
        VerticalLayout {
            id: crate::gen_unique_id(),
            items: Map::new(),
            item_space: 5.0,
            size_mode: SizeMode::Auto,
            padding: Padding::same(0.0),
            direction,
            offset: Offset::new(Pos::new()),
            fill_render: None,
        }
    }

    pub fn top_to_bottom() -> VerticalLayout {
        VerticalLayout::new(LayoutDirection::Min)
    }

    pub fn bottom_to_top() -> VerticalLayout {
        VerticalLayout::new(LayoutDirection::Max)
    }

    pub fn with_size(self, w: f32, h: f32) -> Self {
        self.with_width(w).with_height(h)
    }


    pub fn with_width(mut self, w: f32) -> Self {
        self.set_width(w);
        self
    }

    pub fn with_fill(mut self, color: Color) -> Self {
        let mut style = ClickStyle::new();
        style.fill = FillStyle::same(color);
        style.border = BorderStyle::same(Border::new(0.0).radius(Radius::same(0)));
        let fill_render = RenderParam::new(RectParam::new(Rect::new(), style));
        self.fill_render = Some(fill_render);
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

    pub fn item_space(&self) -> f32 {
        self.item_space
    }
}

impl Layout for VerticalLayout {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        let previous_rect = ui.draw_rect.clone();
        let mut width = 0.0;
        let mut height = 0.0;
        match ui.update_type {
            UpdateType::Init => {
                for item in self.items.iter() {
                    if width < item.width() { width = item.width(); }
                    height += item.height() + self.item_space;
                }
                if let Some(ref mut render) = self.fill_render {
                    let (dw, dh) = self.size_mode.size(width, height);
                    render.param.rect.set_size(dw, dh);
                    render.init_rectangle(ui, false, false);
                }
            }
            _ => {
                if let UpdateType::Draw = ui.update_type && let Some(ref mut render) = self.fill_render {
                    render.param.rect.offset_to_rect(&previous_rect);
                    render.update(ui, false, false);
                    let pass = ui.pass.as_mut().unwrap();
                    ui.context.render.rectangle.render(&render, pass);
                }
                let (w, h) = self.size_mode.size(previous_rect.width(), previous_rect.height());
                ui.draw_rect.set_size(w, h);
                //设置布局padding
                ui.draw_rect.add_min_x(self.padding.left);
                ui.draw_rect.add_min_y(self.padding.top);
                ui.draw_rect.add_max_x(-self.padding.right);
                ui.draw_rect.add_max_y(-self.padding.bottom);
                ui.draw_rect.set_y_direction(self.direction);
                ui.draw_rect.set_x_min(ui.draw_rect.dx().min + self.offset.x);
                ui.draw_rect.set_y_min(ui.draw_rect.dy().min + self.offset.y);
                for item in self.items.iter_mut() {
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
    }
    fn items(&self) -> &Map<String, LayoutItem> {
        &self.items
    }

    fn items_mut(&mut self) -> &mut Map<String, LayoutItem> {
        &mut self.items
    }

    fn add_item(&mut self, mut item: LayoutItem) {
        if let Some(space) = item.widget_mut::<Space>() {
            space.set_width(0.0);
        }
        self.items.insert(item.id().to_string(), item);
    }

    fn set_offset(&mut self, offset: Offset) {
        self.offset = offset;
    }

    fn set_size(&mut self, w: f32, h: f32) {
        self.set_width(w);
        self.set_height(h);
    }
}