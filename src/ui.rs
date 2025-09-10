use crate::frame::context::{Context, ContextUpdate, UpdateType};
use crate::frame::App;
// use crate::layout::popup::Popup;
use crate::layout::vertical::VerticalLayout;
use crate::layout::{Layout, LayoutItem, LayoutKind};
use crate::map::Map;
use crate::size::padding::Padding;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::style::Style;
use crate::text::rich::RichText;
use crate::widgets::{Widget, WidgetChange, WidgetKind};
// use crate::window::inner::InnerWindow;
use crate::window::{UserEvent, WindowId};
use crate::{Device, Label, Offset, SAMPLE_COUNT};
use std::any::Any;
use std::fmt::Display;
use std::ops::DerefMut;
use std::sync::atomic::Ordering;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::Duration;
use wgpu::{LoadOp, Operations, RenderPassDescriptor};
use crate::layout::horizontal::HorizontalLayout;

pub struct AppContext {
    pub(crate) device: Device,
    pub(crate) layout: Option<LayoutKind>,
    // pub(crate) popups: Option<Map<String, Popup>>,
    // pub(crate) inner_windows: Option<Map<WindowId, InnerWindow>>,
    pub(crate) style: Style,
    pub(crate) context: Context,
    previous_time: u128,
    redraw_thread: JoinHandle<()>,
}

impl AppContext {
    pub fn new(device: Device, context: Context) -> AppContext {
        let layout = VerticalLayout::top_to_bottom().with_size(context.size.width as f32, context.size.height as f32)
            .with_space(5.0).with_padding(Padding::same(5.0));
        // let layout = LayoutKind::Vertical(VerticalLayout::top_to_bottom()).with_size(context.size.width as f32, context.size.height as f32, Padding::same(5.0));
        AppContext {
            device,
            layout: Some(LayoutKind::new(layout)),
            // popups: Some(Map::new()),
            // inner_windows: Some(Map::new()),
            style: Style::light_style(),
            context,
            previous_time: 0,
            redraw_thread: spawn(|| {}),
        }
    }

