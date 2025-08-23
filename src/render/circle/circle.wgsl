struct CircleParams {
    center: vec2<f32>,         // ⬅️ 圆心坐标（像素坐标）
    radius: f32,               // ⬅️ 圆半径
    border_thickness: f32,     // ⬅️ 边框宽度
    fill_color: vec4<f32>,     // ⬅️ 填充颜色
    border_color: vec4<f32>,   // ⬅️ 边框颜色
//    resolution: vec2<f32>,
};

@group(0) @binding(0)
var<uniform> params: CircleParams;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(1.0, -1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(1.0, 1.0),
        vec2<f32>(-1.0, 1.0),
        vec2<f32>(-1.0, -1.0),
    );
    var out: VertexOutput;
    out.position = vec4<f32>(positions[index], 0.0, 1.0);
    return out;
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let uv = pos.xy;
    let dist = distance(uv, params.center);

    let outer = params.radius;
    let inner = params.radius - params.border_thickness;
    let aa = 1.0;

    // 抗锯齿边缘
    let outer_fade = smoothstep(outer + aa, outer - aa, dist); // 外边界 fade out
    let inner_fade = smoothstep(inner - aa, inner + aa, dist); // 内边界 fade in

    // 计算区域
    let in_outer = outer_fade; // [0,1]
    let in_inner = 1.0 - inner_fade; // [0,1]
    var border_alpha=in_outer * (1.0 - in_inner);// 在内外之间 → 边框
    if (params.border_thickness == 0.0){
        border_alpha=0.0;
    }

//    let border_alpha = in_outer * (1.0 - in_inner); // 在内外之间 → 边框
    let fill_alpha = in_inner; // 在内圆之内

    let border = params.border_color * border_alpha;
    let fill = params.fill_color * fill_alpha;

    let final_color = fill + border;

    // 如果都在外面，final_color会为 0
    return final_color;
}

