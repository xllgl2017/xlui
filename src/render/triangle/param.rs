use crate::render::WrcParam;
use crate::size::pos::Pos;
use crate::style::ClickStyle;
use crate::{Offset, Rect, Ui};
#[cfg(feature = "gpu")]
use crate::Size;
#[cfg(all(windows, not(feature = "gpu")))]
use windows::Win32::Graphics::GdiPlus::PointF;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "gpu", derive(bytemuck::Pod, bytemuck::Zeroable))]
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
    pub(crate) rect: Rect,
    p0: Pos,
    p1: Pos,
    p2: Pos,
    pub(crate) style: ClickStyle,
    draw: TriangleDrawParam,
}

impl TriangleParam {
    pub fn new(p0: Pos, p1: Pos, p2: Pos, style: ClickStyle) -> Self {
        let fill_color = style.dyn_fill(false, false).as_gamma_rgba();
        let border = style.dyn_border(false, false);
        let draw = TriangleDrawParam {
            p0: [p0.x, p0.y],
            p1: [p1.x, p1.y],
            p2: [p2.x, p2.y],
            _pad0: [0.0; 2],
            fill_color,
            border_thickness: border.left_width,
            _pad1: [0.0; 3],
            border_color: border.color.as_gamma_rgba(),
        };
        let xs = vec![p0.x, p1.x, p2.x];
        let ys = vec![p0.y, p1.y, p2.y];
        let mut rect = Rect::new();
        rect.set_x_min(xs.clone().into_iter().reduce(f32::min).unwrap());
        rect.set_x_max(xs.into_iter().reduce(f32::max).unwrap());
        rect.set_y_min(ys.clone().into_iter().reduce(f32::min).unwrap());
        rect.set_x_max(ys.into_iter().reduce(f32::max).unwrap());
        TriangleParam {
            p0,
            p1,
            p2,
            style,
            draw,
            rect,
        }
    }

    pub fn set_poses(&mut self, p0: Pos, p1: Pos, p2: Pos) {
        let xs = vec![p0.x, p1.x, p2.x];
        let ys = vec![p0.y, p1.y, p2.y];
        self.rect.set_x_min(xs.clone().into_iter().reduce(f32::min).unwrap());
        self.rect.set_x_max(xs.into_iter().reduce(f32::max).unwrap());
        self.rect.set_y_min(ys.clone().into_iter().reduce(f32::min).unwrap());
        self.rect.set_x_max(ys.into_iter().reduce(f32::max).unwrap());
        self.p0 = p0;
        self.p1 = p1;
        self.p2 = p2;
    }

    pub fn offset_to_rect(&mut self, rect: &Rect) -> Offset {
        let offset = self.rect.offset_to_rect(rect);
        self.p0.offset(offset.x, offset.y);
        self.p1.offset(offset.x, offset.y);
        self.p2.offset(offset.x, offset.y);
        offset
        // self.p0.x += o.x;
        // self.p0.y += o.y;
        // self.p1.x += o.x;
        // self.p1.y += o.y;
        // self.p2.x += o.x;
        // self.p2.y += o.y;
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.style = style;
    }

    #[cfg(all(windows, not(feature = "gpu")))]
    pub fn as_win32_points(&self) -> [PointF; 3] {
        [
            PointF { X: self.p0.x, Y: self.p0.y },
            PointF { X: self.p1.x, Y: self.p1.y },
            PointF { X: self.p2.x, Y: self.p2.y },
        ]
    }
}

impl WrcParam for TriangleParam {
    #[cfg(feature = "gpu")]
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, _: Size) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered).as_gamma_rgba();
        let border = self.style.dyn_border(mouse_down, hovered);
        self.draw.p0 = [self.p0.x, self.p0.y];
        self.draw.p1 = [self.p1.x, self.p1.y];
        self.draw.p2 = [self.p2.x, self.p2.y];
        self.draw.border_thickness = border.left_width;
        self.draw.border_color = border.color.as_gamma_rgba();
        self.draw.fill_color = fill_color;
        bytemuck::bytes_of(&self.draw)
    }
}