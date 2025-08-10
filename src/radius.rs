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

    #[deprecated]
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
}
