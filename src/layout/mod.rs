pub mod scroll_area;
pub mod popup;

use crate::frame::context::Context;
use crate::map::Map;
use crate::paint::PaintTask;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::Device;
use crate::frame::App;

pub enum LayoutDirection {
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}


pub struct Layout {
    direction: LayoutDirection,
    pub(crate) item_space: f32, //item之间的间隔
    pub max_rect: Rect,
    pub(crate) available_rect: Rect,
    padding: Padding,
    pub width: f32,
    pub height: f32,
    pub(crate) widgets: Map<PaintTask>,
    pub(crate) children: Vec<Layout>,
    display: Vec<usize>,
}

impl Layout {
    pub fn new(direction: LayoutDirection) -> Self {
        Self {
            direction,
            widgets: Map::new(),
            item_space: 5.0,
            available_rect: Rect::new(),
            max_rect: Rect::new(),
            padding: Padding::same(5.0),
            width: 0.0,
            height: 0.0,
            display: vec![],
            children: vec![],
        }
    }

    pub fn left_to_right() -> Self {
        let mut layout = Layout::new(LayoutDirection::LeftToRight);
        layout.available_rect.y.max = 0.0;
        layout
    }

    pub fn right_to_left() -> Self {
        Layout::new(LayoutDirection::RightToLeft)
    }

    pub fn top_to_bottom() -> Self {
        Layout::new(LayoutDirection::TopToBottom)
    }

    pub fn bottom_to_top() -> Self {
        Layout::new(LayoutDirection::BottomToTop)
    }

    pub(crate) fn alloc_rect(&mut self, rect: &Rect) {
        // self.ids.push(id);
        match self.direction {
            LayoutDirection::LeftToRight => {
                self.available_rect.x.min += rect.width() + self.item_space;
                self.width += rect.width() + if self.width == 0.0 { 0.0 } else { self.item_space };
                if self.height < rect.height() { self.height = rect.height(); }
                // if rect.width() < self.available_rect.width() {
                //     if self.available_rect.height() < rect.height() {
                //         self.available_rect.set_height(rect.height());
                //     }
                //     self.available_rect.x.min += rect.width() + self.item_space;
                // } else if rect.width() > self.available_rect.width() && rect.width() < self.max_rect.width() { //单行控件已满，自动换行
                //     self.available_rect.x.min = self.max_rect.x.min;
                //     self.available_rect.y.min = self.available_rect.y.max + self.item_space;
                //     if self.available_rect.y.max > self.max_rect.y.max { self.available_rect.y.max = self.max_rect.y.max; } //超过当前布局，设置最大
                //     self.available_rect.x.min += rect.width() + self.item_space;
                // } else {
                //     todo!()
                // }
                // if self.max_rect.height() - self.available_rect.height() > self.height { self.height = self.max_rect.height() - self.available_rect.height(); } //设置当前高度
                // if self.max_rect.width() - self.available_rect.width() > self.width { self.width = self.max_rect.width() - self.available_rect.width(); } //设置当前宽度
            }
            LayoutDirection::TopToBottom => {
                // let mut out_rect = self.available_rect.clone();
                // out_rect.set_size(rect.width(), rect.height());
                // if out_rect.x.max > self.available_rect.x.max { out_rect.x.max = self.available_rect.x.max; }
                // if out_rect.y.max > self.available_rect.y.max { out_rect.y.max = self.available_rect.y.max; }
                self.available_rect.y.min += rect.height() + self.item_space;
                if self.width < rect.width() { self.width = rect.width() + self.item_space; }
                self.height += rect.height() + if self.height == 0.0 { 0.0 } else { self.item_space };

                // out_rect
            }
            _ => { todo!() }
        }
    }

    pub(crate) fn alloc_layout(&mut self, other: &Layout) {
        match self.direction {
            LayoutDirection::LeftToRight => {
                self.available_rect.x.min += other.size().0 + self.item_space; //宽+间隔即下一个可用坐标起始值
                self.width += other.width + if self.width == 0.0 { 0.0 } else { self.item_space };
                if self.height < other.height { self.height = other.height; }
            }
            LayoutDirection::RightToLeft => {}
            LayoutDirection::TopToBottom => {
                self.available_rect.y.min += other.size().1 + self.item_space; //高+间隔即下一个可用坐标起始值
                if self.width < other.width { self.width = other.width; }
                self.height += other.height + if self.height == 0.0 { 0.0 } else { self.item_space };
            }
            LayoutDirection::BottomToTop => {}
        }
    }

