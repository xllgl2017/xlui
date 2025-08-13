use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutKind, VerticalLayout};
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::scroll::bar::ScrollBar;
use crate::widgets::Widget;
use crate::Offset;

pub struct ScrollArea {
    id: String,
    context_rect: Rect,
    pub(crate) layout: Option<VerticalLayout>,
    padding: Padding,
    v_bar: ScrollBar,
    fill_index: usize,
    fill_param: RectParam,
    fill_buffer: Option<wgpu::Buffer>,
    a: f32,
}

impl ScrollArea {
    pub fn new() -> ScrollArea {
        let mut fill_style = ClickStyle::new();
        fill_style.border.inactive = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        ScrollArea {
            id: crate::gen_unique_id(),
            context_rect: Rect::new(),
            layout: None,
            padding: Padding::same(5.0),
            v_bar: ScrollBar::new(),
            fill_index: 0,
            fill_param: RectParam::new(Rect::new(), fill_style),
            fill_buffer: None,
            a: 0.0,
        }
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        self.fill_param.rect.set_size(w, h);
        self.v_bar.set_height(h);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.set_size(width, height);
        self
    }

    pub fn drawn_rect(&self) -> &Rect {
        &self.fill_param.rect
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.fill_param.rect = rect;
        self.v_bar.set_height(self.fill_param.rect.height());
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.fill_param.style = style;
    }

    pub fn draw(&mut self, ui: &mut Ui, mut callback: impl FnMut(&mut Ui)) {
        //滚动区域
        let data = self.fill_param.as_draw_param(false, false);
        let buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &buffer);
        self.fill_buffer = Some(buffer);
        self.context_rect = self.fill_param.rect.clone();
        self.context_rect.set_width(self.fill_param.rect.width() - 5.0 - self.padding.right);

        let current_layout = VerticalLayout::new().max_rect(self.context_rect.clone(), self.padding.clone());
        let previous_layout = ui.layout.replace(LayoutKind::Vertical(current_layout)).unwrap();
        //视图内容
        callback(ui);
        let current_layout = ui.layout.replace(previous_layout).unwrap();
        match current_layout {
            LayoutKind::Vertical(v) => { self.layout.replace(v); }
            _ => {}
        }
        //滚动条
        let mut v_bar_rect = self.fill_param.rect.clone();
        v_bar_rect.x.min = self.fill_param.rect.x.max - 7.0;
        v_bar_rect.y.min += self.padding.top;
        v_bar_rect.y.max -= self.padding.bottom;
        v_bar_rect.set_width(5.0);
        self.v_bar.set_rect(v_bar_rect);
        println!("scroll {}", self.layout.as_ref().unwrap().height);
        self.v_bar.set_context_height(self.layout.as_ref().unwrap().height);
        self.v_bar.redraw(ui);
    }

    pub fn show(mut self, ui: &mut Ui, callback: impl FnMut(&mut Ui)) {
        self.fill_param.rect = ui.layout().available_rect().clone_with_size(&self.fill_param.rect);
        self.draw(ui, callback);
        ui.layout().add_child(self.id.clone(), LayoutKind::ScrollArea(self));
    }
}

impl Layout for ScrollArea {
    fn update(&mut self, ui: &mut Ui) {
        match ui.update_type {
            UpdateType::None => {}
            // UpdateType::Init => {}
            UpdateType::MouseMove => {
                if ui.device.device_input.pressed_at(&self.context_rect) && ui.device.device_input.mouse.offset_y() != 0.0 {
                    let oy = ui.device.device_input.mouse.offset_y();
                    ui.update_type = UpdateType::Offset(Offset::new_y(-oy));
                }
            }
            UpdateType::MousePress => {}
            UpdateType::MouseRelease => {
                if ui.device.device_input.hovered_at(&self.fill_param.rect) {
                    self.a = ui.device.device_input.mouse.a;
                }
            }
            UpdateType::MouseWheel => {
                if ui.device.device_input.hovered_at(&self.fill_param.rect) {
                    ui.update_type = UpdateType::Offset(Offset::new_y(-ui.device.device_input.mouse.delta_y() * 10.0));
                }
            }
            UpdateType::KeyRelease(_) => {}
            UpdateType::Offset(_) => {}
        }

        self.v_bar.update(ui);
        ui.current_rect = self.layout.as_ref().unwrap().drawn_rect();
        self.layout.as_mut().unwrap().update(ui);
        if let Some(o) = ui.update_type.is_offset() {
            if o.y == 0.0 { self.a = 0.0; }
            ui.update_type = UpdateType::None;
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        if self.a != 0.0 {
            let oy = self.a * 20.0 * 20.0;
            ui.update_type = UpdateType::Offset(Offset::new_y(-oy));
            println!("{}-{}-{}-{}", self.id, self.a, self.a < 0.0, self.a + 0.0005 < 0.0);
            if self.a.abs() - 0.0005 < 0.0 {
                self.a = 0.0;
            } else if self.a > 0.0 {
                self.a -= 0.0005;
            } else if self.a < 0.0 {
                self.a += 0.0005;
            }
            ui.context.window.request_redraw();
            println!("{}-{}", self.id, self.a);
            self.v_bar.update(ui);
            self.layout.as_mut().unwrap().update(ui);
            if let Some(o) = ui.update_type.is_offset() {
                if o.y == 0.0 { self.a = 0.0; }
                ui.update_type = UpdateType::None;
            }
        }
        let pass = ui.pass.as_mut().unwrap();
        //滚动区域
        ui.context.render.rectangle.render(self.fill_index, pass);
        //滚动条
        self.v_bar.redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        //视图内容
        let clip = self.fill_param.rect.clone_add_padding(&self.padding);
        pass.set_scissor_rect(clip.x.min as u32, clip.y.min as u32, clip.width() as u32, clip.height() as u32);
        self.layout.as_mut().unwrap().redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        pass.set_scissor_rect(0, 0, ui.context.size.width, ui.context.size.height);
    }
}