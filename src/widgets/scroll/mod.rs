pub mod bar;

use std::mem;
use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutKind};
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::pos::Pos;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::scroll::bar::ScrollBar;
use crate::widgets::{Widget, WidgetChange};
use crate::{Offset, VerticalLayout};

pub struct ScrollWidget {
    pub(crate) id: String,
    context_rect: Rect,
    pub(crate) layout: Option<LayoutKind>,
    v_bar: ScrollBar,
    h_bar: ScrollBar,
    fill_render: RenderParam<RectParam>,
    a: f32,
    padding: Padding,
}

impl ScrollWidget {
    pub fn new() -> ScrollWidget {
        let mut fill_style = ClickStyle::new();
        fill_style.border.inactive = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        ScrollWidget {
            id: crate::gen_unique_id(),
            context_rect: Rect::new().with_size(400.0, 300.0),
            layout: None,
            v_bar: ScrollBar::vertical(),
            h_bar: ScrollBar::horizontal(),
            fill_render: RenderParam::new(RectParam::new(Rect::new(), fill_style)),
            a: 0.0,
            padding: Padding::same(5.0),
        }
    }

    pub fn set_size(&mut self, w: f32, h: f32) {
        // let layout: &mut VerticalLayout = self.layout.as_mut().unwrap().as_mut_().unwrap();
        // layout.set_size(w, h);
        self.v_bar.set_height(h - self.h_bar.height() - self.padding.vertical());
        self.h_bar.set_width(w - self.v_bar.width() - self.padding.horizontal());
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
        self.v_bar.set_height(self.fill_render.param.rect.height() - self.h_bar.height() - self.padding.vertical());
        self.h_bar.set_width(self.fill_render.param.rect.width() - self.v_bar.width() - self.padding.horizontal());
        // let layout: &mut VerticalLayout = self.layout.as_mut().unwrap().as_mut_().unwrap();
        // layout.with_padding(padding);
        self
    }

    // pub fn set_rect(&mut self, rect: Rect) {
    //     self.fill_render.param.rect = rect;
    //     self.v_bar.set_height(self.fill_render.param.rect.height());
    // }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.fill_render.param.style = style;
    }

    pub(crate) fn draw(&mut self, ui: &mut Ui, mut callback: impl FnMut(&mut Ui)) {
        // self.context_rect = self.fill_render.param.rect.clone();
        let cw = self.fill_render.param.rect.width() - self.v_bar.width() - self.padding.horizontal();
        let ch = self.fill_render.param.rect.height() - self.h_bar.height() - self.padding.vertical();
        let current_layout = VerticalLayout::top_to_bottom().with_size(cw, ch);

        // let mut current_layout = VerticalLayout::top_to_bottom().max_rect(self.context_rect.clone(), self.padding.clone());
        // current_layout.size_mode = SizeMode::Fix;
        let previous_layout = ui.layout.replace(LayoutKind::new(current_layout)).unwrap();
        //视图内容
        callback(ui);
        let mut current_layout = ui.layout.replace(previous_layout).unwrap();
        // if let LayoutKind::Vertical(v) = current_layout {
        //     self.layout.replace(v);
        // }

        let mut context_height = 0.0;
        let layout: &mut VerticalLayout = current_layout.as_mut_().unwrap();
        layout.update(ui);
        // layout.items().iter().for_each(|item| context_height += item.height() + layout.item_space());
        self.v_bar.set_context_height(layout.height());
        self.h_bar.set_context_width(layout.width());
        println!("{} {}", layout.width(),layout.height());

        // let mut context_width = 0.0;
        // layout.items().iter().for_each(|item| if item.width() > context_width { context_width = item.width(); });
        // self.h_bar.set_context_width(context_width);
        // println!("{}", context_width);


        // println!("{}", current_layout.height());
        // self.context_rect.set_width(self.fill_render.param.rect.width() - 5.0 - self.padding.right);
        //滚动条
        // let mut v_bar_rect = self.fill_render.param.rect.clone();
        // v_bar_rect.set_x_min(self.fill_render.param.rect.dx().max - 7.0);
        // v_bar_rect.add_min_y(self.padding.top);
        // v_bar_rect.add_max_y(-self.padding.bottom);
        // v_bar_rect.set_width(5.0);
        // self.v_bar.set_rect(v_bar_rect);
        // self.v_bar.set_context_height(layout.height() + layout.item_space());

        self.layout = Some(current_layout);
        // self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //滚动区域
        self.fill_render.init_rectangle(ui, false, false);
        self.v_bar.update(ui);
        self.h_bar.update(ui);
        // let layout: &mut VerticalLayout = self.layout.as_mut().unwrap().as_mut_().unwrap();
        // layout.update(ui);
        // let mut context_width = 0.0;
        // layout.items().iter().for_each(|item| if item.width() > context_width { context_width = item.width(); });
        // self.h_bar.set_context_width(layout.width());
        // println!("{}", layout.width());

        // self.layout.as_mut().unwrap().update(ui);
        // self.v_bar.set_context_height(self.layout.as_ref().unwrap().height());
    }

    pub fn show(mut self, ui: &mut Ui, callback: impl FnMut(&mut Ui)) {
        // self.fill_render.param.rect = ui.layout().available_rect().clone_with_size(&self.fill_render.param.rect);
        self.draw(ui, callback);
        ui.add(self);
        // ui.layout().add_child(LayoutKind::ScrollArea(self));
    }

    pub fn reset_context_height(&mut self) {
        let layout: &mut VerticalLayout = self.layout.as_mut().unwrap().as_mut_().unwrap();
        self.v_bar.set_context_height(layout.height() + layout.item_space());
    }
}

