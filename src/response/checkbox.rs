use std::any::Any;
use crate::response::{Callback, DrawnEvent, WidgetResponse};
use crate::size::rect::Rect;

pub struct CheckBoxResponse {
    pub(crate) rect: Rect,
    pub(crate) event: DrawnEvent,
    pub(crate) callback: Callback,
}

impl CheckBoxResponse {
    pub fn new(rect: Rect) -> CheckBoxResponse {
        CheckBoxResponse{
            rect,
            event: DrawnEvent::Click,
            callback: Callback::new(),
        }
    }
}

impl WidgetResponse for CheckBoxResponse {
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