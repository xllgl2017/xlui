use crate::{Border, Color, Pos, Rect};
use crate::shape::draw_fan;
use crate::shape::ring::RingShape;
use crate::vertex::Vertex;

pub struct CircleShape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

impl CircleShape {
    pub fn new() -> CircleShape {
        CircleShape {
            vertices: vec![],
            indices: vec![],
        }
    }

    ///绘制一个圆，带边框和填充
    /// * rect-圆所在的区域
    /// * fill-填充色
    /// * border-边框
    pub fn draw(&mut self, rect: &Rect, fill: &Color, border: &Border) {
        self.vertices.clear();
        self.indices.clear();
        let center = Pos {
            x: rect.dx().center(),
            y: rect.dy().center(),
        };
        let mut start_pos = Pos {
            x: rect.dx().center(),
            y: rect.dy().min + border.width(),
        };
        //绘制扇形区域
        let (mut ps, mut iss) = draw_fan(center, start_pos, self.vertices.len() as u16 + 1, fill, 360);
        self.vertices.append(&mut ps);
        self.indices.append(&mut iss);
        //绘制边框
        start_pos.y = rect.dy().min;
        let mut ring_shape = RingShape::new().with_degree(360).with_center(center);
        let (mut ps, mut is) = ring_shape.draw(start_pos, self.vertices.len() as u16 + 1, border); // draw_ring(center, start_pos, self.vertices.len() as u16 + 1, border, 90);
        self.vertices.append(&mut ps);
        self.indices.append(&mut is);
    }
}