use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutKind, VerticalLayout};
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::scroll::bar::ScrollBar;
use crate::widgets::Widget;
use crate::Offset;
use crate::size::pos::Pos;
use crate::size::radius::Radius;

pub struct ScrollArea {
    pub(crate) id: String,
    context_rect: Rect,
    pub(crate) layout: Option<VerticalLayout>,
    padding: Padding,
    v_bar: ScrollBar,
    fill_render: RenderParam<RectParam>,
    // fill_id: String,
    // fill_param: RectParam,
    // fill_buffer: Option<wgpu::Buffer>,
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
            fill_render: RenderParam::new(RectParam::new(Rect::new(), fill_style)),
            // fill_id: "".to_string(),
            // fill_param: RectParam::new(Rect::new(), fill_style),
            // fill_buffer: None,
            a: 0.0,
        }
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        self.fill_render.param.rect.set_size(w, h);
        self.v_bar.set_height(h);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.set_size(width, height);
        self
    }

    pub fn drawn_rect(&self) -> &Rect {
        &self.fill_render.param.rect
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.fill_render.param.rect = rect;
        self.v_bar.set_height(self.fill_render.param.rect.height());
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.fill_render.param.style = style;
    }

    pub fn draw(&mut self, ui: &mut Ui, mut callback: impl FnMut(&mut Ui)) {
        self.context_rect = self.fill_render.param.rect.clone();
        self.context_rect.set_width(self.fill_render.param.rect.width() - 5.0 - self.padding.right);

        let current_layout = VerticalLayout::new().max_rect(self.context_rect.clone(), self.padding.clone());
        let previous_layout = ui.layout.replace(LayoutKind::Vertical(current_layout)).unwrap();
        //视图内容
        callback(ui);
        let current_layout = ui.layout.replace(previous_layout).unwrap();
        if let LayoutKind::Vertical(v) = current_layout {
            self.layout.replace(v);
        }
        //滚动条
        let mut v_bar_rect = self.fill_render.param.rect.clone();
        v_bar_rect.set_x_min(self.fill_render.param.rect.dx().max - 7.0);
        v_bar_rect.add_min_y(self.padding.top);
        v_bar_rect.add_max_y(-self.padding.bottom);
        v_bar_rect.set_width(5.0);
        self.v_bar.set_rect(v_bar_rect);
        self.v_bar.set_context_height(self.layout.as_ref().unwrap().height + self.padding.bottom);
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //滚动区域
        self.fill_render.init_rectangle(ui, false, false);
        // let data = self.fill_param.as_draw_param(false, false);
        // let buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        // self.fill_id = ui.context.render.rectangle.create_bind_group(&ui.device, &buffer);
        // self.fill_buffer = Some(buffer);
        self.v_bar.update(ui);
    }

    pub fn show(mut self, ui: &mut Ui, callback: impl FnMut(&mut Ui)) {
        self.fill_render.param.rect = ui.layout().available_rect().clone_with_size(&self.fill_render.param.rect);
        self.draw(ui, callback);
        ui.layout().add_child(self.id.clone(), LayoutKind::ScrollArea(self));
    }

    pub fn reset_context_height(&mut self) {
        self.v_bar.set_context_height(self.layout.as_ref().unwrap().height + self.padding.bottom);
    }
}

impl Layout for ScrollArea {
    fn update(&mut self, ui: &mut Ui) {
        match ui.update_type {
            UpdateType::ReInit => {
                self.re_init(ui);
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MouseMove => {
                if ui.device.device_input.pressed_at(&self.context_rect) && ui.device.device_input.mouse.offset_y() != 0.0 {
                    let oy = ui.device.device_input.mouse.offset_y();
                    ui.update_type = UpdateType::Offset(Offset::new(ui.device.device_input.mouse.pressed_pos).with_y(-oy));
                    ui.current_rect = self.fill_render.param.rect.clone();
                    self.v_bar.update(ui);
                    return;
                }
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MousePress => self.layout.as_mut().unwrap().update(ui),
            UpdateType::MouseRelease => {
                if ui.device.device_input.hovered_at(&self.fill_render.param.rect) {
                    self.a = ui.device.device_input.mouse.a;
                }
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MouseWheel => {
                if ui.device.device_input.hovered_at(&self.fill_render.param.rect) {
                    ui.update_type = UpdateType::Offset(Offset::new(ui.device.device_input.mouse.lastest).with_y(-ui.device.device_input.mouse.delta_y() * 10.0));
                    self.v_bar.update(ui);
                    return;
                }
            }
            UpdateType::Offset(ref o) => {
                if !self.fill_render.param.rect.has_position(o.pos) { return; }
                ui.can_offset = true;
                self.layout.as_mut().unwrap().update(ui);
                ui.update_type = UpdateType::None;
                ui.can_offset = false;
            }
            _ => {}
        }
        ui.current_rect = self.context_rect.clone();
        self.v_bar.update(ui);
        //
        // self.layout.as_mut().unwrap().update(ui);
        if let Some(o) = ui.update_type.is_offset() {
            if o.y == 0.0 { self.a = 0.0; }
            ui.update_type = UpdateType::None;
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        if self.a != 0.0 {
            let oy = self.a * 10.0 * 10.0;
            let mut pos = Pos::new();
            pos.x = self.fill_render.param.rect.dx().center();
            pos.y = self.fill_render.param.rect.dy().center();
            ui.update_type = UpdateType::Offset(Offset::new(pos).with_y(-oy));
            if self.a.abs() - 0.001 < 0.0 {
                self.a = 0.0;
            } else if self.a > 0.0 {
                self.a -= 0.001;
            } else if self.a < 0.0 {
                self.a += 0.001;
            }
            self.v_bar.update(ui);
            if !self.v_bar.scrolling() { self.a = 0.0; }
        }
        let pass = ui.pass.as_mut().unwrap();
        //滚动区域
        ui.context.render.rectangle.render(&self.fill_render, pass);
        //滚动条
        self.v_bar.redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        //视图内容
        let clip = self.fill_render.param.rect.clone_add_padding(&self.padding);
        pass.set_scissor_rect(clip.dx().min as u32, clip.dy().min as u32, clip.width() as u32, clip.height() as u32);
        self.layout.as_mut().unwrap().redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        pass.set_scissor_rect(0, 0, ui.context.size.width, ui.context.size.height);
    }
}