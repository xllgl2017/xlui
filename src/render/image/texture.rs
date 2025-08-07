use image::GenericImageView;
use crate::Device;

pub struct ImageTexture {
    pub(crate) size: (u32, u32),
    pub(crate) bind_group: wgpu::BindGroup,
}

impl ImageTexture {
    pub fn new(device: &Device, img: image::DynamicImage, layout: &wgpu::BindGroupLayout) -> ImageTexture {
        let size = img.dimensions();
        let bind_group = Self::create_bind_group(device, img, layout);

        ImageTexture {
            bind_group,
            size,
        }
    }

    fn create_bind_group(device: &Device, img: image::DynamicImage, group_layout: &wgpu::BindGroupLayout) -> wgpu::BindGroup {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
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
            bytes_per_row: Some(4 * dimensions.0),
            rows_per_image: Some(dimensions.1),
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
}