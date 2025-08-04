struct VertexInput {
    @location(0) position: vec2<f32>,    // ⬅️顶点位置 (NDC)
    @location(1) color: vec4<f32>,       // ⬅️填充颜色
    @location(2) screen_size:vec2<f32>,  // ⬅️软件窗口大小
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,// ⬅️顶点位置 (NDC)
    @location(0) color: vec4<f32>,         // ⬅️填充颜色
    @location(1) screen_size:vec2<f32>,    // ⬅️软件窗口大小
};

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;
    let pos=vec2<f32>(
        input.position.x*2.0/input.screen_size.x - 1.0,
        1.0 - input.position.y*2.0/input.screen_size.y,
    );
    output.position =vec4(pos,0.0,1.0);
    output.color = input.color;
    output.screen_size = input.screen_size;
    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    return input.color;
}
