struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

struct Size{
    width: f32,
    height: f32,
}

@group(0) @binding(0)
var<uniform> size: Size;

@vertex
fn vs_main(@location(0) position: vec2<f32>, @location(1) color: vec4<f32>)-> VertexOutput {
    var out: VertexOutput;
    let x=(position.x/size.width)*2.0 - 1.0;
    let y=-(position.y/size.height)*2.0 + 1.0;
    out.position = vec4<f32>(x,y, 0.0, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs_main(@location(0) color: vec4<f32>)-> @location(0) vec4<f32> {
    return color;
}