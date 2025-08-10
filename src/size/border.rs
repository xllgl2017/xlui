use crate::radius::Radius;
use crate::style::color::Color;

#[derive(Clone)]
pub struct Border {
    pub width: f32,
    pub radius: Radius,
    pub color: Color,
}

impl Border {
    pub fn new(width: f32) -> Self {
        Border {
            width,
            radius: Radius::same(3),
            color: Color::rgb(0, 0, 0),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn radius(mut self, radius: Radius) -> Self {
        self.radius = radius;
        self
    }

    // #[deprecated]
    // fn vertex_border_radius(&self, r: f32, size: &Size, [a, b]: [f32; 2], t: RadiusBorder, mut is: u16) -> (Vec<Vertex>, Vec<u16>) {
    //     let mut vertexes = vec![];
    //     let mut indices = vec![];
    //     let segments = 90;
    //     for segment in (1..=segments).step_by(5) {
    //         let [p, q] = self.radius.get_p_q_pos(a, b, r, &t, segment);
    //         vertexes.push(Vertex::new(p, &self.color, size));
    //         vertexes.push(Vertex::new(q, &self.color, size));
    //         indices.extend_from_slice(&[is, is + 1]); //圆心和两顶点围成三角形
    //         is += 2;
    //     }
    //     // for _ in 0..self.width {
    //     //     r -= 0.5;
    //     //
    //     // }
    //     (vertexes, indices)
    // }
    //
    // pub fn radius_border_vertex(&self, size: &Size, rect: &Rect) -> (Vec<Vertex>, Vec<u16>) {
    //     if self.width == 0.0 { return (vec![], vec![]); }
    //     //左下圆角边框
    //     let r = self.radius.left_bottom as f32; //半径
    //     let circle = [rect.x.min + r, rect.y.max - r]; //圆心;ax+1/2=a/width
    //     let (mut vertexes, mut indices) = self.vertex_border_radius(r, size, circle, RadiusBorder::LeftBottom, 0);
    //     //----------------------------------------------------------------------------------------
    //     //底下边框-------strip不需要
    //     // let lbrx = rect.x.min + self.radius.left_bottom as f32;
    //     // let rbrx = rect.x.max - self.radius.right_bottom as f32;
    //     // let mut max_y = rect.y.max;
    //     //
    //     // let current = vertexes.len() as u16;
    //     // vertexes.push(Vertex::new([lbrx, max_y], &self.color, size));
    //     // vertexes.push(Vertex::new([rbrx, max_y], &self.color, size));
    //     // indices.extend_from_slice(&[current, current + 1]);
    //     //----------------------------------------------------------------------------------------
    //
    //     //右下圆角边框
    //     let r = self.radius.right_bottom as f32; //半径
    //     let circle = [rect.x.max - r, rect.y.max - r]; //圆心;ax+1/2=a/width
    //     let (mut vs, mut is) = self.vertex_border_radius(r, size, circle, RadiusBorder::RightBottom, vertexes.len() as u16);
    //     vs.reverse();
    //     vertexes.append(&mut vs);
    //     indices.append(&mut is);
    //     //----------------------------------------------------------------------------------------
    //     //右侧边框
    //     // let rbry = rect.y.max - self.radius.right_bottom as f32;
    //     // let rtry = rect.y.min + self.radius.right_top as f32;
    //     // let mut max_x = rect.x.max;
    //     //
    //     // let current = vertexes.len() as u16;
    //     // vertexes.push(Vertex::new([max_x, rbry], &self.color, size));
    //     // vertexes.push(Vertex::new([max_x, rtry], &self.color, size));
    //     // indices.extend_from_slice(&[current, current + 1]);
    //     //----------------------------------------------------------------------------------------
    //     //右上圆角边框
    //     let r = self.radius.right_top as f32; //半径
    //     let circle = [rect.x.max - r, rect.y.min + r]; //圆心;ax+1/2=a/width
    //     let (mut vs, mut is) = self.vertex_border_radius(r, size, circle, RadiusBorder::RightTop, vertexes.len() as u16);
    //     vertexes.append(&mut vs);
    //     indices.append(&mut is);
    //     //----------------------------------------------------------------------------------------
    //     //顶部边框
    //     // let current = vertexes.len() as u16;
    //     // let ltrx = rect.x.min + self.radius.left_top as f32;
    //     // let rtrx = rect.x.max - self.radius.right_top as f32;
    //     // let mut min_y = rect.y.min;
    //     // vertexes.push(Vertex::new([rtrx, min_y], &self.color, size));
    //     // vertexes.push(Vertex::new([ltrx, min_y], &self.color, size));
    //     // indices.extend_from_slice(&[current , current + 1]);
    //     //----------------------------------------------------------------------------------------
    //     //左上圆角边框
    //
    //     let r = self.radius.left_top as f32; //半径
    //     let circle = [rect.x.min + r, rect.y.min + r]; //圆心;ax+1/2=a/width
    //     let (mut vs, mut is) = self.vertex_border_radius(r, size, circle, RadiusBorder::LeftTop, vertexes.len() as u16);
    //     vs.reverse();
    //     vertexes.append(&mut vs);
    //     indices.append(&mut is);
    //
    //     // let lbry = rect.y.max - self.radius.left_bottom as f32;
    //     // let ltry = rect.y.min + self.radius.left_top as f32;
    //     // let mut min_x = rect.x.min;
    //     // println!("{:?} {:?}", vertexes, indices);
    //     // for _ in 0..self.width {
    //     //     //bottom line
    //     //     let current = vertexes.len() as u16;
    //     //     vertexes.push(Vertex::new([lbrx, max_y], &self.color, size));
    //     //     vertexes.push(Vertex::new([rbrx, max_y], &self.color, size));
    //     //     indices.extend_from_slice(&[current, current + 1]);
    //     //     max_y -= 0.5;
    //     //
    //     //     // //top line
    //     //     // vertexes.push(Vertex::new([ltrx, min_y], &self.color, size));
    //     //     // vertexes.push(Vertex::new([rtrx, min_y], &self.color, size));
    //     //     // indices.extend_from_slice(&[current + 2, current + 3]);
    //     //     // min_y += 0.5;
    //     //     // //right line
    //     //     // vertexes.push(Vertex::new([max_x, rtry], &self.color, size));
    //     //     // vertexes.push(Vertex::new([max_x, rbry], &self.color, size));
    //     //     // indices.extend_from_slice(&[current + 4, current + 5]);
    //     //     // max_x -= 0.5;
    //     //     //
    //     //     // //left line
    //     //     // vertexes.push(Vertex::new([min_x, ltry], &self.color, size));
    //     //     // vertexes.push(Vertex::new([min_x, lbry], &self.color, size));
    //     //     // indices.extend_from_slice(&[current + 6, current + 7]);
    //     //     // min_x += 0.5;
    //     // }
    //     indices.push(0);
    //     (vertexes, indices)
    // }
}
