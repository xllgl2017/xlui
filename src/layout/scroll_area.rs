use crate::layout::{Layout, LayoutKind, VerticalLayout};
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::scroll::bar::ScrollBar;
use crate::widgets::Widget;
use crate::Offset;

pub struct ScrollArea {
    id: String,
    pub(crate) rect: Rect,
    layout: Option<VerticalLayout>,
    padding: Padding,
    v_bar: ScrollBar,
    fill_index: usize,
    fill_param: RectParam,
    fill_buffer: Option<wgpu::Buffer>,
}

impl ScrollArea {
    pub fn new() -> ScrollArea {
        let mut fill_style = ClickStyle::new();
        fill_style.border.inactive = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        ScrollArea {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            layout: None,
            padding: Padding::same(5.0),
            v_bar: ScrollBar::new(),
            fill_index: 0,
            fill_param: RectParam::new(Rect::new(), fill_style),
            fill_buffer: None,
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.rect.set_size(width, height);
        self.v_bar.set_height(height);
        self
    }

    pub fn show(mut self, ui: &mut Ui, callback: impl Fn(&mut Ui)) {
        //滚动区域
        let rect = ui.layout().available_rect().clone_with_size(&self.rect);
        self.fill_param.rect = rect.clone();
        self.fill_param.rect.x.max = self.fill_param.rect.x.max - 10.0 - 2.0;
        let data = self.fill_param.as_draw_param(false, false);
        let buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &buffer);
        self.fill_buffer = Some(buffer);
        let current_layout = VerticalLayout::new().max_rect(rect, self.padding.clone());
        let mut previous_layout = ui.layout.replace(LayoutKind::Vertical(current_layout)).unwrap();


        // let mut previous_layout = ui.current_layout.take().unwrap();
        // self.rect = previous_layout.available_rect.clone_with_size(&self.rect);
        // self.rect.set_width(300.0);
        // self.rect.set_height(400.0);

        // let v_bar_width = self.v_bar.rect.width();
        // self.v_bar.rect = self.rect.clone();
        // self.v_bar.rect.x.min = self.v_bar.rect.x.max - v_bar_width; //outer


        // let mut current_layout = self.layout.take().unwrap();
        // current_layout.max_rect = self.rect.clone_add_padding(&self.padding);
        // current_layout.max_rect.set_width(self.rect.width() - v_bar_width - 2.0 - self.padding.horizontal());
        // current_layout.available_rect = current_layout.max_rect.clone();

        // ui.current_layout.replace(current_layout);
        // ui.current_scrollable = true;
        //视图内容
        callback(ui);
        // previous_layout.alloc_rect(&self.fill_param.rect);
        let current_layout = ui.layout.replace(previous_layout).unwrap();
        match current_layout {
            LayoutKind::Vertical(v) => { self.layout.replace(v); }
            _ => {}
        }
        // self.layout.replace(current_layout);

        //滚动条
        let mut v_bar_rect = self.fill_param.rect.clone();
        v_bar_rect.x.min = v_bar_rect.x.max + 2.0;
        v_bar_rect.set_width(10.0);
        self.v_bar = self.v_bar.with_rect(v_bar_rect).context_height(self.layout.as_ref().unwrap().height + self.padding.vertical());
        self.v_bar.draw(ui);

        ui.layout().add_child(self.id.clone(), LayoutKind::ScrollArea(self));
        // previous_layout
        // let mut current_layout = ui.current_layout.take().unwrap();
        // previous_layout.alloc_rect(&self.rect); //分配大小
        // previous_layout.alloc_layout(&current_layout);
        // ui.current_layout.replace(previous_layout);
        // ui.current_scrollable = false;
        // current_layout.children.append(&mut ui.scroll_layouts);
        // self.layout.replace(current_layout);
        // self.layout.insert(0, current_layout);
        // self.layout.append(&mut ui.scroll_layouts);
        // self.draw(ui);
    }


    // fn draw(self, ui: &mut Ui) {
    //     let id = self.id.clone();
    //     // ui.response.insert(self.id.clone(), ButtonResponse::new(self.rect.clone()).event(DrawnEvent::Click));
    //     let task = PaintScrollArea::new(self, ui);
    //     ui.add_paint_task(id, PaintTask::ScrollArea(task));
    // }
}

impl Layout for ScrollArea {
    fn update(&mut self, ui: &mut Ui) {
        if ui.device.device_input.pressed_at(&self.fill_param.rect) {
            let oy = ui.device.device_input.mouse.offset_y();
            ui.canvas_offset = Some(Offset::new_y(-oy));
        }
        self.v_bar.update(ui);
        self.layout.as_mut().unwrap().update(ui);
        ui.canvas_offset = None;
    }

    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        //滚动区域
        ui.context.render.rectangle.render(self.fill_index, pass);
        //滚动条
        self.v_bar.redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        //视图内容
        let clip = self.fill_param.rect.clone_add_padding(&self.padding);
        pass.set_scissor_rect(clip.x.min as u32, clip.y.min as u32, clip.width() as u32, clip.height() as u32);
        self.layout.as_mut().unwrap().redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        pass.set_scissor_rect(0, 0, ui.context.size.width, ui.context.size.height);
        // self.layout.as_mut().unwrap().redraw(ui);
    }
}