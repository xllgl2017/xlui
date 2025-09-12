use crate::response::Response;
use crate::ui::Ui;
use crate::Widget;
use crate::widgets::WidgetSize;

pub struct Space {
    id: String,
    size: WidgetSize,
}

impl Space {
    pub fn new(space: f32) -> Space {
        Space {
            id: crate::gen_unique_id(),
            size: WidgetSize::same(space, space),
        }
    }

    pub fn set_width(&mut self, width: f32) {
        self.size.dw = width;
        self.size.rw = width;
    }

    pub fn set_height(&mut self, height: f32) {
        self.size.dh = height;
        self.size.rh = height;
    }
}


impl Widget for Space {
    fn update(&mut self, _: &mut Ui) -> Response<'_> {
        Response::new(&self.id, self.size.clone())
    }
}