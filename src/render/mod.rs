use crate::ui::Ui;
use crate::{Device, Size, SAMPLE_COUNT};
use wgpu::util::DeviceExt;

pub mod rectangle;
pub mod circle;
pub mod image;
pub mod triangle;

pub trait WrcParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, size: Size) -> &[u8];
}

pub struct RenderParam<T> {
    pub param: T,
    buffer: Option<wgpu::Buffer>,
    bind_group: Option<wgpu::BindGroup>,
}

impl<T: WrcParam> RenderParam<T> {
    pub fn new(param: T) -> RenderParam<T> {
        RenderParam {
            param,
            buffer: None,
            bind_group: None,
        }
    }

    pub fn update(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
        let size = (ui.device.surface_config.width, ui.device.surface_config.height).into();
        let data = self.param.as_draw_param(hovered, pressed, size);
        ui.device.queue.write_buffer(self.buffer.as_ref().unwrap(), 0, data);
    }

    pub fn init_rectangle(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
        let size = (ui.device.surface_config.width, ui.device.surface_config.height).into();
        let data = self.param.as_draw_param(hovered, pressed, size);
        let (buffer, bind_group) = ui.context.render.rectangle.init(&ui.device, data);
        self.buffer = Some(buffer);
        self.bind_group = Some(bind_group);
    }

    pub fn init_triangle(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
        let size = (ui.device.surface_config.width, ui.device.surface_config.height).into();
        let data = self.param.as_draw_param(hovered, pressed, size);
        let (buffer, bind_group) = ui.context.render.triangle.init(&ui.device, data);
        self.buffer = Some(buffer);
        self.bind_group = Some(bind_group);
    }

    pub fn init_circle(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
        let size = (ui.device.surface_config.width, ui.device.surface_config.height).into();
        let data = self.param.as_draw_param(hovered, pressed, size);
        let (buffer, bind_group) = ui.context.render.circle.init(&ui.device, data);
        self.buffer = Some(buffer);
        self.bind_group = Some(bind_group);
    }
}

fn create_pipeline(device: &Device, shader: wgpu::ShaderModule, layout: wgpu::PipelineLayout, buffers: &[wgpu::VertexBufferLayout]) -> wgpu::RenderPipeline {
    device.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: device.texture_format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}


pub(crate) trait WrcRender {
    fn pipeline(&self) -> &wgpu::RenderPipeline;

    // fn bind_groups(&self) -> &Map<wgpu::BindGroup>;

    // fn bind_groups_mut(&mut self) -> &mut Map<wgpu::BindGroup>;

    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout;

    fn init(&mut self, device: &Device, data: &[u8]) -> (wgpu::Buffer, wgpu::BindGroup) {
        let buffer = device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let bind_group_entry = wgpu::BindGroupEntry {
            binding: 0,
            resource: buffer.as_entire_binding(),
        };
        let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: self.bind_group_layout(),
            entries: &[bind_group_entry],
            label: None,
        });
        (buffer, bind_group)
    }

    // fn create_bind_group(&mut self, device: &Device, buffer: &wgpu::Buffer) -> String {
    //     let bind_group_entry = wgpu::BindGroupEntry {
    //         binding: 0,
    //         resource: buffer.as_entire_binding(),
    //     };
    //     let bind_group = device.device.create_bind_group(&wgpu::BindGroupDescriptor {
    //         layout: self.bind_group_layout(),
    //         entries: &[bind_group_entry],
    //         label: None,
    //     });
    //     let key = crate::gen_unique_id();
    //     self.bind_groups_mut().insert(key.clone(), bind_group);
    //     key
    // }
    //
    // fn create_buffer(&self, device: &Device, param: &[u8]) -> wgpu::Buffer {
    //     device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
    //         label: None,
    //         contents: param,
    //         usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    //     })
    // }

    fn render<T>(&self, param: &RenderParam<T>, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(self.pipeline());
        render_pass.set_bind_group(0, param.bind_group.as_ref().unwrap(), &[]);
        render_pass.draw(0..6, 0..1);
    }

    // fn remove(&mut self, key: &String) {
    //     self.bind_groups_mut().remove(key);
    // }
}