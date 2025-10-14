#[cfg(feature = "gpu")]
use crate::render::WrcParam;
use crate::size::pos::Pos;
use crate::style::ClickStyle;
#[cfg(feature = "gpu")]
use crate::Size;
use crate::{Offset, Rect};
#[cfg(all(windows, not(feature = "gpu")))]
use windows::Win32::Graphics::GdiPlus::PointF;
#[cfg(feature = "gpu")]
use crate::render::Screen;
#[cfg(feature = "gpu")]
use crate::vertex::Vertex;

#[cfg(feature = "gpu")]
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
    pub(crate) rect: Rect,
    pub(crate) p0: Pos,
    pub(crate) p1: Pos,
    pub(crate) p2: Pos,
    pub(crate) style: ClickStyle,
    #[cfg(feature = "gpu")]
    draw: TriangleDrawParam,
    #[cfg(feature = "gpu")]
    pub(crate) screen: Screen,
    #[cfg(feature = "gpu")]
    pub(crate) vertices: Vec<Vertex>,
    #[cfg(feature = "gpu")]
    pub(crate) indices: Vec<u16>,
}

impl TriangleParam {
    pub fn new(p0: Pos, p1: Pos, p2: Pos, style: ClickStyle) -> Self {
        #[cfg(feature = "gpu")]
        let fill_color = style.dyn_fill(false, false).as_gamma_rgba();
        #[cfg(feature = "gpu")]
        let border = style.dyn_border(false, false);
        #[cfg(feature = "gpu")]
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
            #[cfg(feature = "gpu")]
            draw,
            #[cfg(feature = "gpu")]
            screen: Screen { size: [1000.0, 800.0] },
            #[cfg(feature = "gpu")]
            vertices: vec![],
            rect,
            #[cfg(feature = "gpu")]
            indices: vec![],
        }
    }

    pub fn set_poses(&mut self, p0: Pos, p1: Pos, p2: Pos) {
        let xs = vec![p0.x, p1.x, p2.x];
        let ys = vec![p0.y, p1.y, p2.y];
        self.rect.set_x_min(xs.clone().into_iter().reduce(f32::min).unwrap());
        self.rect.set_x_max(xs.into_iter().reduce(f32::max).unwrap());
        self.rect.set_y_min(ys.clone().into_iter().reduce(f32::min).unwrap());
        self.rect.set_y_max(ys.into_iter().reduce(f32::max).unwrap());
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

#[cfg(feature = "gpu")]
impl WrcParam for TriangleParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, _: Size) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered).as_gamma_rgba();
        let border = self.style.dyn_border(mouse_down, hovered);
        self.draw.p0 = [self.p0.x, self.p0.y];
        self.draw.p1 = [self.p1.x, self.p1.y];
        self.draw.p2 = [self.p2.x, self.p2.y];
        self.draw.border_thickness = border.left_width;
        self.draw.border_color = border.color.as_gamma_rgba();
        self.draw.fill_color = fill_color;
        self.vertices = vec![
            Vertex {
                position: [self.p0.x, self.p0.y],
                color: fill_color,
            },
            Vertex {
                position: [self.p1.x, self.p1.y],
                color: fill_color,
            },
            Vertex {
                position: [self.p2.x, self.p2.y],
                color: fill_color,
            }
        ];
        self.indices = vec![0, 1, 2, 0];
        bytemuck::bytes_of(&self.draw)
    }
}