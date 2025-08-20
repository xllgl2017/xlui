struct TriangleParams {
    p0: vec2<f32>,          //⬅️ 顶点位置1
    p1: vec2<f32>,          //⬅️ 顶点位置2
    p2: vec2<f32>,          //⬅️ 顶点位置3
    fill_color: vec4<f32>,  //⬅️ 填充颜色
    border_thickness: f32,  //⬅️ 边框宽度
    border_color: vec4<f32>,//⬅️ 边框颜色
};
@group(0) @binding(0)
var<uniform> params: TriangleParams;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 6>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 1.0, -1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>( 1.0,  1.0),
        vec2<f32>(-1.0,  1.0),
        vec2<f32>(-1.0, -1.0),
    );
    var out: VertexOutput;
    out.position = vec4<f32>(positions[index], 0.0, 1.0);
    return out;
}

fn edge(a: vec2<f32>, b: vec2<f32>, p: vec2<f32>) -> f32 {
    return (p.x - a.x) * (b.y - a.y) - (p.y - a.y) * (b.x - a.x);
}

fn sdEquilateralTriangle(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>, c: vec2<f32>) -> f32 {
    // 三角形边向量
    let e0 = b - a;
    let e1 = c - b;
    let e2 = a - c;

    // 点到边的向量
    let v0 = p - a;
    let v1 = p - b;
    let v2 = p - c;

    // 投影到边上的距离
    let pq0 = v0 - e0 * clamp(dot(v0, e0) / dot(e0, e0), 0.0, 1.0);
    let pq1 = v1 - e1 * clamp(dot(v1, e1) / dot(e1, e1), 0.0, 1.0);
    let pq2 = v2 - e2 * clamp(dot(v2, e2) / dot(e2, e2), 0.0, 1.0);

    let d = min(length(pq0), min(length(pq1), length(pq2)));

    // 判断点是否在三角形内部
    let s = sign(e0.x * e2.y - e0.y * e2.x);
    let inside = sign(edge(a, b, p)) == s &&
                 sign(edge(b, c, p)) == s &&
                 sign(edge(c, a, p)) == s;
    var res=d;
    if inside { res=-d; } else { res=d; }


    return res;
}

@fragment
fn fs_main(@builtin(position) pos: vec4<f32>) -> @location(0) vec4<f32> {
    let p = pos.xy;

    let dist = sdEquilateralTriangle(p, params.p0, params.p1, params.p2);
    let aa = fwidth(dist);

    var fill_color = params.fill_color;
    var border_color = params.border_color;

    // 内部填充 alpha
    let inside_alpha = smoothstep(-aa, 0.0, -dist); // dist<0: inside

    var color = fill_color;

    // 边框抗锯齿
    if (params.border_thickness > 0.0) {
        let border_outer = params.border_thickness + aa;
        let border_inner = params.border_thickness - aa;

        // border_alpha 只在距离边界 dist ~ border_thickness 附近生效
        let border_alpha = smoothstep(border_outer, border_inner, abs(dist));

        // 将边框混合到填充色上
        color = mix(color, border_color, border_alpha);
    }

    return vec4(color.rgb, inside_alpha);
}
