use crate::paint::scroll_bar::PaintScrollBar;
use crate::paint::PaintTask;
use crate::response::button::ButtonResponse;
use crate::response::DrawnEvent;
use crate::size::rect::Rect;
use crate::ui::{Ui, UiM};
use crate::widgets::Widget;

pub struct ScrollBar {
    id: String,
    pub(crate) rect: Rect,
}

impl ScrollBar {
    pub fn new() -> ScrollBar {
        ScrollBar {
            id: crate::gen_unique_id(),
            rect: Rect::new().with_size(10.0, 20.0),

        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.rect.set_size(width, height);
        self
    }
}


impl Widget for ScrollBar {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        self.rect.x.min += 5.0;
        layout.alloc_rect(&self.rect);
        let paint_rectangle = PaintScrollBar::new(ui, &self.rect);
        let rectangle_id = format!("{}_rectangle", self.id);
        ui.add_paint_task(rectangle_id.clone(), PaintTask::ScrollBar(paint_rectangle));
        ui.response.insert(self.id.clone(), ButtonResponse::new(self.rect.clone()).event(DrawnEvent::Hover));
    }

    fn update(&mut self, uim: &mut UiM) {}
}