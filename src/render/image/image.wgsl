struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) screen_size:vec2<f32>,  // ⬅️软件窗口大小
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) screen_size:vec2<f32>,    // ⬅️软件窗口大小
};

@group(0) @binding(0) var texture: texture_2d<f32>;
@group(0) @binding(1) var tex_sampler: sampler;

@vertex
fn vvs_main(input: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let pos=vec2<f32>(
        input.position.x*2.0/input.screen_size.x - 1.0,
        1.0 - input.position.y*2.0/input.screen_size.y,
    );
    out.position = vec4<f32>(pos,0.0, 1.0);
    out.tex_coords = input.tex_coords;
    return out;
}

@fragment
fn ffs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, tex_sampler, in.tex_coords);
//    return vec4(1.0,1.0,0.0,1.0);
}