pub mod button;
pub mod slider;
pub mod checkbox;

use std::any::Any;
use std::collections::HashMap;
use crate::Device;
use crate::map::Map;
use crate::response::button::ButtonResponse;
use crate::response::checkbox::CheckBoxResponse;
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
    check: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM, bool)>>,
}

impl Callback {
    pub fn new() -> Self {
        Callback {
            click: None,
            slider: None,
            check: None,
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

    pub(crate) fn create_check<A: 'static>(f: fn(&mut A, &mut UiM, bool)) -> Box<dyn FnMut(&mut dyn Any, &mut UiM, bool)> {
        Box::new(move |target, uim, value| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    pub fn check(f: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM, bool)>>) -> Self {
        let mut res = Callback::new();
        res.check = f;
        res
    }
}

pub struct Response {
    values: Map<Box<dyn WidgetResponse>>,
}

impl Response {
    pub fn new() -> Self {
        Response {
            values: Map::new(),
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

    pub fn check_response(&mut self) -> &mut CheckBoxResponse {
        let resp = self.values.last_mut().unwrap();
        let resp = resp.as_any_mut().downcast_mut::<CheckBoxResponse>().unwrap();
        resp
    }

    fn clicked<A: 'static>(resp: &mut Box<dyn WidgetResponse>, app: &mut A, uim: &mut UiM) {
        if let Some(ref mut callback) = resp.callback().click {
            callback(app, uim);
            uim.context.window.request_redraw();
        }
    }

    fn checked<A: 'static>(resp: &mut Box<dyn WidgetResponse>, app: &mut A, uim: &mut UiM) {
        let check_resp = resp.as_any_mut().downcast_mut::<CheckBoxResponse>();
        if check_resp.is_none() { return; }
        let check_resp = check_resp.unwrap();
        let check_value = check_resp.checked;
        if let Some(ref mut callback) = check_resp.callback.check {
            callback(app, uim, check_value);
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
        self.values.insert(id, Box::new(resp));
        // self.ids.insert(id, self.values.len());
        // self.values.push(Box::new(resp));
    }

    pub fn update(&mut self, id: String, rect: Rect) {
        if let Some(value) = self.values.get_mut(&id) {
            value.set_rect(rect);
        }
        // match self.values.get_mut(&id) {
        //     None => {}
        //     Some(value) => {}
        // }
        // let index = self.ids[&id];
        // let resp = &mut self.values[index];
        // resp.set_rect(rect);
    }

    pub fn mouse_release<A: 'static>(&mut self, app: &mut A, device: &Device, uim: &mut UiM) {
        let (x, y) = device.device_input.mouse.lastest();
        for (_, value) in self.values.iter_mut() {
            let has_pos = value.rect().has_position(x, y);
            if !has_pos { continue; }
            Self::clicked(value, app, uim);
            Self::checked(value, app, uim);
        }
    }

    pub fn mouse_move<A: 'static>(&mut self, app: &mut A, device: &Device, uim: &mut UiM) {
        let (x, y) = device.device_input.mouse.lastest();
        for (_, value) in self.values.iter_mut() {
            let has_pos = value.rect().has_position(x, y);
            if !has_pos { continue; }
            Self::slider(value, app, uim);
        }
    }

    pub fn resp_mut(&mut self, id: &String) -> Option<&mut CheckBoxResponse> {
        let resp = self.values.get_mut(id)?;
        let resp = resp.as_any_mut().downcast_mut::<CheckBoxResponse>()?;
        Some(resp)
    }
}

pub trait WidgetResponse {
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn callback(&mut self) -> &mut Callback;
    fn rect(&self) -> &Rect;
    fn set_rect(&mut self, rect: Rect);
}