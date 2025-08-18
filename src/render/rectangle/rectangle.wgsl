struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
    );

    var out: VertexOutput;
    out.position = vec4<f32>(positions[index], 0.0, 1.0);
    return out;
}

struct DrawParam {
    pos: vec2<f32>,          //⬅️ 左上角顶点位置 (NDC)
    size: vec2<f32>,         //⬅️ 矩形的宽高
    radius_tl: f32,          //⬅️ 左上圆角
    radius_tr: f32,          //⬅️ 右上圆角
    radius_br: f32,          //⬅️ 右下圆角
    radius_bl: f32,          //⬅️ 左下圆角
    border_width: f32,       //⬅️ 边框宽度
    border_color: vec4<f32>, //⬅️ 边框颜色
    shadow_offset: vec2<f32>,//⬅️ 阴影位移
    shadow_spread: f32,      //⬅️ 阴影蔓延
    shadow_color: vec4<f32>, //⬅️ 阴影颜色
    fill_color: vec4<f32>    //⬅️ 填充颜色

};


@group(0) @binding(0)
var<uniform> param: DrawParam;

//@group(0) @binding(1)
//var<uniform> style: Style;

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let p = pos.xy;

    let border = param.border_width;
    let r_tl = param.radius_tl;
    let r_tr = param.radius_tr;
    var r_br = 0.0;
    if param.radius_br <= 2.0 {
        r_br = param.radius_br + 1.0;
    } else {
        r_br = param.radius_br;
    }
    var r_bl = 0.0;
    if param.radius_bl <= 2.0 {
        r_bl = param.radius_bl + 1.0;
    } else {
        r_bl = param.radius_bl;
    }


    let x0 = param.pos.x;
    let y0 = param.pos.y;
    let x1 = x0 + param.size.x;
    let y1 = y0 + param.size.y;

    let fill_color   = param.fill_color;//vec4(1.0, 1.0, 0.0, 1.0);
    let border_color = param.border_color;//vec4(0.2, 0.4, 0.9, 1.0);
    let shadow_color = param.shadow_color;//vec4(1.0, 0.0, 0.0, 0.3);
    let shadow_offset = param.shadow_offset;//vec2(4.0, 4.0);
    let shadow_spread = param.shadow_spread;//10.0;

    let inner_x0 = x0 + border;
    let inner_y0 = y0 + border;
    let inner_x1 = x1 - border;
    let inner_y1 = y1 - border;

    // ==== 主区域裁剪判断 ====
    var corner_dist = -1.0;

    if p.x < x0 + r_tl && p.y < y0 + r_tl {
        corner_dist = length(vec2(x0 + r_tl, y0 + r_tl) - p) - r_tl;
    } else if p.x > x1 - r_tr && p.y < y0 + r_tr {
        corner_dist = length(vec2(x1 - r_tr, y0 + r_tr) - p) - r_tr;
    } else if p.x > x1 - r_br && p.y > y1 - r_br {
        corner_dist = length(vec2(x1 - r_br, y1 - r_br) - p) - r_br;
    } else if p.x < x0 + r_bl && p.y > y1 - r_bl {
        corner_dist = length(vec2(x0 + r_bl, y1 - r_bl) - p) - r_bl;
    }else {
        let dx = max(x0 - p.x, p.x - x1);
        let dy = max(y0 - p.y, p.y - y1);
        corner_dist = max(dx, dy);
    }

