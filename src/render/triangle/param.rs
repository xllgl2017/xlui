use crate::Offset;
use crate::render::WrcParam;
use crate::style::ClickStyle;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct TriangleDrawParam {
    p0: [f32; 2],             //⬅️ 顶点位置1
    p1: [f32; 2],             //⬅️ 顶点位置2
    p2: [f32; 2],             //⬅️ 顶点位置3
    _pad0: [f32; 2],
    fill_color: [f32; 4],     //⬅️ 填充颜色
    border_thickness: f32,    //⬅️ 边框宽度
    _pad1: [f32; 3],
    border_color: [f32; 4],   //⬅️ 边框颜色
}

pub struct TriangleParam {
    pub p0: [f32; 2],
    pub p1: [f32; 2],
    pub p2: [f32; 2],
    pub style: ClickStyle,
    draw: TriangleDrawParam,
}

impl TriangleParam {
    pub fn new(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2], style: ClickStyle) -> Self {
        let fill_color = style.dyn_fill(false, false).as_gamma_rgba();
        let border = style.dyn_border(false, false);
        let draw = TriangleDrawParam {
            p0,
            p1,
            p2,
            _pad0: [0.0; 2],
            fill_color,
            border_thickness: border.width,
            _pad1: [0.0; 3],
            border_color: border.color.as_gamma_rgba(),
        };
        TriangleParam {
            p0,
            p1,
            p2,
            style,
            draw,
        }
    }

    pub fn offset(&mut self, o: &Offset) {
        self.p0[0] += o.x;
        self.p0[1] += o.y;
        self.p1[0] += o.x;
        self.p1[1] += o.y;
        self.p2[0] += o.x;
        self.p2[1] += o.y;
    }
}

impl WrcParam for TriangleParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered).as_gamma_rgba();
        let border = self.style.dyn_border(mouse_down, hovered);
        self.draw.p0 = self.p0;
        self.draw.p1 = self.p1;
        self.draw.p2 = self.p2;
        self.draw.border_thickness = border.width;
        self.draw.border_color = border.color.as_gamma_rgba();
        self.draw.fill_color = fill_color;
        bytemuck::bytes_of(&self.draw)
    }
}