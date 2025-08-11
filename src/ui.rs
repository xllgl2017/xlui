use crate::frame::context::Context;
use crate::frame::App;
use crate::layout::{HorizontalLayout, Layout, LayoutKind, VerticalLayout};
use crate::size::padding::Padding;
use crate::style::Style;
use crate::widgets::button::Button;
use crate::widgets::checkbox::CheckBox;
use crate::widgets::label::Label;
use crate::widgets::radio::RadioButton;
use crate::widgets::slider::Slider;
use crate::widgets::Widget;
use crate::{Device, Offset, SAMPLE_COUNT};
use std::any::Any;
use std::fmt::Display;
use std::ops::{AddAssign, DerefMut, Range, SubAssign};
use wgpu::{LoadOp, Operations, RenderPassDescriptor};
use crate::layout::popup::Popup;
use crate::map::Map;
use crate::size::rect::Rect;
use crate::widgets::image::Image;
use crate::widgets::spinbox::SpinBox;

pub struct AppContext {
    pub(crate) device: Device,
    pub(crate) layout: Option<LayoutKind>,
    pub(crate) popups: Option<Map<Popup>>,
    pub(crate) style: Style,
    pub(crate) context: Context,
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
        }
    }

    pub fn draw<A: App + Any>(&mut self, app: &mut A) {
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            pass: None,
            layout: Some(self.layout.take().unwrap()),
            canvas_offset: None,
            popups: self.popups.take(),
            key: None,
            current_rect: Rect::new(),
        };
        app.draw(&mut ui);
        self.layout = ui.layout.take();
        self.popups = ui.popups.take();
    }

    pub fn update(&mut self, app: &mut impl App) {
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: Some(Box::new(app)),
            pass: None,
            layout: None,
            canvas_offset: None,
            popups: self.popups.take(),
            key: None,
            current_rect: Rect::new(),
        };
        self.layout.as_mut().unwrap().update(&mut ui);
        self.popups = ui.popups.take();
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            popup.update(&mut ui)
        }
    }


    pub fn redraw(&mut self, app: &mut impl App) {
        let surface_texture = self.context.surface.get_current_texture().unwrap();
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
            canvas_offset: None,
            popups: self.popups.take(),
            key: None,
            current_rect: Rect::new(),
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

    pub fn key_input<A: App>(&mut self, key: winit::keyboard::Key, app: &mut A) {
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: Some(Box::new(app)),
            pass: None,
            layout: None,
            canvas_offset: None,
            popups: self.popups.take(),
            key: Some(key),
            current_rect: Rect::new(),
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
    pub(crate) canvas_offset: Option<Offset>,
    pub(crate) popups: Option<Map<Popup>>,
    pub(crate) key: Option<winit::keyboard::Key>,
    pub(crate) current_rect: Rect,
}

impl<'a> Ui<'a> {
    pub fn add_space(&mut self, space: f32) {
        self.layout().add_space(space);
    }

    pub fn add(&mut self, mut widget: impl Widget + 'static) {
        let resp = widget.draw(self);
        self.layout().alloc_rect(&resp.rect);
        self.layout.as_mut().unwrap().add_widget(resp.id, Box::new(widget));
    }

    pub fn add_mut(&mut self, widget: &mut impl Widget) {
        let resp = widget.draw(self);
        self.layout().alloc_rect(&resp.rect);
    }

    pub fn horizontal(&mut self, mut context: impl FnMut(&mut Ui)) {
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

    pub(crate) fn layout(&mut self) -> &mut LayoutKind {
        self.layout.as_mut().unwrap()
    }

    pub fn label(&mut self, text: impl ToString) {
        let label = Label::new(text);
        self.add(label);
    }

    pub fn button(&mut self, text: impl ToString) -> &mut Button {
        let btn = Button::new(text);
        let btn_id = btn.id.clone();
        self.add(btn);
        let layout = self.layout.as_mut().unwrap();
        let widget = layout.get_widget(&btn_id).unwrap();
        let widget = widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<Button>().unwrap()
    }

    pub fn radio(&mut self, v: bool, l: impl ToString) -> &mut RadioButton {
        let radio = RadioButton::new(v, l);
        let radio_id = radio.id.clone();
        self.add(radio);
        let layout = self.layout.as_mut().unwrap();
        let widget = layout.get_widget(&radio_id).unwrap();
        let widget = widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<RadioButton>().unwrap()
    }

    pub fn checkbox(&mut self, v: bool, l: impl ToString) -> &mut CheckBox {
        let checkbox = CheckBox::new(v, l);
        let btn_id = checkbox.id.clone();
        self.add(checkbox);
        let layout = self.layout.as_mut().unwrap();
        let widget = layout.get_widget(&btn_id).unwrap();
        let widget = widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<CheckBox>().unwrap()
    }

    pub fn slider(&mut self, v: f32, r: Range<f32>) -> &mut Slider {
        let slider = Slider::new(v).with_range(r);
        let btn_id = slider.id.clone();
        self.add(slider);
        let layout = self.layout.as_mut().unwrap();
        let widget = layout.get_widget(&btn_id).unwrap();
        let widget = widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<Slider>().unwrap()
    }

    pub fn image(&mut self, source: &'static str, size: (f32, f32)) -> &mut Image {
        let image = Image::new(source).with_size(size.0, size.1);
        let image_id = image.id.clone();
        self.add(image);
        let layout = self.layout.as_mut().unwrap();
        let widget = layout.get_widget(&image_id).unwrap();
        let widget = widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<Image>().unwrap()
    }

    pub fn spinbox<T: Display + PartialOrd + AddAssign + SubAssign + Copy + 'static>(&mut self, v: T, g: T, r: Range<T>) -> &mut SpinBox<T> {
        let spinbox = SpinBox::new(v, g, r);
        let spinbox_id = spinbox.id.clone();
        self.add(spinbox);
        let layout = self.layout.as_mut().unwrap();
        let widget = layout.get_widget(&spinbox_id).unwrap();
        let widget = widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<SpinBox<T>>().unwrap()
    }
}