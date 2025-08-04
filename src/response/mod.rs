pub mod button;
pub mod slider;
pub mod checkbox;

use std::any::Any;
use std::collections::HashMap;
use crate::Device;
use crate::response::button::ButtonResponse;
use crate::response::slider::SliderResponse;
use crate::size::rect::Rect;
use crate::ui::UiM;

pub enum DrawnEvent {
    None,
    Hover,
    Click,
}

pub struct Callback {
    click: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM)>>,
    slider: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM, f32)>>,
}

impl Callback {
    pub fn new() -> Self {
        Callback {
            click: None,
            slider: None,
        }
    }
    pub fn set_click<A: 'static>(&mut self, f: fn(&mut A, &mut UiM)) {
        self.click = Some(Self::create_click(f));
    }

    pub(crate) fn create_click<A: 'static>(f: fn(&mut A, &mut UiM)) -> Box<dyn FnMut(&mut dyn Any, &mut UiM)> {
        Box::new(move |target, uim| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim);
        })
    }

    pub fn click(f: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM)>>) -> Self {
        let mut res = Callback::new();
        res.click = f;
        res
    }

    pub(crate) fn create_slider<A: 'static>(f: fn(&mut A, &mut UiM, f32)) -> Box<dyn FnMut(&mut dyn Any, &mut UiM, f32)> {
        Box::new(move |target, uim, value| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    pub fn slider(f: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM, f32)>>) -> Self {
        let mut res = Callback::new();
        res.slider = f;
        res
    }
}

pub struct Response {
    ids: HashMap<String, usize>,
    values: Vec<Box<dyn WidgetResponse>>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            ids: HashMap::new(),
            values: vec![],
        }
    }

    pub fn button_response(&mut self) -> &mut ButtonResponse {
        let resp = self.values.last_mut().unwrap().as_mut();
        let resp = resp.as_any_mut().downcast_mut::<ButtonResponse>().unwrap();
        resp
    }

    pub fn slider_response(&mut self) -> &mut SliderResponse {
        let resp = self.values.last_mut().unwrap();
        let resp = resp.as_any_mut().downcast_mut::<SliderResponse>().unwrap();
        resp
    }

    pub(crate) fn clicked<A: 'static>(resp: &mut Box<dyn WidgetResponse>, app: &mut A, uim: &mut UiM) {
        if let Some(ref mut callback) = resp.callback().click {
            callback(app, uim);
            uim.context.window.request_redraw();
        }
    }

    pub(crate) fn slider<A: 'static>(resp: &mut Box<dyn WidgetResponse>, app: &mut A, uim: &mut UiM) {
        let slider_resp = resp.as_any_mut().downcast_mut::<SliderResponse>();
        if slider_resp.is_none() { return; }
        let slider_resp = slider_resp.unwrap();
        let slider_value = slider_resp.slider_value;
        if let Some(ref mut callback) = slider_resp.callback().slider {
            callback(app, uim, slider_value);
            uim.context.window.request_redraw();
        }
    }

    pub fn insert(&mut self, id: String, resp: impl WidgetResponse + 'static) {
        self.ids.insert(id, self.values.len());
        self.values.push(Box::new(resp));
    }

    pub fn update(&mut self, id: String, rect: Rect) {
        let index = self.ids[&id];
        let resp = &mut self.values[index];
        resp.set_rect(rect);
    }

    pub fn mouse_release<A: 'static>(&mut self, app: &mut A, device: &Device, uim: &mut UiM) {
        let (x, y) = device.device_input.mouse.lastest();
        for value in self.values.iter_mut() {
            let has_pos = value.rect().has_position(x, y);
            if !has_pos { continue; }
            Self::clicked(value, app, uim);
        }
    }

    pub fn mouse_move<A: 'static>(&mut self, app: &mut A, device: &Device, uim: &mut UiM) {
        let (x, y) = device.device_input.mouse.lastest();
        for value in self.values.iter_mut() {
            let has_pos = value.rect().has_position(x, y);
            if !has_pos { continue; }
            Self::slider(value, app, uim);
        }
    }
}

pub trait WidgetResponse {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn callback(&mut self) -> &mut Callback;
    fn rect(&self) -> &Rect;
    fn set_rect(&mut self, rect: Rect);
}