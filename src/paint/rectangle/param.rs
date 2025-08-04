use crate::size::rect::Rect;
use crate::style::ClickStyle;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
pub struct DrawParam {
    pos: [f32; 2],           //⬅️ 左上角顶点位置 (NDC)
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


pub struct RectangleParam {
    pub(crate) rect: Rect,
    pub(crate) style: ClickStyle,
}

impl RectangleParam {
    pub fn as_draw_param(&self, hovered: bool, mouse_down: bool) -> DrawParam {
        let fill_color = self.style.dyn_fill(mouse_down, hovered).as_gamma_rgba();
        let border = self.style.dyn_border(mouse_down, hovered);
        DrawParam {
            pos: [self.rect.x.min, self.rect.y.min],
            size: [self.rect.width(), self.rect.height()],
            radius_bl: border.radius.left_bottom as f32,
            radius_br: border.radius.right_bottom as f32,
            radius_tr: border.radius.right_top as f32,
            radius_tl: border.radius.left_top as f32,
            border_width: border.width as f32,
            _pad0: [0.0; 3],
            border_color: border.color.as_gamma_rgba(),
            shadow_offset: [0.0, 0.0],
            shadow_spread: 0.0,
            _pad1: [0.0; 1],
            shadow_color: [0.0, 0.0, 0.0, 0.0],
            fill_color,
        }
    }
}