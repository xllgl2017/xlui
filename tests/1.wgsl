struct Uniforms {
    rect_min: vec2<f32>,   // 矩形左下角
    rect_max: vec2<f32>,   // 矩形右上角
    radius: f32,           // 圆角半径
    _padding: f32,
    color: vec4<f32>,      // 矩形颜色
    resolution: vec2<f32>, // 屏幕分辨率
};
@group(0) @binding(0) var<uniform> u: Uniforms;

struct VertexOutput {
    @builtin(position) pos: vec4<f32>,
    @location(0) frag_pos: vec2<f32>, // 屏幕坐标
};

@vertex
fn vs_main(@location(0) pos: vec2<f32>, @location(1) uv: vec2<f32>) -> VertexOutput {
    var out: VertexOutput;
    let ndc = vec2<f32>(
        pos.x / u.resolution.x * 2.0 - 1.0,   // x: 0..width -> -1..1
        1.0 - pos.y / u.resolution.y * 2.0    // y: 0..height -> 1..-1
    );
    out.pos = vec4<f32>(ndc, 0.0, 1.0);
    out.frag_pos = pos; // frag_pos 保留屏幕坐标
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    // 屏幕坐标系：左下角 origin
    let p =in.frag_pos;

    let min = vec2(0.0,0.0);
    let max = vec2(100.0,100.0);
    let r = 5.0;

    // 使用简单的 signed distance 计算圆角矩形
    let q = max(max(min - p, p - max), vec2<f32>(0.0));
    let dist = length(q);

    let alpha = 1.0 - smoothstep(r - 1.0, r, dist);
    if (alpha <= 0.0) { discard; }
    return vec4(1.0,0.0,0.0,alpha);
}
