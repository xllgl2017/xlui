use crate::frame::context::UpdateType;
use crate::frame::App;
use crate::layout::popup::Popup;
use crate::layout::{LayoutItem, LayoutKind};
use crate::map::Map;
use crate::render::{Visual, VisualStyle, WidgetStyle};
use crate::response::Callback;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::{FrameStyle, Shadow};
use crate::ui::Ui;
use crate::widgets::button::Button;
use crate::widgets::WidgetChange;
use crate::window::attribute::WindowAttribute;
use crate::window::WindowId;
use crate::{HorizontalLayout, Offset, VerticalLayout, Widget};
use std::any::Any;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct InnerWindow {
    pub(crate) id: WindowId,
    pub(crate) visual: Visual,
    attr: WindowAttribute,
    title_rect: Rect,
    offset: Offset,
    pub(crate) press_title: bool,
    changed: bool,
    pub(crate) on_close: Option<Box<dyn FnMut(&mut Box<dyn App>, InnerWindow, &mut Ui)>>,
    w: Box<dyn App>,
    layout: Option<LayoutKind>,
    popups: Option<Map<String, Popup>>,
    inner_windows: Option<Map<WindowId, InnerWindow>>,
    pub(crate) request_close: Arc<AtomicBool>,
    pub(crate) top: bool,
}

