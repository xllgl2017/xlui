use crate::frame::context::{Context, ContextUpdate, UpdateType};
use crate::frame::App;
use crate::layout::popup::Popup;
use crate::layout::{HorizontalLayout, Layout, LayoutKind, VerticalLayout};
use crate::map::Map;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::{ClickStyle, Style};
use crate::widgets::button::Button;
use crate::widgets::checkbox::CheckBox;
use crate::widgets::image::Image;
use crate::widgets::label::Label;
use crate::widgets::radio::RadioButton;
use crate::widgets::rectangle::Rectangle;
use crate::widgets::select::SelectItem;
use crate::widgets::slider::Slider;
use crate::widgets::spinbox::SpinBox;
use crate::widgets::{Widget, WidgetKind};
use crate::{Device, NumCastExt, Offset, SAMPLE_COUNT};
use std::any::Any;
use std::fmt::Display;
use std::ops::{AddAssign, DerefMut, Range, SubAssign};
use std::sync::atomic::Ordering;
use wgpu::{LoadOp, Operations, RenderPassDescriptor};
use crate::size::pos::Pos;
use crate::text::rich::RichText;
use crate::window::inner::InnerWindow;

pub struct AppContext {
    pub(crate) device: Device,
    pub(crate) layout: Option<LayoutKind>,
    pub(crate) popups: Option<Map<Popup>>,
    pub(crate) inner_windows: Option<Map<InnerWindow>>,
    pub(crate) style: Style,
    pub(crate) context: Context,
    previous_time: u128,
}

impl AppContext {
    pub fn new(device: Device, context: Context) -> AppContext {
        let layout = LayoutKind::Vertical(VerticalLayout::new()).with_size(context.size.width as f32, context.size.height as f32, Padding::same(5.0));
        AppContext {
            device,
            layout: Some(layout),
            popups: Some(Map::new()),
            inner_windows: Some(Map::new()),
            style: Style::light_style(),
            context,
            previous_time: 0,
        }
    }

