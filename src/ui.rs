use crate::frame::context::{Context, ContextUpdate, UpdateType};
use crate::frame::App;
use crate::layout::horizontal::HorizontalLayout;
use crate::layout::popup::Popup;
use crate::layout::vertical::VerticalLayout;
use crate::layout::{Layout, LayoutItem, LayoutKind};
use crate::map::Map;
use crate::render::image::ImageSource;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::Style;
use crate::text::rich::RichText;
use crate::widgets::checkbox::CheckBox;
use crate::widgets::space::Space;
use crate::widgets::{Widget, WidgetChange, WidgetKind};
use crate::window::inner::InnerWindow;
#[cfg(all(target_os = "linux", not(feature = "gpu")))]
use crate::window::x11::ffi::Cairo;
use crate::window::{UserEvent, WindowId, WindowType};
use crate::*;
use std::cell::RefCell;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{AddAssign, Range, SubAssign};
use std::rc::Rc;
use std::sync::atomic::Ordering;
use std::sync::Arc;
#[cfg(feature = "gpu")]
use std::thread::sleep;
use std::thread::{spawn, JoinHandle};
#[cfg(feature = "gpu")]
use std::time::Duration;
#[cfg(feature = "gpu")]
use wgpu::{LoadOp, Operations, RenderPassDescriptor};
#[cfg(all(windows, not(feature = "gpu")))]
use windows::Win32::Graphics::Gdi::{HDC, PAINTSTRUCT};

pub struct AppContext {
    pub(crate) device: Device,
    pub(crate) layout: Option<LayoutKind>,
    pub(crate) popups: Option<Map<String, Popup>>,
    pub(crate) inner_windows: Option<Map<WindowId, InnerWindow>>,
    pub(crate) style: Rc<RefCell<Style>>,
    pub(crate) context: Context,
    pub(crate) previous_time: u128,
    pub(crate) redraw_thread: JoinHandle<()>,
    attr: WindowAttribute,
}

impl AppContext {
    pub fn new(device: Device, context: Context, attr: WindowAttribute) -> AppContext {
        let size = context.window.size();
        let layout = VerticalLayout::top_to_bottom().with_size(size.width, size.height)
            .with_space(5.0).with_padding(Padding::same(5.0));
        AppContext {
            device,
            layout: Some(LayoutKind::new(layout)),
            popups: Some(Map::new()),
            inner_windows: Some(Map::new()),
            style: Rc::new(RefCell::new(Style::light_style())),
            context,
            previous_time: 0,
            redraw_thread: spawn(|| {}),
            attr,
        }
    }