impl InnerWindow {
    pub fn new(w: impl App, ui: &mut Ui) -> Self {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            blur: 1.0,
            color: Color::rgba(0, 0, 0, 30),
        };
        let attr = w.window_attributes();
        let mut rect = Rect::new().with_size(attr.inner_width_f32(), attr.inner_height_f32());
        rect.offset_to(attr.pos_x_f32(), attr.pos_y_f32());
        let style = VisualStyle::same(WidgetStyle {
            fill: Color::rgb(240, 240, 240),
            border: Border::same(1.0).color(Color::rgb(144, 209, 255)),
            radius: Radius::same(5),
            shadow,
        });
        let layout = VerticalLayout::top_to_bottom().with_size(rect.width(), rect.height());
        let mut window = InnerWindow {
            id: WindowId::unique_id(),
            visual: Visual::new().with_enable().with_style(style).with_rect(rect),
            layout: Some(LayoutKind::new(layout)),
            popups: Some(Map::new()),
            title_rect: Rect::new().with_size(attr.inner_width_f32(), 22.0),
            offset: Offset::new().covered(),
            press_title: false,
            changed: false,
            on_close: None,
            w: Box::new(w),
            inner_windows: Some(Map::new()),
            request_close: Arc::new(AtomicBool::new(false)),
            attr,
            top: false,
        };
        window.draw_title(ui);
        window.draw_context(ui);
        window
    }

    fn draw_title(&mut self, ui: &mut Ui) {
        let style = FrameStyle {
            fill: Color::rgb(210, 210, 210),
            shadow: Shadow::new(),
            border: Border::same(0.0),
            radius: Radius::same(0).with_left_top(1).with_right_top(1),
        };
        let mut title_layout = HorizontalLayout::left_to_right()
            .with_size(self.visual.rect().width(), 22.0)
            .with_padding(Padding::ZERO.top(1.0).left(1.0));
        title_layout.set_style(style);
        let title_layout = LayoutKind::new(title_layout);
        let previous_layout = ui.layout.replace(title_layout);
        ui.update_type = UpdateType::Init;
        ui.image("logo.jpg", (16.0, 16.0));
        ui.label(self.attr.title.as_str());
        ui.add_layout(HorizontalLayout::right_to_left(), |ui| {
            let mut style = VisualStyle::same(WidgetStyle {
                fill: Color::TRANSPARENT,
                border: Border::same(0.0),
                radius: Radius::same(0),
                shadow: Shadow::new(),
            });
            style.hovered.fill = Color::rgba(255, 0, 0, 100);
            style.pressed.fill = Color::rgba(255, 0, 0, 150);
            let mut btn = Button::new("×").width(20.0).height(20.0);
            btn.set_style(style.clone());
            let closed = self.request_close.clone();
            btn.set_inner_callback(move || {
                println!("request close");
                closed.store(true, Ordering::SeqCst);
            });
            ui.add(btn);

            let mut btn = Button::new("□").width(20.0).height(20.0);
            style.hovered.fill = Color::rgba(160, 160, 160, 100);
            style.pressed.fill = Color::rgba(160, 160, 160, 150);
            btn.set_style(style.clone());
            ui.add(btn);
        });

        let mut title_layout = ui.layout.take().unwrap(); //防止crash
        title_layout.update(ui);
        ui.update_type = UpdateType::None;
        ui.layout = previous_layout;
        self.layout.as_mut().unwrap().add_item(LayoutItem::Layout(title_layout));
    }

    fn draw_context(&mut self, oui: &mut Ui) {
        let context_layout = VerticalLayout::top_to_bottom().with_padding(Padding::same(5.0));
        let mut nui = Ui {
            device: oui.device,
            context: oui.context,
            app: None,
            layout: Some(LayoutKind::new(context_layout)),
            popups: self.popups.take(),
            update_type: UpdateType::Init,
            can_offset: false,
            inner_windows: None,
            request_update: None,
            draw_rect: self.visual.rect().clone(),
            widget_changed: WidgetChange::None,
            paint: None,
            disabled: false,
        };


        nui.update_type = UpdateType::Init;
        self.w.draw(&mut nui);
        let mut context_layout = nui.layout.take().unwrap();
        context_layout.update(&mut nui);
        nui.update_type = UpdateType::None;
        self.popups = nui.popups.take();
        self.layout.as_mut().unwrap().add_item(LayoutItem::Layout(context_layout));
    }

    fn window_update(&mut self, ui: &mut Ui) -> bool {
        match ui.update_type {
            #[cfg(feature = "gpu")]
            UpdateType::ReInit => self.fill_render.re_init(),
            UpdateType::MouseMove => {
                if self.press_title && ui.device.device_input.mouse.pressed {
                    let (ox, oy) = ui.device.device_input.mouse.offset();
                    self.visual.rect_mut().offset(&Offset::new().with_x(ox).with_y(oy).covered());
                    self.offset.x += ox;
                    self.offset.y += oy;
                    self.changed = true;
                    ui.context.window.request_redraw();
                    return false;
                }
            }
            UpdateType::MousePress => {
                self.top = ui.device.device_input.pressed_at(self.visual.rect());
                self.press_title = ui.device.device_input.pressed_at(&self.title_rect) && self.top;
                ui.context.window.request_redraw();
                if self.press_title { return false; }
            }
            UpdateType::MouseRelease => self.press_title = false,
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

    pub fn redraw(&mut self, oui: &mut Ui) {
        let mut nui = Ui {
            device: oui.device,
            context: oui.context,
            app: None,
            layout: self.layout.take(),
            popups: self.popups.take(),
            update_type: UpdateType::Draw,
            can_offset: false,
            inner_windows: None,
            request_update: None,
            draw_rect: self.visual.rect().clone(),
            widget_changed: WidgetChange::None,
            paint: oui.paint.take(),
            disabled: false,
        };

        self.title_rect.offset_to_rect(&nui.draw_rect);
        self.visual.draw(&mut nui, false, false, false, false);
        self.w.update(&mut nui);
        self.layout = nui.layout.take();
        self.layout.as_mut().unwrap().update(&mut nui);
        self.offset.x = 0.0;
        self.offset.y = 0.0;
        self.popups = nui.popups.take();
        oui.paint = nui.paint.take();
    }

    pub fn update(&mut self, oui: &mut Ui) {
        if self.window_update(oui) { return; }
        if !oui.device.device_input.hovered_at(self.visual.rect()) && !self.press_title { return; }
        let mut nui = Ui {
            device: oui.device,
            context: oui.context,
            app: None,
            layout: self.layout.take(),
            popups: self.popups.take(),
            update_type: oui.update_type.clone(),
            can_offset: oui.can_offset,
            inner_windows: self.inner_windows.take(),
            request_update: None,
            draw_rect: self.visual.rect().clone(),
            widget_changed: WidgetChange::None,
            paint: None,
            disabled: false,
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