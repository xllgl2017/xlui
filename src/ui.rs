use std::ops::Range;
use crate::frame::context::Context;
use crate::layout::Layout;
use crate::paint::PaintTask;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::{ClickStyle, Style};
use crate::text::text_buffer::TextBuffer;
use crate::widgets::button::Button;
use crate::widgets::image::Image;
use crate::widgets::label::Label;
use crate::widgets::spinbox::SpinBox;
use crate::widgets::Widget;
use crate::{Device, SAMPLE_COUNT};
use wgpu::{LoadOp, Operations, RenderPassDescriptor};
use crate::frame::App;
use crate::layout::popup::Popup;
use crate::map::Map;
use crate::paint::button::PaintButton;
use crate::paint::checkbox::PaintCheckBox;
use crate::paint::radio::PaintRadioButton;
use crate::paint::rectangle::PaintRectangle;
use crate::paint::slider::PaintSlider;
use crate::paint::spinbox::PaintSpinBox;
use crate::widgets::checkbox::CheckBox;
use crate::widgets::radio::RadioButton;
use crate::widgets::slider::Slider;

pub struct UiM {
    pub(crate) layouts: Vec<Layout>,
    pub(crate) popups: Map<Popup>,
    pub(crate) context: Context,
}

impl UiM {
    pub fn new(context: Context) -> Self {
        UiM {
            layouts: vec![],
            popups: Map::new(),
            context,
        }
    }


    pub fn update_text_task(&mut self, buffer: &TextBuffer) {
        for layout in self.layouts.iter_mut() {
            match layout.widgets.get_mut(&buffer.id) {
                None => {}
                Some(paint) => {
                    paint.paint_text_mut().set_text(&mut self.context, &buffer.text);
                    paint.paint_text_mut().set_wrap(&mut self.context, &buffer.text_wrap);
                    paint.paint_text_mut().set_font_size(&mut self.context, &buffer.text_size);
                }
            }
        }
    }

    pub fn get_edit_text(&self, id: &str) -> String {
        for layout in self.layouts.iter() {
            if let Some(value) = layout.widgets.get(id) {
                return value.paint_edit().text();
            }
        }
        panic!("id错误");
    }
}

pub struct Ui {
    pub(crate) device: Device,
    pub(crate) style: Style,
    pub(crate) current_layout: Option<Layout>,
    pub(crate) ui_manage: UiM,
    pub(crate) current_scrollable: bool,
    pub(crate) scroll_layouts: Vec<Layout>,
    pub(crate) ids: Vec<String>,
}

pub struct DrawParam<'a, A> {
    pub device: &'a Device,
    pub context: &'a mut Context,
    pub app: &'a mut A,
}


impl Ui {
    pub fn new(device: Device, context: Context, style: Style) -> Self {
        let rect = Rect::new().with_size(context.size.width as f32, context.size.height as f32);
        let mut layout = Layout::top_to_bottom().with_max_rect(rect.clone()); //默认布局是垂直布局，大小为总大小
        layout.available_rect = rect.clone_add_padding(&Padding::same(5.0));
        Ui {
            device,
            style,
            current_layout: Some(layout),
            current_scrollable: false,
            ui_manage: UiM::new(context),
            scroll_layouts: vec![],
            ids: vec![],
        }
    }

    pub(crate) fn add_paint_task(&mut self, id: String, paint_task: PaintTask) {
        assert!(!self.ids.contains(&id));
        self.ids.push(id.clone());
        let layout = self.current_layout.as_mut().unwrap();
        layout.insert_widget(id, paint_task);
    }

    pub fn horizontal(&mut self, mut callback: impl FnMut(&mut Ui)) {
        let mut previous_layout = self.current_layout.take().unwrap();
        let rect = previous_layout.available_rect.clone();
        let current_layout = Layout::left_to_right().with_max_rect(rect);
        self.current_layout.replace(current_layout); //设置当前布局
        callback(self);
        let current_layout = self.current_layout.take().unwrap();
        previous_layout.alloc_layout(&current_layout);
        self.current_layout.replace(previous_layout);
        if self.current_scrollable {
            self.scroll_layouts.push(current_layout);
        } else {
            self.ui_manage.layouts.push(current_layout);
        }
    }

    pub fn vertical(&mut self, callback: impl Fn(&mut Ui)) {
        let mut previous_layout = self.current_layout.take().unwrap();
        let rect = previous_layout.available_rect.clone();
        let current_layout = Layout::top_to_bottom().with_max_rect(rect);
        self.current_layout.replace(current_layout);
        callback(self);
        let current_layout = self.current_layout.take().unwrap();
        previous_layout.alloc_layout(&current_layout);
        self.current_layout.replace(previous_layout);
        if self.current_scrollable {
            self.scroll_layouts.push(current_layout);
        } else {
            self.ui_manage.layouts.push(current_layout);
        }
    }

    pub fn label(&mut self, text: impl ToString) {
        let mut label = Label::new(text.to_string());
        label.draw(self);
    }

    pub fn button(&mut self, text: &str) -> &mut PaintButton {
        let mut button = Button::new(text.to_string());
        button.draw(self);
        let layout = self.current_layout.as_mut().unwrap();
        let task = layout.widgets.get_mut(&button.id).unwrap();
        task.paint_btn_mut()
    }

