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
use crate::{Device, NumCastExt, SAMPLE_COUNT};
use std::any::Any;
use std::fmt::Display;
use std::ops::{AddAssign, DerefMut, Range, SubAssign};
use wgpu::{LoadOp, Operations, RenderPassDescriptor};

pub struct AppContext {
    pub(crate) device: Device,
    pub(crate) layout: Option<LayoutKind>,
    pub(crate) popups: Option<Map<Popup>>,
    pub(crate) style: Style,
    pub(crate) context: Context,
    pub(crate) need_rebuild: bool,
}

impl AppContext {
    pub fn new(device: Device, context: Context) -> AppContext {
        let layout = VerticalLayout::new().with_size(context.size.width as f32, context.size.height as f32, Padding::same(5.0));
        AppContext {
            device,
            layout: Some(LayoutKind::Vertical(layout)),
            popups: Some(Map::new()),
            style: Style::light_style(),
            context,
            need_rebuild: false,
        }
    }

    pub fn draw<A: App + Any>(&mut self, app: &mut A) {
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            pass: None,
            layout: Some(self.layout.take().unwrap()),
            popups: self.popups.take(),
            current_rect: Rect::new(),
            update_type: UpdateType::None,
        };
        app.draw(&mut ui);
        self.layout = ui.layout.take();
        // self.layout.as_mut().unwrap().redraw(&mut ui);
        self.popups = ui.popups.take();
    }

    pub fn update<A: App + Any>(&mut self, ut: UpdateType, app: &mut A) {
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            pass: None,
            layout: self.layout.take(),
            popups: None,
            current_rect: Rect::new(),
            update_type: ut,
        };
        app.update(&mut ui);
        ui.app = Some(Box::new(app));
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            popup.update(&mut ui)
        }
        ui.popups = self.popups.take();
        self.layout = ui.layout.take();
        self.layout.as_mut().unwrap().update(&mut ui);
        self.popups = ui.popups.take();
    }

    pub fn configure_surface(&mut self) {
        self.context.surface.configure(&self.device.device, &self.device.surface_config);
    }


    pub fn redraw(&mut self, app: &mut impl App) {
        let surface_texture = match self.context.surface.get_current_texture() {
            Ok(res) => res,
            Err(wgpu::SurfaceError::Lost) | Err(wgpu::SurfaceError::Outdated) => {
                self.need_rebuild = true;
                return;
            }
            Err(e) => {
                println!("Failed to get surface texture: {:?}", e);
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
        };
        app.redraw(&mut ui);
        ui.app = Some(Box::new(app));
        self.layout.as_mut().unwrap().redraw(&mut ui);
        self.popups = ui.popups.take();
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            popup.redraw(&mut ui);
        }
        drop(ui);
        self.device.queue.submit([encoder.finish()]);
        surface_texture.present();
    }

    pub fn key_input<A: App>(&mut self, ut: UpdateType, app: &mut A) {
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: Some(Box::new(app)),
            pass: None,
            layout: None,
            popups: self.popups.take(),
            current_rect: Rect::new(),
            update_type: ut,
        };
        self.layout.as_mut().unwrap().update(&mut ui);
        self.popups = ui.popups.take();
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            popup.update(&mut ui)
        }
    }
}

pub struct Ui<'a> {
    pub(crate) device: &'a Device,
    pub(crate) context: &'a mut Context,
    pub(crate) app: Option<Box<&'a mut dyn Any>>,
    pub(crate) pass: Option<wgpu::RenderPass<'a>>,
    pub(crate) layout: Option<LayoutKind>,
    pub(crate) popups: Option<Map<Popup>>,
    pub(crate) current_rect: Rect,
    pub(crate) update_type: UpdateType,
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

    pub(crate) fn get_layout(&mut self, id: impl ToString) -> Option<&mut LayoutKind> {
        self.layout().get_layout(&id.to_string())
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
        let resp = widget.redraw(self);
        self.layout().alloc_rect(&resp.rect);
    }

    pub fn request_update(&mut self) {
        self.context.window.request_redraw();
    }

    pub fn horizontal(&mut self, mut context: impl FnOnce(&mut Ui)) {
        let current_layout = HorizontalLayout::new().max_rect(self.layout().available_rect().clone(), Padding::same(0.0));
        let previous_layout = self.layout.replace(LayoutKind::Horizontal(current_layout)).unwrap();
        context(self);
        let current_layout = self.layout.replace(previous_layout).unwrap();
        self.layout().add_child(crate::gen_unique_id(), current_layout);
    }

    pub fn vertical(&mut self, mut context: impl FnMut(&mut Ui)) {
        let current_layout = VerticalLayout::new().max_rect(self.layout().available_rect().clone(), Padding::same(0.0));
        let previous_layout = self.layout.replace(LayoutKind::Vertical(current_layout)).unwrap();
        context(self);
        let current_layout = self.layout.replace(previous_layout).unwrap();
        self.layout().add_child(crate::gen_unique_id(), current_layout);
    }


    pub fn available_rect(&self) -> &Rect {
        self.layout.as_ref().unwrap().available_rect()
    }

    pub fn paint_rect(&mut self, rect: Rect, style: ClickStyle) {
        let paint_rect = Rectangle::new(rect, style);
        let widget = WidgetKind::new(self, paint_rect);
        self.layout().add_widget(widget.id.clone(), widget);
    }

    pub fn label(&mut self, text: impl ToString) {
        let label = Label::new(text);
        self.add(label);
    }

    pub fn button(&mut self, text: impl ToString) -> &mut Button {
        let btn = Button::new(text);
        self.add(btn)
    }

    pub fn radio(&mut self, v: bool, l: impl ToString) -> &mut RadioButton {
        let radio = RadioButton::new(v, l);
        self.add(radio)
    }

    pub fn checkbox(&mut self, v: bool, l: impl ToString) -> &mut CheckBox {
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
}