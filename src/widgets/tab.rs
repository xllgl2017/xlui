use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::SizeMode;
use crate::text::buffer::TextBuffer;
use crate::widgets::{WidgetChange, WidgetSize};
use crate::{Align, Border, BorderStyle, ClickStyle, Color, FillStyle, LayoutKind, Radius, Rect, RichText, Ui, UpdateType, VerticalLayout, Widget};

pub struct TabLabel {
    id: String,
    text: TextBuffer,
    fill: RenderParam<RectParam>,
    changed: bool,
}

impl TabLabel {
    fn new(text: impl Into<RichText>) -> TabLabel {
        let mut tab_style = ClickStyle::new();
        tab_style.fill = FillStyle::same(Color::GREEN);
        tab_style.border = BorderStyle::same(Border::new(0.0).radius(Radius::same(5)));
        TabLabel {
            id: crate::gen_unique_id(),
            text: TextBuffer::new(text).with_align(Align::Center).height(25.0),
            fill: RenderParam::new(RectParam::new(Rect::new().with_height(25.0), tab_style)),
            changed: false,
        }
    }


    fn init(&mut self, ui: &mut Ui) {
        self.text.init(ui);
        self.fill.param.rect.set_size(if self.text.geometry.width() < 50.0 { 50.0 } else { self.text.geometry.width() }, self.text.geometry.height());
        println!("{:?}", self.fill.param.rect);
        self.fill.init_rectangle(ui, false, false);
    }


    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill.param.rect.offset_to_rect(&ui.draw_rect);
            ui.widget_changed |= WidgetChange::Value;
            self.text.geometry.set_pos(ui.draw_rect.dx().min, ui.draw_rect.dy().min);
            // self.text.rect.offset_to_rect(&ui.draw_rect);
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
    items: Vec<TabItem>,
    size_mode: SizeMode,
}

impl TabWidget {
    pub fn new() -> TabWidget {
        TabWidget {
            id: crate::gen_unique_id(),
            current: None,
            items: vec![],
            size_mode: SizeMode::Auto,
        }
    }
    pub fn add_tab(&mut self, ui: &mut Ui, name: impl Into<RichText>, context: impl FnOnce(&mut Ui)) {
        if let Some(previous) = self.current {
            self.items[previous].label.fill.param.style.fill = FillStyle::same(Color::YELLOW);
        }
        self.current = Some(self.items.len());
        let ut = ui.update_type.clone();
        ui.update_type = UpdateType::Init;
        let mut label = TabLabel::new(name);
        label.update(ui);
        let current_layout = VerticalLayout::top_to_bottom();
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
}


impl Widget for TabWidget {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        let mut context_rect = ui.draw_rect.clone();
        let mut tab_text_rect = ui.draw_rect.clone();
        for index in 0..self.items.len() {
            ui.draw_rect = tab_text_rect.clone();
            let item = &mut self.items[index];
            let clicked = if let UpdateType::MouseRelease = ui.update_type { true } else { false };
            let resp = item.label.update(ui);
            ui.draw_rect.set_size(resp.size.dw, resp.size.dh);
            tab_text_rect.add_min_x(resp.size.dw);
            if clicked && ui.draw_rect.has_position(ui.device.device_input.mouse.lastest.relative) {
                let previous = self.current.replace(index);
                item.label.fill.param.style.fill = FillStyle::same(Color::GREEN);
                if let Some(previous) = previous {
                    self.items[previous].label.fill.param.style.fill = FillStyle::same(Color::YELLOW);
                }

                ui.context.window.request_redraw();
            }
        }

        context_rect.add_min_y(25.0);
        if let Some(current) = self.current {
            ui.draw_rect = context_rect;
            self.items[current].layout.update(ui);
        }
        let (w, h) = self.size_mode.size(100.0, 100.0);
        Response::new(&self.id, WidgetSize::same(w, h))
    }
}