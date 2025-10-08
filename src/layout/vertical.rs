use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutDirection, LayoutItem};
use crate::map::Map;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
#[cfg(feature = "gpu")]
use crate::render::WrcRender;
use crate::response::Response;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle, FrameStyle};
use crate::ui::Ui;
use crate::widgets::space::Space;
use crate::widgets::WidgetSize;
use crate::{Border, Margin, Offset, Padding, Radius, Rect, Widget};
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
    geometry: Geometry,
    direction: LayoutDirection,
    fill_render: Option<RenderParam>,
    marin: Margin,
}

impl VerticalLayout {
    fn new(direction: LayoutDirection) -> VerticalLayout {
        VerticalLayout {
            id: crate::gen_unique_id(),
            items: Map::new(),
            item_space: 5.0,
            geometry: Geometry::new(),
            direction,
            offset: Offset::new(),
            fill_render: None,
            marin: Margin::ZERO,
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
        style.border = BorderStyle::same(Border::same(0.0).radius(Radius::same(0)));
        let fill_render = RenderParam::new(RenderKind::Rectangle(RectParam::new().with_style(style)));
        self.fill_render = Some(fill_render);
        self
    }

    ///设置背景的样式
    pub fn set_style(&mut self, style: FrameStyle) {
        match self.fill_render {
            None => {
                let fill_render = RenderParam::new(RenderKind::Rectangle(RectParam::new_frame(Rect::new(), style)));
                self.fill_render = Some(fill_render);
            }
            Some(ref mut render) => render.set_frame_style(style),
        }
    }

    pub fn set_width(&mut self, w: f32) {
        self.geometry.set_fix_width(w);
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.set_height(h);
        self
    }

    pub fn set_height(&mut self, h: f32) {
        self.geometry.set_fix_height(h);
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
        self.geometry.set_padding(p);
    }

    pub fn set_margin(&mut self, m: Margin) {
        self.marin = m;
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
                    self.geometry.set_size(width, height);
                    // let (dw, dh) = self.size_mode.size(width + self.padding.horizontal() + self.marin.horizontal(), height + self.padding.vertical() + self.marin.vertical());
                    render.rect_mut().set_size(self.geometry.width() - self.marin.horizontal(), self.geometry.height() - self.marin.vertical());
                    #[cfg(feature = "gpu")]
                    render.init(ui, false, false);
                }
            }
            _ => {
                if let UpdateType::Draw = ui.update_type && let Some(ref mut render) = self.fill_render {
                    render.rect_mut().offset_to_rect(&previous_rect);
                    render.rect_mut().offset(&Offset::new().with_y(self.marin.top).with_x(self.marin.left));
                    // render.param.rect.add_min_x(self.marin.left);
                    // render.param.rect.add_min_y(self.marin.top);
                    // #[cfg(feature = "gpu")]
                    // render.update(ui, false, false);
                    // #[cfg(feature = "gpu")]
                    // let pass = ui.pass.as_mut().unwrap();
                    // #[cfg(feature = "gpu")]
                    // ui.context.render.rectangle.render(&render, pass);
                    render.draw(ui, false, false);
                }
                self.geometry.set_size(previous_rect.width(), previous_rect.height());
                // let (w, h) = self.size_mode.size(previous_rect.width(), previous_rect.height());
                ui.draw_rect.set_size(self.geometry.width() - self.marin.horizontal(), self.geometry.height() - self.marin.vertical());
                //设置布局padding
                ui.draw_rect.add_min_x(self.geometry.padding().left + self.marin.left);
                ui.draw_rect.add_min_y(self.geometry.padding().top + self.marin.top);
                ui.draw_rect.add_max_x(-self.geometry.padding().right - self.marin.right);
                ui.draw_rect.add_max_y(-self.geometry.padding().bottom - self.marin.bottom);
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
        // width += self.padding.horizontal() + self.marin.horizontal();
        // height += self.padding.vertical() + self.marin.vertical();
        self.geometry.set_size(width, height);
        // let (dw, dh) = self.size_mode.size(width, height);
        Response::new(&self.id, WidgetSize {
            dw: self.geometry.width() + self.marin.horizontal(),
            dh: self.geometry.height() + self.marin.vertical(),
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
            space.geometry().set_width(0.0);
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