#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TriangleParam {
    pub p0: [f32; 2],
    pub p1: [f32; 2],

    pub p2: [f32; 2],
    pub _pad0: [f32; 2],  // 8 (对齐 vec4)
    pub color: [f32; 4],

    pub _pad1: [f32; 2],  // 8 (对齐 vec4)
    pub _pad2: [f32; 2],  // 8 (对齐 vec4)
}