//    let in_outer = p.x >= x0 && p.x <= x1 && p.y >= y0 && p.y <= y1 && corner_dist <= 0.0;
//    let in_inner = p.x >= inner_x0 && p.x <= inner_x1 && p.y >= inner_y0 && p.y <= inner_y1;
    var border_region = false;
    var fill_region = false;
        var corner_radius = 0.0;
        var corner_center = vec2(0.0);
    if p.x >= x0 && p.x <= x1 && p.y >= y0 && p.y <= y1 {


        if p.x < x0 + r_tl && p.y < y0 + r_tl {
            corner_center = vec2(x0 + r_tl, y0 + r_tl);
            corner_radius = r_tl;
        } else if p.x > x1 - r_tr && p.y < y0 + r_tr {
            corner_center = vec2(x1 - r_tr, y0 + r_tr);
            corner_radius = r_tr;
        } else if p.x > x1 - r_br && p.y > y1 - r_br {
            corner_center = vec2(x1 - r_br, y1 - r_br);
            corner_radius = r_br;
        } else if p.x < x0 + r_bl && p.y > y1 - r_bl {
            corner_center = vec2(x0 + r_bl, y1 - r_bl);
            corner_radius = r_bl;
        }

        if corner_radius > 0.0 {
            let d = length(p - corner_center);
            fill_region = d <= (corner_radius - border);
            border_region = d <= corner_radius && !fill_region;

        } else {
            // 非圆角区域
            fill_region = p.x >= inner_x0 && p.x <= inner_x1 && p.y >= inner_y0 && p.y <= inner_y1;
            border_region = !fill_region;
        }
    }

    // ==== 渲染颜色 ====
    var color = vec4(0.0);
    var aa_width=1.0;
   if fill_region {
        color = fill_color;
        let d = length(p - corner_center);
        let dist_to_edge = d - corner_radius;
        let dist_to_inner = d - (corner_radius - border);

        let fill_alpha = 1.0 - smoothstep(0.0, aa_width, dist_to_inner);
        let border_alpha = smoothstep(0.0, aa_width, dist_to_inner) * (1.0 - smoothstep(0.0, aa_width, dist_to_edge));

        fill_region = fill_alpha > 0.01;
        border_region = border_alpha > 0.01;

        color = mix(color, fill_color, fill_alpha);
        color = mix(color, border_color, border_alpha);
   } else if border_region {
        color = border_color;
        let d_left   = p.x - inner_x0;
        let d_right  = inner_x1 - p.x;
        let d_top    = p.y - inner_y0;
        let d_bottom = inner_y1 - p.y;

        let fill_alpha = min(min(smoothstep(0.0, aa_width, d_left),
                                 smoothstep(0.0, aa_width, d_right)),
                            min(smoothstep(0.0, aa_width, d_top),
                                 smoothstep(0.0, aa_width, d_bottom)));

        fill_region = fill_alpha > 0.01;
        color = mix(color, border_color, fill_alpha);
   } else {
        // ==== 阴影 ====
        let sp = p - shadow_offset;
        var shadow_dist = 0.0;

        if sp.x < x0 + r_tl && sp.y < y0 + r_tl {
            shadow_dist = length(vec2(x0 + r_tl, y0 + r_tl) - sp) - r_tl;
        } else if sp.x > x1 - r_tr && sp.y < y0 + r_tr {
            shadow_dist = length(vec2(x1 - r_tr, y0 + r_tr) - sp) - r_tr;
        } else if sp.x > x1 - r_br && sp.y > y1 - r_br {
            shadow_dist = length(vec2(x1 - r_br, y1 - r_br) - sp) - r_br;
        } else if sp.x < x0 + r_bl && sp.y > y1 - r_bl {
            shadow_dist = length(vec2(x0 + r_bl, y1 - r_bl) - sp) - r_bl;
        } else {
            let dx = max(x0 - sp.x, sp.x - x1);
            let dy = max(y0 - sp.y, sp.y - y1);
            shadow_dist = max(dx, dy);
        }

        if shadow_dist < shadow_spread {
            let alpha = 1.0 - clamp(shadow_dist / shadow_spread, 0.0, 1.0);
            color = vec4(shadow_color.rgb, shadow_color.a * alpha);
//            let alpha = 1.0 - smoothstep(shadow_spread, shadow_spread + aa_width, shadow_dist);
//            color = vec4(shadow_color.rgb, shadow_color.a * alpha);
        }
    }

    if color.a < 0.01 {
        discard;
    }

    return color;
}





