use crate::frame::context::Context;
use crate::paint::image::PaintImage;
use crate::paint::PaintTask;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::ui::{Ui, UiM};
use crate::vertex::ImageVertex;
use crate::widgets::Widget;
use crate::{Device, Pos, SAMPLE_COUNT};
use image::GenericImageView;
use wgpu::util::DeviceExt;
use crate::map::Map;


// pub struct ImageReader {
//     bind_group_layout: wgpu::BindGroupLayout,
//     bind_group: wgpu::BindGroup,
//     index_buffer: wgpu::Buffer,
//     render_pipeline: wgpu::RenderPipeline,
//     vertex_buffer: wgpu::Buffer,
//     vertexes: Vec<ImageVertex>,
//
// }
//
// impl ImageReader {
//     pub(crate) fn new(ui: &mut Ui, img: image::DynamicImage, rect: &Rect) -> ImageReader {
//         let bind_group_layout = ui.device.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
//             entries: &[
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 0,
//                     visibility: wgpu::ShaderStages::FRAGMENT,
//                     ty: wgpu::BindingType::Texture {
//                         multisampled: false,
//                         view_dimension: wgpu::TextureViewDimension::D2,
//                         sample_type: wgpu::TextureSampleType::Float { filterable: true },
//                     },
//                     count: None,
//                 },
//                 wgpu::BindGroupLayoutEntry {
//                     binding: 1,
//                     visibility: wgpu::ShaderStages::FRAGMENT,
//                     ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
//                     count: None,
//                 },
//             ],
//             label: None,
//         });
//         let bind_group = Self::create_bind_group(&ui.device, img, &bind_group_layout);
//
//         let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
//         let vertexes = vec![
//             ImageVertex::new_coord(rect.left_top(), [0.0, 0.0], &ui.ui_manage.context.size),
//             ImageVertex::new_coord(rect.left_bottom(), [0.0, 1.0], &ui.ui_manage.context.size),
//             ImageVertex::new_coord(rect.right_bottom(), [1.0, 1.0], &ui.ui_manage.context.size),
//             ImageVertex::new_coord(rect.right_top(), [1.0, 0.0], &ui.ui_manage.context.size)
//         ];
//
//
//         let vertex_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Vertex Buffer"),
//             contents: bytemuck::cast_slice(vertexes.as_slice()),
//             usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
//         });
//         let index_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Index Buffer"),
//             contents: bytemuck::cast_slice(&indices),
//             usage: wgpu::BufferUsages::INDEX,
//         });
//         let render_pipeline = Self::create_pipeline(&ui.device, &bind_group_layout);
//
//
//         ImageReader {
//             bind_group_layout,
//             bind_group,
//             vertex_buffer,
//             index_buffer,
//             render_pipeline,
//             vertexes,
//             textures: Map::new(),
//         }
//     }
//
//     fn create_pipeline(device: &Device, group_layout: &wgpu::BindGroupLayout) -> wgpu::RenderPipeline {
//         let shader = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
//             label: None,
//             source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
//         });
//         let render_pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
//             label: None,
//             bind_group_layouts: &[group_layout],
//             push_constant_ranges: &[],
//         });
//         let render_pipeline = device.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
//             label: None,
//             layout: Some(&render_pipeline_layout),
//             vertex: wgpu::VertexState {
//                 module: &shader,
//                 entry_point: Some("vvs_main"),
//                 compilation_options: Default::default(),
//                 buffers: &[ImageVertex::desc()],
//             },
//             fragment: Some(wgpu::FragmentState {
//                 module: &shader,
//                 entry_point: Some("ffs_main"),
//                 compilation_options: Default::default(),
//                 targets: &[Some(wgpu::ColorTargetState {
//                     format: device.surface_config.format,
//                     blend: Some(wgpu::BlendState::ALPHA_BLENDING),
//                     write_mask: wgpu::ColorWrites::ALL,
//                 })],
//             }),
//             primitive: wgpu::PrimitiveState {
//                 topology: wgpu::PrimitiveTopology::TriangleList,
//                 strip_index_format: None,
//                 front_face: wgpu::FrontFace::Ccw,
//                 cull_mode: Some(wgpu::Face::Back),
//                 polygon_mode: wgpu::PolygonMode::Fill,
//                 unclipped_depth: false,
//                 conservative: false,
//             },
//             depth_stencil: None,
//             multisample: wgpu::MultisampleState {
//                 count: SAMPLE_COUNT,
//                 mask: !0,
//                 alpha_to_coverage_enabled: false,
//             },
//             multiview: None,
//             cache: None,
//         });
//         render_pipeline
//     }
//
//     pub(crate) fn prepare(&mut self, device: &Device, context: &Context) {
//         if !context.resize { return; } //仅在改变大小时调用
//         self.vertexes.iter_mut().for_each(|x| x.screen_size = context.size.as_gamma_size());
//         device.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(self.vertexes.as_slice()));
//     }
//
//     pub(crate) fn offset(&mut self, device: &Device, rect: &Rect) {
//         for (index, v) in self.vertexes.iter_mut().enumerate() {
//             match index {
//                 0 => v.position = rect.left_top(),
//                 1 => v.position = rect.left_bottom(),
//                 2 => v.position = rect.right_bottom(),
//                 3 => v.position = rect.right_top(),
//                 _ => {}
//             }
//         }
//         device.queue.write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(self.vertexes.as_slice()));
//     }
//
//     fn create_bind_group(device: &Device, img: image::DynamicImage, group_layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
//         let rgba = img.to_rgba8();
//         let dimensions = img.dimensions();
//
//         let size = wgpu::Extent3d {
//             width: dimensions.0,
//             height: dimensions.1,
//             depth_or_array_layers: 1,
//         };
//         let texture = device.device.create_texture(&wgpu::TextureDescriptor {
//             label: None,
//             size,
//             mip_level_count: 1,
//             sample_count: 1,
//             dimension: wgpu::TextureDimension::D2,
//             format: wgpu::TextureFormat::Rgba8UnormSrgb,
//             usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
//             view_formats: &[],
//         });
//         let copy_texture = wgpu::TexelCopyTextureInfo {
//             texture: &texture,
//             mip_level: 0,
//             origin: wgpu::Origin3d::ZERO,
//             aspect: wgpu::TextureAspect::All,
//         };
//         let copy_buffer_layout = wgpu::TexelCopyBufferLayout {
//             offset: 0,
//             bytes_per_row: Some(4 * dimensions.0),
//             rows_per_image: Some(dimensions.1),
//         };
//         device.queue.write_texture(copy_texture, &rgba, copy_buffer_layout, size);
//         let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
//         let sampler = device.device.create_sampler(&wgpu::SamplerDescriptor {
//             address_mode_u: wgpu::AddressMode::ClampToEdge,
//             address_mode_v: wgpu::AddressMode::ClampToEdge,
//             address_mode_w: wgpu::AddressMode::ClampToEdge,
//             mag_filter: wgpu::FilterMode::Linear,
//             min_filter: wgpu::FilterMode::Linear,
//             mipmap_filter: wgpu::FilterMode::Linear,
//             ..Default::default()
//         });
//
//         let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
//             layout: &group_layout,
//             entries: &[
//                 wgpu::BindGroupEntry {
//                     binding: 0,
//                     resource: wgpu::BindingResource::TextureView(&texture_view),
//                 },
//                 wgpu::BindGroupEntry {
//                     binding: 1,
//                     resource: wgpu::BindingResource::Sampler(&sampler),
//                 },
//             ],
//             label: None,
//         });
//         bind_group
//     }
//
//     pub(crate) fn render(&mut self, render_pass: &mut wgpu::RenderPass) {
//         render_pass.set_pipeline(&self.render_pipeline);
//         render_pass.set_bind_group(0, &self.bind_group, &[]);
//         render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
//         render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
//         render_pass.draw_indexed(0..6, 0, 0..1);
//     }
// }