    pub fn draw(&mut self, app: &mut Box<dyn App>) {
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            pass: None,
            layout: Some(self.layout.take().unwrap()),
            popups: self.popups.take(),
            current_rect: Rect::new(),
            update_type: UpdateType::Init,
            can_offset: false,
            inner_windows: None,
            request_update: None,
            offset: Offset::new(Pos::new()),

        };
        app.draw(&mut ui);
        self.layout = ui.layout.take();
        self.popups = ui.popups.take();
    }

    pub fn update(&mut self, ut: UpdateType, app: &mut Box<dyn App>) {
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            pass: None,
            layout: self.layout.take(),
            popups: None,
            current_rect: Rect::new(),
            update_type: ut,
            can_offset: false,
            inner_windows: self.inner_windows.take(),
            request_update: None,
            offset: Offset::new(Pos::new()),

        };
        app.update(&mut ui);

        self.inner_windows = ui.inner_windows.take();
        for i in 0..self.inner_windows.as_ref().unwrap().len() {
            let inner_widget = &mut self.inner_windows.as_mut().unwrap()[i];
            inner_widget.update(&mut ui);
            let closed = inner_widget.request_close.load(Ordering::SeqCst);
            if !closed { continue; }
            let wid = inner_widget.id.clone();
            let mut inner_window = self.inner_windows.as_mut().unwrap().remove(&wid).unwrap();
            let callback = inner_window.on_close.take();
            if let Some(mut callback) = callback {
                callback(app, inner_window, &mut ui);
            }
        }
        ui.app = Some(app);
        for inner_window in self.inner_windows.as_mut().unwrap().iter_mut() {
            inner_window.update(&mut ui);
        }
        ui.inner_windows = self.inner_windows.take();
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            popup.update(&mut ui)
        }
        ui.popups = self.popups.take();
        self.layout = ui.layout.take();
        self.layout.as_mut().unwrap().update(&mut ui);
        self.popups = ui.popups.take();
        if let Some(u) = ui.request_update.take() {
            ui.context.event.send_event(u).unwrap();
        }
        self.inner_windows = ui.inner_windows.take();
    }

    pub fn configure_surface(&mut self) {
        self.device.surface.configure(&self.device.device, &self.device.surface_config);
    }


    pub fn redraw(&mut self, app: &mut Box<dyn App>) {
        if crate::time_ms() - self.previous_time < 10 {
            let window = self.context.window.clone();
            let t = self.previous_time;
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(crate::time_ms() as u64 - t as u64));
                window.request_redraw();
            });
            return;
        }
        println!("{} frame/ms", crate::time_ms() - self.previous_time);
        let surface_texture = match self.device.surface.get_current_texture() {
            Ok(res) => res,
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        };
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let msaa_texture = self.device.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: self.context.size.width,
                height: self.context.size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: SAMPLE_COUNT,
            dimension: wgpu::TextureDimension::D2,
            format: self.device.texture_format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.device.create_command_encoder(&Default::default());
        let render_pass_desc = RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &msaa_view,
                resolve_target: Some(&view),
                ops: Operations {
                    load: LoadOp::Clear(self.style.window.fill.as_wgpu_color()),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        };
        let pass = encoder.begin_render_pass(&render_pass_desc);
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            pass: Some(pass),
            layout: None,
            popups: self.popups.take(),
            current_rect: Rect::new(),
            update_type: UpdateType::None,
            can_offset: false,
            inner_windows: None,
            request_update: None,
            offset: Offset::new(Pos::new()),

        };
        app.redraw(&mut ui);
        ui.app = Some(app);
        self.layout.as_mut().unwrap().redraw(&mut ui);
        self.popups = ui.popups.take();
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            popup.redraw(&mut ui);
        }
        if let Some(u) = ui.request_update.take() {
            ui.context.event.send_event(u).unwrap();
        }
        for inner_window in self.inner_windows.as_mut().unwrap().iter_mut() {
            inner_window.redraw(&mut ui);
        }
        drop(ui);
        self.device.queue.submit([encoder.finish()]);
        surface_texture.present();
        self.previous_time = crate::time_ms();
    }

    pub fn key_input(&mut self, ut: UpdateType, app: &mut Box<dyn App>) {
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: Some(app),
            pass: None,
            layout: None,
            popups: self.popups.take(),
            current_rect: Rect::new(),
            update_type: ut,
            can_offset: false,
            inner_windows: None,
            request_update: None,
            offset: Offset::new(Pos::new()),
        };
        for inner_window in self.inner_windows.as_mut().unwrap().iter_mut() {
            inner_window.update(&mut ui);
        }
        ui.inner_windows = self.inner_windows.take();
        self.layout.as_mut().unwrap().update(&mut ui);
        self.popups = ui.popups.take();
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            popup.update(&mut ui)
        }
        self.inner_windows = ui.inner_windows.take();
        if let Some(u) = ui.request_update.take() {
            ui.context.event.send_event(u).unwrap();
        }
    }
}

pub struct Ui<'a> {
    pub(crate) device: &'a Device,
    pub(crate) context: &'a mut Context,
    pub(crate) app: Option<&'a mut Box<dyn App>>,
    pub(crate) pass: Option<wgpu::RenderPass<'a>>,
    pub(crate) layout: Option<LayoutKind>,
    pub(crate) popups: Option<Map<Popup>>,
    pub(crate) current_rect: Rect,
    pub(crate) update_type: UpdateType,
    pub(crate) can_offset: bool,
    pub(crate) inner_windows: Option<Map<InnerWindow>>,
    pub(crate) request_update: Option<(winit::window::WindowId, UpdateType)>,
    pub(crate) offset: Offset,
}


impl<'a> Ui<'a> {
    pub(crate) fn layout(&mut self) -> &mut LayoutKind {
        self.layout.as_mut().expect("仅能在App::update中调用")
    }

    pub(crate) fn send_updates(&mut self, ids: &Vec<String>, ct: ContextUpdate) {
        for id in ids {
            self.context.updates.insert(id.to_string(), ct.clone());
        }
    }
}

impl<'a> Ui<'a> {
    pub fn add_space(&mut self, space: f32) {
        self.layout().add_space(space);
    }

