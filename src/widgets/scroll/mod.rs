pub mod bar;

use crate::frame::context::UpdateType;
use crate::layout::{Layout, LayoutKind};
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
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
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::{Offset, VerticalLayout};
use crate::size::Geometry;

pub struct ScrollWidget {
    pub(crate) id: String,
    context_rect: Rect,
    pub(crate) layout: Option<LayoutKind>,
    v_bar: ScrollBar,
    h_bar: ScrollBar,
    fill_render: RenderParam,
    a: f32,
    horiz_scrollable: bool,
    vert_scrollable: bool,
    geometry: Geometry,
    state: WidgetState,
}

impl ScrollWidget {
    fn new() -> ScrollWidget {
        let mut fill_style = ClickStyle::new();
        fill_style.border.inactive = Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        ScrollWidget {
            id: crate::gen_unique_id(),
            context_rect: Rect::new(),
            layout: None,
            v_bar: ScrollBar::vertical(),
            h_bar: ScrollBar::horizontal(),
            fill_render: RenderParam::new(RenderKind::Rectangle(RectParam::new().with_style(fill_style))),
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

    pub fn set_style(&mut self, style: ClickStyle) {
        self.fill_render.set_style(style);
    }

    pub(crate) fn draw(&mut self, ui: &mut Ui, mut callback: impl FnMut(&mut Ui)) {
        let mut current_layout = self.layout.take().unwrap_or_else(|| LayoutKind::new(VerticalLayout::top_to_bottom()));
        // let lw = self.geometry.width() - self.geometry.padding().horizontal() - self.v_bar.geometry().width(); //self.width - self.padding.horizontal() - self.v_bar.width();
        // let lh = self.geometry.height() - self.geometry.padding().vertical() - self.h_bar.geometry().height(); //self.height - self.padding.vertical() - self.h_bar.height();
        current_layout.set_size(self.geometry.context_width(), self.geometry.context_height());
        let previous_layout = ui.layout.replace(current_layout).unwrap();
        //视图内容
        callback(ui);
        let mut current_layout = ui.layout.replace(previous_layout).unwrap();
        let resp = current_layout.update(ui);
        self.fill_render.rect_mut().set_size(self.geometry.padding_width(), self.geometry.padding_height());
        self.v_bar.geometry().set_fix_height(self.geometry.context_height() - self.h_bar.geometry().context_height() - self.geometry.padding().vertical());
        self.v_bar.set_context_height(resp.size.rh);
        self.h_bar.geometry().set_fix_width(self.geometry.context_width() - self.v_bar.geometry().context_width() - self.geometry.padding().horizontal());
        self.h_bar.set_context_width(resp.size.rw);
        self.layout = Some(current_layout);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        //滚动区域
        #[cfg(feature = "gpu")]
        self.fill_render.init(ui, false, false);
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
            let mut pos = Pos::new();
            pos.x = self.fill_render.rect().dx().center();
            pos.y = self.fill_render.rect().dy().center();
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
            self.fill_render.offset_to_rect(&ui.draw_rect);
        }
        self.fill_render.draw(ui, false, false);
        let clip = self.fill_render.rect().clone_add_padding(self.geometry.padding());
        ui.context.window.set_clip_rect(ui.paint.as_mut().unwrap(), clip);
        let resp = if ui.widget_changed.contains(WidgetChange::Position) {
            self.context_rect = ui.draw_rect.clone();
            self.context_rect.set_width(self.fill_render.rect().width() - self.geometry.padding().horizontal() - self.v_bar.geometry().context_width());
            self.context_rect.set_height(self.fill_render.rect().height() - self.geometry.padding().vertical() - self.h_bar.geometry().context_height());
            self.context_rect.add_min_x(self.geometry.padding().left);
            self.context_rect.add_min_y(self.geometry.padding().top);
            let previous_rect = ui.draw_rect.clone();
            ui.draw_rect = self.context_rect.clone();
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
                let previous_rect = ui.draw_rect.clone();
                let mut v_bar_rect = previous_rect.clone();
                v_bar_rect.add_min_x(resp.size.dw + self.geometry.padding().left);
                v_bar_rect.add_max_x(-self.geometry.padding().right);
                v_bar_rect.add_min_y(self.geometry.padding().top);
                v_bar_rect.add_max_y(-self.geometry.padding().bottom);
                ui.draw_rect = v_bar_rect;
                self.v_bar.redraw(ui);
                ui.draw_rect = previous_rect;
            } else {
                self.v_bar.redraw(ui);
            }
        }

        if self.horiz_scrollable {
            //水平滚动条
            if ui.widget_changed.contains(WidgetChange::Position) {
                let previous_rect = ui.draw_rect.clone();
                let mut h_bar_rect = previous_rect.clone();
                h_bar_rect.add_min_y(resp.size.dh + self.geometry.padding().top);
                h_bar_rect.add_max_y(-self.geometry.padding().bottom);
                h_bar_rect.add_min_x(self.geometry.padding().left);
                h_bar_rect.add_max_x(-self.geometry.padding().right);
                ui.draw_rect = h_bar_rect;
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
                if ui.device.device_input.pressed_at(&self.context_rect) {
                    let oy = ui.device.device_input.mouse.offset_y();
                    let ox = ui.device.device_input.mouse.offset_x();
                    self.bar_offset(ox, oy);
                    ui.context.window.request_redraw();
                    return Response::new(&self.id, WidgetSize::same(self.fill_render.rect().width(), self.fill_render.rect().height()));
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
                if ui.device.device_input.hovered_at(self.fill_render.rect()) {
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
                if ui.device.device_input.hovered_at(self.fill_render.rect()) {
                    let oy = ui.device.device_input.mouse.delta_y() * 10.0;
                    self.bar_offset(0.0, oy);
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.fill_render.rect().width(), self.fill_render.rect().height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}