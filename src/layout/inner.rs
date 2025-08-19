use crate::frame::context::UpdateType;
use crate::frame::{App, WindowAttribute};
use crate::layout::{HorizontalLayout, Layout, LayoutKind, VerticalLayout};
use crate::layout::popup::Popup;
use crate::map::Map;
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
    attr: WindowAttribute,
    title_rect: Rect,
    offset: Offset,
    press_title: bool,
    change: bool,
    w: Box<dyn App>,
    layout: Option<LayoutKind>,
    popups: Option<Map<Popup>>,
    inner_windows: Option<Map<InnerWindow>>,
}

impl InnerWindow {
    pub fn new(w: impl App, ui: &mut Ui) -> Self {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            color: Color::rgba(0, 0, 0, 30),
        };
        let attr = w.window_attributes();
        let mut rect = Rect::new().with_size(attr.inner_width_f32(), attr.inner_height_f32());
        rect.add_min_x(attr.pos_x_f32());
        rect.add_min_y(attr.pos_y_f32());
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
            layout: Some(layout),
            popups: Some(Map::new()),
            padding,
            attr: Default::default(),
            title_rect: Rect::new(),
            offset: Offset::new(Pos::new()).delete_offset(),
            press_title: false,
            change: false,
            w: Box::new(w),
            inner_windows: Some(Map::new()),
        };
        window.draw_title(ui);
        window.draw_context(ui);
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
        self.layout.as_mut().unwrap().add_child(crate::gen_unique_id(), title_layout);
    }

    fn draw_context(&mut self, ui: &mut Ui) {
        let context_rect = self.layout.as_ref().unwrap().available_rect().clone();
        let mut context_layout = VerticalLayout::new();
        context_layout.max_rect = context_rect;
        context_layout.available_rect = context_layout.max_rect.clone();
        let previous_layout = ui.layout.replace(LayoutKind::Vertical(context_layout));
        ui.update_type = UpdateType::Init;
        self.w.draw(ui);
        let context_layout = ui.layout.take().unwrap();
        ui.update_type = UpdateType::None;
        ui.layout = previous_layout;
        self.layout.as_mut().unwrap().add_child(crate::gen_unique_id(), context_layout);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.change { return; }
        self.change = false;
        let data = self.fill_param.as_draw_param(false, false);
        ui.device.queue.write_buffer(&self.fill_buffer, 0, data);
    }

    fn window_update(&mut self, ui: &mut Ui) -> bool {
        match ui.update_type {
            UpdateType::MouseMove => {
                if self.press_title {
                    let (ox, oy) = ui.device.device_input.mouse.offset();
                    self.offset.x = ox;
                    self.offset.y = oy;
                    self.offset.pos = ui.device.device_input.mouse.lastest;
                    self.fill_param.rect.offset(&self.offset);
                    self.title_rect.offset(&self.offset);
                    ui.update_type = UpdateType::Offset(self.offset.clone());
                    ui.can_offset = true;
                    self.change = true;
                    return false;
                }
            }
            UpdateType::MousePress => {
                self.press_title = ui.device.device_input.pressed_at(&self.title_rect);
                if self.press_title { return true; }
            }
            UpdateType::MouseRelease => {
                self.press_title = false;
                if ui.device.device_input.hovered_at(&self.title_rect) { return true; }
            }
            _ => {}
        }
        false
    }

    pub fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        self.layout.as_mut().unwrap().redraw(ui);
        self.w.redraw(ui);
    }

    pub fn update(&mut self, oui: &mut Ui) {
        if self.window_update(oui) { return; }
        if !oui.device.device_input.hovered_at(&self.fill_param.rect) && !self.press_title { return; }
        let mut nui = Ui {
            device: oui.device,
            context: oui.context,
            app: None,
            pass: None,
            layout: self.layout.take(),
            popups: self.popups.take(),
            current_rect: self.fill_param.rect.clone(),
            update_type: oui.update_type.clone(),
            can_offset: oui.can_offset,
            inner_windows: self.inner_windows.take(),
            request_update: None,
        };

        self.w.update(&mut nui);
        nui.app = Some(&mut self.w);
        self.inner_windows = nui.inner_windows.take();
        for inner_window in self.inner_windows.as_mut().unwrap().iter_mut() {
            inner_window.update(&mut nui);
        }
        nui.inner_windows = self.inner_windows.take();
        self.popups = nui.popups.take();
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            popup.update(&mut nui);
        }
        nui.popups = self.popups.take();
        self.layout = nui.layout.take();
        self.layout.as_mut().unwrap().update(&mut nui);
        self.popups = nui.popups.take();
        self.inner_windows = nui.inner_windows.take();
        oui.request_update = nui.request_update;
        oui.update_type = UpdateType::None;
    }
}