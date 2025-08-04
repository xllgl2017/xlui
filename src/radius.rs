use std::f32::consts::PI;

pub(crate) enum RadiusBorder {
    LeftBottom,
    RightBottom,
    RightTop,
    LeftTop,

}

#[derive(Clone)]
pub struct Radius {
    pub left_bottom: u8,
    pub right_bottom: u8,
    pub right_top: u8,
    pub left_top: u8,
}

impl Radius {
    pub fn same(radius: u8) -> Radius {
        Radius {
            left_bottom: radius,
            right_bottom: radius,
            right_top: radius,
            left_top: radius,
        }
    }

    pub fn with_left_bottom(mut self, radius: u8) -> Radius {
        self.left_bottom = radius;
        self
    }

    pub fn with_right_bottom(mut self, radius: u8) -> Radius {
        self.right_bottom = radius;
        self
    }

    pub fn with_right_top(mut self, radius: u8) -> Radius {
        self.right_top = radius;
        self
    }

    pub fn with_left_top(mut self, radius: u8) -> Radius {
        self.left_top = radius;
        self
    }

    pub(crate) fn get_p_q_pos(&self, a: f32, b: f32, r: f32, t: &RadiusBorder, segment: i32) -> [[f32; 2]; 2] {
        let p_0 = match t {
            RadiusBorder::LeftBottom => PI - PI / 180.0 * (segment - 1) as f32,
            RadiusBorder::RightBottom => PI / 180.0 * (segment - 1) as f32,
            RadiusBorder::RightTop => -PI / 180.0 * (segment - 1) as f32,
            RadiusBorder::LeftTop => PI + PI / 180.0 * (segment - 1) as f32
        }; //p点与x轴之间的夹角
        let q_0 = match t {
            RadiusBorder::LeftBottom => PI - PI / 180.0 * segment as f32,
            RadiusBorder::RightBottom => PI / 180.0 * segment as f32,
            RadiusBorder::RightTop => -PI / 180.0 * segment as f32,
            RadiusBorder::LeftTop => PI + PI / 180.0 * segment as f32
        }; //q点与x轴之间的夹角
        let (px, py) = (a + r * p_0.cos(), b + r * p_0.sin());
        let (qx, qy) = (a + r * q_0.cos(), b + r * q_0.sin());
        [[px, py], [qx, qy]]
    }
    // fn vertex_fill_radius(&self, r: f32, size: &Size, fill: &Color, [a, b]: [f32; 2], t: RadiusBorder, is: u16) -> (Vec<Vertex>, Vec<u16>) {
    //     let mut vertexes = vec![];
    //     let mut indices = vec![];
    //     vertexes.push(Vertex::new([a, b], fill, size));
    //     let mut current = is + 1;
    //     let segments = 90;
    //     for segment in (1..=segments).step_by(1) {
    //         let [p, q] = self.get_p_q_pos(a, b, r, &t, segment);
    //         vertexes.push(Vertex::new(p, fill, size));
    //         vertexes.push(Vertex::new(q, fill, size));
    //         indices.extend_from_slice(&[0, current, current + 1]); //圆心和两顶点围成三角形
    //         current += 2;
    //     }
    //     (vertexes, indices)
    // }
    //
    //
    // fn vertex_left_bottom(&self, size: &Size, rect: &Rect, fill: &Color, is: u16) -> (Vec<Vertex>, Vec<u16>) {
    //     let r = self.left_bottom as f32; //半径
    //     let circle = [rect.x.min + r, rect.y.max - r]; //圆心;ax+1/2=a/width
    //     self.vertex_fill_radius(r, size, fill, circle, RadiusBorder::LeftBottom, is)
    // }
    // fn vertex_right_bottom(&self, size: &Size, rect: &Rect, fill: &Color, is: u16) -> (Vec<Vertex>, Vec<u16>) {
    //     let r = self.right_bottom as f32; //半径
    //     let circle = [rect.x.max - r, rect.y.max - r]; //圆心;ax+1/2=a/width
    //     self.vertex_fill_radius(r, size, fill, circle, RadiusBorder::RightBottom, is)
    // }
    // fn vertex_right_top(&self, size: &Size, rect: &Rect, fill: &Color, is: u16) -> (Vec<Vertex>, Vec<u16>) {
    //     let r = self.right_top as f32; //半径
    //     let circle = [rect.x.max - r, rect.y.min + r]; //圆心;ax+1/2=a/width
    //     self.vertex_fill_radius(r, size, fill, circle, RadiusBorder::RightTop, is)
    // }
    //
    // fn vertex_left_top(&self, size: &Size, rect: &Rect, fill: &Color, is: u16) -> (Vec<Vertex>, Vec<u16>) {
    //     let r = self.left_top as f32; //半径
    //     let circle = [rect.x.min + r, rect.y.min + r]; //圆心;ax+1/2=a/width
    //     self.vertex_fill_radius(r, size, fill, circle, RadiusBorder::LeftTop, is)
    // }
    //
    // pub fn radius_fill_vertex(&self, size: &Size, rect: &Rect, fill: &Color) -> (Vec<Vertex>, Vec<u16>) {
    //     let mut vertexes = vec![];
    //     let mut indices = vec![];
    //     //left-bottom
    //     match (self.left_bottom, self.right_bottom, self.right_top, self.left_top) {
    //         (0, 0, 0, 0) => {
    //             vertexes.push(Vertex::new(rect.left_bottom(), fill, size));
    //             vertexes.push(Vertex::new(rect.right_bottom(), fill, size));
    //             vertexes.push(Vertex::new(rect.right_top(), fill, size));
    //             vertexes.push(Vertex::new(rect.left_top(), fill, size));
    //             indices.extend_from_slice(&[0, 1, 2, 2, 3, 0]);
    //         }
    //         (1.., 0, 0, 0) => { //左下圆角
    //             let (mut vs, mut is) = self.vertex_left_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rx = rect.x.min + self.left_bottom as f32;
    //             let ry = rect.y.max - self.left_bottom as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([rx, ry], fill, size)); //圆角中心点
    //             vertexes.push(Vertex::new([rx, rect.y.max], fill, size));
    //             vertexes.push(Vertex::new(rect.right_bottom(), fill, size));
    //             vertexes.push(Vertex::new(rect.right_top(), fill, size));
    //             vertexes.push(Vertex::new(rect.left_top(), fill, size));
    //             vertexes.push(Vertex::new([rect.x.min, ry], fill, size));
    //             indices.extend_from_slice(&[current, current + 1, current + 2]);
    //             indices.extend_from_slice(&[current, current + 2, current + 3]);
    //             indices.extend_from_slice(&[current, current + 3, current + 4]);
    //             indices.extend_from_slice(&[current, current + 4, current + 5]);
    //         }
    //         (0, 1.., 0, 0) => { //右下圆角
    //             let (mut vs, mut is) = self.vertex_right_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rx = rect.x.max - self.right_bottom as f32;
    //             let ry = rect.y.max - self.right_bottom as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([rx, ry], fill, size)); //圆角中心点
    //             vertexes.push(Vertex::new([rx, rect.y.max], fill, size));
    //             vertexes.push(Vertex::new(rect.left_bottom(), fill, size));
    //             vertexes.push(Vertex::new(rect.right_top(), fill, size));
    //             vertexes.push(Vertex::new(rect.left_top(), fill, size));
    //             vertexes.push(Vertex::new([rect.x.max, ry], fill, size));
    //             indices.extend_from_slice(&[current, current + 3, current + 5]);
    //             indices.extend_from_slice(&[current, current + 3, current + 4]);
    //             indices.extend_from_slice(&[current, current + 2, current + 4]);
    //             indices.extend_from_slice(&[current, current + 2, current + 1]);
    //         }
    //         (0, 0, 1.., 0) => { //右上圆角
    //             let (mut vs, mut is) = self.vertex_right_top(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rx = rect.x.max - self.right_top as f32;
    //             let ry = rect.y.min + self.right_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([rx, ry], fill, size)); //圆角中心点
    //             vertexes.push(Vertex::new([rx, rect.y.min], fill, size));
    //             vertexes.push(Vertex::new([rect.x.max, ry, ], fill, size));
    //             vertexes.push(Vertex::new(rect.left_bottom(), fill, size));
    //             vertexes.push(Vertex::new(rect.right_bottom(), fill, size));
    //             vertexes.push(Vertex::new(rect.left_top(), fill, size));
    //             indices.extend_from_slice(&[current, current + 1, current + 5]);
    //             indices.extend_from_slice(&[current, current + 3, current + 5]);
    //             indices.extend_from_slice(&[current, current + 3, current + 4]);
    //             indices.extend_from_slice(&[current, current + 2, current + 4]);
    //         }
    //         (0, 0, 0, 1..) => {
    //             let (mut vs, mut is) = self.vertex_left_top(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rx = rect.x.min + self.left_top as f32;
    //             let ry = rect.y.min + self.left_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([rx, ry], fill, size)); //圆角中心点
    //             vertexes.push(Vertex::new([rx, rect.y.min], fill, size)); //1
    //             vertexes.push(Vertex::new([rect.x.min, ry], fill, size)); //2
    //             vertexes.push(Vertex::new(rect.left_bottom(), fill, size)); //3
    //             vertexes.push(Vertex::new(rect.right_bottom(), fill, size)); //4
    //             vertexes.push(Vertex::new(rect.right_top(), fill, size)); //5
    //             indices.extend_from_slice(&[current, current + 1, current + 5]);
    //             indices.extend_from_slice(&[current, current + 4, current + 5]);
    //             indices.extend_from_slice(&[current, current + 3, current + 4]);
    //             indices.extend_from_slice(&[current, current + 2, current + 3]);
    //         }
    //         (1.., 1.., 0, 0) => {
    //             let (mut vs, mut is) = self.vertex_left_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let (mut vs, mut is) = self.vertex_right_bottom(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let lrx = rect.x.min + self.left_bottom as f32;
    //             let lry = rect.y.max - self.left_bottom as f32;
    //             let rrx = rect.x.max - self.right_bottom as f32;
    //             let rry = rect.y.max - self.right_bottom as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([lrx, lry], fill, size)); //左圆角中心点
    //             vertexes.push(Vertex::new([rrx, rry], fill, size)); //右圆角中心点//1
    //             vertexes.push(Vertex::new([rect.x.min, lry], fill, size)); //2
    //             vertexes.push(Vertex::new([lrx, rect.y.max], fill, size)); //3
    //             vertexes.push(Vertex::new([rrx, rect.y.max], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, rry], fill, size)); //5
    //             vertexes.push(Vertex::new(rect.right_top(), fill, size)); //6
    //             vertexes.push(Vertex::new(rect.left_top(), fill, size)); //7
    //             indices.extend_from_slice(&[current, current + 1, current + 3]);
    //             indices.extend_from_slice(&[current + 1, current + 3, current + 4]);
    //             indices.extend_from_slice(&[current + 2, current + 6, current + 7]);
    //             indices.extend_from_slice(&[current + 2, current + 5, current + 6]);
    //         }
    //         (1.., 0, 1.., 0) => {
    //             let (mut vs, mut is) = self.vertex_left_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let (mut vs, mut is) = self.vertex_right_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let lrx = rect.x.min + self.left_bottom as f32;
    //             let lry = rect.y.max - self.left_bottom as f32;
    //             let rrx = rect.x.max - self.right_top as f32;
    //             let rry = rect.y.min + self.right_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([lrx, lry], fill, size)); //左圆角中心点
    //             vertexes.push(Vertex::new([rrx, rry], fill, size)); //右圆角中心点//1
    //             vertexes.push(Vertex::new([rect.x.min, lry], fill, size)); //2
    //             vertexes.push(Vertex::new([lrx, rect.y.max], fill, size)); //3
    //             vertexes.push(Vertex::new([rrx, rect.y.min], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, rry], fill, size)); //5
    //             vertexes.push(Vertex::new(rect.right_bottom(), fill, size)); //6
    //             vertexes.push(Vertex::new(rect.left_top(), fill, size)); //7
    //             indices.extend_from_slice(&[current, current + 1, current + 6]);
    //             indices.extend_from_slice(&[current, current + 6, current + 3]);
    //             indices.extend_from_slice(&[current + 1, current + 6, current + 5]);
    //             indices.extend_from_slice(&[current, current + 1, current + 7]);
    //             indices.extend_from_slice(&[current, current + 2, current + 7]);
    //             indices.extend_from_slice(&[current + 1, current + 4, current + 7]);
    //         }
    //         (1.., 0, 0, 1..) => {
    //             let (mut vs, mut is) = self.vertex_left_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let (mut vs, mut is) = self.vertex_left_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let txr = rect.x.min + self.left_bottom as f32;
    //             let tyr = rect.y.max - self.left_bottom as f32;
    //             let bxr = rect.x.min + self.left_top as f32;
    //             let byr = rect.y.min + self.left_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([txr, tyr], fill, size)); //左圆角中心点
    //             vertexes.push(Vertex::new([bxr, byr], fill, size)); //右圆角中心点//1
    //             vertexes.push(Vertex::new([rect.x.min, tyr], fill, size)); //2
    //             vertexes.push(Vertex::new([txr, rect.y.max], fill, size)); //3
    //             vertexes.push(Vertex::new([bxr, rect.y.min], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.min, byr], fill, size)); //5
    //             vertexes.push(Vertex::new(rect.right_bottom(), fill, size)); //6
    //             vertexes.push(Vertex::new(rect.right_top(), fill, size)); //7
    //             indices.extend_from_slice(&[current, current + 1, current + 2]);
    //             indices.extend_from_slice(&[current + 1, current + 5, current + 2]);
    //             indices.extend_from_slice(&[current + 3, current + 7, current + 6]);
    //             indices.extend_from_slice(&[current + 4, current + 3, current + 7]);
    //         }
    //         (0, 1.., 1.., 0) => {
    //             let (mut vs, mut is) = self.vertex_right_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let (mut vs, mut is) = self.vertex_right_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let txr = rect.x.max - self.right_top as f32;
    //             let tyr = rect.y.min + self.right_top as f32;
    //             let bxr = rect.x.max - self.right_bottom as f32;
    //             let byr = rect.y.max - self.right_bottom as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([txr, tyr], fill, size)); //左圆角中心点
    //             vertexes.push(Vertex::new([bxr, byr], fill, size)); //右圆角中心点//1
    //             vertexes.push(Vertex::new([rect.x.max, tyr], fill, size)); //2
    //             vertexes.push(Vertex::new([txr, rect.y.min], fill, size)); //3
    //             vertexes.push(Vertex::new([bxr, rect.y.max], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, byr], fill, size)); //5
    //             vertexes.push(Vertex::new(rect.left_top(), fill, size)); //6
    //             vertexes.push(Vertex::new(rect.left_bottom(), fill, size)); //7
    //             indices.extend_from_slice(&[current, current + 1, current + 5]);
    //             indices.extend_from_slice(&[current, current + 2, current + 5]);
    //             indices.extend_from_slice(&[current + 3, current + 4, current + 6]);
    //             indices.extend_from_slice(&[current + 4, current + 6, current + 7]);
    //         }
    //         (0, 1.., 0, 1..) => {
    //             let (mut vs, mut is) = self.vertex_right_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let (mut vs, mut is) = self.vertex_left_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let txr = rect.x.min + self.left_top as f32;
    //             let tyr = rect.y.min + self.left_top as f32;
    //             let bxr = rect.x.max - self.right_bottom as f32;
    //             let byr = rect.y.max - self.right_bottom as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([txr, tyr], fill, size)); //左圆角中心点
    //             vertexes.push(Vertex::new([bxr, byr], fill, size)); //右圆角中心点//1
    //             vertexes.push(Vertex::new([rect.x.min, tyr], fill, size)); //2
    //             vertexes.push(Vertex::new([txr, rect.y.min], fill, size)); //3
    //             vertexes.push(Vertex::new([bxr, rect.y.max], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, byr], fill, size)); //5
    //             vertexes.push(Vertex::new(rect.right_top(), fill, size)); //6
    //             vertexes.push(Vertex::new(rect.left_bottom(), fill, size)); //7
    //             indices.extend_from_slice(&[current, current + 1, current + 6]);
    //             indices.extend_from_slice(&[current, current + 1, current + 7]);
    //             indices.extend_from_slice(&[current, current + 2, current + 7]);
    //             indices.extend_from_slice(&[current + 1, current + 4, current + 7]);
    //             indices.extend_from_slice(&[current, current + 3, current + 6]);
    //             indices.extend_from_slice(&[current + 1, current + 5, current + 6]);
    //         }
    //         (0, 0, 1.., 1..) => {
    //             let (mut vs, mut is) = self.vertex_right_top(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let (mut vs, mut is) = self.vertex_left_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let lxr = rect.x.min + self.left_top as f32;
    //             let lyr = rect.y.min + self.left_top as f32;
    //             let rxr = rect.x.max - self.right_top as f32;
    //             let ryr = rect.y.min + self.right_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([lxr, lyr], fill, size)); //左圆角中心点
    //             vertexes.push(Vertex::new([rxr, ryr], fill, size)); //右圆角中心点//1
    //             vertexes.push(Vertex::new([rect.x.min, lyr], fill, size)); //2
    //             vertexes.push(Vertex::new([lxr, rect.y.min], fill, size)); //3
    //             vertexes.push(Vertex::new([rxr, rect.y.min], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, ryr], fill, size)); //5
    //             vertexes.push(Vertex::new(rect.left_bottom(), fill, size)); //6
    //             vertexes.push(Vertex::new(rect.right_bottom(), fill, size)); //7
    //             indices.extend_from_slice(&[current, current + 1, current + 4]);
    //             indices.extend_from_slice(&[current, current + 3, current + 4]);
    //             indices.extend_from_slice(&[current + 2, current + 6, current + 7]);
    //             indices.extend_from_slice(&[current + 2, current + 5, current + 7]);
    //         }
    //         (1.., 1.., 1.., 0) => {
    //             let (mut vs, mut is) = self.vertex_left_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let lbrx = rect.x.min + self.left_bottom as f32;
    //             let lbry = rect.y.max - self.left_bottom as f32;
    //             let (mut vs, mut is) = self.vertex_right_bottom(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rbrx = rect.x.max - self.right_bottom as f32;
    //             let rbry = rect.y.max - self.right_bottom as f32;
    //             let (mut vs, mut is) = self.vertex_right_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rtrx = rect.x.max - self.right_top as f32;
    //             let rtry = rect.y.min + self.right_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([lbrx, lbry], fill, size)); //0
    //             vertexes.push(Vertex::new([lbrx, rect.y.max], fill, size)); //1
    //             vertexes.push(Vertex::new([rect.x.min, lbry], fill, size)); //2
    //             vertexes.push(Vertex::new([rbrx, rbry], fill, size)); //3
    //             vertexes.push(Vertex::new([rbrx, rect.y.max], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, rbry], fill, size)); //5
    //             vertexes.push(Vertex::new([rtrx, rtry], fill, size)); //6
    //             vertexes.push(Vertex::new([rtrx, rect.y.min], fill, size)); //7
    //             vertexes.push(Vertex::new([rect.x.max, rtry], fill, size)); //8
    //             vertexes.push(Vertex::new(rect.left_top(), fill, size)); //9
    //             indices.extend_from_slice(&[current, current + 7, current + 4]);
    //             indices.extend_from_slice(&[current, current + 9, current + 7]);
    //             indices.extend_from_slice(&[current, current + 9, current + 2]);
    //             indices.extend_from_slice(&[current, current + 4, current + 1]);
    //             indices.extend_from_slice(&[current + 3, current + 6, current + 8]);
    //             indices.extend_from_slice(&[current + 3, current + 5, current + 8]);
    //         }
    //         (1.., 1.., 0, 1..) => {
    //             let (mut vs, mut is) = self.vertex_left_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let lbrx = rect.x.min + self.left_bottom as f32;
    //             let lbry = rect.y.max - self.left_bottom as f32;
    //             let (mut vs, mut is) = self.vertex_right_bottom(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rbrx = rect.x.max - self.right_bottom as f32;
    //             let rbry = rect.y.max - self.right_bottom as f32;
    //             let (mut vs, mut is) = self.vertex_left_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let ltrx = rect.x.min + self.left_top as f32;
    //             let ltry = rect.y.min + self.left_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([lbrx, lbry], fill, size)); //0
    //             vertexes.push(Vertex::new([lbrx, rect.y.max], fill, size)); //1
    //             vertexes.push(Vertex::new([rect.x.min, lbry], fill, size)); //2
    //             vertexes.push(Vertex::new([rbrx, rbry], fill, size)); //3
    //             vertexes.push(Vertex::new([rbrx, rect.y.max], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, rbry], fill, size)); //5
    //             vertexes.push(Vertex::new([ltrx, ltry], fill, size)); //6
    //             vertexes.push(Vertex::new([ltrx, rect.y.min], fill, size)); //7
    //             vertexes.push(Vertex::new([rect.x.min, ltry], fill, size)); //8
    //             vertexes.push(Vertex::new(rect.right_top(), fill, size)); //9
    //             indices.extend_from_slice(&[current + 7, current + 4, current + 1]);
    //             indices.extend_from_slice(&[current + 7, current + 4, current + 1]);
    //             indices.extend_from_slice(&[current + 7, current + 4, current + 1]);
    //             indices.extend_from_slice(&[current + 7, current + 9, current + 4]);
    //             indices.extend_from_slice(&[current + 9, current + 5, current + 3]);
    //             indices.extend_from_slice(&[current + 8, current + 6, current]);
    //             indices.extend_from_slice(&[current, current + 2, current + 8]);
    //         }
    //         (1.., 0, 1.., 1..) => {
    //             let (mut vs, mut is) = self.vertex_left_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let lbrx = rect.x.min + self.left_bottom as f32;
    //             let lbry = rect.y.max - self.left_bottom as f32;
    //             let (mut vs, mut is) = self.vertex_right_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rtrx = rect.x.max - self.right_top as f32;
    //             let rtry = rect.y.min + self.right_top as f32;
    //             let (mut vs, mut is) = self.vertex_left_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let ltrx = rect.x.min + self.left_top as f32;
    //             let ltry = rect.y.min + self.left_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([lbrx, lbry], fill, size)); //0
    //             vertexes.push(Vertex::new([lbrx, rect.y.max], fill, size)); //1
    //             vertexes.push(Vertex::new([rect.x.min, lbry], fill, size)); //2
    //             vertexes.push(Vertex::new([rtrx, rtry], fill, size)); //3
    //             vertexes.push(Vertex::new([rtrx, rect.y.min], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, rtry], fill, size)); //5
    //             vertexes.push(Vertex::new([ltrx, ltry], fill, size)); //6
    //             vertexes.push(Vertex::new([ltrx, rect.y.min], fill, size)); //7
    //             vertexes.push(Vertex::new([rect.x.min, ltry], fill, size)); //8
    //             vertexes.push(Vertex::new(rect.right_bottom(), fill, size)); //9
    //             indices.extend_from_slice(&[current + 7, current + 9, current + 1]);
    //             indices.extend_from_slice(&[current + 3, current + 9, current + 7]);
    //             indices.extend_from_slice(&[current + 5, current + 9, current + 3]);
    //             indices.extend_from_slice(&[current + 3, current + 4, current + 7]);
    //             indices.extend_from_slice(&[current + 6, current + 8, current]);
    //             indices.extend_from_slice(&[current, current + 2, current + 8]);
    //         }
    //         (0, 1.., 1.., 1..) => {
    //             let (mut vs, mut is) = self.vertex_right_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rbrx = rect.x.max - self.right_bottom as f32;
    //             let rbry = rect.y.max - self.right_bottom as f32;
    //             let (mut vs, mut is) = self.vertex_right_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rtrx = rect.x.max - self.right_top as f32;
    //             let rtry = rect.y.min + self.right_top as f32;
    //             let (mut vs, mut is) = self.vertex_left_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let ltrx = rect.x.min + self.left_top as f32;
    //             let ltry = rect.y.min + self.left_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([rbrx, rbry], fill, size)); //0
    //             vertexes.push(Vertex::new([rbrx, rect.y.max], fill, size)); //1
    //             vertexes.push(Vertex::new([rect.x.max, rbry], fill, size)); //2
    //             vertexes.push(Vertex::new([rtrx, rtry], fill, size)); //3
    //             vertexes.push(Vertex::new([rtrx, rect.y.min], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, rtry], fill, size)); //5
    //             vertexes.push(Vertex::new([ltrx, ltry], fill, size)); //6
    //             vertexes.push(Vertex::new([ltrx, rect.y.min], fill, size)); //7
    //             vertexes.push(Vertex::new([rect.x.min, ltry], fill, size)); //8
    //             vertexes.push(Vertex::new(rect.left_bottom(), fill, size)); //9
    //             indices.extend_from_slice(&[current + 4, current + 1, current + 9]);
    //             indices.extend_from_slice(&[current + 4, current + 9, current + 6]);
    //             indices.extend_from_slice(&[current + 6, current + 9, current + 8]);
    //             indices.extend_from_slice(&[current + 6, current + 7, current + 4]);
    //             indices.extend_from_slice(&[current + 5, current + 3, current]);
    //             indices.extend_from_slice(&[current, current + 2, current + 5]);
    //         }
    //         (_, _, _, _) => {
    //             let (mut vs, mut is) = self.vertex_left_bottom(size, rect, fill, 0);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let lbrx = rect.x.min + self.left_bottom as f32;
    //             let lbry = rect.y.max - self.left_bottom as f32;
    //             let (mut vs, mut is) = self.vertex_right_bottom(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rbrx = rect.x.max - self.right_bottom as f32;
    //             let rbry = rect.y.max - self.right_bottom as f32;
    //             let (mut vs, mut is) = self.vertex_right_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let rtrx = rect.x.max - self.right_top as f32;
    //             let rtry = rect.y.min + self.right_top as f32;
    //             let (mut vs, mut is) = self.vertex_left_top(size, rect, fill, vertexes.len() as u16);
    //             vertexes.append(&mut vs);
    //             indices.append(&mut is);
    //             let ltrx = rect.x.min + self.left_top as f32;
    //             let ltry = rect.y.min + self.left_top as f32;
    //             let current = vertexes.len() as u16;
    //             vertexes.push(Vertex::new([rbrx, rbry], fill, size)); //0
    //             vertexes.push(Vertex::new([rbrx, rect.y.max], fill, size)); //1
    //             vertexes.push(Vertex::new([rect.x.max, rbry], fill, size)); //2
    //             vertexes.push(Vertex::new([rtrx, rtry], fill, size)); //3
    //             vertexes.push(Vertex::new([rtrx, rect.y.min], fill, size)); //4
    //             vertexes.push(Vertex::new([rect.x.max, rtry], fill, size)); //5
    //             vertexes.push(Vertex::new([ltrx, ltry], fill, size)); //6
    //             vertexes.push(Vertex::new([ltrx, rect.y.min], fill, size)); //7
    //             vertexes.push(Vertex::new([rect.x.min, ltry], fill, size)); //8
    //             vertexes.push(Vertex::new(rect.left_bottom(), fill, size)); //9
    //             vertexes.push(Vertex::new([lbrx, lbry], fill, size)); //10
    //             vertexes.push(Vertex::new([lbrx, rect.y.max], fill, size)); //11
    //             vertexes.push(Vertex::new([rect.x.min, lbry], fill, size)); //12
    //             indices.extend_from_slice(&[current + 7, current + 11, current + 1]);
    //             indices.extend_from_slice(&[current + 4, current + 7, current + 1]);
    //             indices.extend_from_slice(&[current + 10, current + 12, current + 6]);
    //             indices.extend_from_slice(&[current + 6, current + 8, current + 12]);
    //             indices.extend_from_slice(&[current + 5, current + 3, current]);
    //             indices.extend_from_slice(&[current, current + 2, current + 5]);
    //         }
    //     }
    //     (vertexes, indices)
    // }
}
