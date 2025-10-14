#[cfg(feature = "gpu")]
use crate::render::WrcParam;
use crate::size::rect::Rect;
use crate::style::{ClickStyle, FrameStyle, Shadow};
use crate::*;
#[cfg(feature = "gpu")]
use crate::render::Screen;
#[cfg(feature = "gpu")]
use crate::shape::rectangle::RectangleShape;

#[cfg(feature = "gpu")]
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RectDrawParam {
    pos: [f32; 2],           //⬅️ 左上角顶点位置
    size: [f32; 2],          //⬅️ 矩形的宽高
    radius_tl: f32,          //⬅️ 左上圆角
    radius_tr: f32,          //⬅️ 右上圆角
    radius_br: f32,          //⬅️ 右下圆角
    radius_bl: f32,          //⬅️ 左下圆角
    border_width: f32,       //⬅️ 边框宽度
    _pad0: [f32; 3],
    border_color: [f32; 4],  //⬅️ 边框颜色
    shadow_offset: [f32; 2], //⬅️ 阴影位移
    shadow_spread: f32,      //⬅️ 阴影蔓延
    _pad1: [f32; 1],
    shadow_color: [f32; 4],  //⬅️ 阴影颜色
    fill_color: [f32; 4],    //⬅️ 填充颜色
}
#[cfg(feature = "gpu")]
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct RectDrawParam2 {
    center_position: [f32; 2],    //⬅️ 矩形中心(x,y)
    radius: [f32; 2],             //⬅️ 半径(w/2,h/2)
    corner_radii: [f32; 4],       //⬅️ 圆角(左上、右上、右下、左下)
    border_widths: [f32; 4],      //⬅️ 边框(左、右、下、上)
    fill_color: [f32; 4],         //⬅️ 填充颜色(rgba)
    border_color: [f32; 4],       //⬅️ 边框颜色(rgba)
    screen: [f32; 4],             //⬅️ 总大小（宽、高）,缩放比例、填充
    shadow_params: [f32; 4],      //⬅️ 阴影(x,y)、模糊半径、强度
    shadow_color: [f32; 4],       //⬅️ 阴影颜色
}

pub struct RectParam {
    pub(crate) rect: Rect,
    pub(crate) style: ClickStyle,
    pub(crate) shadow: Shadow,
    #[cfg(feature = "gpu")]
    draw: RectDrawParam2,
    #[cfg(feature = "gpu")]
    pub(crate) rect_shape: RectangleShape,
    #[cfg(feature = "gpu")]
    pub(crate) screen:Screen
}

impl RectParam {
    pub fn new() -> Self {
        // let rect = Rect::new();
        // let style = ClickStyle::new();
        // let fill_color = style.dyn_fill(false, false).as_gamma_rgba();
        // let border = style.dyn_border(false, false);
        // let shadow = Shadow::new();
        // let draw = RectDrawParam {
        //     pos: [rect.dx().min, rect.dy().min],
        //     size: [rect.width(), rect.height()],
        //     radius_bl: border.radius.left_bottom as f32,
        //     radius_br: border.radius.right_bottom as f32,
        //     radius_tr: border.radius.right_top as f32,
        //     radius_tl: border.radius.left_top as f32,
        //     border_width: border.width,
        //     _pad0: [0.0; 3],
        //     border_color: border.color.as_gamma_rgba(),
        //     shadow_offset: shadow.offset,
        //     shadow_spread: shadow.spread,
        //     _pad1: [0.0; 1],
        //     shadow_color: shadow.color.as_gamma_rgba(),
        //     fill_color,
        // };
        // let x = (rect.dx().min + rect.dx().max) / 2.0;
        // let y = (rect.dy().min + rect.dy().max) / 2.0;
        // let draw = RectDrawParam2 {
        //     center_position: [x, y],
        //     radius: [rect.width() / 2.0, rect.height() / 2.0],
        //     corner_radii: [
        //         border.radius.left_top as f32,
        //         border.radius.right_top as f32,
        //         border.radius.right_bottom as f32,
        //         border.radius.left_bottom as f32,
        //     ],
        //     border_widths: [border.left_width, border.right_width, border.bottom_width, border.top_width],
        //     fill_color,
        //     border_color: border.color.as_gamma_rgba(),
        //     screen: [800.0, 600.0, 1.0, 0.0],
        //     shadow_params: [shadow.offset[0], shadow.offset[1], shadow.spread, shadow.blur],
        //     shadow_color: shadow.color.as_gamma_rgba(),
        // };
        RectParam {
            rect: Rect::new(),
            style: ClickStyle::new(),
            shadow: Shadow::new(),
            #[cfg(feature = "gpu")]
            draw: RectDrawParam2 {
                center_position: [0.0; 2],
                radius: [0.0; 2],
                corner_radii: [0.0; 4],
                border_widths: [0.0; 4],
                fill_color: [0.0; 4],
                border_color: [0.0; 4],
                screen: [0.0; 4],
                shadow_params: [0.0; 4],
                shadow_color: [0.0; 4],
            },
            #[cfg(feature = "gpu")]
            rect_shape: RectangleShape::new(),
            #[cfg(feature = "gpu")]
            screen: Screen {
                size: [1000.0,800.0],
            },
        }
    }