    pub fn draw(&mut self, app: &mut Box<dyn App>) {
        let size = self.context.window.size();
        let draw_rect = Rect::new().with_size(size.width, size.height);
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            layout: Some(self.layout.take().unwrap()),
            popups: self.popups.take(),
            update_type: UpdateType::Init,
            can_offset: false,
            inner_windows: None,
            request_update: None,
            draw_rect,
            widget_changed: WidgetChange::None,
            style: self.style.clone(),
            paint: None,
        };
        app.draw(&mut ui);
        self.layout = ui.layout.take();
        self.layout.as_mut().unwrap().update(&mut ui);
        self.popups = ui.popups.take();
    }

    #[cfg(not(feature = "winit"))]
    pub fn user_update(&mut self, app: &mut Box<dyn App>) {
        let size = self.context.window.size();
        let draw_rect = Rect::new().with_size(size.width, size.height);
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            layout: self.layout.take(),
            popups: None,
            update_type: UpdateType::None,
            can_offset: false,
            inner_windows: None,
            request_update: None,
            draw_rect,
            widget_changed: WidgetChange::None,
            style: self.style.clone(),
            paint: None,
        };
        app.update(&mut ui);
        self.layout = ui.layout.take();
    }

    pub fn update(&mut self, ut: UpdateType, app: &mut Box<dyn App>) {
        let size = self.context.window.size();
        let draw_rect = Rect::new().with_size(size.width, size.height);
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            layout: self.layout.take(),
            popups: None,
            update_type: ut,
            can_offset: false,
            inner_windows: None,
            request_update: None,
            draw_rect,
            widget_changed: WidgetChange::None,
            style: self.style.clone(),
            paint: None,
        };
        app.update(&mut ui);
        ui.app = Some(app);
        let mut event_win = None;
        let inner_windows = self.inner_windows.as_ref().unwrap();
        for i in 0..inner_windows.len() {
            let win = &inner_windows[inner_windows.len() - i - 1];
            if self.device.device_input.hovered_at(win.fill_render.rect()) || win.press_title {
                event_win = Some(win.id);
                break;
            }
        }

        if let Some(wid) = event_win {
            let inner_win = &mut self.inner_windows.as_mut().unwrap()[&wid];
            inner_win.update(&mut ui);
            if inner_win.top {
                let win = self.inner_windows.as_mut().unwrap().remove(&wid).unwrap();
                if win.request_close.load(Ordering::SeqCst) {
                    if let Some(win) = self.inner_windows.as_mut().unwrap().last_mut() {
                        win.top = true;
                    }
                } else {
                    self.inner_windows.as_mut().unwrap().iter_mut().for_each(|x| x.top = false);
                    self.inner_windows.as_mut().unwrap().insert(win.id, win);
                }
            }
        };

        ui.inner_windows = self.inner_windows.take();
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            popup.update(&mut ui);
        }
        ui.popups = self.popups.take();
        self.layout = ui.layout.take();
        self.layout.as_mut().unwrap().update(&mut ui);
        self.popups = ui.popups.take();
        self.inner_windows = ui.inner_windows.take();
    }

    pub fn redraw(&mut self, app: &mut Box<dyn App>, paint: Option<PaintParam>) { //ps: Option<PAINTSTRUCT>, hdc: Option<HDC>
        #[cfg(feature = "gpu")]
        let _ = paint.is_none();
        #[cfg(feature = "gpu")]
        if !self.redraw_thread.is_finished() { return; }
        #[cfg(feature = "gpu")]
        if time_ms() - self.previous_time < 10 {
            let window = self.context.window.clone();
            let t = self.previous_time;
            self.redraw_thread = spawn(move || {
                sleep(Duration::from_millis(crate::time_ms() as u64 - t as u64));
                window.request_redraw();
            });
            return;
        }
        #[cfg(feature = "gpu")]
        let surface_texture = match self.device.surface.get_current_texture() {
            Ok(res) => res,
            Err(e) => {
                println!("{}", e.to_string());
                return;
            }
        };
        #[cfg(feature = "gpu")]
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        #[cfg(feature = "gpu")]
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
        #[cfg(feature = "gpu")]
        let msaa_view = msaa_texture.create_view(&wgpu::TextureViewDescriptor::default());
        #[cfg(feature = "gpu")]
        let mut encoder = self.device.device.create_command_encoder(&Default::default());
        #[cfg(feature = "gpu")]
        let render_pass_desc = RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &msaa_view,
                resolve_target: Some(&view),
                ops: Operations {
                    load: LoadOp::Clear(self.attr.fill.as_wgpu_color()),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        };
        #[cfg(feature = "gpu")]
        let pass = encoder.begin_render_pass(&render_pass_desc);
        #[cfg(feature = "gpu")]
        let paint = Some(PaintParam { pass });
        let size = self.context.window.size();
        let draw_rect = Rect::new().with_size(size.width, size.height);
        let mut ui = Ui {
            device: &self.device,
            context: &mut self.context,
            app: None,
            layout: self.layout.take(),
            popups: self.popups.take(),
            update_type: UpdateType::Draw,
            can_offset: false,
            inner_windows: None,
            request_update: None,
            draw_rect,
            widget_changed: WidgetChange::None,
            style: self.style.clone(),
            paint,
        };
        app.update(&mut ui);
        ui.app = Some(app);
        self.layout = ui.layout.take();
        self.layout.as_mut().unwrap().update(&mut ui);
        self.popups = ui.popups.take();
        for popup in self.popups.as_mut().unwrap().iter_mut() {
            if !popup.opened() { continue; }
            popup.update(&mut ui);
        }
        if let Some(u) = ui.request_update.take() {
            ui.context.user_update = u;
            ui.context.window.request_update_event(UserEvent::ReqUpdate);
        }
        self.inner_windows.as_mut().unwrap().sort_by_key(|x| x.value().top);
        for inner_window in self.inner_windows.as_mut().unwrap().iter_mut() {
            inner_window.redraw(&mut ui);
        }
        drop(ui);
        #[cfg(feature = "gpu")]
        self.device.queue.submit([encoder.finish()]);
        #[cfg(feature = "gpu")]
        surface_texture.present();
        self.previous_time = time_ms();
    }
}

