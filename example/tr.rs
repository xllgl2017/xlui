use wgpu::util::DeviceExt;
use xlui::style::ClickStyle;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct TriangleDrawParam {
    p0: [f32; 2],             //⬅️ 顶点位置1
    p1: [f32; 2],             //⬅️ 顶点位置2
    p2: [f32; 2],             //⬅️ 顶点位置3
    _pad0: [f32; 2],
    fill_color: [f32; 4],     //⬅️ 填充颜色
    border_thickness: f32,    //⬅️ 边框宽度
    _pad1: [f32; 3],
    border_color: [f32; 4],   //⬅️ 边框颜色
}

pub struct TriangleParam {
    pub p0: [f32; 2],
    pub p1: [f32; 2],
    pub p2: [f32; 2],
    pub style: ClickStyle,
    draw: TriangleDrawParam,
}

impl TriangleParam {
    pub fn new(p0: [f32; 2], p1: [f32; 2], p2: [f32; 2], style: ClickStyle) -> Self {
        let fill_color = style.dyn_fill(false, false).as_gamma_rgba();
        let border = style.dyn_border(false, false);
        let draw = TriangleDrawParam {
            p0,
            p1,
            p2,
            _pad0: [0.0; 2],
            fill_color,
            border_thickness: border.width,
            _pad1: [0.0; 3],
            border_color: border.color.as_gamma_rgba(),
        };
        TriangleParam {
            p0,
            p1,
            p2,
            style,
            draw,
        }
    }

    pub fn as_draw_param(&mut self, hovered: bool, mouse_down: bool) -> &[u8] {
        let fill_color = self.style.dyn_fill(mouse_down, hovered).as_gamma_rgba();
        let border = self.style.dyn_border(mouse_down, hovered);
        self.draw.p0 = self.p0;
        self.draw.p1 = self.p1;
        self.draw.p2 = self.p2;
        self.draw.border_thickness = border.width;
        self.draw.border_color = border.color.as_gamma_rgba();
        self.draw.fill_color = fill_color;
        bytemuck::bytes_of(&self.draw)
    }
}

pub struct TriangleRender {
    pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
}

impl TriangleRender {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let params = TriangleDrawParam {
            p0: [100.0, 100.0],
            p1: [400.0, 100.0],
            p2: [250.0, 300.0],
            _pad0: [0.0; 2],
            fill_color: [0.2, 0.8, 0.3, 1.0],
            border_thickness: 3.0,
            _pad1: [0.0; 3],
            border_color: [0.0, 0.0, 0.0, 1.0],
        };



        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Triangle BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Uniform Buffer"),
            contents: bytemuck::bytes_of(&params),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Triangle BG"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        // === Shader ===
        let shader = device.create_shader_module(wgpu::include_wgsl!("1.wgsl"));

        // === Pipeline ===
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Triangle PipelineLayout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
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
        TriangleRender {
            pipeline,
            bind_group,
        }
    }

    pub fn render(&mut self, rpass: &mut wgpu::RenderPass) {
        rpass.set_pipeline(&self.pipeline);
        rpass.set_bind_group(0, &self.bind_group, &[]);
        rpass.draw(0..6, 0..1); // 全屏矩形
    }
}