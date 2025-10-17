#[cfg(feature = "gpu")]
use crate::shape::circle::CircleShape;
#[cfg(feature = "gpu")]
use crate::shape::rectangle::RectangleShape;
use crate::shape::triangle::TriangleShape;
#[cfg(feature = "gpu")]
use crate::vertex::Vertex;
use crate::*;
#[cfg(feature = "gpu")]
mod rectangle;
#[cfg(feature = "gpu")]
mod ring;
#[cfg(feature = "gpu")]
mod circle;
mod triangle;

pub enum Shape {
    #[cfg(not(feature = "gpu"))]
    Rectangle,
    #[cfg(not(feature = "gpu"))]
    Circle,
    #[cfg(feature = "gpu")]
    Rectangle(RectangleShape),
    #[cfg(feature = "gpu")]
    Circle(CircleShape),
    Triangle(TriangleShape),
}

impl Shape {
    pub fn triangle() -> Shape {
        Shape::Triangle(TriangleShape {
            #[cfg(feature = "gpu")]
            vertices: vec![],
            #[cfg(feature = "gpu")]
            indices: vec![],
            p0: Pos::new(),
            p1: Pos::new(),
            p2: Pos::new(),
        })
    }

    pub fn rectangle() -> Shape {
        #[cfg(feature = "gpu")]
        return Shape::Rectangle(RectangleShape::new());
        #[cfg(not(feature = "gpu"))]
        return Shape::Rectangle;
    }

    pub fn circle() -> Shape {
        #[cfg(feature = "gpu")]
        return Shape::Circle(CircleShape::new());
        #[cfg(not(feature = "gpu"))]
        return Shape::Circle;
    }
}

#[cfg(feature = "gpu")]
impl Shape {
    pub fn vertices(&self) -> &Vec<Vertex> {
        match self {
            Shape::Rectangle(rect) => &rect.vertices,
            Shape::Circle(circle) => &circle.vertices,
            Shape::Triangle(triangle) => &triangle.vertices,
        }
    }

    pub fn indices(&self) -> &Vec<u16> {
        match self {
            Shape::Rectangle(rect) => &rect.indices,
            Shape::Circle(circle) => &circle.indices,
            Shape::Triangle(triangle) => &triangle.indices,
        }
    }

    pub fn vertices_size(&self) -> u64 {
        match self {
            Shape::Rectangle(_) => 8192,
            Shape::Circle(_) => 8192,
            Shape::Triangle(_) => 72,
        }
    }

    pub fn indices_size(&self) -> u64 {
        match self {
            Shape::Rectangle(_) => 2048,
            Shape::Circle(_) => 2048,
            Shape::Triangle(_) => 12,
        }
    }

    pub fn update(&mut self, rect: &Rect, style: &WidgetStyle) {
        match self {
            Shape::Rectangle(rectangle) => rectangle.update(rect, style),
            Shape::Circle(circle) => circle.update(rect, style),
            Shape::Triangle(triangle) => triangle.update(rect, style),
        }
    }
}

#[cfg(feature = "gpu")]
pub fn draw_fan(center: Pos, start_pos: Pos, mut start_index: u16, fill: &Color, degree: i32) -> (Vec<Vertex>, Vec<u16>) {
    let mut points = vec![Vertex {
        position: [center.x, center.y],
        color: fill.as_gamma_rgba()
    }];
    let center_index = start_index - 1;
    let mut indices: Vec<u16> = vec![];
    for a in (0..=degree).step_by(10) {
        let rotated = rotate_point_deg(start_pos, center, a as f32);

        let v = Vertex {
            position: [rotated.x, rotated.y],
            color: fill.as_gamma_rgba(),
        };
        points.push(v);

        if a > 0 {
            indices.extend_from_slice(&[center_index, start_index - 1, start_index]);
            // indices.push(center_index); // 中心
            // indices.push(start_index - 1);
            // indices.push(start_index);
        }
        start_index += 1;
    }
    (points, indices)
}

#[cfg(feature = "gpu")]
#[allow(dead_code)]
fn draw_line() -> (Vec<Vertex>, Vec<u16>) {
    let poses = vec![
        Pos {
            x: 1.0,
            y: 1.0,
        },
        Pos {
            x: 5.0,
            y: 1.0
        },
        Pos {
            x: 15.0,
            y: 5.0
        },
        Pos {
            x: 20.0,
            y: 8.0
        },
        Pos {
            x: 26.0,
            y: 15.0
        }
    ];
    let mut points = vec![Vertex {
        position: [poses[0].x, poses[0].y],
        color: [1.0, 0.0, 0.0, 1.0]
    }];
    let mut indices = vec![];
    let mut start_indices = 1;
    for i in 0..poses.len() - 1 {
        let p0 = poses[i];
        let p1 = poses[i + 1];
        let (_, cc) = get_circle_pos(p0, p1, 2.0);
        points.push(Vertex {
            position: [cc.x, cc.y],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        points.push(Vertex {
            position: [p1.x, p1.y],
            color: [1.0, 0.0, 0.0, 1.0],
        });
        indices.push(start_indices - 1);
        indices.push(start_indices);
        indices.push(start_indices + 1);
        if i != 0 {
            indices.push(start_indices);
            indices.push(start_indices - 1);
            indices.push(start_indices - 2);
        }
        start_indices += 2;
    }
    (points, indices)
}

#[cfg(feature = "gpu")]
///计算圆心坐标(a,b)
///* p1,p2为屏幕坐标，均>0.0
fn get_circle_pos(p1: Pos, p2: Pos, w: f32) -> (Pos, Pos) {
    let dx = p2.x - p1.x;
    let dy = p2.y - p1.y;
    let l = (dx * dx + dy * dy).sqrt();

    // 单位方向向量（垂直弦）
    let ux = dy / l;
    let uy = -dx / l;

    // 弦中点
    let mx = (p1.x + p2.x) / 2.0;
    let my = (p1.y + p2.y) / 2.0;

    // 圆心相对偏移（沿垂线方向）
    let cx1 = mx + ux * w;
    let cy1 = my + uy * w;
    let cx2 = mx - ux * w;
    let cy2 = my - uy * w;

    (Pos { x: cx1, y: cy1 }, Pos { x: cx2, y: cy2 })
}

#[cfg(feature = "gpu")]
/// 将点 A 绕圆心 C 旋转 delta_rad（弧度），返回点 B
fn rotate_point_around_center(a: Pos, c: Pos, delta_rad: f32) -> Pos {
    let dx = a.x - c.x;
    let dy = a.y - c.y;

    let cos_t = delta_rad.cos();
    let sin_t = delta_rad.sin();

    let rx = cos_t * dx - sin_t * dy;
    let ry = sin_t * dx + cos_t * dy;

    Pos { x: c.x + rx, y: c.y + ry }
}

#[cfg(feature = "gpu")]
fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

#[cfg(feature = "gpu")]
fn rotate_point_deg(a: Pos, c: Pos, delta_deg: f32) -> Pos {
    rotate_point_around_center(a, c, deg_to_rad(delta_deg))
}