pub(crate) struct PaintParam<'p> {
    #[cfg(all(target_os = "linux", not(feature = "gpu")))]
    pub(crate) cairo: &'p mut Cairo,
    #[cfg(all(target_os = "linux", not(feature = "gpu")))]
    pub(crate) window: x11::xlib::Window,
    #[cfg(all(target_os = "linux", not(feature = "gpu")))]
    pub(crate) draw: *mut x11::xft::XftDraw,

    #[cfg(all(windows, not(feature = "gpu")))]
    pub(crate) paint_struct: PAINTSTRUCT,
    #[cfg(all(windows, not(feature = "gpu")))]
    pub(crate) hdc: HDC,
    #[cfg(all(windows, not(feature = "gpu")))]
    pub(crate) saved_hdc: i32,
    #[cfg(all(target_os = "windows", not(feature = "gpu")))]
    pub(crate) _s: &'p str,
    #[cfg(feature = "gpu")]
    pub(crate) pass: wgpu::RenderPass<'p>,
}

impl<'a> Debug for PaintParam<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PaintParam")
    }
}

impl<'a> Drop for PaintParam<'a> {
    fn drop(&mut self) {
        #[cfg(all(target_os = "linux", not(feature = "gpu")))]
        unsafe { x11::xft::XftDrawDestroy(self.draw) };
    }
}


pub struct Ui<'a, 'p> {
    pub(crate) device: &'a Device,
    pub(crate) context: &'a mut Context,
    pub(crate) app: Option<&'a mut Box<dyn App>>,
    pub(crate) layout: Option<LayoutKind>,
    pub(crate) popups: Option<Map<String, Popup>>,
    pub(crate) update_type: UpdateType,
    pub(crate) can_offset: bool,
    pub(crate) inner_windows: Option<Map<WindowId, InnerWindow>>,
    pub(crate) request_update: Option<(WindowId, UpdateType)>,
    pub(crate) draw_rect: Rect,
    pub(crate) widget_changed: WidgetChange,
    pub style: Rc<RefCell<Style>>,
    pub(crate) paint: Option<PaintParam<'p>>,
}


impl<'a, 'p> Ui<'a, 'p> {
    pub fn layout(&mut self) -> &mut LayoutKind {
        self.layout.as_mut().expect("仅能在App::update中调用")
    }

    pub(crate) fn send_updates(&mut self, ids: &Vec<String>, ct: ContextUpdate) {
        for id in ids {
            self.context.updates.insert(id.to_string(), ct.clone());
        }
    }

    pub fn get_value(&mut self, cid: impl ToString) -> Option<ContextUpdate> {
        self.context.updates.remove(&cid.to_string())
    }

    pub fn set_value(&mut self, wid: impl ToString, value: impl Into<ContextUpdate>) {
        self.context.updates.insert(wid.to_string(), value.into());
    }
}

impl<'a, 'p> Ui<'a, 'p> {
    pub fn window(&self) -> Arc<WindowType> {
        self.context.window.clone()
    }

    ///添加一些间隔，可能是水平方向，也可能是垂直方向
    pub fn add_space(&mut self, space: f32) {
        let space = Space::new(space);
        self.add(space);
    }

    ///添加一个控件，控件必须实现是Widget Trait
    pub fn add<T: Widget>(&mut self, widget: T) -> Option<&mut T> {
        let widget = WidgetKind::new(self, widget);
        let wid = widget.id().to_owned();
        let layout = self.layout.as_mut()?;
        layout.add_item(LayoutItem::Widget(widget));
        layout.get_item_mut(&wid)?.widget_mut()
    }

    ///查询控件，id为控件的ID
    pub fn get_widget<T: Widget>(&mut self, id: impl ToString) -> Option<&mut T> {
        let layout = self.layout.as_mut()?;
        layout.get_widget(&id.to_string())
    }

    ///请求更新，只执行update
    pub fn request_update(&mut self, ut: UpdateType) {
        let wid = self.context.window.id();
        self.request_update = Some((wid, ut));
    }

    ///添加一个布局
    pub fn add_layout(&mut self, layout: impl Layout + 'static, context: impl FnOnce(&mut Ui)) {
        let layout = LayoutKind::new(layout);
        let previous_layout = self.layout.replace(layout).unwrap();
        context(self);
        let mut current_layout = self.layout.replace(previous_layout).unwrap();
        current_layout.update(self);
        self.layout().add_item(LayoutItem::Layout(current_layout));
    }

