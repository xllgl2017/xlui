use std::any::Any;
use crate::response::{Callback, DrawnEvent, WidgetResponse};
use crate::size::rect::Rect;
use crate::ui::UiM;

pub struct SpinBoxResponse {
    pub(crate) rect: Rect,
    pub(crate) event: DrawnEvent,
    pub(crate) callback: Callback,
    pub(crate) value: i32,
}

impl SpinBoxResponse {
    pub fn new(rect: Rect) -> SpinBoxResponse {
        SpinBoxResponse {
            rect,
            event: DrawnEvent::Click,
            callback: Callback::new(),
            value: 0,
        }
    }

    pub fn connect<A: 'static>(&mut self, f: fn(&mut A, &mut UiM, i32)) {
        self.callback.spinbox = Some(Callback::create_spinbox(f));
    }
}

impl WidgetResponse for SpinBoxResponse {
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
}