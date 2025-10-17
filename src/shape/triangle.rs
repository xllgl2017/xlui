#[cfg(feature = "gpu")]
use crate::vertex::Vertex;
use crate::*;

pub struct TriangleShape {
    #[cfg(feature = "gpu")]
    pub vertices: Vec<Vertex>,
    #[cfg(feature = "gpu")]
    pub indices: Vec<u16>,
    pub p0: Pos,
    pub p1: Pos,
    pub p2: Pos,
}

impl TriangleShape {
    pub fn set_poses(&mut self, p0: Pos, p1: Pos, p2: Pos) {
        self.p0 = p0;
        self.p1 = p1;
        self.p2 = p2;
    }

    pub fn offset(&mut self, offset: &Offset) {
        self.p0.offset(offset.x, offset.y);
        self.p1.offset(offset.x, offset.y);
        self.p2.offset(offset.x, offset.y)
    }

    #[cfg(feature = "gpu")]
    pub fn update(&mut self, rect: &Rect, style: &WidgetStyle) {
        self.vertices.clear();
        self.indices.clear();
        let offset = Offset::new().with_x(rect.get_ox()).with_y(rect.get_oy());
        self.offset(&offset);
        self.vertices.push(Vertex {
            position: [self.p0.x, self.p0.y],
            color: style.fill.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [self.p1.x, self.p1.y],
            color: style.fill.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [self.p2.x, self.p2.y],
            color: style.fill.as_gamma_rgba(),
        });
        self.indices.extend_from_slice(&[0, 1, 2, 0])
    }
}