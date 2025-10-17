use crate::{Pos, Rect};
use crate::vertex::Vertex;

pub struct TriangleShape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    pub p0: Pos,
    pub p1: Pos,
    pub p2: Pos,
}