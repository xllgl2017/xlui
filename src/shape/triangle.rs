#[cfg(feature = "gpu")]
use crate::vertex::Vertex;
use crate::{Offset, Pos};

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
}