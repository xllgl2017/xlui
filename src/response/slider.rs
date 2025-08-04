use std::any::Any;
use crate::response::{Callback, DrawnEvent, WidgetResponse};
use crate::size::rect::Rect;
use crate::ui::UiM;

pub struct SliderResponse {
    pub(crate) rect: Rect,
    pub(crate) event: DrawnEvent,
    pub(crate) callback: Callback,
    pub(crate) value: f32,
}

impl SliderResponse {
    pub fn new(rect: Rect) -> SliderResponse {
        SliderResponse {
            rect,
            event: DrawnEvent::None,
            callback: Callback::new(),
            value: 0.0,
        }
    }

    pub fn connect<A: 'static>(&mut self, f: fn(&mut A, &mut UiM, f32)) {
        self.callback.slider = Some(Callback::create_slider(f));
    }

    fn set_slider_value(&mut self, value: f32) {
        self.value = value;
    }

}

impl WidgetResponse for SliderResponse {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        &mut *self
    }

    fn callback(&mut self) -> &mut Callback {
        &mut self.callback
    }


    fn rect(&self) -> &Rect {
        &self.rect
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }


    // fn mouse_press(&self) -> bool {
    //     self.mouse_press
    // }
    //
    // fn focused(&self) -> bool {
    //     self.focused
    // }

}