    pub(crate) fn add_space(&mut self, space: f32) {
        match self.direction {
            LayoutDirection::LeftToRight => {
                self.available_rect.x.min += space;
                self.width += space;
            }
            LayoutDirection::RightToLeft => {}
            LayoutDirection::TopToBottom => {
                self.available_rect.y.min += space;
                self.height += space;
            }
            LayoutDirection::BottomToTop => {}
        }
    }

    pub(crate) fn size(&self) -> (f32, f32) {
        (self.width, self.height)
    }
    pub(crate) fn with_max_rect(mut self, rect: Rect) -> Self {
        self.max_rect = rect.clone();
        self.available_rect = rect;
        self
    }

    pub(crate) fn draw(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        for child in self.children.iter_mut() {
            child.draw(device, context, render_pass);
        }
        for index in &self.display {
            self.widgets[*index].draw(device, context, render_pass);
        }
    }

    pub(crate) fn offset(&mut self, device: &Device, ox: f32, oy: f32) {
        if ox == 0.0 && oy == 0.0 { return; }
        self.display.clear();
        let rect = self.drawn_rect();
        for child in self.children.iter_mut() {
            child.max_rect.offset(ox, oy);
            child.offset(device, ox, oy)
        }
        for (index, widget) in self.widgets.iter_mut().enumerate() {
            match widget {
                PaintTask::Text(paint_text) => { //text外部无response，如果添加response，此处需增加，否则在滚动视图中事件错误
                    paint_text.offset(ox, oy);
                    if !paint_text.rect.out_of_rect(&rect) { self.display.push(index); }
                }
                PaintTask::Button(paint_btn) => {
                    paint_btn.offset(device, ox, oy);
                    if !paint_btn.rect().out_of_rect(&rect) { self.display.push(index); }
                }
                PaintTask::Image(paint_image) => {
                    paint_image.offset(device, ox, oy);
                    if !paint_image.rect.out_of_rect(&rect) { self.display.push(index); }
                }
                PaintTask::Rectangle(paint_rect) => {
                    paint_rect.offset(device, ox, oy);
                    if !paint_rect.rect().out_of_rect(&rect) { self.display.push(index); }
                }
                _ => {}
            }
        }
    }

    pub(crate) fn insert_widget(&mut self, id: String, widget: PaintTask) {
        let out_of_max = widget.rect().out_of_rect(&self.max_rect);
        self.widgets.insert(id, widget);
        if out_of_max { return; }
        self.display.push(self.widgets.len() - 1)
    }


    fn drawn_rect(&self) -> Rect {
        let mut rect = self.max_rect.clone();
        rect.set_width(self.width);
        rect.set_height(self.height);
        rect
    }
}

impl Layout {
    pub(crate) fn mouse_move<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        for widget in self.widgets.iter_mut() {
            widget.mouse_move(device, context, app)
        }
    }

    pub(crate) fn mouse_down<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        for widget in self.widgets.iter_mut() {
            widget.mouse_down(device, context, app);
        }
    }

    pub(crate) fn mouse_release<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        for widget in self.widgets.iter_mut() {
            widget.mouse_release(device, context, app);
        }
    }

    pub(crate) fn window_resize(&mut self, device: &Device, context: &mut Context) {
        for widget in self.widgets.iter_mut() {
            match widget {
                PaintTask::SpinBox(paint_spinbox) => paint_spinbox.prepare(device, context),
                PaintTask::ComboBox(paint_combo) => paint_combo.resize(device, context),
                _ => {}
            }
        }
    }

    pub(crate) fn key_input<A: App>(&mut self, device: &Device, context: &mut Context, key: winit::keyboard::Key, app: &mut A) {
        for widget in self.widgets.iter_mut() {
            match widget {
                PaintTask::Text(_) => {}
                PaintTask::Image(_) => {}
                PaintTask::ScrollBar(_) => {}
                PaintTask::TextEdit(paint_edit) => paint_edit.key_input(device, context, key.clone(), app),
                PaintTask::SpinBox(pain_spinbox) => pain_spinbox.key_input(device, context, key.clone(), app),
                _ => {}
            }
        }
    }

    pub(crate) fn delta_input(&mut self, device: &Device, context: &Context) {
        for widget in self.widgets.iter_mut() {
            match widget {
                PaintTask::ScrollBar(_) => {}
                PaintTask::ScrollArea(paint_area) => paint_area.delta_input(device, context),
                _ => {}
            }
        }
    }
}
