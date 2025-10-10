use crate::size::Size;

// #[repr(C)]
// #[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable, Debug)]
// pub struct Vertex {
//     pub position: [f32; 2],
//     pub color: [f32; 4],
//     pub screen_size: [f32; 2],
// }
//
// impl Vertex {
//     pub fn new(pos: [f32; 2], color: &Color, screen: &Size) -> Vertex {
//         Vertex {
//             position: pos,
//             color: color.as_gamma_rgba(),
//             screen_size: screen.as_gamma_size(),
//         }
//     }
//
//     pub const ATTRIBS: [wgpu::VertexAttribute; 3] =
//         wgpu::vertex_attr_array![
//             0 => Float32x2, // position
//             1 => Float32x4, // color
//             2 => Float32x2, // screen_size
//         ];
//
//     pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
//         wgpu::VertexBufferLayout {
//             array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
//             step_mode: wgpu::VertexStepMode::Vertex,
//             attributes: &Self::ATTRIBS,
//         }
//     }
// }

#[repr(C)]
#[derive(Copy, Clone, Debug,bytemuck::Pod, bytemuck::Zeroable)]
pub struct ImageVertex {
    pub position: [f32; 2],
    pub tex_coords: [f32; 2],
    pub screen_size: [f32; 2],

}

impl ImageVertex {
    // pub fn new(pos: [f32; 2], color: &Color, screen: &Size) -> ImageVertex {
    //     ImageVertex {
    //         position: pos,
    //         tex_coords: [1.0, 0.0],
    //         screen_size: screen.as_gamma_size(),
    //     }
    // }

    pub fn new_coord(pos: [f32; 2], coord: [f32; 2], screen: Size) -> ImageVertex {
        ImageVertex {
            position: pos,
            tex_coords: coord,
            screen_size: screen.as_gamma_size(),
        }
    }
    pub const ATTRIBS: [wgpu::VertexAttribute; 3] =
        wgpu::vertex_attr_array![
            0 => Float32x2, // position
            1 => Float32x2, // color
            2 => Float32x2, // screen_size
        ];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<ImageVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}
