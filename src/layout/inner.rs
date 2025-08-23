use crate::frame::context::UpdateType;
use crate::frame::App;
use crate::layout::popup::Popup;
use crate::layout::{HorizontalLayout, Layout, LayoutKind, VerticalLayout};
use crate::map::Map;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Callback;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::pos::Pos;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle, Shadow};
use crate::ui::Ui;
use crate::widgets::button::Button;
use crate::Offset;
use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct InnerWindow {
    pub(crate) id: String,
    fill_render: RenderParam<RectParam>,
    title_rect: Rect,
    offset: Offset,
    press_title: bool,
    change: bool,
    pub(crate) on_close: Option<Box<dyn FnMut(&mut Box<dyn App>, InnerWindow, &mut Ui)>>,
    w: Box<dyn App>,
    layout: Option<LayoutKind>,
    popups: Option<Map<Popup>>,
    inner_windows: Option<Map<InnerWindow>>,
    pub(crate) request_close: Arc<AtomicBool>,
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
        let fill_param = RectParam::new(rect.clone(), Popup::popup_style())
            .with_shadow(shadow);
        let mut fill_render = RenderParam::new(fill_param);
        fill_render.init_rectangle(ui, false, false);
        let padding = Padding::same(5.0);
        let mut layout = VerticalLayout::new();
        layout.max_rect = rect;
        layout.available_rect = layout.max_rect.clone_add_padding(&padding);
        let layout = LayoutKind::Vertical(layout);
        let mut window = InnerWindow {
            id: crate::gen_unique_id(),
            fill_render,
            layout: Some(layout),
            popups: Some(Map::new()),
            title_rect: Rect::new(),
            offset: Offset::new(Pos::new()).delete_offset(),
            press_title: false,
            change: false,
            on_close: None,
            w: Box::new(w),
            inner_windows: Some(Map::new()),
            request_close: Arc::new(AtomicBool::new(false)),
        };
        window.draw_title(ui);
        window.draw_context(ui);
        window
    }

    fn draw_title(&mut self, ui: &mut Ui) {
        let mut title_layout = HorizontalLayout::left_to_right();
        title_layout.max_rect = self.fill_render.param.rect.clone();
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
        let mut title_layout = ui.layout.take().unwrap(); //防止crash
        let mut rect = title_layout.available_rect().clone();
        rect.set_x_max(title_layout.max_rect().dx().max);
        let mut title_close_layout = HorizontalLayout::right_to_left().max_rect(title_layout.available_rect().clone(), Padding::ZERO);
        title_close_layout.item_space = 0.0;
        ui.layout = Some(LayoutKind::Horizontal(title_close_layout));
        let mut style = ClickStyle::new();
        style.fill.inactive = Color::TRANSPARENT;
        style.fill.hovered = Color::rgba(255, 0, 0, 100);
        style.fill.clicked = Color::rgba(255, 0, 0, 150);
        style.border = BorderStyle::same(Border::new(0.0).radius(Radius::same(0)));
        let mut btn = Button::new("×").width(20.0).height(20.0);
        btn.set_style(style.clone());
        let closed = self.request_close.clone();
        btn.set_inner_callback(move || {
            closed.store(true, Ordering::SeqCst);
        });
        ui.add(btn);
        let mut btn = Button::new("□").width(20.0).height(20.0);
        style.fill.hovered = Color::rgba(160, 160, 160, 100);
        style.fill.clicked = Color::rgba(160, 160, 160, 150);
        btn.set_style(style.clone());
        ui.add(btn);
        // let mut btn = Button::new("-").width(20.0).height(20.0);
        // btn.set_style(style);
        // ui.add(btn);
        let title_close_layout = ui.layout.take().unwrap();


        ui.update_type = UpdateType::None;
        ui.layout = previous_layout;
        title_layout.add_child(title_close_layout);
        self.layout.as_mut().unwrap().add_child(title_layout);
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
        self.layout.as_mut().unwrap().add_child( context_layout);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.change { return; }
        self.change = false;
        self.fill_render.update(ui, false, false);
    }

    fn window_update(&mut self, ui: &mut Ui) -> bool {
        match ui.update_type {
            UpdateType::MouseMove => {
                if self.press_title {
                    let (ox, oy) = ui.device.device_input.mouse.offset();
                    self.offset.x = ox;
                    self.offset.y = oy;
                    self.offset.pos = ui.device.device_input.mouse.lastest;
                    self.fill_render.param.rect.offset(&self.offset);
                    self.title_rect.offset(&self.offset);
                    ui.update_type = UpdateType::Offset(self.offset.clone());
                    ui.can_offset = true;
                    self.change = true;
                    return false;
                }
            }
            UpdateType::MousePress => {
                self.press_title = ui.device.device_input.pressed_at(&self.title_rect);
                if self.press_title { return false; }
            }
            UpdateType::MouseRelease => {
                self.press_title = false;
                if ui.device.device_input.hovered_at(&self.title_rect) { return false; }
            }
            _ => {}
        }
        false
    }

    pub fn to_<W: 'static>(self) -> W {
        let app: Box<dyn Any> = self.w;
        let app = app.downcast().unwrap();
        *app
    }

    pub fn on_close<A: App>(&mut self, f: impl FnMut(&mut A, InnerWindow, &mut Ui) + 'static) {
        self.on_close = Some(Callback::create_inner_close(f));
    }

    pub fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.layout.as_mut().unwrap().redraw(ui);
        self.w.redraw(ui);
    }

    pub fn update(&mut self, oui: &mut Ui) {
        if self.window_update(oui) { return; }
        if !oui.device.device_input.hovered_at(&self.fill_render.param.rect) && !self.press_title { return; }
        let mut nui = Ui {
            device: oui.device,
            context: oui.context,
            app: None,
            pass: None,
            layout: self.layout.take(),
            popups: self.popups.take(),
            current_rect: self.fill_render.param.rect.clone(),
            update_type: oui.update_type.clone(),
            can_offset: oui.can_offset,
            inner_windows: self.inner_windows.take(),
            request_update: None,
            offset: Offset::new(Pos::new()),
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