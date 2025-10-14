use crate::{Border, Pos};
use crate::shape::{get_circle_pos, rotate_point_deg};
use crate::vertex::Vertex;

pub struct RingShape {
    step: i32,
    degree: i32,
    center: Pos,
    vertices: Vec<Vertex>,
    indices: Vec<u16>,
}

impl RingShape {
    pub fn new() -> RingShape {
        RingShape {
            step: 5,
            degree: 90,
            center: Pos::new(),
            vertices: vec![],
            indices: vec![],
        }
    }

    pub fn with_degree(mut self, degree: i32) -> Self {
        self.degree = degree;
        self
    }

    pub fn with_step(mut self, step: i32) -> Self {
        self.step = step;
        self
    }

    pub fn with_center(mut self, center: Pos) -> Self {
        self.set_center(center);
        self
    }

    pub fn set_center(&mut self, center: Pos) {
        self.center = center;
    }

    ///补齐开始段，防止尖边
    fn draw_start(&mut self, start: Pos, rotated: Pos, border: &Border, index: u16) {
        let pm = Pos {
            x: (start.x + rotated.x) / 2.0,
            y: (start.y + rotated.y) / 2.0,
        };
        let pb = Pos {
            x: (3.0 * start.x - rotated.x) / 2.0,
            y: (3.0 * start.y - rotated.y) / 2.0,
        };
        let (kk, _) = get_circle_pos(pm, pb, border.width());
        self.vertices.push(Vertex {
            position: [kk.x, kk.y],
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rotated.x, rotated.y],
            color: border.color.as_gamma_rgba(),
        });
        self.indices.extend_from_slice(&[index - 1, index + 1, index + 2]);
    }

    ///补齐结束段，防止尖边
    fn draw_end(&mut self, border: &Border, index: u16) {
        let last = self.vertices.last().unwrap();
        let last2 = &self.vertices[self.vertices.len() - 3];
        let pm = Pos {
            x: (last.position[0] + last2.position[0]) / 2.0,
            y: (last.position[1] + last2.position[1]) / 2.0,
        };
        let pb = Pos {
            x: (3.0 * last.position[0] - last2.position[0]) / 2.0,
            y: (3.0 * last.position[1] - last2.position[1]) / 2.0,
        };
        let (_, kk) = get_circle_pos(pm, pb, border.width());
        self.vertices.push(Vertex {
            position: [kk.x, kk.y],
            color: border.color.as_gamma_rgba(),
        });
        self.indices.extend_from_slice(&[index - 1, index, index - 2]);
    }

    ///根据已有的角度和圆心绘制圆环
    ///* start-开始的坐标
    ///* index-当前indices值+1
    ///* border-样式
    pub fn draw(&mut self, start: Pos, mut index: u16, border: &Border) -> (Vec<Vertex>, Vec<u16>) {
        let mut lastest_pos = start;
        self.vertices = vec![Vertex {
            position: [lastest_pos.x, lastest_pos.y],
            color: border.color.as_gamma_rgba()
        }];
        self.indices.clear();
        for i in (self.step..=self.degree).step_by(self.step as usize) {
            let rotated = rotate_point_deg(start, self.center, i as f32);
            let (_, cc) = get_circle_pos(lastest_pos, rotated, border.width());
            self.vertices.push(Vertex {
                position: [cc.x, cc.y],
                color: border.color.as_gamma_rgba(),
            });
            lastest_pos = rotated;
            self.indices.extend_from_slice(&[index - 1, index, index + 1]);
            if i != self.step {
                //非起始时，需要把另一般三角区填充
                self.indices.extend_from_slice(&[index, index - 1, index - 2]);
                self.vertices.push(Vertex {
                    position: [rotated.x, rotated.y],
                    color: border.color.as_gamma_rgba(),
                });
            } else if i == self.step {
                //当i为起始时需要找平线段
                self.draw_start(start, rotated, border, index);
                index += 1;
            }
            index += 2;
        }
        //结束时把末端三角区填充
        self.draw_end(border, index);
        (std::mem::take(&mut self.vertices), std::mem::take(&mut self.indices))
    }
}