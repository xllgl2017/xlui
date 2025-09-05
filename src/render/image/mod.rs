pub mod texture;

use image::GenericImageView;
use crate::{Device, SAMPLE_COUNT};
use crate::map::Map;
use crate::render::image::texture::ImageTexture;
use crate::vertex::ImageVertex;

pub struct ImageRender {
    pipeline: wgpu::RenderPipeline,
    textures: Map<String, ImageTexture>,
    bind_group_layout: wgpu::BindGroupLayout,

}

impl ImageRender {
    pub fn new(device: &Device) -> ImageRender {
        let entry_texture = wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Texture {
                multisampled: false,
                view_dimension: wgpu::TextureViewDimension::D2,
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
            },
            count: None,
        };
        let entry_sampler = wgpu::BindGroupLayoutEntry {
            binding: 1,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            count: None,
        };
        let desc = wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[entry_texture, entry_sampler],
        };

        let bind_group_layout = device.device.create_bind_group_layout(&desc);
        let pipeline = Self::create_pipeline(device, &bind_group_layout);
        ImageRender {
            pipeline,
            bind_group_layout,
            textures: Map::new(),
        }
    }


    fn create_pipeline(device: &Device, group_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
        let shader = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("image.wgsl").into()),
        });
        let render_pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[group_layout],
            push_constant_ranges: &[],
        });
        let render_pipeline = device.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vvs_main"),
                compilation_options: Default::default(),
                buffers: &[ImageVertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("ffs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format: device.surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: SAMPLE_COUNT,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });
        render_pipeline
    }

    pub fn insert_image(&mut self, device: &Device, uri: String, fp: &str) -> (u32, u32) {
        match self.textures.get(&uri) {
            None => {
                println!("1");
                let img = image::open(fp).unwrap();
                let size = img.dimensions();
                let texture = ImageTexture::new(device, img, &self.bind_group_layout);
                self.textures.insert(uri, texture);
                size
            }
            Some(texture) => texture.size
        }
    }

    pub(crate) fn render(&self, uri: &String, vb: &wgpu::Buffer, ib: &wgpu::Buffer, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        let texture = self.textures.get(uri).unwrap();
        render_pass.set_bind_group(0, &texture.bind_group, &[]);
        render_pass.set_vertex_buffer(0, vb.slice(..));
        render_pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}