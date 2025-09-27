use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::Geometry;
use crate::text::buffer::TextBuffer;
use crate::widgets::{WidgetChange, WidgetSize};
use crate::{Align, Border, BorderStyle, ClickStyle, Color, FillStyle, LayoutKind, Padding, Radius, Rect, RichText, Ui, UpdateType, VerticalLayout, Widget};

pub struct TabLabel {
    id: String,
    text: TextBuffer,
    fill: RenderParam<RectParam>,
    changed: bool,
}

impl TabLabel {
    fn new(text: impl Into<RichText>) -> TabLabel {
        let mut tab_style = ClickStyle::new();
        tab_style.fill = FillStyle::same(Color::WHITE);
        let mut border = Border::same(1.0).radius(Radius::same(1)).color(Color::rgb(160, 160, 160));
        border.bottom_width = 0.0;
        tab_style.border = BorderStyle::same(border);
        TabLabel {
            id: crate::gen_unique_id(),
            text: TextBuffer::new(text).with_align(Align::Center).fix_height(25.0).min_width(50.0),
            fill: RenderParam::new(RectParam::new(Rect::new().with_height(25.0), tab_style)),
            changed: false,
        }
    }


    fn init(&mut self, ui: &mut Ui) {
        self.text.init(ui);
        self.fill.param.rect.set_size(self.text.geometry.width(), self.text.geometry.height());
        self.fill.init_rectangle(ui, false, false);
    }


    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill.param.rect.offset_to_rect(&ui.draw_rect);
            ui.widget_changed |= WidgetChange::Value;
            self.text.geometry.offset_to_rect(&ui.draw_rect);
        }
        if ui.widget_changed.contains(WidgetChange::Value) {
            self.fill.update(ui, false, false);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill, pass);
        self.text.redraw(ui);
    }
}

impl Widget for TabLabel {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.draw(ui),
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.fill.param.rect.width(), self.fill.param.rect.height()))
    }
}

pub struct TabItem {
    label: TabLabel,
    layout: LayoutKind,
}

pub struct TabWidget {
    id: String,
    current: Option<usize>,
    space: f32,
    items: Vec<TabItem>,
    geometry: Geometry,
    fill: RenderParam<RectParam>,
}

impl TabWidget {
    pub fn new() -> TabWidget {
        let mut fill_style = ClickStyle::new();
        fill_style.fill = FillStyle::same(Color::WHITE);
        fill_style.border = BorderStyle::same(Border::same(1.0).radius(Radius::same(1)).color(Color::rgba(144, 209, 255, 255)));
        TabWidget {
            id: crate::gen_unique_id(),
            current: None,
            space: 2.0,
            items: vec![],
            geometry: Geometry::new(),
            fill: RenderParam::new(RectParam::new(Rect::new(), fill_style)),
        }
    }
    pub fn add_tab(&mut self, ui: &mut Ui, name: impl Into<RichText>, context: impl FnOnce(&mut Ui)) {
        if let Some(previous) = self.current {
            self.items[previous].label.fill.param.style.fill = FillStyle::same(Color::TRANSPARENT);
        }
        self.current = Some(self.items.len());
        let ut = ui.update_type.clone();
        ui.update_type = UpdateType::Init;
        let mut label = TabLabel::new(name);
        label.update(ui);
        let current_layout = VerticalLayout::top_to_bottom().with_padding(Padding::same(2.0));
        let previous_layout = ui.layout.replace(LayoutKind::new(current_layout));
        context(ui);
        let current_layout = ui.layout.take().unwrap();
        ui.layout = previous_layout;
        let item = TabItem {
            label,
            layout: current_layout,
        };
        self.items.push(item);
        ui.update_type = ut;
    }

    fn init(&mut self, ui: &mut Ui) {
        self.fill.param.rect.set_size(self.geometry.width(), self.geometry.height());
        self.fill.init_rectangle(ui, false, false);
    }

    pub fn with_width(mut self, w: f32) -> Self {
        self.geometry.set_fix_width(w);
        self
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.geometry.set_fix_height(h);
        self
    }

    pub fn with_size(self, w: f32, h: f32) -> Self {
        self.with_width(w).with_height(h)
    }
}


impl Widget for TabWidget {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        let mut context_rect = ui.draw_rect.clone();
        context_rect.add_min_y(24.0);
        if let UpdateType::Draw = ui.update_type {
            if ui.widget_changed.contains(WidgetChange::Position) {
                self.fill.param.rect.offset_to_rect(&context_rect);
                self.fill.update(ui, false, false);
            }
            let pass = ui.pass.as_mut().unwrap();
            ui.context.render.rectangle.render(&self.fill, pass);
        }
        context_rect.add_min_y(1.0);
        let mut tab_text_rect = ui.draw_rect.clone();
        let mut width = 0.0;
        for index in 0..self.items.len() {
            ui.draw_rect = tab_text_rect.clone();
            let item = &mut self.items[index];
            let clicked = if let UpdateType::MouseRelease = ui.update_type { true } else { false };
            let resp = item.label.update(ui);
            width += resp.size.dw + self.space;
            ui.draw_rect.set_size(resp.size.dw, resp.size.dh);
            tab_text_rect.add_min_x(resp.size.dw + self.space);
            if clicked && ui.draw_rect.has_position(ui.device.device_input.mouse.lastest.relative) {
                let previous = self.current.replace(index);
                item.label.fill.param.style.fill = FillStyle::same(Color::WHITE);
                if let Some(previous) = previous && previous != index {
                    self.items[previous].label.fill.param.style.fill = FillStyle::same(Color::TRANSPARENT);
                }

                ui.context.window.request_redraw();
            }
        }
        if let Some(current) = self.current {
            ui.draw_rect = context_rect;
            let resp = self.items[current].layout.update(ui);
            self.geometry.set_size(if width > resp.size.dw { width } else { resp.size.dw }, resp.size.dh + 25.0);
        }
        match ui.update_type {
            UpdateType::Init | UpdateType::ReInit | _ => self.init(ui),
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }
}