use crate::response::Response;
use crate::size::Geometry;
use crate::ui::Ui;
use crate::Widget;
use crate::widgets::{WidgetSize, WidgetState};

pub struct Space {
    id: String,
    geometry: Geometry,
    state: WidgetState,
}

impl Space {
    pub fn new(space: f32) -> Space {
        Space {
            id: crate::gen_unique_id(),
            geometry: Geometry::new().with_size(space, space),
            state: WidgetState::default(),
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

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}