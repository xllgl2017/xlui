use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutDirection, LayoutItem};
use crate::map::Map;
use crate::render::Visual;
use crate::response::Response;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::style::FrameStyle;
use crate::ui::Ui;
use crate::widgets::space::Space;
use crate::widgets::WidgetSize;
use crate::*;
use std::mem;

///### 水平布局的使用
///```rust
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///    //快速创建一个左到右的水平布局
///    ui.horizontal(|ui|{
///        //添加布局内容
///    });
///    //创建一个从右到左的布局
///    let layout=HorizontalLayout::right_to_left()
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

pub struct HorizontalLayout {
    id: String,
    items: Map<String, LayoutItem>,
    item_space: f32, //item之间的间隔
    geometry: Geometry,
    direction: LayoutDirection,
    offset: Offset,
    visual: Visual,
    window: bool,
    pressed: bool,
    press_pos: Pos,
}

impl HorizontalLayout {
    fn new(direction: LayoutDirection) -> HorizontalLayout {
        HorizontalLayout {
            id: gen_unique_id(),
            items: Map::new(),
            item_space: 5.0,
            direction,
            geometry: Geometry::new(),
            offset: Offset::new(),
            visual: Visual::new(),
            window: false,
            pressed: false,
            press_pos: Pos::new(),
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

    pub fn with_size(self, w: f32, h: f32) -> Self {
        self.with_width(w).with_height(h)
    }


    pub fn with_fill(mut self, color: Color) -> Self {
        self.visual.style_mut().inactive.fill = color;
        // let mut style = ClickStyle::new();
        // style.fill = FillStyle::same(color);
        // style.border = BorderStyle::same(Border::same(0.0).radius(Radius::same(0)));
        // let fill_render = RenderParam::new(RenderKind::Rectangle(RectParam::new().with_style(style)));
        // self.fill_render = Some(fill_render);
        self
    }

    ///设置背景的样式
    pub fn set_style(&mut self, style: FrameStyle) {
        self.visual.style_mut().inactive.fill = style.fill;
        self.visual.style_mut().inactive.border = style.border;
        self.visual.style_mut().inactive.shadow = style.shadow;
        self.visual.style_mut().inactive.radius = style.radius;
        // match self.fill_render {
        //     None => {
        //         let fill_render = RenderParam::new(RenderKind::Rectangle(RectParam::new_frame(Rect::new(), style)));
        //         self.fill_render = Some(fill_render);
        //     }
        //     Some(ref mut render) => render.set_frame_style(style),
        // }
    }


    pub fn with_width(mut self, w: f32) -> Self {
        self.set_width(w);
        self
    }
    ///设置布局的宽度
    pub fn set_width(&mut self, w: f32) {
        self.geometry.set_fix_width(w);
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.set_height(h);
        self
    }

    ///设置布局的高度
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

    pub fn item_space(&self) -> f32 {
        self.item_space
    }

    pub fn moving(mut self) -> Self {
        self.window = true;
        self
    }
}

impl Layout for HorizontalLayout {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        self.geometry.offset_to_rect(&ui.draw_rect);
        let mut width = 0.0;
        let mut height = 0.0;
        match ui.update_type {
            UpdateType::Init => {
                for item in self.items.iter() {
                    if height < item.height() { height = item.height(); }
                    width += item.width() + self.item_space;
                }
                width -= self.item_space;
                self.geometry.set_context_size(width, height);
                self.visual.rect_mut().set_size(self.geometry.padding_width(), self.geometry.padding_height());
                // self.visual.draw(ui, false, false, false, false);
                // if let Some(ref mut render) = self.fill_render {
                //     self.geometry.set_context_size(width, height);
                //     render.rect_mut().set_size(self.geometry.padding_width(), self.geometry.padding_height());
                //     // #[cfg(feature = "gpu")]
                //     // render.init(ui, false, false);
                // }
            }
            _ => {
                self.geometry.set_margin_size(ui.draw_rect.width(), ui.draw_rect.height());
                if let UpdateType::MouseMove = ui.update_type && self.window && self.pressed {
                    ui.context.window.request_redraw();
                }
                if let UpdateType::MousePress = ui.update_type {
                    self.pressed = ui.device.device_input.pressed_at(&ui.draw_rect);
                    self.press_pos.x = ui.device.device_input.mouse.lastest.relative.x - ui.draw_rect.dx().min;
                    self.press_pos.y = ui.device.device_input.mouse.lastest.relative.y - ui.draw_rect.dy().min;
                }
                if let UpdateType::MouseRelease = ui.update_type {
                    self.pressed = false;
                }
                #[cfg(not(feature = "winit"))]
                if let UpdateType::Draw = ui.update_type && self.window && self.pressed {
                    let x = ui.device.device_input.mouse.lastest.absolute.x - self.press_pos.x;
                    let y = ui.device.device_input.mouse.lastest.absolute.y - self.press_pos.y;
                    #[cfg(target_os = "linux")]
                    ui.context.window.x11().move_window(x, y);
                }
                //设置布局padding
                let mut rect = self.geometry.context_rect().with_x_direction(self.direction);

                if let UpdateType::Draw = ui.update_type {
                    self.visual.rect_mut().offset_to_rect(&rect);
                    self.visual.draw(ui, false, false, false, false);
                    // render.rect_mut().offset_to_rect(&rect);
                    // render.draw(ui, false, false);
                }
                rect.offset(&self.offset);
                let previous_rect = mem::replace(&mut ui.draw_rect, rect);
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
            space.geometry().set_context_height(0.0);
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