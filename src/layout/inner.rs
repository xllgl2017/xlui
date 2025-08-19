use crate::frame::context::UpdateType;
use crate::frame::WindowAttribute;
use crate::layout::{HorizontalLayout, LayoutKind, VerticalLayout};
use crate::layout::popup::Popup;
use crate::Offset;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle, Shadow};
use crate::ui::Ui;

pub struct InnerWindow {
    pub(crate) id: String,
    padding: Padding,
    fill_index: usize,
    fill_param: RectParam,
    fill_buffer: wgpu::Buffer,
    layout: LayoutKind,
    attr: WindowAttribute,
    title_rect: Rect,
    offset: Offset,
    press_title: bool,
    change: bool,
}

impl InnerWindow {
    pub fn new(ui: &mut Ui) -> Self {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            color: Color::rgba(0, 0, 0, 30),
        };
        let mut rect = Rect::new().with_size(500.0, 400.0);
        rect.add_min_x(20.0);
        rect.add_min_y(20.0);
        let mut fill_param = RectParam::new(rect.clone(), Popup::popup_style())
            .with_shadow(shadow);
        let data = fill_param.as_draw_param(false, false);
        let fill_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        let fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        let padding = Padding::same(5.0);
        let mut layout = VerticalLayout::new();
        layout.max_rect = rect;
        layout.available_rect = layout.max_rect.clone_add_padding(&padding);
        let layout = LayoutKind::Vertical(layout);
        let mut window = InnerWindow {
            id: crate::gen_unique_id(),
            fill_index,
            fill_param,
            fill_buffer,
            layout,
            padding,
            attr: Default::default(),
            title_rect: Rect::new(),
            offset: Offset::new(Pos::new()),
            press_title: false,
            change: false,
        };
        window.draw_title(ui);
        window
    }

    fn draw_title(&mut self, ui: &mut Ui) {
        let mut title_layout = HorizontalLayout::new();
        title_layout.max_rect = self.fill_param.rect.clone();
        title_layout.max_rect.contract(1.0, 1.0); //向内缩小1像素
        title_layout.max_rect.set_height(22.0);
        title_layout.width = title_layout.max_rect.width();
        title_layout.height = 22.0;
        title_layout.available_rect = title_layout.max_rect.clone_add_padding(&Padding::same(2.0));
        self.title_rect = title_layout.max_rect.clone();
        let title_layout = LayoutKind::Horizontal(title_layout);
        let previous_layout = ui.layout.replace(title_layout);
        ui.update_type = UpdateType::Init;
        let title_style = ClickStyle {
            fill: FillStyle::same(Color::rgb(210, 210, 210)),
            border: BorderStyle::same(Border::new(0.0)),
        };
        ui.paint_rect(self.title_rect.clone(), title_style);
        ui.image("logo.jpg", (16.0, 16.0));
        ui.label("InnerWindow");
        let title_layout = ui.layout.take().unwrap(); //防止crash
        ui.update_type = UpdateType::None;
        ui.layout = previous_layout;
        self.layout.add_child(crate::gen_unique_id(), title_layout);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.change { return; }
        self.change = false;
        let data = self.fill_param.as_draw_param(false, false);
        ui.device.queue.write_buffer(&self.fill_buffer, 0, data);
    }

    pub fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        self.layout.redraw(ui);
    }

    pub fn update(&mut self, ui: &mut Ui) {
        match ui.update_type {
            UpdateType::Init | UpdateType::ReInit => self.layout.update(ui),
            UpdateType::MouseMove => {
                if self.press_title {
                    let (ox, oy) = ui.device.device_input.mouse.offset();
                    self.offset.x += ox;
                    self.offset.y += oy;
                    self.offset.pos = ui.device.device_input.mouse.lastest;
                    self.fill_param.rect.offset(&self.offset);
                    self.title_rect.offset(&self.offset);
                    ui.update_type = UpdateType::Offset(self.offset.clone());
                    ui.can_offset = true;
                    self.layout.update(ui);
                    self.change = true;
                }
                if !ui.device.device_input.hovered_at(&self.fill_param.rect) { return; }
                self.layout.update(ui);
                ui.update_type = UpdateType::None;
            }
            UpdateType::MousePress => {
                self.press_title = ui.device.device_input.pressed_at(&self.title_rect);
                if !ui.device.device_input.hovered_at(&self.fill_param.rect) { return; }
                self.layout.update(ui);
                ui.update_type = UpdateType::None;
            }
            UpdateType::MouseRelease => {
                self.press_title = false;
                if !ui.device.device_input.hovered_at(&self.fill_param.rect) { return; }
                self.layout.update(ui);
                ui.update_type = UpdateType::None;
            }
            UpdateType::MouseWheel => {
                if !ui.device.device_input.hovered_at(&self.fill_param.rect) { return; }
                self.layout.update(ui);
                ui.update_type = UpdateType::None;
            }
            _ => {}
        }
    }
}