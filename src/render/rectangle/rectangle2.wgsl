@vertex
fn vs_main(@builtin(vertex_index) v: u32) -> @builtin(position) vec4<f32> {
    var pos = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -3.0),
        vec2<f32>(3.0, 1.0),
        vec2<f32>(-1.0, 1.0)
    );
    let p = pos[v];
    return vec4<f32>(p, 0.0, 1.0);
}

struct Uniforms {
    center_position:vec2<f32>,  //⬅️ 矩形中心(x,y)
    radius:vec2<f32>,           //⬅️ 半径(w/2,h/2)
    corner_radii: vec4<f32>,    //⬅️ 圆角(左上、右上、右下、左下)
    border_widths: vec4<f32>,   //⬅️ 边框(左、右、下、上)
    fill_color: vec4<f32>,      //⬅️ 填充颜色(rgba)
    border_color: vec4<f32>,    //⬅️ 边框颜色(rgba)
    screen: vec4<f32>,          //⬅️ 屏幕大小（宽、高）,缩放比例、填充
    shadow_params: vec4<f32>,   //⬅️ 阴影(x,y)、模糊半径、强度
    shadow_color: vec4<f32>     //⬅️ 阴影颜色
};
@group(0) @binding(0) var<uniform> u: Uniforms;

fn select_corner_radius(p: vec2<f32>, radii: vec4<f32>) -> f32 {
    // choose corner radius depending on quadrant
    let right = p.x > 0.0;
    let top = p.y > 0.0;
    // order: tl, tr, br, bl
    return select(
        select(radii.x, radii.y, right),
        select(radii.w, radii.z, right),
        top
    );
}

fn sd_rounded_rect_with_radius(p: vec2<f32>, half: vec2<f32>, r: f32) -> f32 {
    let q = abs(p) - (half - vec2<f32>(r, r));
    let outside = max(q, vec2<f32>(0.0, 0.0));
    return length(outside) - r;
}

fn sd_rounded_rect_vary(p: vec2<f32>, half: vec2<f32>, radii: vec4<f32>) -> f32 {
    // pick corner radius for this point and evaluate SDF using that single radius
    let r = select_corner_radius(p, radii);
    return sd_rounded_rect_with_radius(p, half, r);
}

fn sd_shrunk_rounded_rect(p: vec2<f32>, half: vec2<f32>, radii: vec4<f32>, borders: vec4<f32>) -> f32 {
    // approximate inner rect for asymmetric borders
    let left = borders.x;
    let right = borders.y;
    let top = borders.z;
    let bottom = borders.w;
    let shift = vec2<f32>((right - left) * 0.5, (top - bottom) * 0.5);
    let p_shifted = p + shift;
    let half_inner = vec2<f32>(half.x - (left + right) * 0.5, half.y - (top + bottom) * 0.5);
    let half_inner_clamped = max(half_inner, vec2<f32>(0.0, 0.0));
    let r_tl = max(0.0, radii.x - max(left, top));
    let r_tr = max(0.0, radii.y - max(right, top));
    let r_br = max(0.0, radii.z - max(right, bottom));
    let r_bl = max(0.0, radii.w - max(left, bottom));
    let r = select(
        select(r_tl, r_tr, p_shifted.x > 0.0),
        select(r_bl, r_br, p_shifted.x > 0.0),
        p_shifted.y > 0.0
    );
    return sd_rounded_rect_with_radius(p_shifted, half_inner_clamped, r);
}

@fragment
fn fs_main(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let res = vec2<f32>(u.screen.x, u.screen.y);
    let uv = frag_coord.xy / res;
    let center =u.center_position; //vec2<f32>(u.center_half.x, u.center_half.y);
    let half = u.radius;//vec2<f32>(u.center_half.z, u.center_half.w);
    let p = (uv * res) - center;

    // outer and inner SDF
    let sd_out = sd_rounded_rect_vary(p, half, u.corner_radii);
    let sd_in = sd_shrunk_rounded_rect(p, half, u.corner_radii, u.border_widths);

    // antialias width (in pixels)
    var aa = fwidth(distance(uv,center))*0.8;//0.0;
//    let has_border = any(u.border_widths > vec4<f32>(0.5));
//    if (has_border&&half.x<10){
//        aa=1.0;
//    }

    // base fill and border masks
    let fill_mask = 1.0 - smoothstep(0.0, aa, sd_in);
    let outer_mask = 1.0 - smoothstep(0.0, aa, sd_out);
    let border_mask = outer_mask - fill_mask;

    // shadow: evaluate SDF for the rect shifted by shadow offset, then use smoothstep with blur radius
    let shadow_off = vec2<f32>(u.shadow_params.x, u.shadow_params.y);
    let shadow_blur = max(1.0, u.shadow_params.z);
    let shadow_strength = u.shadow_params.w;
    let p_shadow = p - shadow_off; // shift rect relative to pixel
    let sd_shadow = sd_rounded_rect_vary(p_shadow, half, u.corner_radii);
    // shadow mask is soft - outside of rect (sd_shadow > 0) will form the drop; inside still cast
    // We'll make shadow strongest near the rect edge and fade with blur
    let shadow_mask = (1.0 - smoothstep(-shadow_blur, shadow_blur, sd_shadow)) * shadow_strength;
    // combine shadow color with its alpha mask
    let shadow_col = vec4<f32>(u.shadow_color.rgb, u.shadow_color.a * shadow_mask);

    // compose fill and border
    let fill_col = u.fill_color * fill_mask;
    let border_col = vec4<f32>(u.border_color.rgb * u.border_color.a, u.border_color.a) * border_mask;

    // Simple over compositing: shadow below, then fill, then border
    // Start with transparent
    var out_col = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    // place shadow (under everything): out = shadow + out*(1-a)
    out_col = shadow_col + out_col * (1.0 - shadow_col.a);
    // add fill
    out_col = fill_col + out_col * (1.0 - fill_col.a);
    // add border on top
    out_col = border_col + out_col * (1.0 - border_col.a);

    return out_col;
}