    ///快速水平布局
    pub fn horizontal(&mut self, context: impl FnOnce(&mut Ui)) {
        let current_layout = HorizontalLayout::left_to_right().with_padding(Padding::same(0.0));
        let previous_layout = self.layout.replace(LayoutKind::new(current_layout)).unwrap();
        context(self);
        let mut current_layout = self.layout.replace(previous_layout).unwrap();
        current_layout.update(self);
        self.layout().add_item(LayoutItem::Layout(current_layout));
    }

    ///快速垂直布局
    pub fn vertical(&mut self, mut context: impl FnMut(&mut Ui)) {
        let current_layout = VerticalLayout::top_to_bottom().with_padding(Padding::same(0.0));
        let previous_layout = self.layout.replace(LayoutKind::new(current_layout)).unwrap();
        context(self);
        let mut current_layout = self.layout.replace(previous_layout).unwrap();
        current_layout.update(self);
        self.layout().add_item(LayoutItem::Layout(current_layout));
    }

    /// 创建一个内部子窗口
    pub fn create_inner_window<W: App>(&mut self, w: W) -> &mut InnerWindow {
        let mut inner_window = InnerWindow::new(w, self);
        inner_window.top = true;
        self.inner_windows.as_mut().unwrap().iter_mut().for_each(|x| x.top = false);
        let id = inner_window.id.clone();
        self.inner_windows.as_mut().unwrap().insert(inner_window.id.clone(), inner_window);
        self.inner_windows.as_mut().unwrap().get_mut(&id).unwrap()
    }

    ///创建一个外部独立窗口
    pub fn create_window<W: App>(&mut self, w: W) {
        let app = Box::new(w);
        self.context.new_window = Some(app);
        self.context.window.request_update_event(UserEvent::CreateChild);
    }

    ///快速创建一个label
    pub fn label(&mut self, text: impl Into<RichText>) {
        let label = Label::new(text);
        self.add(label);
    }
    ///快速创建一个button
    pub fn button(&mut self, text: impl Into<RichText>) -> &mut Button {
        let btn = Button::new(text);
        self.add(btn).unwrap()
    }

    ///快速创建一个radio
    pub fn radio(&mut self, v: bool, l: impl Into<RichText>) -> &mut RadioButton {
        let radio = RadioButton::new(v, l);
        self.add(radio).unwrap()
    }

    pub fn radio_groups(&mut self, mut groups: Vec<RadioButton>) {
        let ids = groups.iter().map(|x| x.id.clone()).collect::<Vec<_>>();
        for (index, mut group) in groups.into_iter().enumerate() {
            for (i, id) in ids.iter().enumerate() {
                if i == index { continue; }
                group.set_group_by_id(id);
            }
            self.add(group).unwrap();
        }
    }

    ///快速创建一个checkbox
    pub fn checkbox(&mut self, v: bool, l: impl Into<RichText>) -> &mut CheckBox {
        let checkbox = CheckBox::new(v, l);
        self.add(checkbox).unwrap()
    }

    ///快速创建一个slider
    pub fn slider(&mut self, v: f32, r: Range<f32>) -> &mut Slider {
        let slider = Slider::new(v).with_range(r);
        self.add(slider).unwrap()
    }

    ///快速创建一个image,这里需要给指定绘制的大小
    pub fn image(&mut self, source: impl Into<ImageSource>, size: (f32, f32)) -> &mut Image {
        let image = Image::new(source).with_size(size.0, size.1);
        self.add(image).unwrap()
    }

    ///快速创建一个spinbox，支持i8,u8,i16,u16,i32,u32,f32,i64,u64,f64等
    pub fn spinbox<T: Display + NumCastExt + PartialOrd + AddAssign + SubAssign + Copy + 'static>(&mut self, v: T, g: T, r: Range<T>) -> &mut SpinBox<T> {
        let spinbox = SpinBox::new(v, g, r);
        self.add(spinbox).unwrap()
    }

    pub fn select_value<T: Display + PartialEq + 'static>(&mut self, t: T) -> &mut SelectItem<T> {
        let select_value = SelectItem::new(t);
        self.add(select_value).unwrap()
    }
}