use std::any::Any;
use crate::response::{Callback, DrawnEvent, WidgetResponse};
use crate::size::rect::Rect;
use crate::ui::UiM;

pub struct ButtonResponse {
    pub(crate) rect: Rect,
    pub(crate) event: DrawnEvent,
    pub(crate) callback: Callback,
}

impl ButtonResponse {
    pub fn has_event(&self) -> bool {
        match self.event {
            DrawnEvent::None => false,
            DrawnEvent::Hover | DrawnEvent::Click => true,
        }
    }

    pub fn new(rect: Rect) -> ButtonResponse {
        ButtonResponse {
            rect,
            event: DrawnEvent::None,
            callback: Callback::new(),
        }
    }

    pub fn event(mut self, event: DrawnEvent) -> ButtonResponse {
        self.event = event;
        self
    }

    pub fn connect<A: 'static>(&mut self, f: fn(&mut A, &mut UiM)) {
        self.callback.click = Some(Callback::create_click(f));
    }
}

impl WidgetResponse for ButtonResponse {
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