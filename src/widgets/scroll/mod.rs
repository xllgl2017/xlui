pub mod bar;

use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutKind};
use crate::render::{Visual, VisualStyle, WidgetStyle};
use crate::response::Response;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::pos::Pos;
use crate::size::radius::Radius;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::ui::Ui;
use crate::widgets::scroll::bar::ScrollBar;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::{Offset, Shadow, VerticalLayout};
use std::mem;

pub struct ScrollWidget {
    pub(crate) id: String,
    pub(crate) layout: Option<LayoutKind>,
    v_bar: ScrollBar,
    h_bar: ScrollBar,
    visual: Visual,
    a: f32,
    horiz_scrollable: bool,
    vert_scrollable: bool,
    geometry: Geometry,
    state: WidgetState,
}

impl ScrollWidget {
    fn new() -> ScrollWidget {
        let fill_style = VisualStyle::same(WidgetStyle {
            fill: Color::WHITE,
            border: Border::same(1.0).color(Color::rgba(144, 209, 255, 255)),
            radius: Radius::same(2),
            shadow: Shadow::new(),
        });
        ScrollWidget {
            id: crate::gen_unique_id(),
            layout: None,
            v_bar: ScrollBar::vertical(),
            h_bar: ScrollBar::horizontal(),
            visual: Visual::new().with_enable().with_style(fill_style),
            a: 0.0,
            horiz_scrollable: false,
            vert_scrollable: false,
            geometry: Geometry::new().with_context_size(400.0, 300.0).with_padding(Padding::same(5.0)),
            state: WidgetState::default(),
        }
    }

    pub fn vertical() -> ScrollWidget {
        ScrollWidget::new().enable_vscroll()
    }

    pub fn horizontal() -> ScrollWidget {
        ScrollWidget::new().enable_hscroll()
    }

    pub fn enable_vscroll(mut self) -> Self {
        self.vert_scrollable = true;
        self
    }

    pub fn enable_hscroll(mut self) -> Self {
        self.horiz_scrollable = true;
        self
    }

    pub fn with_layout(mut self, layout: impl Layout + 'static) -> Self {
        self.set_layout(LayoutKind::new(layout));
        self
    }

    pub fn set_layout(&mut self, layout: LayoutKind) {
        self.layout = Some(layout);
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.geometry.set_fix_size(width, height);
        self
    }

    pub fn with_width(mut self, w: f32) -> Self {
        self.geometry.set_fix_width(w);
        self
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.geometry.set_fix_height(h);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.geometry.set_padding(padding);
        self
    }

    pub fn set_style(&mut self, style: VisualStyle) {
        self.visual.set_style(style);
    }

    pub(crate) fn draw(&mut self, ui: &mut Ui, mut callback: impl FnMut(&mut Ui)) {
        let mut current_layout = self.layout.take().unwrap_or_else(|| LayoutKind::new(VerticalLayout::top_to_bottom()));
        current_layout.set_size(self.geometry.context_width(), self.geometry.context_height());
        let previous_layout = ui.layout.replace(current_layout).unwrap();
        //视图内容
        callback(ui);
        let mut current_layout = ui.layout.replace(previous_layout).unwrap();
        let resp = current_layout.update(ui);
        self.visual.rect_mut().set_size(self.geometry.padding_width(), self.geometry.padding_height());
        self.v_bar.geometry().set_fix_height(self.geometry.context_height() - self.h_bar.geometry().context_height());
        self.v_bar.set_context_height(resp.size.rh);
        self.h_bar.geometry().set_fix_width(self.geometry.context_width() - self.v_bar.geometry().context_width());
        self.h_bar.set_context_width(resp.size.rw);
        self.layout = Some(current_layout);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //滚动区域
        self.v_bar.update(ui);
        self.h_bar.update(ui);
    }

    pub fn show(mut self, ui: &mut Ui, callback: impl FnMut(&mut Ui)) {
        self.draw(ui, callback);
        ui.add(self);
    }

    pub fn reset_context_height(&mut self, h: f32) {
        self.v_bar.set_context_height(h);
        self.bar_offset(0.0, 0.0);
    }

