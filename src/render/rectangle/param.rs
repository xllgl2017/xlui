use crate::render::WrcParam;
use crate::size::rect::Rect;
use crate::style::{ClickStyle, FrameStyle, Shadow};
use crate::{BorderStyle, FillStyle};

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
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


pub struct RectParam {
    pub(crate) rect: Rect,
    pub(crate) style: ClickStyle,
    pub(crate) shadow: Shadow,
    draw: RectDrawParam,
}

impl RectParam {
    pub fn new(rect: Rect, style: ClickStyle) -> Self {
        let fill_color = style.dyn_fill(false, false).as_gamma_rgba();
        let border = style.dyn_border(false, false);
        let shadow = Shadow::new();
        let draw = RectDrawParam {
            pos: [rect.dx().min, rect.dy().min],
            size: [rect.width(), rect.height()],
            radius_bl: border.radius.left_bottom as f32,
            radius_br: border.radius.right_bottom as f32,
            radius_tr: border.radius.right_top as f32,
            radius_tl: border.radius.left_top as f32,
            border_width: border.width,
            _pad0: [0.0; 3],
            border_color: border.color.as_gamma_rgba(),
            shadow_offset: shadow.offset,
            shadow_spread: shadow.spread,
            _pad1: [0.0; 1],
            shadow_color: shadow.color.as_gamma_rgba(),
            fill_color,
        };
        RectParam {
            rect,
            style,
            shadow,
            draw,
        }
    }

    pub fn new_frame(rect: Rect, frame: FrameStyle) -> Self {
        let mut style = ClickStyle::new();
        style.fill = FillStyle::same(frame.fill);
        style.border = BorderStyle::same(frame.border);
        let res = Self::new(rect, style);
        res.with_shadow(frame.shadow)
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
        // self.draw.shadow_offset = self.shadow.offset;
        // self.draw.shadow_spread = self.shadow.spread;
        // self.draw.shadow_color = self.shadow.color.as_gamma_rgba();
        self
    }
}

impl WrcParam for RectParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered).as_gamma_rgba();
        let border = self.style.dyn_border(mouse_down, hovered);
        self.draw.pos = [self.rect.dx().min, self.rect.dy().min];
        self.draw.size = [self.rect.width(), self.rect.height()];
        self.draw.radius_bl = border.radius.left_bottom as f32;
        self.draw.radius_br = border.radius.right_bottom as f32;
        self.draw.radius_tr = border.radius.right_top as f32;
        self.draw.radius_tl = border.radius.left_top as f32;
        self.draw.border_width = border.width;
        self.draw.border_color = border.color.as_gamma_rgba();
        self.draw.fill_color = fill_color;
        self.draw.shadow_offset = self.shadow.offset;
        self.draw.shadow_spread = self.shadow.spread;
        self.draw.shadow_color = self.shadow.color.as_gamma_rgba();
        bytemuck::bytes_of(&self.draw)
    }
}