    pub fn spinbox(&mut self, value: i32, range: Range<i32>) -> &mut PaintSpinBox {
        let mut spinbox = SpinBox::new(value).with_range(range);
        spinbox.draw(self);
        let layout = self.current_layout.as_mut().unwrap();
        let task = layout.widgets.get_mut(&spinbox.id).unwrap();
        task.paint_spinbox_mut()
    }

    pub fn image(&mut self, source: &'static str, size: (f32, f32)) {
        let mut image = Image::new(source).with_size(size.0, size.1);
        image.draw(self);
    }

    pub fn slider(&mut self, v: f32, r: Range<f32>) -> &mut PaintSlider {
        let mut slider = Slider::new(v).with_range(r);
        slider.draw(self);
        let layout = self.current_layout.as_mut().unwrap();
        let task = layout.widgets.get_mut(&slider.id).unwrap();
        task.paint_slider_mut()
    }

    pub fn checkbox(&mut self, check: bool, label: impl ToString) -> &mut PaintCheckBox {
        let mut checkbox = CheckBox::new(check, label);
        checkbox.draw(self);
        let layout = self.current_layout.as_mut().unwrap();
        let task = layout.widgets.get_mut(&checkbox.id).unwrap();
        task.paint_checkbox_mut()
    }

    pub fn radio(&mut self, check: bool, label: impl ToString) -> &mut PaintRadioButton {
        let mut radio_btn = RadioButton::new(check, label);
        radio_btn.draw(self);
        let layout = self.current_layout.as_mut().unwrap();
        let task = layout.widgets.get_mut(&radio_btn.id).unwrap();
        task.paint_radio_mut()
    }

    pub fn add_space(&mut self, space: f32) {
        let layout = self.current_layout.as_mut().unwrap();
        layout.add_space(space);
    }

    pub fn available_rect(&self) -> Rect {
        self.current_layout.as_ref().unwrap().available_rect.clone()
    }


    pub fn paint_rect(&mut self, rect: Rect, style: ClickStyle) -> &mut PaintRectangle {
        println!("{:?}", rect);
        let mut task = PaintRectangle::new(self, rect);
        task.set_style(style);
        task.prepare(&self.device, false, false);
        let id = task.id.clone();
        self.add_paint_task(task.id.clone(), PaintTask::Rectangle(task));
        let layout = self.current_layout.as_mut().unwrap();
        let task = layout.widgets.get_mut(&id).unwrap();
        task.paint_rect_mut()
    }

    pub fn add(&mut self, mut widget: impl Widget) {
        widget.draw(self);
    }

    fn draw_widget<A: App>(&mut self, render_pass: &mut wgpu::RenderPass, app: &mut A) {
        let mut param = DrawParam {
            device: &self.device,
            context: &mut self.ui_manage.context,
            app,
        };
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.draw(&mut param, render_pass);
        }
        for popup in self.ui_manage.popups.iter_mut() {
            popup.draw(&mut param, render_pass);
        }
    }

    pub fn draw<A: App>(&mut self, app: &mut A) {
        if self.current_layout.is_some() { self.ui_manage.layouts.insert(0, self.current_layout.take().unwrap()); } //将总布局存入
        let surface_texture = self.ui_manage.context.surface.get_current_texture().unwrap();
        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let msaa_texture = self.device.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: wgpu::Extent3d {
                width: self.ui_manage.context.size.width,
                height: self.ui_manage.context.size.height,
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
        let mut render_pass = encoder.begin_render_pass(&render_pass_desc);
        self.draw_widget(&mut render_pass, app);

        drop(render_pass);
        self.device.queue.submit([encoder.finish()]);
        surface_texture.present();
    }
}

impl Ui {
    pub(crate) fn mouse_move<A: App>(&mut self, app: &mut A) {
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.mouse_move(&self.device, &mut self.ui_manage.context, app)
        }
        for popup in self.ui_manage.popups.iter_mut() {
            popup.layout.mouse_move(&self.device, &mut self.ui_manage.context, app)
        }
    }

    pub(crate) fn mouse_down<A: App>(&mut self, app: &mut A) {
        self.device.device_input.mouse.pressed = true;
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.mouse_down(&self.device, &mut self.ui_manage.context, app);
        }
    }

    pub(crate) fn mouse_release<A: App>(&mut self, app: &mut A) {
        self.device.device_input.mouse.pressed = false;
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.mouse_release(&self.device, &mut self.ui_manage.context, app)
        }

        for popup in self.ui_manage.popups.iter_mut() {
            popup.click(&self.device, &mut self.ui_manage.context);
        }
    }

    pub(crate) fn resize(&mut self) {
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.window_resize(&self.device, &mut self.ui_manage.context)
        }
    }

    pub(crate) fn key_input<A: App>(&mut self, key: winit::keyboard::Key, app: &mut A) {
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.key_input(&self.device, &mut self.ui_manage.context, key.clone(), app)
        }
    }

    pub(crate) fn delta_input(&mut self) {
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.delta_input(&self.device, &self.ui_manage.context)
        }
    }
}