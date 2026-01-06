use crate::style::{Visual, VisualStyle, WidgetStyle};
use crate::response::Response;
use crate::size::Geometry;
use crate::text::buffer::TextBuffer;
use crate::widgets::{WidgetChange, WidgetSize, WidgetState};
use crate::*;

pub struct TabHeader {
    id: String,
    text: TextBuffer,
    visual: Visual,
    state: WidgetState,
}

impl TabHeader {
    fn new(text: impl Into<RichText>) -> TabHeader {
        let tab_style = VisualStyle::same(WidgetStyle {
            fill: Color::WHITE,
            border: Border::same(1.0).with_bottom(0.0).color(Color::rgb(160, 160, 160)),
            radius: Radius::same(1),
            shadow: Shadow::new(),
        });
        TabHeader {
            id: gen_unique_id(),
            text: TextBuffer::new(text).with_align(Align::Center).fix_height(25.0).min_width(50.0).padding(Padding::same(3.0)),
            visual: Visual::new().with_enable().with_style(tab_style),
            state: WidgetState::default(),
        }
    }

    pub fn set_style(&mut self, style: VisualStyle) {
        self.visual.set_style(style);
    }

    fn init(&mut self, ui: &mut Ui) {
        self.text.init(ui);
        self.visual.rect_mut().set_size(self.text.geometry.padding_width(), self.text.geometry.padding_height());
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.visual.rect_mut().offset_to_rect(&ui.draw_rect);
            ui.widget_changed |= WidgetChange::Value;
            self.text.geometry.offset_to_rect(&ui.draw_rect);
        }
    }

    fn draw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.visual.draw(ui, self.state.disabled, false, false, false);
        self.text.redraw(ui);
    }
}

impl Widget for TabHeader {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.draw(ui),
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.visual.rect_mut().width(), self.visual.rect_mut().height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.text.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}

pub struct TabItem {
    header: TabHeader,
    layout: LayoutKind,
}

/// ### TabWidget的示例用法
/// ```rust
/// use xlui::*;
///
/// fn draw<A:App>(ui:&mut Ui){
///     let mut tab=TabWidget::new()
///         //设置大小
///         .with_size(400.0,300.0);
///     let header=tab.add_tab(ui,"tab1",|ui|ui.label("这里是tab1"));
///     //这里可以对tab头进行设置
///     header.geometry().set_padding(Padding::same(2.0));
///     tab.add_tab(ui,"tab2",|ui|ui.label("这里是tab2"));
///
/// }
/// ```

pub struct TabWidget {
    id: String,
    current: Option<usize>,
    space: f32,
    items: Vec<TabItem>,
    geometry: Geometry,
    visual: Visual,
    state: WidgetState,
}

impl TabWidget {
    pub fn new() -> TabWidget {
        let fill_style = VisualStyle::same(WidgetStyle {
            fill: Color::WHITE,
            border: Border::same(1.0).color(Color::rgba(144, 209, 255, 255)),
            radius: Radius::same(1),
            shadow: Shadow::new(),
        });
        TabWidget {
            id: gen_unique_id(),
            current: None,
            space: 2.0,
            items: vec![],
            geometry: Geometry::new(),
            visual: Visual::new().with_enable().with_style(fill_style),
            state: WidgetState::default(),
        }
    }
    pub fn add_tab(&mut self, ui: &mut Ui, name: impl Into<RichText>, context: impl FnOnce(&mut Ui)) -> &mut TabHeader {
        if let Some(previous) = self.current {
            self.items[previous].header.visual.style_mut().inactive.fill = Color::TRANSPARENT;
        }
        self.current = Some(self.items.len());
        let ut = ui.update_type.clone();
        ui.update_type = UpdateType::Init;
        let mut header = TabHeader::new(name);
        header.update(ui);
        let current_layout = VerticalLayout::top_to_bottom().with_padding(Padding::same(2.0));
        let previous_layout = ui.layout.replace(LayoutKind::new(current_layout));
        context(ui);
        let current_layout = ui.layout.take().unwrap();
        ui.layout = previous_layout;
        let item = TabItem {
            header,
            layout: current_layout,
        };
        self.items.push(item);
        ui.update_type = ut;
        &mut self.items.last_mut().unwrap().header
    }

    fn init(&mut self) {
        self.visual.rect_mut().set_size(self.geometry.padding_width(), self.geometry.padding_height());
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
                self.visual.rect_mut().offset_to_rect(&context_rect);
            }
            self.visual.draw(ui, self.state.disabled, false, false, false);
        }
        context_rect.add_min_y(1.0);
        let mut tab_text_rect = ui.draw_rect.clone();
        let mut width = 0.0;
        for index in 0..self.items.len() {
            ui.draw_rect = tab_text_rect.clone();
            let item = &mut self.items[index];
            let clicked = if let UpdateType::MouseRelease = ui.update_type { true } else { false };
            let resp = item.header.update(ui);
            width += resp.size.dw + self.space;
            ui.draw_rect.set_size(resp.size.dw, resp.size.dh);
            tab_text_rect.add_min_x(resp.size.dw + self.space);
            if clicked && ui.draw_rect.has_position(ui.device.device_input.mouse.lastest.relative) {
                let previous = self.current.replace(index);
                item.header.visual.style_mut().inactive.fill = Color::WHITE;
                if let Some(previous) = previous && previous != index {
                    self.items[previous].header.visual.style_mut().inactive.fill = Color::TRANSPARENT;
                }

                ui.context.window.request_redraw();
            }
        }
        if let Some(current) = self.current {
            ui.draw_rect = context_rect;
            let resp = self.items[current].layout.update(ui);
            self.geometry.set_context_size(if width > resp.size.dw { width } else { resp.size.dw }, resp.size.dh + 25.0);
        }
        match ui.update_type {
            UpdateType::Init | UpdateType::ReInit | _ => self.init(),
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