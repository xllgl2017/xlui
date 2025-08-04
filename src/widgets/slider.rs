use std::any::Any;
use std::ops::Range;
use crate::paint::PaintTask;
use crate::paint::slider::PaintSlider;
use crate::response::{Callback, DrawnEvent};
use crate::response::slider::SliderResponse;
use crate::size::rect::Rect;
use crate::ui::{Ui, UiM};
use crate::widgets::Widget;

pub struct Slider {
    pub(crate) id: String,
    pub(crate) rect: Rect,
    pub(crate) value: f32,
    pub(crate) range: Range<f32>,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut UiM, f32)>>,
}

impl Slider {
    pub fn new(v: f32) -> Slider {
        Slider {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            value: v,
            range: 0.0..1.0,
            callback: None,
        }
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut UiM, f32)) -> Self {
        self.callback = Some(Callback::create_slider(f));
        self
    }

    pub fn with_range(mut self, range: Range<f32>) -> Self {
        self.range = range;
        self
    }
}

impl Widget for Slider {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone();
        self.rect.x.min += 8.0;
        self.rect.x.max += 8.0;
        self.rect.set_size(130.0, 16.0);
        layout.alloc_rect(&self.rect);
        let task = PaintSlider::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::Slider(task));
        ui.response.insert(self.id.clone(), SliderResponse {
            rect: self.rect.clone(),
            event: DrawnEvent::Click,
            callback: Callback::slider(self.callback.take()),
            value: self.value,
        });
    }

    fn update(&mut self, uim: &mut UiM) {
        todo!()
    }
}