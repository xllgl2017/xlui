use crate::ui::Ui;
use crate::*;
#[cfg(feature = "gpu")]
use wgpu::util::DeviceExt;
use crate::render::circle::param::CircleParam;
use crate::render::rectangle::param::RectParam;
use crate::render::triangle::param::TriangleParam;

pub mod rectangle;
pub mod circle;
pub mod image;
pub mod triangle;

#[cfg(feature = "gpu")]
pub trait WrcParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool, size: Size) -> &[u8];
}

pub enum RenderKind {
    Rectangle(RectParam),
    Circle(CircleParam),
    Triangle(TriangleParam),
}

pub struct RenderParam {
    kind: RenderKind,
    #[cfg(feature = "gpu")]
    buffer: Option<wgpu::Buffer>,
    #[cfg(feature = "gpu")]
    bind_group: Option<wgpu::BindGroup>,
}


impl RenderParam {
    pub fn new(kind: RenderKind) -> RenderParam {
        RenderParam {
            kind,
            #[cfg(feature = "gpu")]
            buffer: None,
            #[cfg(feature = "gpu")]
            bind_group: None,
        }
    }

    #[cfg(feature = "gpu")]
    pub fn update(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
        let size = (ui.device.surface_config.width, ui.device.surface_config.height).into();
        let data = match self.kind {
            RenderKind::Rectangle(ref mut param) => param.as_draw_param(hovered, pressed, size),
            RenderKind::Circle(ref mut param) => param.as_draw_param(hovered, pressed, size),
            RenderKind::Triangle(ref mut param) => param.as_draw_param(hovered, pressed, size),
        };
        ui.device.queue.write_buffer(self.buffer.as_ref().unwrap(), 0, data);
    }

    #[cfg(feature = "gpu")]
    pub fn init(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
        let size = (ui.device.surface_config.width, ui.device.surface_config.height).into();
        let (buffer, bind_group) = match self.kind {
            RenderKind::Rectangle(ref mut param) => {
                let data = param.as_draw_param(hovered, pressed, size);
                ui.context.render.rectangle.init(&ui.device, data)
            }
            RenderKind::Circle(ref mut param) => {
                let data = param.as_draw_param(hovered, pressed, size);
                ui.context.render.circle.init(&ui.device, data)
            }
            RenderKind::Triangle(ref mut param) => {
                let data = param.as_draw_param(hovered, pressed, size);
                ui.context.render.triangle.init(&ui.device, data)
            }
        };
        self.buffer = Some(buffer);
        self.bind_group = Some(bind_group);
    }

    pub fn rect_mut(&mut self) -> &mut Rect {
        match self.kind {
            RenderKind::Rectangle(ref mut param) => &mut param.rect,
            RenderKind::Circle(ref mut param) => &mut param.rect,
            RenderKind::Triangle(ref mut param) => &mut param.rect,
        }
    }

    pub fn rect(&self) -> &Rect {
        match self.kind {
            RenderKind::Rectangle(ref param) => &param.rect,
            RenderKind::Circle(ref param) => &param.rect,
            RenderKind::Triangle(ref param) => &param.rect,
        }
    }

    pub fn set_shadow(&mut self, shadow: Shadow) {
        match self.kind {
            RenderKind::Rectangle(ref mut param) => param.shadow = shadow,
            RenderKind::Circle(_) => {}
            RenderKind::Triangle(_) => {}
        }
    }

    pub fn rect_param_mut(&mut self) -> &mut RectParam {
        match self.kind {
            RenderKind::Rectangle(ref mut param) => param,
            _ => panic!("not rect")
        }
    }

    pub fn set_frame_style(&mut self, style: FrameStyle) {
        match self.kind {
            RenderKind::Rectangle(ref mut param) => param.set_frame(style),
            RenderKind::Circle(_) => {}
            RenderKind::Triangle(_) => {}
        }
    }

    pub fn set_poses(&mut self, p0: Pos, p1: Pos, p2: Pos) {
        match self.kind {
            RenderKind::Rectangle(_) => {}
            RenderKind::Circle(_) => {}
            RenderKind::Triangle(ref mut param) => param.set_poses(p0, p1, p2)
        }
    }

    pub fn set_style(&mut self, style: ClickStyle) {
        match self.kind {
            RenderKind::Rectangle(ref mut param) => param.set_style(style),
            RenderKind::Circle(ref mut param) => param.set_style(style),
            RenderKind::Triangle(ref mut param) => param.set_style(style),
        }
    }

    pub fn style_mut(&mut self) -> &mut ClickStyle {
        match self.kind {
            RenderKind::Rectangle(ref mut param) => &mut param.style,
            RenderKind::Circle(ref mut param) => &mut param.style,
            RenderKind::Triangle(ref mut param) => &mut param.style,
        }
    }