    pub fn new_frame(rect: Rect, frame: FrameStyle) -> Self {
        let mut style = ClickStyle::new();
        style.fill = FillStyle::same(frame.fill);
        style.border = BorderStyle::same(frame.border);
        let res = Self::new().with_rect(rect).with_style(style);
        res.with_shadow(frame.shadow)
    }

    pub fn with_rect(mut self, rect: Rect) -> Self {
        self.rect = rect;
        self
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.rect.set_size(w, h);
        self
    }

    pub fn with_height(mut self, h: f32) -> Self {
        self.rect.set_height(h);
        self
    }

    pub fn with_style(mut self, style: ClickStyle) -> Self {
        self.set_style(style);
        self
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        self.style = style;
    }

    pub fn set_frame(&mut self, frame: FrameStyle) {
        let mut style = ClickStyle::new();
        style.fill = FillStyle::same(frame.fill);
        style.border = BorderStyle::same(frame.border);
        self.style = style;
        self.shadow = frame.shadow;
    }

    pub fn with_shadow(mut self, shadow: Shadow) -> RectParam {
        self.shadow = shadow;
        self
    }
}
#[cfg(feature = "gpu")]
impl WrcParam for RectParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, size: Size) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered);
        let border = self.style.dyn_border(mouse_down, hovered);
        let x = (self.rect.dx().min + self.rect.dx().max) / 2.0;
        let y = (self.rect.dy().min + self.rect.dy().max) / 2.0;
        self.draw.center_position = [x, y];
        self.draw.radius = [self.rect.width() / 2.0, self.rect.height() / 2.0];
        self.draw.border_widths = [border.left_width, border.right_width, border.bottom_width, border.top_width];
        self.draw.corner_radii = [
            border.radius.left_top as f32,
            border.radius.right_top as f32,
            border.radius.right_bottom as f32,
            border.radius.left_bottom as f32,
        ];
        self.draw.fill_color = fill_color.as_gamma_rgba();
        self.draw.border_color = border.color.as_gamma_rgba();
        self.draw.screen = [size.width, size.height, 1.0, 0.0];
        // self.draw.fill_color = [0.0; 4];
        // self.draw.shadow_params = [0.0; 4];
        // self.draw.corner_radii = [0.0; 4];
        // self.draw.shadow_color = [0.0; 4];
        self.draw.shadow_params = [self.shadow.offset[0], self.shadow.offset[1], self.shadow.spread, self.shadow.blur];
        self.draw.shadow_color = self.shadow.color.as_gamma_rgba();
        self.rect_shape.reset(&self.rect, fill_color, border);
        // let draw = RectDrawParam2 {
        //     center_position: [x, y],
        //     radius:,
        //     corner_radii: [],
        //     border_widths: [border.width, border.width, border.width, border.width],
        //     fill_color,
        //     border_color: border.color.as_gamma_rgba(),
        //     screen: [0.0, 0.0, 1.0, 0.0],
        //     shadow_params: [shadow.offset[0], shadow.offset[1], shadow.spread, 1.0],
        //     shadow_color: shadow.color.as_gamma_rgba(),
        // };
        // self.draw.pos = [self.rect.dx().min, self.rect.dy().min];
        // self.draw.size = [self.rect.width(), self.rect.height()];
        // self.draw.radius_bl = border.radius.left_bottom as f32;
        // self.draw.radius_br = border.radius.right_bottom as f32;
        // self.draw.radius_tr = border.radius.right_top as f32;
        // self.draw.radius_tl = border.radius.left_top as f32;
        // self.draw.border_width = border.width;
        // self.draw.border_color = border.color.as_gamma_rgba();
        // self.draw.fill_color = fill_color;
        // self.draw.shadow_offset = self.shadow.offset;
        // self.draw.shadow_spread = self.shadow.spread;
        // self.draw.shadow_color = self.shadow.color.as_gamma_rgba();
        bytemuck::bytes_of(&self.draw)
    }
}