use crate::layout::LayoutKind;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
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
    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,
    hovered: bool,
    layout: Option<LayoutKind>,
    padding: Padding,
    current: Arc<RwLock<Option<String>>>,
    callback: Option<Box<dyn Fn(&String, &mut Ui)>>,
    selected: bool,
}

impl ItemWidget {
    pub fn new(layout: LayoutKind) -> Self {
        ItemWidget {
            id: crate::gen_unique_id(),
            fill_param: RectParam::new(Rect::new(), ClickStyle::new()),
            fill_index: 0,
            fill_buffer: None,
            hovered: false,
            layout: Some(layout),
            padding: Padding::same(2.0),
            current: Arc::new(RwLock::new(None)),
            callback: None,
            selected: false,
        }
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.fill_param.rect.set_size(w, h);
        self
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.fill_param.rect.set_height(h);
        self
    }

    pub fn with_style(mut self, style: ClickStyle) -> Self {
        self.fill_param.style = style;
        self
    }

    pub fn with_padding(mut self, padding: Padding) -> Self {
        self.padding = padding;
        self
    }

    pub fn show(mut self, ui: &mut Ui, mut context: impl FnMut(&mut Ui)) {
        self.fill_param.rect = ui.layout().available_rect().clone_with_size(&self.fill_param.rect);
        self.layout.as_mut().unwrap().set_rect(self.fill_param.rect.clone(), &self.padding);
        let previous_layout = ui.layout.replace(self.layout.take().unwrap()).unwrap();
        context(ui);
        self.layout = ui.layout.replace(previous_layout);
        ui.add(self);
    }

    fn update_rect(&mut self, ui: &mut Ui) {
        let data = self.fill_param.as_draw_param(self.hovered || self.selected, ui.device.device_input.mouse.pressed || self.selected);
        ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
        ui.context.window.request_redraw();
    }

    pub fn connect(mut self, f: impl Fn(&String, &mut Ui) + 'static) -> Self {
        self.callback = Some(Box::new(f));
        self
    }

    pub(crate) fn parent(mut self, parent: Arc<RwLock<Option<String>>>) -> Self {
        self.current = parent;
        self
    }

    fn init(&mut self, ui: &mut Ui) {
        let data = self.fill_param.as_draw_param(false, false);
        let buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &buffer);
        self.fill_buffer = Some(buffer);
    }
}

impl Widget for ItemWidget {
    fn redraw(&mut self, ui: &mut Ui) -> Response {
        if self.fill_buffer.is_none() { self.init(ui); }
        if ui.pass.is_none() { return Response::new(&self.id, &self.fill_param.rect); }
        let current = self.current.read().unwrap();
        if current.as_ref() != Some(&self.id) && self.selected {
            drop(current);
            self.selected = false;
            self.update_rect(ui);
        }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        self.layout.as_mut().unwrap().redraw(ui);
        Response::new(&self.id, &self.fill_param.rect)
    }

    fn update(&mut self, ui: &mut Ui) {
        self.layout.as_mut().unwrap().update(ui);
        match ui.update_type {
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.fill_param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.update_rect(ui);
                }
            }
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.fill_param.rect) {
                    self.selected = true;
                    if let Some(ref mut callback) = self.callback {
                        callback(&self.id, ui);
                    }
                    self.update_rect(ui);
                    ui.context.window.request_redraw();
                    return;
                }
            }
            UpdateType::Offset(ref o) => {
                self.fill_param.rect.offset(o.x, o.y);
                self.update_rect(ui);
            }
            _ => {}
        }
    }
}