    fn bar_offset(&mut self, ox: f32, oy: f32) {
        println!("offset {} {}", ox, oy);
        let roy = self.v_bar.set_vbar_value_by_offset(-oy);
        let rox = self.h_bar.set_hbar_value_by_offset(-ox);
        let offset = Offset::new()
            .with_x(if self.horiz_scrollable { rox } else { 0.0 })
            .with_y(if self.vert_scrollable { roy } else { 0.0 });
        self.layout.as_mut().unwrap().set_offset(offset);
    }

    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        if self.a != 0.0 {
            let oy = self.a * 10.0 * 10.0;
            // let mut pos = Pos::new();
            // pos.x = self.visual.rect().dx().center();
            // pos.y = self.visual.rect().dy().center();
            if self.a.abs() - 0.001 < 0.0 {
                self.a = 0.0;
            } else if self.a > 0.0 {
                self.a -= 0.001;
            } else if self.a < 0.0 {
                self.a += 0.001;
            }
            self.bar_offset(0.0, oy);
            ui.context.window.request_redraw();
        }
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.visual.rect_mut().offset_to_rect(&ui.draw_rect);
        }
        self.visual.draw(ui, self.state.disabled, false, false, false);
        let clip = self.visual.rect().clone_add_padding(self.geometry.padding());
        ui.context.window.set_clip_rect(ui.paint.as_mut().unwrap(), clip);
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            let mut context_rect = self.geometry.context_rect();
            context_rect.add_max_x(-self.v_bar.geometry().context_width());
            context_rect.add_max_y(-self.h_bar.geometry().context_height());
            let previous_rect = mem::replace(&mut ui.draw_rect, context_rect);
            let resp = self.layout.as_mut().unwrap().update(ui);
            ui.draw_rect = previous_rect;
            resp
        } else {
            self.layout.as_mut().unwrap().update(ui)
        };
        ui.context.window.reset_clip(ui.paint.as_mut().unwrap());
        if self.vert_scrollable {
            //垂直滚动条
            if ui.widget_changed.contains(WidgetChange::Position) {
                let mut v_bar_rect = ui.draw_rect.clone();
                v_bar_rect.set_x_min(self.geometry.context_right() - self.v_bar.geometry().context_width());
                v_bar_rect.add_min_y(self.geometry.padding().top);
                let previous_rect = mem::replace(&mut ui.draw_rect, v_bar_rect);
                self.v_bar.redraw(ui);
                ui.draw_rect = previous_rect;
            } else {
                self.v_bar.redraw(ui);
            }
        }

        if self.horiz_scrollable {
            //水平滚动条
            if ui.widget_changed.contains(WidgetChange::Position) {
                let mut h_bar_rect = ui.draw_rect.clone();
                h_bar_rect.add_min_x(self.geometry.padding().left);
                h_bar_rect.set_y_min(self.geometry.context_bottom() - self.h_bar.geometry().context_height());
                let previous_rect = mem::replace(&mut ui.draw_rect, h_bar_rect);
                self.h_bar.redraw(ui);
                ui.draw_rect = previous_rect;
            } else {
                self.h_bar.redraw(ui);
            }
        }
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
                let mut rect = self.visual.rect().clone();
                rect.add_max_x(-self.v_bar.geometry().context_width() - self.geometry.padding().right);
                rect.add_max_y(-self.h_bar.geometry().context_height() - self.geometry.padding().bottom);
                if ui.device.device_input.pressed_at(&rect) {
                    let oy = ui.device.device_input.mouse.offset_y();
                    let ox = ui.device.device_input.mouse.offset_x();
                    self.bar_offset(ox, oy);
                    ui.context.window.request_redraw();
                    return Response::new(&self.id, WidgetSize::same(self.visual.rect().width(), self.visual.rect().height()));
                }
                let mut offset = Offset::new();
                if self.vert_scrollable {
                    self.v_bar.update(ui);
                    offset.y = self.v_bar.offset();
                }
                if self.horiz_scrollable {
                    self.h_bar.update(ui);
                    offset.x = self.h_bar.offset();
                }
                self.layout.as_mut().unwrap().set_offset(offset);
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MousePress => {
                self.layout.as_mut().unwrap().update(ui);
                if self.vert_scrollable { self.v_bar.update(ui); }
                if self.horiz_scrollable { self.h_bar.update(ui); }
            }
            UpdateType::MouseRelease => {
                let mut rect = self.visual.rect().clone();
                rect.add_max_x(-self.v_bar.geometry().context_width() - self.geometry.padding().right);
                rect.add_max_y(-self.h_bar.geometry().context_height() - self.geometry.padding().bottom);
                if ui.device.device_input.hovered_at(&rect) {
                    self.a = ui.device.device_input.mouse.a;
                }
                if self.vert_scrollable {
                    self.v_bar.update(ui);
                }
                if self.horiz_scrollable {
                    self.h_bar.update(ui);
                }
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MouseWheel => {
                let mut rect = self.visual.rect().clone();
                rect.add_max_x(-self.v_bar.geometry().context_width() - self.geometry.padding().right);
                rect.add_max_y(-self.h_bar.geometry().context_height() - self.geometry.padding().bottom);
                if ui.device.device_input.hovered_at(&rect) {
                    let oy = ui.device.device_input.mouse.delta_y() * 10.0;
                    self.bar_offset(0.0, oy);
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.margin_width(), self.geometry.margin_height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}