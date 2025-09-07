#[cfg(target_os = "windows")]
use std::ptr::null_mut;
#[cfg(not(target_os = "windows"))]
use image::GenericImageView;
#[cfg(target_os = "windows")]
use windows::core::PCWSTR;
#[cfg(target_os = "windows")]
use windows::Win32::Foundation::GENERIC_READ;
#[cfg(target_os = "windows")]
use windows::Win32::Graphics::Imaging::{CLSID_WICImagingFactory, GUID_WICPixelFormat32bppRGBA, IWICImagingFactory, WICBitmapDitherTypeNone, WICBitmapPaletteTypeCustom, WICDecodeMetadataCacheOnLoad};
#[cfg(target_os = "windows")]
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_INPROC_SERVER};
use crate::{Device, Size};
use crate::error::UiResult;
#[cfg(target_os = "windows")]
use crate::window::win32;

pub struct ImageTexture {
    size: Size,
    bind_group: wgpu::BindGroup,
}

impl ImageTexture {
    pub fn new(device: &Device, fp: &str, layout: &wgpu::BindGroupLayout) -> ImageTexture {
        #[cfg(target_os = "windows")]
        let (rgba, size) = ImageTexture::load_win32_image(fp).unwrap();
        #[cfg(not(target_os = "windows"))]
        let (rgba, size) = ImageTexture::load_image(fp).unwrap();
        let bind_group = Self::create_bind_group(device, rgba, size, layout);
        ImageTexture {
            bind_group,
            size,
        }
    }

    #[cfg(target_os = "windows")]
    fn load_win32_image(fp: &str) -> UiResult<(Vec<u8>, Size)> {
        unsafe { CoInitialize(None).ok()?; }
        let factory: IWICImagingFactory = unsafe { CoCreateInstance(&CLSID_WICImagingFactory, None, CLSCTX_INPROC_SERVER)? };
        let filename = win32::until::to_wstr(fp);
        let decoder = unsafe {
            factory.CreateDecoderFromFilename(
                PCWSTR(filename.as_ptr()), None, GENERIC_READ, WICDecodeMetadataCacheOnLoad)?
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

    #[cfg(not(target_os = "windows"))]
    fn load_image(fp: &str) -> UiResult<(Vec<u8>, Size)> {
        let img = image::open(fp)?;
        let (w, h) = img.dimensions();
        Ok((img.to_rgba8().to_vec(), Size { width: w, height: h }))
    }

    fn create_bind_group(device: &Device, rgba: Vec<u8>, size: Size, group_layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        let size = wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        };
        let texture = device.device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let copy_texture = wgpu::TexelCopyTextureInfo {
            texture: &texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        };
        let copy_buffer_layout = wgpu::TexelCopyBufferLayout {
            offset: 0,
            bytes_per_row: Some(4 * size.width),
            rows_per_image: Some(size.height),
        };
        device.queue.write_texture(copy_texture, &rgba, copy_buffer_layout, size);
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: None,
        });
        bind_group
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}