impl Widget for ScrollWidget {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.re_init(ui),
            UpdateType::ReInit => {
                self.re_init(ui);
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MouseMove => {
                if ui.device.device_input.pressed_at(&self.context_rect) {
                    let oy = ui.device.device_input.mouse.offset_y();
                    let roy = self.v_bar.set_vbar_value_by_offset(-oy);
                    let ox = ui.device.device_input.mouse.offset_x();
                    let rox = self.h_bar.set_hbar_value_by_offset(-ox);
                    let offset = Offset::new(Pos::new()).with_x(rox).with_y(roy);


                    let layout: &mut VerticalLayout = self.layout.as_mut().unwrap().as_mut_().unwrap();
                    layout.set_offset(offset);
                    ui.context.window.request_redraw();
                    // ui.update_type = UpdateType::Offset(Offset::new(ui.device.device_input.mouse.pressed_pos).with_y(-oy));
                    // ui.current_rect = self.fill_render.param.rect.clone();
                    // self.v_bar.update(ui);
                    return Response::new(&self.id, self.fill_render.param.rect.width(), self.fill_render.param.rect.height());
                }
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MousePress => {
                self.layout.as_mut().unwrap().update(ui);
            }
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
                    return Response::new(&self.id, self.fill_render.param.rect.width(), self.fill_render.param.rect.height());
                }
            }
            // UpdateType::Offset(ref mut o) => {
            //     if !self.fill_render.param.rect.has_position(o.pos) { return; }
            //     ui.can_offset = true;
            //     o.target_id = self.layout.as_ref().unwrap().id.to_string();
            //     self.layout.as_mut().unwrap().update(ui);
            //     ui.update_type = UpdateType::None;
            //     ui.can_offset = false;
            // }
            _ => {}
        }
        // ui.current_rect = self.context_rect.clone();
        // self.v_bar.update(ui);
        // if let Some(o) = ui.update_type.is_offset() {
        //     if o.y == 0.0 { self.a = 0.0; }
        //     ui.update_type = UpdateType::None;
        // }
        Response::new(&self.id, self.fill_render.param.rect.width(), self.fill_render.param.rect.height())
    }

    fn redraw(&mut self, ui: &mut Ui) {
        // println!("{:?}-{:?}-{}", ui.update_type, ui.draw_rect, ui.widget_changed as u32);
        // if self.a != 0.0 {
        //     let oy = self.a * 10.0 * 10.0;
        //     let mut pos = Pos::new();
        //     pos.x = self.fill_render.param.rect.dx().center();
        //     pos.y = self.fill_render.param.rect.dy().center();
        //     ui.update_type = UpdateType::Offset(Offset::new(pos).with_y(-oy));
        //     if self.a.abs() - 0.001 < 0.0 {
        //         self.a = 0.0;
        //     } else if self.a > 0.0 {
        //         self.a -= 0.001;
        //     } else if self.a < 0.0 {
        //         self.a += 0.001;
        //     }
        //     self.v_bar.update(ui);
        //     if !self.v_bar.scrolling() { self.a = 0.0; }
        // }
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.param.rect.offset_to_rect(&ui.draw_rect);
            self.fill_render.update(ui, false, false);
        }

        let pass = ui.pass.as_mut().unwrap();

        //背景
        ui.context.render.rectangle.render(&self.fill_render, pass);
        let clip = self.fill_render.param.rect.clone_add_padding(&self.padding);
        pass.set_scissor_rect(clip.dx().min as u32, clip.dy().min as u32, clip.width() as u32, clip.height() as u32);
        let resp = if ui.widget_changed.contains(WidgetChange::Position) {
            self.context_rect = ui.draw_rect.clone();
            self.context_rect.set_width(self.fill_render.param.rect.width() - self.padding.horizontal() - self.v_bar.width());
            self.context_rect.set_height(self.fill_render.param.rect.height() - self.padding.vertical() - self.h_bar.height());
            self.context_rect.add_min_x(self.padding.left);
            self.context_rect.add_min_y(self.padding.top);
            // self.context_rect.add_min_x(self.padding.left);
            // self.context_rect.add_max_x(-self.v_bar.width() - self.padding.right);
            // self.context_rect.add_min_y(self.padding.left);
            // self.context_rect.add_max_y(-self.h_bar.height() - self.padding.bottom);
            let previous_rect = mem::take(&mut ui.draw_rect);
            ui.draw_rect = self.context_rect.clone();
            let resp = self.layout.as_mut().unwrap().update(ui);
            ui.draw_rect = previous_rect;
            resp
        } else {
            self.layout.as_mut().unwrap().update(ui)
        };
        let pass = ui.pass.as_mut().unwrap();
        pass.set_scissor_rect(0, 0, ui.context.size.width, ui.context.size.height);
        // let resp = self.layout.as_mut().unwrap().update(ui);
        //垂直滚动条
        if ui.widget_changed.contains(WidgetChange::Position) {
            let previous_rect = ui.draw_rect.clone();
            let mut v_bar_rect = previous_rect.clone();
            v_bar_rect.add_min_x(resp.width + self.padding.left);
            v_bar_rect.add_max_x(-self.padding.right);
            v_bar_rect.add_min_y(self.padding.top);
            v_bar_rect.add_max_y(-self.padding.bottom);
            ui.draw_rect = v_bar_rect;
            self.v_bar.redraw(ui);
            ui.draw_rect = previous_rect;
        } else {
            self.v_bar.redraw(ui);
        }

        //水平滚动条
        if ui.widget_changed.contains(WidgetChange::Position) {
            let previous_rect = ui.draw_rect.clone();
            let mut h_bar_rect = previous_rect.clone();
            h_bar_rect.add_min_y(resp.height + self.padding.top);
            h_bar_rect.add_max_y(-self.padding.bottom);
            h_bar_rect.add_min_x(self.padding.left);
            h_bar_rect.add_max_x(-self.padding.right);
            ui.draw_rect = h_bar_rect;
            self.h_bar.redraw(ui);
            ui.draw_rect = previous_rect;
        } else {
            self.h_bar.redraw(ui);
        }


        // let pass = ui.pass.as_mut().unwrap();
        //视图内容

        // self.layout.as_mut().unwrap().redraw(ui);

    }
}