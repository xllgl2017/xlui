use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
struct DrawParam {
    pos: [f32; 2],           //⬅️ 左上角顶点位置 (NDC)
    size: [f32; 2],          //⬅️ 矩形的宽高
    radius_tl: f32,          //⬅️ 左上圆角
    radius_tr: f32,          //⬅️ 右上圆角
    radius_br: f32,          //⬅️ 右下圆角
    radius_bl: f32,          //⬅️ 左下圆角
    border_width: f32,       //⬅️ 边框宽度
    _pad0: [f32; 3],
    border_color: [f32; 4],  //⬅️ 边框颜色
    shadow_offset: [f32; 2], //⬅️ 阴影位移
    shadow_spread: f32,      //⬅️ 阴影蔓延
    _pad1: [f32; 1],
    shadow_color: [f32; 4],  //⬅️ 阴影颜色
    fill_color: [f32; 4],    //⬅️ 填充颜色

}

pub struct RoundedBorderRenderer {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

impl RoundedBorderRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../src/render/rectangle/rectangle.wgsl"));

        let draw_param = DrawParam {
            pos: [20.0, 20.0],
            size: [200.0, 100.0],
            radius_bl: 10.0,
            radius_tl: 10.0,
            radius_tr: 10.0,
            radius_br: 10.0,
            border_width: 2.0,
            _pad0: [0.0; 3],
            border_color: [1.0, 0.0, 0.0, 1.0],
            shadow_offset: [1.0, 1.0],
            shadow_spread: 1.0,
            shadow_color: [0.0, 0.0, 1.0, 0.8],
            fill_color: [1.0, 1.0, 0.0, 1.0],
            _pad1: [0.0; 1],
        };
        println!("{:?}", draw_param);

        let param_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Rect Uniform"),
            contents: bytemuck::bytes_of(&draw_param),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });


        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Rounded Border Layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: param_buffer.as_entire_binding(),
                },
            ],
            label: Some("Rounded Border Bind Group"),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Rounded Border Pipeline Layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Rounded Border Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: Default::default(),
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
            cache: None,
        });

        Self {
            pipeline,
            bind_group,
            // style_buffer,
        }
    }

    // pub fn update_rect(&self, queue: &wgpu::Queue, x: f32, y: f32, width: f32, height: f32) {
    //     let rect = DrawParam { pos: [x, y], size: [width, height] };
    //     queue.write_buffer(&self.rect_buffer, 0, bytemuck::bytes_of(&rect));
    // }

    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..6, 0..1);
    }
}
