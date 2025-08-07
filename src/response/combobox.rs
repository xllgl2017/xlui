use std::any::Any;
use crate::paint::combobox::PaintComboBox;
use crate::response::{Callback, DrawnEvent, WidgetResponse};
use crate::size::rect::Rect;
use crate::ui::UiM;

pub struct ComboBoxResponse {
    pub(crate) rect: Rect,
    pub(crate) event: DrawnEvent,
    pub(crate) callback: Callback,
    pub(crate) row: usize,
}

impl ComboBoxResponse {
    pub fn new(rect: Rect) -> ComboBoxResponse {
        ComboBoxResponse {
            rect,
            event: DrawnEvent::Click,
            callback: Callback::new(),
            row: 0,
        }
    }


    pub fn connect<A: 'static>(&mut self, f: fn(&mut A, &mut UiM)) {
        // self.callback.click = Some(Callback::create_click(f));
    }

    pub(crate) fn connect_paint(&mut self, f: fn(&mut PaintComboBox, usize)) {
        self.callback.combo_item = Some(Box::new(f));
    }

    pub(crate) fn call<A: 'static>(&mut self, app: A, uim: &mut UiM) {
        // for layout in uim.layouts {
        //     for child in layout.children {
        //         child.widgets.get()
        //     }
        //     layout.widgets.get()
        // }
        // if let Some(ref mut combo_item) = self.callback.combo_item {
        //     combo_item(paint, self.row)
        // }
        //
        // if let Some(ref mut click) = self.callback.click {
        //     click(app, uim);
        // }
    }
}

impl WidgetResponse for ComboBoxResponse {
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