    pub fn offset_to_rect(&mut self, rect: &Rect) {
        match self.kind {
            RenderKind::Rectangle(ref mut param) => param.rect.offset_to_rect(rect),
            RenderKind::Circle(ref mut param) => param.rect.offset_to_rect(rect),
            RenderKind::Triangle(ref mut param) => param.offset_to_rect(rect),
        };
    }

    // #[cfg(feature = "gpu")]
    // pub fn init_rectangle(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
    //     let size = (ui.device.surface_config.width, ui.device.surface_config.height).into();
    //     let data = self.param.as_draw_param(hovered, pressed, size);
    //     let (buffer, bind_group) = ui.context.render.rectangle.init(&ui.device, data);
    //     self.buffer = Some(buffer);
    //     self.bind_group = Some(bind_group);
    // }
    //
    // #[cfg(feature = "gpu")]
    // pub fn init_triangle(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
    //     let size = (ui.device.surface_config.width, ui.device.surface_config.height).into();
    //     let data = self.param.as_draw_param(hovered, pressed, size);
    //     let (buffer, bind_group) = ui.context.render.triangle.init(&ui.device, data);
    //     self.buffer = Some(buffer);
    //     self.bind_group = Some(bind_group);
    // }
    //
    // #[cfg(feature = "gpu")]
    // pub fn init_circle(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
    //     let size = (ui.device.surface_config.width, ui.device.surface_config.height).into();
    //     let data = self.param.as_draw_param(hovered, pressed, size);
    //     let (buffer, bind_group) = ui.context.render.circle.init(&ui.device, data);
    //     self.buffer = Some(buffer);
    //     self.bind_group = Some(bind_group);
    // }

    pub fn draw(&mut self, ui: &mut Ui, hovered: bool, pressed: bool) {
        match self.kind {
            #[cfg(not(feature = "gpu"))]
            RenderKind::Rectangle(ref param) => {
                let fill = param.style.dyn_fill(pressed, hovered);
                let border = param.style.dyn_border(pressed, hovered);
                #[cfg(windows)]
                ui.context.window.win32().paint_rect(ui.hdc.unwrap(), fill, border, &param.rect);
                #[cfg(target_os = "linux")]
                ui.context.window.x11().paint_rect(ui.paint.as_mut().unwrap().cairo, fill, border, &param.rect);
            }
            #[cfg(feature = "gpu")]
            RenderKind::Rectangle(_) => {
                self.update(ui, hovered, pressed);
                let pass = ui.pass.as_mut().unwrap();
                ui.context.render.rectangle.render(&self, pass);
            }
            #[cfg(not(feature = "gpu"))]
            RenderKind::Circle(ref param) => {
                let fill = param.style.dyn_fill(pressed, hovered);
                let border = param.style.dyn_border(pressed, hovered);
                #[cfg(windows)]
                ui.context.window.win32().paint_circle(ui.hdc.unwrap(), &param.rect, fill, border);
                #[cfg(target_os = "linux")]
                ui.context.window.x11().paint_circle(ui.paint.as_mut().unwrap().cairo, fill, border, &param.rect);
            }
            #[cfg(not(feature = "gpu"))]
            RenderKind::Triangle(ref param) => {
                let fill = param.style.dyn_fill(pressed, hovered);
                let border = param.style.dyn_border(pressed, hovered);
                #[cfg(windows)]
                ui.context.window.win32().paint_triangle(ui.hdc.unwrap(), param.as_win32_points(), fill, border);
                #[cfg(target_os = "linux")]
                ui.context.window.x11().paint_triangle(ui.paint.as_mut().unwrap().cairo, param.p0, param.p1, param.p2, fill, border);
            }
            #[cfg(feature = "gpu")]
            RenderKind::Circle(_) => {
                self.update(ui, hovered, pressed);
                let pass = ui.pass.as_mut().unwrap();
                ui.context.render.circle.render(&self, pass);
            }
            #[cfg(feature = "gpu")]
            RenderKind::Triangle(_) => {
                self.update(ui, hovered, pressed);
                let pass = ui.pass.as_mut().unwrap();
                ui.context.render.triangle.render(&self, pass);
            }
        }
    }
}

#[cfg(feature = "gpu")]
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

#[cfg(feature = "gpu")]
pub(crate) trait WrcRender {

    fn pipeline(&self) -> &wgpu::RenderPipeline;

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

    fn render(&self, param: &RenderParam, render_pass: &mut wgpu::RenderPass) {
        render_pass.set_pipeline(self.pipeline());
        render_pass.set_bind_group(0, param.bind_group.as_ref().unwrap(), &[]);
        render_pass.draw(0..6, 0..1);
    }
}