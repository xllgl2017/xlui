#[cfg(feature = "gpu")]
use crate::render::WrcRender;
#[cfg(feature = "gpu")]
use crate::Device;
#[cfg(feature = "gpu")]
use wgpu::include_wgsl;
#[cfg(feature = "gpu")]
use crate::vertex::Vertex;

pub mod param;

#[cfg(feature = "gpu")]
pub struct CircleRender {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
}

#[cfg(feature = "gpu")]
impl CircleRender {
    pub fn new(device: &Device) -> Self {
        let shader = device.device.create_shader_module(include_wgsl!("../../bin/1.wgsl"));
        let bind_group_layout_entry = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT | wgpu::ShaderStages::VERTEX,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        let bind_group_layout_desc = wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[bind_group_layout_entry],
        };
        let bind_group_layout = device.device.create_bind_group_layout(&bind_group_layout_desc);
        let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = super::create_pipeline(device, shader, pipeline_layout, &[Vertex::desc()]);
        CircleRender {
            pipeline: render_pipeline,
            bind_group_layout,
        }
    }
}

#[cfg(feature = "gpu")]
impl WrcRender for CircleRender {
    fn pipeline(&self) -> &wgpu::RenderPipeline {
        &self.pipeline
    }

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}


// #[cfg(feature = "gpu")]
// pub struct CircleRender {
//     pipeline: wgpu::RenderPipeline,
//     bind_group_layout: wgpu::BindGroupLayout,
// }
// #[cfg(feature = "gpu")]
// impl CircleRender {
//     pub fn new(device: &Device) -> Self {
//         let shader = device.device.create_shader_module(include_wgsl!("circle.wgsl"));
//         let bind_group_layout_entry = wgpu::BindGroupLayoutEntry {
//             binding: 0,
//             visibility: wgpu::ShaderStages::FRAGMENT,
//             ty: wgpu::BindingType::Buffer {
//                 ty: wgpu::BufferBindingType::Uniform,
//                 has_dynamic_offset: false,
//                 min_binding_size: None,
//             },
//             count: None,
//         };
//
//         let bind_group_layout_desc = wgpu::BindGroupLayoutDescriptor {
//             label: None,
//             entries: &[bind_group_layout_entry],
//         };
//         let bind_group_layout = device.device.create_bind_group_layout(&bind_group_layout_desc);
//         let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: None,
//             bind_group_layouts: &[&bind_group_layout],
//             push_constant_ranges: &[],
//         });
//
//         let render_pipeline = super::create_pipeline(device, shader, pipeline_layout, &[]);
//         CircleRender {
//             pipeline: render_pipeline,
//             bind_group_layout,
//         }
//     }
// }
// #[cfg(feature = "gpu")]
// impl WrcRender for CircleRender {
//     fn pipeline(&self) -> &wgpu::RenderPipeline {
//         &self.pipeline
//     }
//
//     fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
//         &self.bind_group_layout
//     }
// }
