use crate::response::Response;
use crate::size::Geometry;
use crate::ui::Ui;
use crate::Widget;
use crate::widgets::WidgetSize;

pub struct Space {
    id: String,
    geometry: Geometry,
}

impl Space {
    pub fn new(space: f32) -> Space {
        Space {
            id: crate::gen_unique_id(),
            geometry: Geometry::new().with_size(space, space),
        }
    }
}


impl Widget for Space {
    fn update(&mut self, _: &mut Ui) -> Response<'_> {
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }
}