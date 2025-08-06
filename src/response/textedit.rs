use std::any::Any;
use crate::response::{Callback, DrawnEvent, WidgetResponse};
use crate::size::rect::Rect;
use crate::ui::UiM;

pub struct TextEditResponse {
    pub(crate) rect: Rect,
    pub(crate) event: DrawnEvent,
    pub(crate) callback: Callback,
    pub(crate) value: String,
}

impl TextEditResponse {
    pub fn new(rect: Rect) -> TextEditResponse {
        TextEditResponse {
            rect,
            event: DrawnEvent::Click,
            callback: Callback::new(),
            value: "".to_string(),
        }
    }

    pub fn connect<A: 'static>(&mut self, f: fn(&mut A, &mut UiM, &str)) {
        self.callback.textedit = Some(Callback::create_textedit(f));
    }

    pub(crate) fn call<A: 'static>(&mut self, app: &mut A, uim: &mut UiM) {
        if let Some(ref mut callback) = self.callback.textedit {
            callback(app, uim, &self.value);
        }
    }
}

impl WidgetResponse for TextEditResponse {
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