pub struct Image {
    pub(crate) id: String,
    pub(crate) source: &'static str,
    pub(crate) rect: Rect,
    size_mode: SizeMode,
}

impl Image {
    pub fn new(fp: &'static str) -> Self {
        Image {
            id: crate::gen_unique_id(),
            source: fp,
            rect: Rect {
                x: Pos {
                    min: 0.0,
                    max: 0.0,
                },
                y: Pos {
                    min: 300.0,
                    max: 400.0,
                },
            },
            size_mode: SizeMode::Fix,
        }
    }

    fn reset_size(&mut self, (width, height): (u32, u32)) {
        match self.size_mode {
            SizeMode::Auto => self.rect.set_size(width as f32, height as f32),
            SizeMode::FixWidth => {
                let scale = self.rect.height() / height as f32;
                self.rect.set_width(scale * width as f32)
            }
            SizeMode::FixHeight => {
                let scale = self.rect.width() / width as f32;
                self.rect.set_height(scale * height as f32);
            }
            _ => {}
        }
    }


    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.rect.set_width(width);
        self.rect.set_height(height);
        self.size_mode = SizeMode::Fix;
        self
    }
}

impl Widget for Image {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        let size = ui.ui_manage.context.render.image.insert_image(&ui.device, self.source.to_string(), self.source);
        self.reset_size(size);
        println!("image {:?}", self.rect);
        layout.alloc_rect(&self.rect);
        let task = PaintImage::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::Image(task));
    }

    fn update(&mut self, uim: &mut UiM) {
        todo!() //替换图片
    }
}