    pub fn draw(&mut self, app: &mut Box<dyn App>) {
        let draw_rect = Rect::new().with_size(self.device.surface_config.width as f32, self.device.surface_config.height as f32);
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            pass: None,
            layout: Some(self.layout.take().unwrap()),
            // popups: self.popups.take(),
            current_rect: Rect::new(),
            update_type: UpdateType::Init,
            can_offset: false,
            // inner_windows: None,
            request_update: None,
            offset: Offset::new(Pos::new()),
            draw_rect,
            widget_changed: WidgetChange::None,
        };
        app.draw(&mut ui);
        self.layout = ui.layout.take();
        // self.layout.as_mut().unwrap().update(&mut ui);
        // self.popups = ui.popups.take();
    }

    pub fn update(&mut self, ut: UpdateType, app: &mut Box<dyn App>) {
        let draw_rect = Rect::new().with_size(self.device.surface_config.width as f32, self.device.surface_config.height as f32);
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            pass: None,
            layout: self.layout.take(),
            // popups: None,
            current_rect: Rect::new(),
            update_type: ut,
            can_offset: false,
            // inner_windows: None,
            request_update: None,
            offset: Offset::new(Pos::new()),

            draw_rect,
            widget_changed: WidgetChange::None,
        };
        app.update(&mut ui);

        // self.inner_windows = ui.inner_windows.take();
        // for i in 0..self.inner_windows.as_ref().unwrap().len() {
        //     let inner_widget = &mut self.inner_windows.as_mut().unwrap()[i];
        //     inner_widget.update(&mut ui);
        //     let closed = inner_widget.request_close.load(Ordering::SeqCst);
        //     if !closed { continue; }
        //     let wid = inner_widget.id.clone();
        //     let mut inner_window = self.inner_windows.as_mut().unwrap().remove(&wid).unwrap();
        //     let callback = inner_window.on_close.take();
        //     if let Some(mut callback) = callback {
        //         callback(app, inner_window, &mut ui);
        //     }
        // }
        ui.app = Some(app);
        // let mut event_win = None;
        // let inner_windows = self.inner_windows.as_ref().unwrap();
        // for i in 0..inner_windows.len() {
        //     let win = &inner_windows[inner_windows.len() - i - 1];
        //     if self.device.device_input.hovered_at(&win.fill_render.param.rect) || win.press_title {
        //         event_win = Some(win.id);
        //         break;
        //     }
        // }

        // if let Some(wid) = event_win {
        //     let inner_win = &mut self.inner_windows.as_mut().unwrap()[&wid];
        //     inner_win.update(&mut ui);
        //     if inner_win.top {
        //         let win = self.inner_windows.as_mut().unwrap().remove(&wid).unwrap();
        //         if win.request_close.load(Ordering::SeqCst) {
        //             if let Some(win) = self.inner_windows.as_mut().unwrap().last_mut() {
        //                 win.top = true;
        //             }
        //         } else {
        //             self.inner_windows.as_mut().unwrap().iter_mut().for_each(|x| x.top = false);
        //             self.inner_windows.as_mut().unwrap().insert(win.id, win);
        //         }
        //     }
        // };

        // let inner_windows = self.inner_windows.as_mut().unwrap();
        // inner_windows.sort_by_key(|x| x.value().top);
        // for i in 0..inner_windows.len() {
        //     let inner_window = &mut inner_windows[i];
        //     inner_window.update(&mut ui);
        //     if !inner_window.top { continue; }
        //     inner_windows.iter_mut().for_each(|x| x.top = false);
        //     inner_windows[i].top = true;
        //     if i != inner_windows.len() - 1 {
        //         println!("requst redraw for inner window");
        //         ui.context.window.request_redraw();
        //     }
        //     break;
        // }
        // ui.inner_windows = self.inner_windows.take();


        // for popup in self.popups.as_mut().unwrap().iter_mut() {
        //     popup.update(&mut ui)
        // }
        // ui.popups = self.popups.take();
        self.layout = ui.layout.take();
        self.layout.as_mut().unwrap().update(&mut ui);
        // self.popups = ui.popups.take();
        // if let Some(u) = ui.request_update.take() {
        //     ui.context.user_update = u;
        //     ui.context.window.request_update(UserEvent::ReqUpdate);
        // }
        // self.inner_windows = ui.inner_windows.take();
    }

    pub fn redraw(&mut self, app: &mut Box<dyn App>) {
        if !self.redraw_thread.is_finished() { return; }
        if crate::time_ms() - self.previous_time < 10 {
            let window = self.context.window.clone();
            let t = self.previous_time;
            self.redraw_thread = spawn(move || {
                sleep(Duration::from_millis(crate::time_ms() as u64 - t as u64));
                window.request_redraw();
            });
            return;
        }
        let surface_texture = match self.device.surface.get_current_texture() {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e.to_string());
                return;
            }
        };
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let msaa_texture = self.device.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: self.device.surface_config.width,
                height: self.device.surface_config.height,
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
        let draw_rect = Rect::new().with_size(self.device.surface_config.width as f32, self.device.surface_config.height as f32);
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            pass: Some(pass),
            layout: self.layout.take(),
            // popups: self.popups.take(),
            current_rect: Rect::new(),
            update_type: UpdateType::Draw,
            can_offset: false,
            // inner_windows: None,
            request_update: None,
            offset: Offset::new(Pos::new()),
            draw_rect,
            widget_changed: WidgetChange::None,
        };
        app.redraw(&mut ui);
        ui.app = Some(app);
        self.layout = ui.layout.take();
        self.layout.as_mut().unwrap().update(&mut ui);
        // self.popups = ui.popups.take();
        // for popup in self.popups.as_mut().unwrap().iter_mut() {
        //     popup.redraw(&mut ui);
        // }
        // if let Some(u) = ui.request_update.take() {
        //     ui.context.user_update = u;
        //     ui.context.window.request_update(UserEvent::ReqUpdate);
        // }
        // self.inner_windows.as_mut().unwrap().sort_by_key(|x| x.value().top);
        // for inner_window in self.inner_windows.as_mut().unwrap().iter_mut() {
        //     inner_window.redraw(&mut ui);
        // }
        drop(ui);
        self.device.queue.submit([encoder.finish()]);
        surface_texture.present();
        self.previous_time = crate::time_ms();
    }

    // #[deprecated = "use AppContext::update"]
    // pub fn key_input(&mut self, ut: UpdateType, app: &mut Box<dyn App>) {
    //     let mut ui = Ui {
    //         device: &self.device,
    //         context: &mut self.context,
    //         app: Some(app),
    //         pass: None,
    //         layout: None,
    //         popups: self.popups.take(),
    //         current_rect: Rect::new(),
    //         update_type: ut,
    //         can_offset: false,
    //         inner_windows: None,
    //         request_update: None,
    //         offset: Offset::new(Pos::new()),
    //     };
    //     for inner_window in self.inner_windows.as_mut().unwrap().iter_mut() {
    //         inner_window.update(&mut ui);
    //     }
    //     ui.inner_windows = self.inner_windows.take();
    //     self.layout.as_mut().unwrap().update(&mut ui);
    //     self.popups = ui.popups.take();
    //     for popup in self.popups.as_mut().unwrap().iter_mut() {
    //         popup.update(&mut ui)
    //     }
    //     self.inner_windows = ui.inner_windows.take();
    //     if let Some(u) = ui.request_update.take() {
    //         ui.context.user_update = u;
    //         ui.context.window.request_update(UserEvent::ReqUpdate);
    //     }
    // }
}

