use crate::frame::context::Context;
// use crate::paint::checkbox::PaintCheckBox;
// use crate::paint::PaintTask;
use crate::response::{Callback, Response};
use crate::size::rect::Rect;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;
use std::any::Any;
use crate::frame::App;
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::border::Border;
use crate::size::SizeMode;
use crate::style::ClickStyle;
use crate::style::color::Color;

pub struct CheckBox {
    pub(crate) id: String,
    rect: Rect,
    text: TextBuffer,
    check_text: TextBuffer,
    value: bool,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui, bool)>>,
    size_mode: SizeMode,

    check_param: RectParam,
    check_index: usize,
    check_buffer: Option<wgpu::Buffer>,

    hovered: bool,
}

impl CheckBox {
    pub fn new(v: bool, label: impl ToString) -> CheckBox {
        let mut check_style = ClickStyle::new();
        check_style.fill.inactive = Color::rgb(210, 210, 210);
        check_style.fill.hovered = Color::rgb(210, 210, 210);
        check_style.fill.clicked = Color::rgb(210, 210, 210);
        check_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        check_style.border.hovered = Border::new(1.0).color(Color::BLACK).radius(Radius::same(2));
        check_style.border.clicked = Border::new(1.0).color(Color::BLACK).radius(Radius::same(2));
        CheckBox {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            text: TextBuffer::new(label.to_string()),
            check_text: TextBuffer::new("√".to_string()),
            value: v,
            callback: None,
            size_mode: SizeMode::Auto,
            check_param: RectParam::new(Rect::new(), check_style),
            check_index: 0,
            check_buffer: None,
            hovered: false,
        }
    }


    pub fn reset_size(&mut self, context: &Context) {
        self.text.rect = self.rect.clone();
        self.text.reset_size(context);
        self.text.rect.offset_x(15.0);
        match self.size_mode {
            SizeMode::Auto => {
                self.rect.set_width(15.0 + self.text.rect.width());
                self.rect.set_height(20.0);
            }
            SizeMode::FixWidth => self.rect.set_height(20.0),
            SizeMode::FixHeight => self.rect.set_width(15.0 + self.text.rect.width()),
            SizeMode::Fix => {}
        }

        self.text.rect.set_height(self.rect.height());
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, bool)) -> Self {
        self.callback = Some(Callback::create_check(f));
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.rect.set_width(width);
        self.size_mode = SizeMode::FixWidth;
        self
    }

    pub fn set_callback<A: App>(&mut self, f: fn(&mut A, &mut Ui, bool)) {
        self.callback = Some(Callback::create_check(f));
    }
}

impl Widget for CheckBox {
    fn draw(&mut self, ui: &mut Ui) -> Response {
        //分配大小
        self.rect = ui.layout().available_rect().clone_with_size(&self.rect);
        self.reset_size(&ui.context);
        // ui.layout().alloc_rect(&self.rect);
        //复选框
        self.check_param.rect = self.rect.clone();
        self.check_param.rect.set_width(15.0);
        self.check_param.rect.set_height(15.0);
        let data = self.check_param.as_draw_param(false, self.value);
        let check_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.check_index = ui.context.render.rectangle.create_bind_group(&ui.device, &check_buffer);
        self.check_buffer = Some(check_buffer);
        //文本
        self.text.draw(ui);
        self.check_text.reset_size(&ui.context);
        self.check_text.rect = self.check_param.rect.clone();
        // self.check_text.rect.y.min += 2.0;
        self.check_text.draw(ui);
        Response {
            id: self.id.clone(),
            rect: self.rect.clone(),
        }
        // let layout = ui.current_layout.as_mut().unwrap();
        // self.rect = layout.available_rect.clone_with_size(&self.rect);
        // self.reset_size(&ui.ui_manage.context);
        // layout.alloc_rect(&self.rect);
        // let task = PaintCheckBox::new(ui, self);
        // ui.add_paint_task(self.id.clone(), PaintTask::CheckBox(task));
    }

    fn update(&mut self, ui: &mut Ui) {
        if ui.device.device_input.click_at(&self.rect) {
            self.value = !self.value;
            let data = self.check_param.as_draw_param(self.value, self.value);
            ui.device.queue.write_buffer(self.check_buffer.as_ref().unwrap(), 0, data);
            if let Some(ref mut callback) = self.callback {
                let app = ui.app.take().unwrap();
                callback(*app, ui, self.value);
                ui.app.replace(app);
            }
            ui.context.window.request_redraw();
            return;
        }

        let hovered = ui.device.device_input.hovered_at(&self.rect);
        if self.hovered != hovered {
            self.hovered = hovered;
            let data = self.check_param.as_draw_param(self.hovered, ui.device.device_input.mouse.pressed);
            ui.device.queue.write_buffer(self.check_buffer.as_ref().unwrap(), 0, data);
            ui.context.window.request_redraw();
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.check_index, pass);
        self.text.redraw(ui);
        if self.value { self.check_text.redraw(ui); }
    }
}