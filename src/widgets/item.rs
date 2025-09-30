use crate::frame::context::UpdateType;
use crate::layout::LayoutKind;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize};
use std::mem;
use std::sync::{Arc, RwLock};
use crate::size::Geometry;

pub struct ItemWidget {
    id: String,
    fill_render: RenderParam<RectParam>,
    hovered: bool,
    layout: Option<LayoutKind>,
    data_str: String,
    current: Arc<RwLock<Option<String>>>,
    callback: Option<Box<dyn Fn(&String, &mut Ui)>>,
    selected: bool,
    changed: bool,
    geometry: Geometry,
}

impl ItemWidget {
    pub fn new(layout: LayoutKind, data_str: String) -> Self {
        ItemWidget {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new()),
            hovered: false,
            layout: Some(layout),
            data_str,
            current: Arc::new(RwLock::new(None)),
            callback: None,
            selected: false,
            changed: false,
            geometry: Geometry::new(),
        }
    }

    // pub fn with_size(mut self, w: f32, h: f32) -> Self {
    //     self.fill_render.param.rect.set_size(w, h);
    //     self
    // }

    pub fn with_style(mut self, style: ClickStyle) -> Self {
        self.fill_render.param.style = style;
        self
    }

    pub fn show(&mut self, ui: &mut Ui, mut context: impl FnMut(&mut Ui)) {
        let previous_layout = ui.layout.replace(self.layout.take().unwrap()).unwrap();
        context(ui);
        self.layout = ui.layout.replace(previous_layout);
        let resp = self.layout.as_mut().unwrap().update(ui);
        self.geometry.set_size(resp.size.dw, resp.size.dh);
        self.fill_render.param.rect.set_size(resp.size.dw, resp.size.dh);
        // ui.add(self);
    }

    // fn update_rect(&mut self, ui: &mut Ui) {
    //     self.fill_render.update(ui, self.hovered || self.selected, ui.device.device_input.mouse.pressed || self.selected);
    //     ui.context.window.request_redraw();
    // }

    pub fn connect(mut self, f: impl Fn(&String, &mut Ui) + 'static) -> Self {
        self.callback = Some(Box::new(f));
        self
    }

    pub(crate) fn parent(mut self, parent: Arc<RwLock<Option<String>>>) -> Self {
        self.current = parent;
        self
    }

    fn init(&mut self, ui: &mut Ui) {
        self.fill_render.init_rectangle(ui, false, false);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        let current = self.current.read().unwrap();
        if current.as_ref() != Some(&self.data_str) && self.selected {
            drop(current);
            self.selected = false;
            ui.widget_changed |= WidgetChange::Value;
        }
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.fill_render.param.rect.offset_to_rect(&ui.draw_rect);
            self.fill_render.update(ui, self.hovered || self.selected, ui.device.device_input.mouse.pressed || self.selected);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            self.fill_render.update(ui, self.hovered || self.selected, ui.device.device_input.mouse.pressed || self.selected);
        }
    }

    pub fn layout(&mut self) -> &mut LayoutKind {
        self.layout.as_mut().unwrap()
    }

    pub fn store_and_reset(&mut self) -> (bool, bool) {
        (mem::take(&mut self.hovered), mem::take(&mut self.selected))
    }

    pub fn restore_status(&mut self, hovered: bool, selected: bool, data_str: String) {
        self.hovered = hovered;
        self.selected = selected;
        self.data_str = data_str;
        self.changed = hovered || selected
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.layout.as_mut().unwrap().update(ui);
    }
}

impl Widget for ItemWidget {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        // self.layout.as_mut().unwrap().update(ui);注意这里不能直接调widgets的update
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => {
                self.init(ui);
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.fill_render.param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.changed = true;
                    ui.context.window.request_redraw();
                }
                self.layout.as_mut().unwrap().update(ui);
            }
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.fill_render.param.rect) {
                    self.selected = true;
                    if let Some(ref mut callback) = self.callback {
                        callback(&self.data_str, ui);
                    }
                    self.changed = true;
                    ui.context.window.request_redraw();
                    return Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()));
                }
            }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry { &mut self.geometry }
    // fn store(&mut self, datum: &dyn Any) {
    //     // let datum: &String = datum.downcast_ref().unwrap();
    //     // let layout = self.layout.as_mut().unwrap();
    //     // let label: &mut Label = layout.get_widget(&"list_item".to_string()).unwrap();
    //     // label.set_text(datum);
    //     // self.hovered = false;
    //     // self.selected = false;
    // }
}