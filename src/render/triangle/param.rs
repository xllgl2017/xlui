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

pub struct TriangleParam {
    pub(crate) rect: Rect,
    pub(crate) p0: Pos,
    pub(crate) p1: Pos,
    pub(crate) p2: Pos,
    pub(crate) style: ClickStyle,
    #[cfg(feature = "gpu")]
    pub(crate) screen: Screen,
    #[cfg(feature = "gpu")]
    pub(crate) vertices: Vec<Vertex>,
    #[cfg(feature = "gpu")]
    pub(crate) indices: Vec<u16>,
}

impl TriangleParam {
    pub fn new(p0: Pos, p1: Pos, p2: Pos, style: ClickStyle) -> Self {
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
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, size: Size) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered).as_gamma_rgba();
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
        self.screen.size=[size.width,size.height];
        bytemuck::bytes_of(&self.screen)
    }
}