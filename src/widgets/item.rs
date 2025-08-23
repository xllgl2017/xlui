use crate::layout::LayoutKind;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::Widget;
use std::sync::{Arc, RwLock};
use crate::frame::context::UpdateType;

pub struct ItemWidget {
    pub(crate) id: String,
    fill_render: RenderParam<RectParam>,
    hovered: bool,
    layout: Option<LayoutKind>,
    padding: Padding,
    current: Arc<RwLock<Option<String>>>,
    callback: Option<Box<dyn Fn(&String, &mut Ui)>>,
    selected: bool,
    changed: bool,
}

impl ItemWidget {
    pub fn new(layout: LayoutKind) -> Self {
        ItemWidget {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new(), ClickStyle::new())),
            hovered: false,
            layout: Some(layout),
            padding: Padding::same(2.0),
            current: Arc::new(RwLock::new(None)),
            callback: None,
            selected: false,
            changed: false,
        }
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.fill_render.param.rect.set_size(w, h);
        self
    }

    pub fn with_style(mut self, style: ClickStyle) -> Self {
        self.fill_render.param.style = style;
        self
    }

    pub fn show(mut self, ui: &mut Ui, mut context: impl FnMut(&mut Ui)) {
        self.fill_render.param.rect = ui.layout().available_rect().clone_with_size(&self.fill_render.param.rect);
        self.layout.as_mut().unwrap().set_rect(self.fill_render.param.rect.clone(), &self.padding);
        let previous_layout = ui.layout.replace(self.layout.take().unwrap()).unwrap();
        if let UpdateType::Init = ui.update_type {
            println!("init", );
        }

        context(ui);
        self.layout = ui.layout.replace(previous_layout);
        ui.add(self);
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
        if current.as_ref() != Some(&self.id) && self.selected {
            drop(current);
            self.selected = false;
            self.changed = true;
        }
        if !self.changed && !ui.can_offset { return; }
        // println!("{} {:?}", ui.can_offset, ui.offset);
        let layout = self.layout.as_mut().unwrap();
        ui.update_type = UpdateType::Offset(ui.offset.clone());
        layout.update(ui);
        ui.update_type = UpdateType::None;
        self.fill_render.param.rect.offset(&ui.offset);
        self.fill_render.update(ui, self.hovered || self.selected, ui.device.device_input.mouse.pressed || self.selected);
    }
}

impl Widget for ItemWidget {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.layout.as_mut().unwrap().redraw(ui);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        // self.layout.as_mut().unwrap().update(ui);注意这里不能直接调widgets的update
        match ui.update_type {
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
                        callback(&self.id, ui);
                    }
                    self.changed = true;
                    ui.context.window.request_redraw();
                    return Response::new(&self.id, &self.fill_render.param.rect);
                }
            }
            UpdateType::Offset(ref o) => {
                if !ui.can_offset { return Response::new(&self.id, &self.fill_render.param.rect); }
                self.changed = true;
                ui.context.window.request_redraw();
                self.layout.as_mut().unwrap().update(ui);
            }
            _ => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}