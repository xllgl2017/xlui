use std::ops::Range;
use crate::frame::context::Context;
use crate::layout::Layout;
use crate::paint::PaintTask;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::Style;
use crate::text::text_buffer::TextBuffer;
use crate::widgets::button::Button;
use crate::widgets::image::Image;
use crate::widgets::label::Label;
use crate::widgets::spinbox::SpinBox;
use crate::widgets::Widget;
use crate::{Device, SAMPLE_COUNT};
use wgpu::{LoadOp, Operations, RenderPassDescriptor};
use crate::response::button::ButtonResponse;
use crate::response::Response;
use crate::response::slider::SliderResponse;
use crate::widgets::slider::Slider;

pub struct UiM {
    pub(crate) layouts: Vec<Layout>,
    pub(crate) context: Context,
}

impl UiM {
    pub fn new(context: Context) -> Self {
        UiM {
            layouts: vec![],
            context,
        }
    }


    pub fn update_text_task(&mut self, buffer: &TextBuffer) {
        for layout in self.layouts.iter_mut() {
            match layout.widgets.get_mut(&buffer.id) {
                None => {}
                Some(paint) => {
                    paint.paint_text().set_text(&mut self.context, &buffer.text);
                    paint.paint_text().set_wrap(&mut self.context, &buffer.text_wrap);
                    paint.paint_text().set_font_size(&mut self.context, &buffer.text_size);
                }
            }
        }
    }
}

pub struct Ui {
    pub(crate) device: Device,
    pub(crate) style: Style,
    pub(crate) current_layout: Option<Layout>,
    pub(crate) ui_manage: UiM,
    pub(crate) response: Response,
    pub(crate) current_scrollable: bool,
    pub(crate) scroll_layouts: Vec<Layout>,
    change: bool,
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
            response: Response::new(),
            change: true,
            scroll_layouts: vec![],
        }
    }

    pub(crate) fn add_paint_task(&mut self, id: String, paint_task: PaintTask) {
        let layout = self.current_layout.as_mut().unwrap();
        layout.insert_widget(id,paint_task);
        // layout.ids.insert(id, layout.widgets.len());
        // layout.widgets.push(paint_task);
    }

    pub fn horizontal(&mut self, callback: impl Fn(&mut Ui)) {
        let mut previous_layout = self.current_layout.take().unwrap();
        let mut rect = previous_layout.available_rect.clone();
        rect.y.max = 0.0;
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

    pub fn button(&mut self, text: &str) -> &mut ButtonResponse {
        let mut button = Button::new(text.to_string());
        button.draw(self);
        self.response.button_response()
    }

    pub fn spinbox(&mut self, value: i32) {
        let mut spinbox = SpinBox::new(value);
        spinbox.draw(self);
    }

    pub fn image(&mut self, source: &'static str, size: (f32, f32)) {
        let mut image = Image::new(source);
        image.set_size(size.0, size.1);
        image.draw(self);
    }

    pub fn slider(&mut self, v: f32, r: Range<f32>) -> &mut SliderResponse {
        let mut slider = Slider::new().with_value(v).with_range(r);
        slider.draw(self);
        self.response.slider_response()
    }

    pub fn add(&mut self, mut widget: impl Widget) {
        widget.draw(self);
    }

    fn draw_widget(&mut self, render_pass: &mut wgpu::RenderPass) {
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.draw(&self.device, &mut self.ui_manage.context, render_pass);
        }
    }

    pub fn draw(&mut self) {
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
        self.draw_widget(&mut render_pass);

        drop(render_pass);
        self.device.queue.submit([encoder.finish()]);
        surface_texture.present();
    }
}

impl Ui {
    pub(crate) fn mouse_move<A: 'static>(&mut self, app: &mut A) {
        let mut updates = vec![];
        for layout in self.ui_manage.layouts.iter_mut() {
            updates.append(&mut layout.mouse_move(&self.device, &mut self.ui_manage.context));
        }
        for (id, rect) in updates {
            self.response.update(id, rect);
        }
        self.response.mouse_move(app, &self.device, &mut self.ui_manage);
    }

    pub(crate) fn mouse_down(&mut self) {
        self.device.device_input.mouse.pressed = true;
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.mouse_down(&self.device, &mut self.ui_manage.context);
        }
    }

    pub(crate) fn mouse_release<A: 'static>(&mut self, app: &mut A) {
        self.device.device_input.mouse.pressed = false;
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.mouse_release(&self.device, &mut self.ui_manage.context)
        }
        self.response.mouse_release(app, &self.device, &mut self.ui_manage);
        self.ui_manage.context.window.request_redraw();
    }

    pub(crate) fn resize(&mut self) {
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.window_resize(&self.device, &mut self.ui_manage.context)
        }
    }

    pub(crate) fn key_input(&mut self, key: winit::keyboard::Key) {
        for layout in self.ui_manage.layouts.iter_mut() {
            layout.key_input(&self.device, &mut self.ui_manage.context, key.clone())
        }
    }

    pub(crate) fn delta_input(&mut self) {
        let mut updates = vec![];
        for layout in self.ui_manage.layouts.iter_mut() {
            updates.append(&mut layout.delta_input(&self.device, &self.ui_manage.context));
        }
        for (id, rect) in updates {
            self.response.update(id, rect);
        }
    }
}