    pub fn add<T: Widget>(&mut self, widget: T) -> &mut T {
        let widget = WidgetKind::new(self, widget);
        let wid = widget.id.clone();
        self.layout().alloc_rect(&widget.rect);
        self.layout().add_widget(widget.id.clone(), widget);
        let widget = self.layout().get_widget(&wid).unwrap();
        let widget = widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<T>().unwrap()
    }

    pub fn get_widget<T: Widget>(&mut self, id: impl ToString) -> Option<&mut T> {
        let widget = self.layout().get_widget(&id.to_string())?;
        let widget = widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<T>()
    }

    pub fn add_mut(&mut self, widget: &mut impl Widget) {
        let resp = widget.update(self);
        self.layout().alloc_rect(&resp.rect);
    }

    pub fn request_update(&mut self, ut: UpdateType) {
        let wid = self.context.window.id();
        self.request_update = Some((wid, ut));
    }

    pub fn horizontal(&mut self, context: impl FnOnce(&mut Ui)) {
        let current_layout = HorizontalLayout::left_to_right().max_rect(self.layout().available_rect().clone(), Padding::same(0.0));
        let previous_layout = self.layout.replace(LayoutKind::Horizontal(current_layout)).unwrap();
        context(self);
        let current_layout = self.layout.replace(previous_layout).unwrap();
        self.layout().add_child(current_layout);
    }

    pub fn vertical(&mut self, mut context: impl FnMut(&mut Ui)) {
        let current_layout = VerticalLayout::new().max_rect(self.layout().available_rect().clone(), Padding::same(0.0));
        let previous_layout = self.layout.replace(LayoutKind::Vertical(current_layout)).unwrap();
        context(self);
        let current_layout = self.layout.replace(previous_layout).unwrap();
        self.layout().add_child(current_layout);
    }

    pub fn create_inner_window<W: App>(&mut self, w: W) -> &mut InnerWindow {
        let inner_window = InnerWindow::new(w, self);
        let id = inner_window.id.clone();
        self.inner_windows.as_mut().unwrap().insert(inner_window.id.clone(), inner_window);
        self.inner_windows.as_mut().unwrap().get_mut(&id).unwrap()
    }


    pub fn available_rect(&self) -> &Rect {
        self.layout.as_ref().unwrap().available_rect()
    }

    pub fn paint_rect(&mut self, rect: Rect, style: ClickStyle) {
        let paint_rect = Rectangle::new(rect, style);
        let widget = WidgetKind::new(self, paint_rect);
        self.layout().add_widget(widget.id.clone(), widget);
    }

    pub fn label(&mut self, text: impl Into<RichText>) {
        let label = Label::new(text);
        self.add(label);
    }

    pub fn button(&mut self, text: impl Into<RichText>) -> &mut Button {
        let btn = Button::new(text);
        self.add(btn)
    }

    pub fn radio(&mut self, v: bool, l: impl Into<RichText>) -> &mut RadioButton {
        let radio = RadioButton::new(v, l);
        self.add(radio)
    }

    pub fn checkbox(&mut self, v: bool, l: impl Into<RichText>) -> &mut CheckBox {
        let checkbox = CheckBox::new(v, l);
        self.add(checkbox)
    }

    pub fn slider(&mut self, v: f32, r: Range<f32>) -> &mut Slider {
        let slider = Slider::new(v).with_range(r);
        self.add(slider)
    }

    pub fn image(&mut self, source: &'static str, size: (f32, f32)) -> &mut Image {
        let image = Image::new(source).with_size(size.0, size.1);
        self.add(image)
    }

    pub fn spinbox<T: Display + NumCastExt + PartialOrd + AddAssign + SubAssign + Copy + 'static>(&mut self, v: T, g: T, r: Range<T>) -> &mut SpinBox<T> {
        let spinbox = SpinBox::new(v, g, r);
        self.add(spinbox)
    }

    pub fn select_value<T: Display + PartialEq + 'static>(&mut self, t: T) -> &mut SelectItem<T> {
        let select_value = SelectItem::new(t);
        self.add(select_value)
    }

    pub fn set_image_handle(&mut self, source: &str) {
        self.context.render.image.insert_image(&self.device, source.to_string(), source);
    }
}