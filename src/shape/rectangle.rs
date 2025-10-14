use crate::{Border, Color, Pos, Rect};
use crate::shape::draw_fan;
use crate::shape::ring::RingShape;
use crate::vertex::Vertex;

pub struct RectangleShape {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
    ring_shape: RingShape,
}

impl RectangleShape {
    pub fn new() -> RectangleShape {
        RectangleShape {
            vertices: vec![],
            indices: vec![
                0, 1, 2,
                2, 0, 3,
                4, 5, 6,
                6, 4, 7
            ],
            ring_shape: RingShape::new().with_degree(90).with_step(5),
        }
    }

    fn draw_base_rectangle(&mut self, rect: &Rect, fill: &Color, border_width: f32, as_s: f32) {
        //垂直矩形
        self.vertices.push(Vertex {
            position: [rect.dx().min + as_s, rect.dy().min + border_width], //lt
            color: fill.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max - as_s, rect.dy().min + border_width], //rt
            color: fill.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max - as_s, rect.dy().max - border_width], //rb
            color: fill.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().min + as_s, rect.dy().max - border_width], //lb,
            color: fill.as_gamma_rgba(),
        });
        //水平矩形
        self.vertices.push(Vertex {
            position: [rect.dx().min + border_width, rect.dy().min + as_s], //lt
            color: fill.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max - border_width, rect.dy().min + as_s], //rt
            color: fill.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max - border_width, rect.dy().max - as_s], //rb,
            color: fill.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().min + border_width, rect.dy().max - as_s], //lb,
            color: fill.as_gamma_rgba(),
        });
    }

    //绘制直线边框
    fn draw_border_line(&mut self, rect: &Rect, as_s: f32, border: &Border) {
        if border.width() <= 0.0 { return; }
        self.draw_top_border(rect, as_s, border);
        self.draw_right_border(rect, as_s, border);
        self.draw_bottom_border(rect, as_s, border);
        self.draw_left_border(rect, as_s, border);
    }

    //上边框
    fn draw_top_border(&mut self, rect: &Rect, as_s: f32, border: &Border) {
        let width = border.width();
        self.vertices.push(Vertex {
            position: [rect.dx().min + as_s, rect.dy().min], //lt
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max - as_s, rect.dy().min], //rt
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max - as_s, rect.dy().min + width], //rb
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().min + as_s, rect.dy().min + width], //lb
            color: border.color.as_gamma_rgba(),
        });
        self.indices.extend_from_slice(&[8, 9, 10, 8, 10, 11]);
    }

    //右边框
    fn draw_right_border(&mut self, rect: &Rect, as_s: f32, border: &Border) {
        let width = border.width();
        self.vertices.push(Vertex {
            position: [rect.dx().max - width, rect.dy().min + as_s], //lt
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max, rect.dy().min + as_s], //rt
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max, rect.dy().max - as_s], //rb
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max - width, rect.dy().max - as_s], //lb
            color: border.color.as_gamma_rgba(),
        });
        self.indices.extend_from_slice(&[12, 13, 14, 12, 14, 15]);
    }

    //下边框
    fn draw_bottom_border(&mut self, rect: &Rect, as_s: f32, border: &Border) {
        let width = border.width();
        self.vertices.push(Vertex {
            position: [rect.dx().min + as_s, rect.dy().max - width], //lt
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max - as_s, rect.dy().max - width], //rt
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().max - as_s, rect.dy().max], //rb
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().min + as_s, rect.dy().max], //lb
            color: border.color.as_gamma_rgba(),
        });
        self.indices.extend_from_slice(&[16, 17, 18, 16, 18, 19]);
    }

    //左边框
    fn draw_left_border(&mut self, rect: &Rect, as_s: f32, border: &Border) {
        let width = border.width();
        self.vertices.push(Vertex {
            position: [rect.dx().min, rect.dy().min + as_s], //lt
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().min + width, rect.dy().min + as_s], //rt
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().min + width, rect.dy().max - as_s], //rb
            color: border.color.as_gamma_rgba(),
        });
        self.vertices.push(Vertex {
            position: [rect.dx().min, rect.dy().max - as_s], //lb
            color: border.color.as_gamma_rgba(),
        });
        self.indices.extend_from_slice(&[20, 21, 22, 20, 22, 23])
    }

    //左上角圆角+边框
    fn draw_lt_arc(&mut self, rect: &Rect, as_s: f32, fill: &Color, border: &Border) {
        let lt_center = Pos {
            x: rect.dx().min + as_s,
            y: rect.dy().min + as_s,
        };
        let mut lt_start = Pos {
            x: rect.dx().min + border.width(),
            y: rect.dy().min + as_s,
        };
        let (mut rp, mut ri) = draw_fan(lt_center, lt_start, self.vertices.len() as u16 + 1, fill, 90);
        self.vertices.append(&mut rp);
        self.indices.append(&mut ri);
        //左上角边框
        if border.width() > 0.0 {
            lt_start.x = rect.dx().min;
            self.ring_shape.set_center(lt_center);
            let (mut rp, mut ri) = self.ring_shape.draw(lt_start, self.vertices.len() as u16 + 1, border); //draw_ring(lt_center, lt_start, self.vertices.len() as u16 + 1, border, 90);
            self.vertices.append(&mut rp);
            self.indices.append(&mut ri);
        }
    }

    //右上角圆角+边框
    fn draw_rt_arc(&mut self, rect: &Rect, as_s: f32, fill: &Color, border: &Border) {
        let rt_center = Pos {
            x: rect.dx().max - as_s,
            y: rect.dy().min + as_s,
        };
        let mut rt_start = Pos {
            x: rect.dx().max - as_s,
            y: rect.dy().min + border.width(),
        };
        let (mut rp, mut ri) = draw_fan(rt_center, rt_start, self.vertices.len() as u16 + 1, fill, 90);
        self.vertices.append(&mut rp);
        self.indices.append(&mut ri);
        if border.width() > 0.0 {
            rt_start.y = rect.dy().min;
            self.ring_shape.set_center(rt_center);
            let (mut rp, mut ri) = self.ring_shape.draw(rt_start, self.vertices.len() as u16 + 1, border); //draw_ring(rt_center, rt_start, self.vertices.len() as u16 + 1, border, 90);
            self.vertices.append(&mut rp);
            self.indices.append(&mut ri);
        }
    }

    //右下角圆角+边框
    fn draw_rb_arc(&mut self, rect: &Rect, as_s: f32, fill: &Color, border: &Border) {
        let rb_center = Pos {
            x: rect.dx().max - as_s,
            y: rect.dy().max - as_s,
        };
        let mut rb_start = Pos {
            x: rect.dx().max - border.width(),
            y: rect.dy().max - as_s,
        };
        let (mut rp, mut ri) = draw_fan(rb_center, rb_start, self.vertices.len() as u16 + 1, fill, 90);
        self.vertices.append(&mut rp);
        self.indices.append(&mut ri);
        if border.width() > 0.0 {
            rb_start.x = rect.dx().max;
            self.ring_shape.set_center(rb_center);
            let (mut rp, mut ri) = self.ring_shape.draw(rb_start, self.vertices.len() as u16 + 1, border); //draw_ring(rb_center, rb_start, self.vertices.len() as u16 + 1, border, 90);
            self.vertices.append(&mut rp);
            self.indices.append(&mut ri);
        }
    }

    //左下角圆角+边框
    fn draw_lb_arc(&mut self, rect: &Rect, as_s: f32, fill: &Color, border: &Border) {
        let lb_center = Pos {
            x: rect.dx().min + as_s,
            y: rect.dy().max - as_s,
        };
        let mut lb_start = Pos {
            x: rect.dx().min + as_s,
            y: rect.dy().max - border.width(),
        };
        let (mut rp, mut ri) = draw_fan(lb_center, lb_start, self.vertices.len() as u16 + 1, fill, 90);
        self.vertices.append(&mut rp);
        self.indices.append(&mut ri);
        if border.width() > 0.0 {
            lb_start.y = rect.dy().max;
            self.ring_shape.set_center(lb_center);
            let (mut rp, mut ri) = self.ring_shape.draw(lb_start, self.vertices.len() as u16 + 1, border); //draw_ring(lb_center, lb_start, self.vertices.len() as u16 + 1, border, 90);
            self.vertices.append(&mut rp);
            self.indices.append(&mut ri);
        }
    }

    pub fn reset(&mut self, rect: &Rect, fill: &Color, border: &Border) {
        self.vertices.clear();
        self.indices = vec![
            0, 1, 2,
            2, 0, 3,
            4, 5, 6,
            6, 4, 7
        ];
        let as_s = border.width() + border.radius.left_bottom as f32;
        self.draw_base_rectangle(rect, fill, border.width(), as_s);
        self.draw_border_line(rect, as_s, border);
        self.draw_lt_arc(rect, as_s, fill, border);
        self.draw_rt_arc(rect, as_s, fill, border);
        self.draw_rb_arc(rect, as_s, fill, border);
        self.draw_lb_arc(rect, as_s, fill, border);
    }
}