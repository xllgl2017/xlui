pub mod texture;

use std::hash::{DefaultHasher, Hasher};
use std::path::{Path, PathBuf};
#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::GENERIC_READ;
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Imaging::{CLSID_WICImagingFactory, GUID_WICPixelFormat32bppRGBA, IWICImagingFactory, WICBitmapDitherTypeNone, WICBitmapPaletteTypeCustom, WICDecodeMetadataCacheOnLoad};
#[cfg(target_os = "windows")]
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_INPROC_SERVER};
use windows::Win32::UI::Shell::SHCreateMemStream;
use crate::error::UiResult;
use crate::{Device, Size, SAMPLE_COUNT};
use crate::map::Map;
use crate::render::image::texture::ImageTexture;
use crate::vertex::ImageVertex;

pub enum ImageSource {
    File(PathBuf),
    Bytes(Vec<u8>),
}

impl ImageSource {
    pub fn uri(&self) -> String {
        let res = match self {
            ImageSource::File(f) => {
                let mut hasher = DefaultHasher::new();
                hasher.write(format!("{}", f.display()).as_bytes());
                hasher.finish().to_string()
            }
            ImageSource::Bytes(b) => {
                let mut hasher = DefaultHasher::new();
                hasher.write(b);
                hasher.finish().to_string()
            }
        };
        res
    }
}

impl From<PathBuf> for ImageSource {
    fn from(p: PathBuf) -> Self {
        ImageSource::File(p)
    }
}

impl From<Vec<u8>> for ImageSource {
    fn from(b: Vec<u8>) -> Self {
        ImageSource::Bytes(b)
    }
}

impl From<&str> for ImageSource {
    fn from(value: &str) -> Self {
        ImageSource::File(PathBuf::from(value))
    }
}

impl From<&[u8]> for ImageSource {
    fn from(value: &[u8]) -> Self {
        ImageSource::Bytes(value.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for ImageSource {
    fn from(value: &[u8; N]) -> Self {
        ImageSource::Bytes(value.to_vec())
    }
}


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

    pub fn insert_image(&mut self, device: &Device, source: &ImageSource) -> Size {
        let uri = source.uri();
        match self.textures.get(&uri) {
            None => {
                let texture = ImageTexture::new(device, source, &self.bind_group_layout);
                let size = texture.size();
                self.textures.insert(uri, texture);
                size
            }
            Some(texture) => texture.size()
        }
    }

    pub(crate) fn render(&self, uri: &String, vb: &wgpu::Buffer, ib: &wgpu::Buffer, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(&self.pipeline);
        let texture = self.textures.get(uri).unwrap();
        render_pass.set_bind_group(0, texture.bind_group(), &[]);
        render_pass.set_vertex_buffer(0, vb.slice(..));
        render_pass.set_index_buffer(ib.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}

#[cfg(target_os = "windows")]
pub fn load_win32_image(source: ImageSource) -> UiResult<(Vec<u8>, Size)> {
    // let fp = fp.as_ref().to_str().ok_or("图片路径错误")?;
    unsafe { CoInitialize(None).ok()?; }
    let factory: IWICImagingFactory = unsafe { CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)? };
    let decoder = match source {
        ImageSource::File(fp) => {
            let fp = fp.to_str().ok_or("图片路径错误")?;
            let filename: Vec<u16> = fp.encode_utf16().chain(Some(0)).collect();
            unsafe {
                factory.CreateDecoderFromFilename(
                    PCWSTR(filename.as_ptr()), None, GENERIC_READ, WICDecodeMetadataCacheOnLoad)?
            }
        }
        ImageSource::Bytes(bytes) => {
            let stream = unsafe { SHCreateMemStream(Some(&bytes)) }.unwrap();
            unsafe {
                factory.CreateDecoderFromStream(&stream, null_mut(), WICDecodeMetadataCacheOnLoad)?
            }
        }
    };
    let frame = unsafe { decoder.GetFrame(0) }?;
    let converter = unsafe { factory.CreateFormatConverter() }?;
    unsafe { converter.Initialize(&frame, &GUID_WICPixelFormat32bppRGBA, WICBitmapDitherTypeNone, None, 0.0, WICBitmapPaletteTypeCustom)?; }
    let mut size = Size { width: 0, height: 0 };
    unsafe { converter.GetSize(&mut size.width, &mut size.height)?; }
    let stride = (size.width * 4) as usize;
    let buf_size = stride * size.height as usize;
    let mut buffer = vec![0; buf_size];
    unsafe { converter.CopyPixels(null_mut(), stride as u32, &mut buffer)?; }
    Ok((buffer, size))
}

pub fn load_image_file(fp: impl AsRef<Path>) -> UiResult<(Vec<u8>, Size)> {
    #[cfg(target_os = "windows")]
    let (rgba, size) = load_win32_image(ImageSource::File(fp.as_ref().to_path_buf()))?;
    #[cfg(target_os = "windows")]
    return Ok((rgba, size));
    #[cfg(not(target_os = "windows"))]
    let img = image::open(fp)?;
    #[cfg(not(target_os = "windows"))]
    Ok((img.to_rgba8().to_vec(), Size { width: img.width(), height: img.height() }))
}


pub fn load_image_bytes(bytes: &[u8]) -> UiResult<(Vec<u8>, Size)> {
    #[cfg(target_os = "windows")]
    let (rgba, size) = load_win32_image(ImageSource::Bytes(bytes.to_vec()))?;
    #[cfg(target_os = "windows")]
    return Ok((rgba, size));
    #[cfg(not(target_os = "windows"))]
    let img = image::load_from_memory(bytes)?;
    #[cfg(not(target_os = "windows"))]
    Ok((img.to_rgba8().to_vec(), Size { width: img.width(), height: img.height() }))
}