pub struct Ui<'a> {
    pub(crate) device: &'a Device,
    pub(crate) context: &'a mut Context,
    pub(crate) app: Option<&'a mut Box<dyn App>>,
    pub(crate) pass: Option<wgpu::RenderPass<'a>>,
    pub(crate) layout: Option<LayoutKind>,
    // pub(crate) popups: Option<Map<String, Popup>>,
    #[deprecated = "use Ui::draw_rect"]
    pub(crate) current_rect: Rect,
    pub(crate) update_type: UpdateType,
    pub(crate) can_offset: bool,
    // pub(crate) inner_windows: Option<Map<WindowId, InnerWindow>>,
    pub(crate) request_update: Option<(WindowId, UpdateType)>,
    pub(crate) offset: Offset,
    pub(crate) draw_rect: Rect,
    pub(crate) widget_changed: WidgetChange,
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
    // pub fn add_space(&mut self, space: f32) {
    //     self.layout().add_space(space);
    // }

    pub fn add<T: Widget>(&mut self, widget: T) -> Option<&mut T> {
        let widget = WidgetKind::new(self, widget);
        let wid = widget.id().to_owned();
        let layout = self.layout.as_mut()?;
        layout.add_item(LayoutItem::Widget(widget));
        layout.get_item_mut(&wid)?.widget()

        // let items = layout.layout_mut().items_mut();
        // items.insert(wid.clone(), LayoutItem::Widget(widget));
        // items.get_mut(&wid).unwrap().widget()
        // self.layout().alloc_rect(&widget.rect);
        // self.layout().add_widget(widget.id.clone(), widget);
        // let items = self.layout.as_mut().unwrap().layout_mut().items_mut();
        // let widget = self.layout().layout_mut().get_item(&wid).unwrap();
        // widget.widget()
        // let widget = widget.deref_mut() as &mut dyn Any;
        // widget.downcast_mut::<T>().unwrap()
    }

    pub fn get_widget<T: Widget>(&mut self, id: impl ToString) -> Option<&mut T> {
        let layout = self.layout.as_mut()?;
        layout.get_item_mut(&id.to_string()).unwrap().widget()
        // let items = layout.layout_mut().items_mut();
        // Some(items.get_mut(&id.to_string())?.widget())
        // let widget = self.layout().get_widget(&id.to_string())?;
        // let widget = widget.deref_mut() as &mut dyn Any;
        // widget.downcast_mut::<T>()
    }

    // pub fn add_mut(&mut self, widget: &mut impl Widget) {
    //     let resp = widget.update(self);
    //     self.layout().alloc_rect(&resp.rect);
    // }

    pub fn request_update(&mut self, ut: UpdateType) {
        let wid = self.context.window.id();
        self.request_update = Some((wid, ut));
    }

    pub fn add_layout(&mut self, layout: impl Layout + 'static, context: impl FnOnce(&mut Ui)) {
        let mut layout = LayoutKind::new(layout);
        // let x_direction = layout.max_rect().x_direction();
        // let y_direction = layout.max_rect().y_direction();
        // layout.set_rect(self.layout().available_rect().with_x_direction(x_direction).with_y_direction(y_direction), &Padding::same(0.0));
        // println!("add layout={:?}", layout.max_rect());

        let previous_layout = self.layout.replace(layout).unwrap();
        context(self);
        let mut current_layout = self.layout.replace(previous_layout).unwrap();
        current_layout.update(self);
        self.layout().add_item(LayoutItem::Layout(current_layout));
        // self.layout().layout_mut().items_mut().insert(current_layout.id().to_string(), LayoutItem::Layout(current_layout));
    }

    pub fn horizontal(&mut self, context: impl FnOnce(&mut Ui)) {
        let current_layout = HorizontalLayout::left_to_right().with_padding(Padding::same(0.0));
        let previous_layout = self.layout.replace(LayoutKind::new(current_layout)).unwrap();
        context(self);
        let mut current_layout = self.layout.replace(previous_layout).unwrap();
        current_layout.update(self);
        self.layout().add_item(LayoutItem::Layout(current_layout));
        // self.layout().layout_mut().items_mut().insert(current_layout.id().to_string(), LayoutItem::Layout(current_layout));
        // self.layout().add_child(current_layout);
    }

    pub fn vertical(&mut self, mut context: impl FnMut(&mut Ui)) {
        let current_layout = VerticalLayout::top_to_bottom().with_padding(Padding::same(0.0));
        let previous_layout = self.layout.replace(LayoutKind::new(current_layout)).unwrap();
        context(self);
        let mut current_layout = self.layout.replace(previous_layout).unwrap();
        current_layout.update(self);
        self.layout().add_item(LayoutItem::Layout(current_layout));
        // self.layout().layout_mut().items_mut().insert(current_layout.id().to_string(), LayoutItem::Layout(current_layout));
        // self.layout().add_child(current_layout);
    }

    // pub fn create_inner_window<W: App>(&mut self, w: W) -> &mut InnerWindow {
    //     let mut inner_window = InnerWindow::new(w, self);
    //     inner_window.top = true;
    //     self.inner_windows.as_mut().unwrap().iter_mut().for_each(|x| x.top = false);
    //     let id = inner_window.id.clone();
    //     self.inner_windows.as_mut().unwrap().insert(inner_window.id.clone(), inner_window);
    //     self.inner_windows.as_mut().unwrap().get_mut(&id).unwrap()
    // }

    pub fn create_window<W: App>(&mut self, w: W) {
        let attr = w.window_attributes();
        let app = Box::new(w);
        self.context.new_window = Some((app, attr));
        self.context.window.request_update(UserEvent::CreateChild);
    }


    // pub fn available_rect(&self) -> Rect {
    //     self.layout.as_ref().unwrap().available_rect()
    // }

    // pub fn paint_rect(&mut self, rect: Rect, style: ClickStyle) {
    //     let paint_rect = Rectangle::new(rect, style);
    //     let widget = WidgetKind::new(paint_rect);
    //     self.layout().add_widget(widget.id.clone(), widget);
    // }

    pub fn label(&mut self, text: impl Into<RichText>) {
        let label = Label::new(text);
        self.add(label);
    }

    // pub fn button(&mut self, text: impl Into<RichText>) -> &mut Button {
    //     let btn = Button::new(text);
    //     self.add(btn)
    // }
    //
    // pub fn radio(&mut self, v: bool, l: impl Into<RichText>) -> &mut RadioButton {
    //     let radio = RadioButton::new(v, l);
    //     self.add(radio)
    // }
    //
    // pub fn checkbox(&mut self, v: bool, l: impl Into<RichText>) -> &mut CheckBox {
    //     let checkbox = CheckBox::new(v, l);
    //     self.add(checkbox)
    // }
    //
    // pub fn slider(&mut self, v: f32, r: Range<f32>) -> &mut Slider {
    //     let slider = Slider::new(v).with_range(r);
    //     self.add(slider)
    // }
    //
    // pub fn image(&mut self, source: impl Into<ImageSource>, size: (f32, f32)) -> &mut Image {
    //     let image = Image::new(source).with_size(size.0, size.1);
    //     self.add(image)
    // }
    //
    // pub fn spinbox<T: Display + NumCastExt + PartialOrd + AddAssign + SubAssign + Copy + 'static>(&mut self, v: T, g: T, r: Range<T>) -> &mut SpinBox<T> {
    //     let spinbox = SpinBox::new(v, g, r);
    //     self.add(spinbox)
    // }
    //
    // pub fn select_value<T: Display + PartialEq + 'static>(&mut self, t: T) -> &mut SelectItem<T> {
    //     let select_value = SelectItem::new(t);
    //     self.add(select_value)
    // }
}