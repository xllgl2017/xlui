use crate::response::Response;
use crate::size::Geometry;
use crate::ui::Ui;
use crate::Widget;
use crate::widgets::WidgetSize;

pub struct Space {
    id: String,
    geometry: Geometry,
    #[deprecated = "use Geometry"]
    size: WidgetSize,
}

impl Space {
    pub fn new(space: f32) -> Space {
        Space {
            id: crate::gen_unique_id(),
            geometry: Geometry::new().with_size(space, space),
            size: WidgetSize::same(space, space),
        }
    }

    #[deprecated = "use Geometry::set_fix_width"]
    pub fn set_width(&mut self, width: f32) {
        self.size.dw = width;
        self.size.rw = width;
        self.geometry.set_fix_width(width);
    }

    #[deprecated = "use Geometry::set_fix_height"]
    pub fn set_height(&mut self, height: f32) {
        self.size.dh = height;
        self.size.rh = height;
        self.geometry.set_fix_height(height);
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