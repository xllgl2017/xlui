use crate::frame::context::UpdateType;
use crate::layout::LayoutKind;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::Response;
use crate::size::Geometry;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use std::sync::{Arc, RwLock};

pub struct ItemWidget {
    id: String,
    fill_render: RenderParam,
    layout: Option<LayoutKind>,
    data_str: String,
    current: Arc<RwLock<Option<String>>>,
    callback: Option<Box<dyn Fn(&String, &mut Ui)>>,
    geometry: Geometry,
    state: WidgetState,
}

impl ItemWidget {
    pub fn new(layout: LayoutKind, data_str: String) -> Self {
        ItemWidget {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RenderKind::Rectangle(RectParam::new())),
            layout: Some(layout),
            data_str,
            current: Arc::new(RwLock::new(None)),
            callback: None,
            geometry: Geometry::new(),
            state: WidgetState::default(),
        }
    }

    pub fn with_style(mut self, style: ClickStyle) -> Self {
        self.fill_render.set_style(style);
        self
    }

    pub fn show(&mut self, ui: &mut Ui, mut context: impl FnMut(&mut Ui)) {
        let previous_layout = ui.layout.replace(self.layout.take().unwrap()).unwrap();
        context(ui);
        self.layout = ui.layout.replace(previous_layout);
        let resp = self.layout.as_mut().unwrap().update(ui);
        self.geometry.set_size(resp.size.dw, resp.size.dh);
        self.fill_render.rect_mut().set_size(resp.size.dw, resp.size.dh);
    }

    pub fn connect(mut self, f: impl Fn(&String, &mut Ui) + 'static) -> Self {
        self.callback = Some(Box::new(f));
        self
    }

    pub(crate) fn parent(mut self, parent: Arc<RwLock<Option<String>>>) -> Self {
        self.current = parent;
        self
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        // let current = self.current.read().unwrap();
        // if current.as_ref() != Some(&self.data_str) {
        //     drop(current);
        //     self.state.changed = true;
        // }
        if self.state.changed { ui.widget_changed |= WidgetChange::Value; }
        self.state.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.fill_render.offset_to_rect(&ui.draw_rect);
            // #[cfg(feature = "gpu")]
            // self.fill_render.update(ui, self.hovered || self.selected, ui.device.device_input.mouse.pressed || self.selected);
        }

        // if ui.widget_changed.contains(WidgetChange::Value) {
        //     // #[cfg(feature = "gpu")]
        //     // self.fill_render.update(ui, self.hovered || self.selected, ui.device.device_input.mouse.pressed || self.selected);
        // }
    }

    pub fn layout(&mut self) -> &mut LayoutKind {
        self.layout.as_mut().unwrap()
    }

    pub fn state(&self) -> &WidgetState {
        &self.state
    }

    // pub fn store_and_reset(&mut self) -> &WidgetState {
    //     mem::take(&mut self.state)
    // }

    pub fn restore_status(&mut self, hovered: bool, data_str: String) {
        self.state.hovered = hovered;
        self.data_str = data_str;
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let current = self.current.read().unwrap();
        let selected = current.as_ref() == Some(&self.data_str);
        self.fill_render.draw(ui, self.state.hovered || selected, selected);
        self.layout.as_mut().unwrap().update(ui);
    }
}

impl Widget for ItemWidget {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        // self.layout.as_mut().unwrap().update(ui);注意这里不能直接调widgets的update
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            #[cfg(feature = "gpu")]
            UpdateType::Init => self.fill_render.init(ui, false, false),
            UpdateType::ReInit => {
                #[cfg(feature = "gpu")]
                self.fill_render.init(ui, false, false);
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(self.fill_render.rect());
                if self.state.on_hovered(hovered) { ui.context.window.request_redraw(); }
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MouseRelease => {
                let clicked = ui.device.device_input.click_at(self.fill_render.rect());
                if self.state.on_clicked(clicked) {
                    if let Some(ref mut callback) = self.callback {
                        callback(&self.data_str, ui);
                    }
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry { &mut self.geometry }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}