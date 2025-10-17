#[cfg(feature = "gpu")]
use wgpu::IndexFormat;
use crate::ui::Ui;
use crate::*;
#[cfg(feature = "gpu")]
use wgpu::util::DeviceExt;
use crate::shape::Shape;

pub mod rectangle;
pub mod circle;
pub mod image;
pub mod triangle;

#[cfg(feature = "gpu")]
pub trait WrcParam {
    fn as_draw_param(&mut self, hovered: bool, mouse_down: bool);
}

pub struct RenderParam {
    rect: Rect,
    shape: Shape,
    style: VisualStyle,
    #[cfg(feature = "gpu")]
    bind_buffer: Option<wgpu::Buffer>,
    #[cfg(feature = "gpu")]
    bind_group: Option<wgpu::BindGroup>,
    #[cfg(feature = "gpu")]
    vertices_buffer: Option<wgpu::Buffer>,
    #[cfg(feature = "gpu")]
    indices_buffer: Option<wgpu::Buffer>,
}


impl RenderParam {
    pub fn new(shape: Shape) -> RenderParam {
        RenderParam {
            rect: Rect::new(),
            shape,
            style: VisualStyle::new(),
            #[cfg(feature = "gpu")]
            bind_buffer: None,
            #[cfg(feature = "gpu")]
            bind_group: None,
            #[cfg(feature = "gpu")]
            vertices_buffer: None,
            #[cfg(feature = "gpu")]
            indices_buffer: None,
        }
    }

    pub fn with_style(mut self, style: VisualStyle) -> RenderParam {
        self.style = style;
        self
    }

    pub fn set_style(&mut self, style: VisualStyle) {
        self.style = style;
    }

    pub fn style_mut(&mut self) -> &mut VisualStyle { &mut self.style }

    pub fn style(&self) -> &VisualStyle { &self.style }

    pub fn with_size(mut self, w: f32, h: f32) -> RenderParam {
        self.rect.set_size(w, h);
        self
    }

    pub fn rect(&self) -> &Rect { &self.rect }

    pub fn rect_mut(&mut self) -> &mut Rect { &mut self.rect }

    #[cfg(feature = "gpu")]
    pub fn update(&mut self, ui: &mut Ui, style: WidgetStyle) {
        let size: Size = (&ui.device.surface_config).into();
        let bind_data = bytemuck::bytes_of(&size);
        match self.bind_buffer {
            None => {
                let (buffer, group) = ui.context.render.rectangle.init(&ui.device, bind_data);
                self.bind_buffer = Some(buffer);
                self.bind_group = Some(group);
            }
            Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, bind_data),
        }
        let vertices_data = bytemuck::cast_slice(self.shape.vertices());
        let indices_data = bytemuck::cast_slice(self.shape.indices());
        match self.vertices_buffer {
            None => {
                let buffer = Self::create_buffer(ui, wgpu::BufferUsages::VERTEX, self.shape.vertices_size());
                ui.device.queue.write_buffer(&buffer, 0, vertices_data);
                self.vertices_buffer = Some(buffer);
            }
            Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, vertices_data),
        }
        match self.indices_buffer {
            None => {
                let buffer = Self::create_buffer(ui, wgpu::BufferUsages::INDEX, self.shape.indices_size());
                ui.device.queue.write_buffer(&buffer, 0, indices_data);
                self.indices_buffer = Some(buffer);
            }
            Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, indices_data),
        }
        // match self.kind {
        //     Shape::Rectangle(ref mut param) => {
        //         // param.as_draw_param(hovered, pressed);
        //         let bind_data = bytemuck::bytes_of(&size);
        //         match self.bind_buffer {
        //             None => {
        //                 let (buffer, group) = ui.context.render.rectangle.init(&ui.device, bind_data);
        //                 self.bind_buffer = Some(buffer);
        //                 self.bind_group = Some(group);
        //             }
        //             Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, bind_data),
        //         }
        //         let vertices_data = bytemuck::cast_slice(&param.rect_shape.vertices);
        //         let indices_data = bytemuck::cast_slice(&param.rect_shape.indices);
        //         match self.vertices_buffer {
        //             None => {
        //                 let buffer = Self::create_buffer(ui, wgpu::BufferUsages::VERTEX, 8192);
        //                 ui.device.queue.write_buffer(&buffer, 0, vertices_data);
        //                 self.vertices_buffer = Some(buffer);
        //             }
        //             Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, vertices_data),
        //         }
        //         match self.indices_buffer {
        //             None => {
        //                 let buffer = Self::create_buffer(ui, wgpu::BufferUsages::INDEX, 2048);
        //                 ui.device.queue.write_buffer(&buffer, 0, indices_data);
        //                 self.indices_buffer = Some(buffer);
        //             }
        //             Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, indices_data),
        //         }
        //     }
        //     RenderKind::Circle(ref mut param) => {
        //         param.as_draw_param(hovered, pressed);
        //         let data = bytemuck::bytes_of(&size);
        //         match self.bind_buffer {
        //             None => {
        //                 let (buffer, group) = ui.context.render.circle.init(&ui.device, data);
        //                 self.bind_buffer = Some(buffer);
        //                 self.bind_group = Some(group);
        //             }
        //             Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, data),
        //         }
        //         match self.vertices_buffer {
        //             None => {
        //                 let buffer = Self::create_buffer(ui, wgpu::BufferUsages::VERTEX, 8192);
        //                 ui.device.queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&param.circle_shape.vertices));
        //                 self.vertices_buffer = Some(buffer);
        //             }
        //             Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&param.circle_shape.vertices)),
        //         }
        //         match self.indices_buffer {
        //             None => {
        //                 let buffer = Self::create_buffer(ui, wgpu::BufferUsages::INDEX, 2048);
        //                 ui.device.queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&param.circle_shape.indices));
        //                 self.indices_buffer = Some(buffer);
        //             }
        //             Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&param.circle_shape.indices)),
        //         }
        //     }
        //     RenderKind::Triangle(ref mut param) => {
        //         param.as_draw_param(hovered, pressed);
        //         let data = bytemuck::bytes_of(&size);
        //         match self.bind_buffer {
        //             None => {
        //                 let (buffer, group) = ui.context.render.triangle.init(&ui.device, data);
        //                 self.bind_buffer = Some(buffer);
        //                 self.bind_group = Some(group);
        //             }
        //             Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, data),
        //         }
        //         match self.vertices_buffer {
        //             None => {
        //                 let buffer = Self::create_buffer(ui, wgpu::BufferUsages::VERTEX, 72);
        //                 ui.device.queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&param.vertices));
        //                 self.vertices_buffer = Some(buffer);
        //             }
        //             Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&param.vertices)),
        //         }
        //         match self.indices_buffer {
        //             None => {
        //                 let buffer = Self::create_buffer(ui, wgpu::BufferUsages::INDEX, 12);
        //                 ui.device.queue.write_buffer(&buffer, 0, bytemuck::cast_slice(&param.indices));
        //                 self.indices_buffer = Some(buffer);
        //             }
        //             Some(ref buffer) => ui.device.queue.write_buffer(buffer, 0, bytemuck::cast_slice(&param.indices)),
        //         }
        //     }
        // };
    }

    #[cfg(feature = "gpu")]
    pub(crate) fn re_init(&mut self) {
        let buffer = self.bind_buffer.take();
        if let Some(buffer) = buffer {
            buffer.destroy();
            drop(buffer);
        }
        let buffer = self.vertices_buffer.take();
        if let Some(buffer) = buffer {
            buffer.destroy();
            drop(buffer);
        }
        let buffer = self.indices_buffer.take();
        if let Some(buffer) = buffer {
            buffer.destroy();
            drop(buffer);
        }
        let group = self.bind_group.take();
        if let Some(group) = group { drop(group); }
    }

    #[cfg(feature = "gpu")]
    fn create_buffer(ui: &mut Ui, usage: wgpu::BufferUsages, size: u64) -> wgpu::Buffer {
        ui.device.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage: usage | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        })
    }

    // pub fn rect_mut(&mut self) -> &mut Rect {
    //     match self.kind {
    //         RenderKind::Rectangle(ref mut param) => &mut param.rect,
    //         RenderKind::Circle(ref mut param) => &mut param.rect,
    //         RenderKind::Triangle(ref mut param) => &mut param.rect,
    //     }
    // }

    // pub fn rect(&self) -> &Rect {
    //     match self.kind {
    //         RenderKind::Rectangle(ref param) => &param.rect,
    //         RenderKind::Circle(ref param) => &param.rect,
    //         RenderKind::Triangle(ref param) => &param.rect,
    //     }
    // }

    // pub fn set_shadow(&mut self, shadow: Shadow) {
    //     match self.kind {
    //         RenderKind::Rectangle(ref mut param) => param.shadow = shadow,
    //         RenderKind::Circle(_) => {}
    //         RenderKind::Triangle(_) => {}
    //     }
    // }

    // pub fn rect_param_mut(&mut self) -> &mut RectParam {
    //     match self.kind {
    //         RenderKind::Rectangle(ref mut param) => param,
    //         _ => panic!("not rect")
    //     }
    // }

    // pub fn rect_param(&self) -> &RectParam {
    //     match self.kind {
    //         RenderKind::Rectangle(ref param) => param,
    //         _ => panic!("not rect")
    //     }
    // }

    // pub fn set_frame_style(&mut self, style: FrameStyle) {
    //     match self.kind {
    //         RenderKind::Rectangle(ref mut param) => param.set_frame(style),
    //         RenderKind::Circle(_) => {}
    //         RenderKind::Triangle(_) => {}
    //     }
    // }
    //
    pub fn set_poses(&mut self, p0: Pos, p1: Pos, p2: Pos) {
        match self.shape {
            Shape::Triangle(ref mut param) => param.set_poses(p0, p1, p2),
            _ => {}
        }
    }

    // pub fn set_style(&mut self, style: ClickStyle) {
    //     match self.kind {
    //         RenderKind::Rectangle(ref mut param) => param.set_style(style),
    //         RenderKind::Circle(ref mut param) => param.set_style(style),
    //         RenderKind::Triangle(ref mut param) => param.set_style(style),
    //     }
    // }

    // pub fn style_mut(&mut self) -> &mut ClickStyle {
    //     match self.kind {
    //         RenderKind::Rectangle(ref mut param) => &mut param.style,
    //         RenderKind::Circle(ref mut param) => &mut param.style,
    //         RenderKind::Triangle(ref mut param) => &mut param.style,
    //     }
    // }

    pub fn offset_to_rect(&mut self, rect: &Rect) {
        match self.shape {
            Shape::Triangle(ref mut param) => {
                let offset = self.rect.offset_to_rect(rect);
                param.offset(&offset);
                offset
            }
            _ => self.rect.offset_to_rect(rect)
        };
    }
    #[cfg(feature = "gpu")]
    pub fn indices(&self) -> &Vec<u16> {
        match self.kind {
            RenderKind::Rectangle(ref param) => &param.rect_shape.indices,
            RenderKind::Circle(ref param) => &param.circle_shape.indices,
            RenderKind::Triangle(ref param) => &param.indices
        }
    }

    pub fn draw(&mut self, ui: &mut Ui, disabled: bool, hovered: bool, pressed: bool) {
        let style = self.style.dyn_style(ui.disabled || disabled, hovered, pressed);
        match self.shape {
            #[cfg(not(feature = "gpu"))]
            Shape::Rectangle => {
                #[cfg(windows)]
                ui.context.window.win32().paint_rect(ui.paint.as_mut().unwrap().hdc, fill, border, &param.rect);
                #[cfg(target_os = "linux")]
                ui.context.window.x11().paint_rect(ui.paint.as_mut().unwrap().cairo, style, &self.rect);
            }
            #[cfg(feature = "gpu")]
            Shape::Rectangle(_) => {
                self.update(ui, style, hovered, pressed);
                let pass = &mut ui.paint.as_mut().unwrap().pass;
                ui.context.render.rectangle.render(&self, pass);
            }
            #[cfg(not(feature = "gpu"))]
            Shape::Circle => {
                #[cfg(windows)]
                ui.context.window.win32().paint_circle(ui.paint.as_mut().unwrap().hdc, &param.rect, fill, border);
                #[cfg(target_os = "linux")]
                ui.context.window.x11().paint_circle(ui.paint.as_mut().unwrap().cairo, style, &self.rect);
            }
            #[cfg(not(feature = "gpu"))]
            Shape::Triangle(ref mut param) => {
                #[cfg(windows)]
                ui.context.window.win32().paint_triangle(ui.paint.as_mut().unwrap().hdc, param.as_win32_points(), fill, border);
                #[cfg(target_os = "linux")]
                ui.context.window.x11().paint_triangle(ui.paint.as_mut().unwrap().cairo, param.p0, param.p1, param.p2, style);
            }
            #[cfg(feature = "gpu")]
            RenderKind::Circle(_) => {
                self.update(ui, hovered, pressed);
                let pass = &mut ui.paint.as_mut().unwrap().pass;
                ui.context.render.circle.render(&self, pass);
            }
            #[cfg(feature = "gpu")]
            RenderKind::Triangle(_) => {
                self.update(ui, hovered, pressed);
                let pass = &mut ui.paint.as_mut().unwrap().pass;
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
        render_pass.set_vertex_buffer(0, param.vertices_buffer.as_ref().unwrap().slice(..));
        render_pass.set_index_buffer(param.indices_buffer.as_ref().unwrap().slice(..), IndexFormat::Uint16);
        render_pass.draw_indexed(0..param.indices().len() as u32, 0, 0..1);
    }
}

#[derive(Clone)]
pub struct WidgetStyle {
    pub fill: Color,
    pub border: Border,
    pub radius: Radius,
    pub shadow: Shadow,
}

impl WidgetStyle {
    pub fn new() -> WidgetStyle {
        WidgetStyle {
            fill: Color::new(),
            border: Border::same(0.0),
            radius: Radius::same(0),
            shadow: Shadow::new(),
        }
    }
}

impl From<(Color, f32, u8)> for WidgetStyle {
    fn from(value: (Color, f32, u8)) -> Self {
        let mut res = WidgetStyle::new();
        res.fill = value.0;
        res.border = Border::same(value.1);
        res.radius = Radius::same(value.2);
        res
    }
}

#[derive(Clone)]
pub struct VisualStyle {
    pub disabled: WidgetStyle,
    pub inactive: WidgetStyle,
    pub hovered: WidgetStyle,
    pub pressed: WidgetStyle,
}

impl VisualStyle {
    pub fn new() -> VisualStyle {
        VisualStyle {
            disabled: WidgetStyle::new(),
            inactive: WidgetStyle::new(),
            hovered: WidgetStyle::new(),
            pressed: WidgetStyle::new(),
        }
    }

    pub fn same(style: WidgetStyle) -> VisualStyle {
        let mut res = VisualStyle::new();
        res.disabled = style.clone();
        res.inactive = style.clone();
        res.hovered = style.clone();
        res.pressed = style;
        res
    }

    pub fn dyn_style(&self, disabled: bool, hovered: bool, pressed: bool) -> &WidgetStyle {
        if disabled { return &self.disabled; }
        if pressed { return &self.pressed; }
        if hovered { return &self.hovered; }
        &self.inactive
    }
}


pub struct Visual {
    render: RenderParam,
    disable: bool,
    foreground: bool,
}

impl Visual {
    pub fn new() -> Visual {
        Visual {
            #[cfg(feature = "gpu")]
            render: RenderParam::new(Shape::Rectangle(RectangleShape::new())),
            render: RenderParam::new(Shape::Rectangle),
            disable: true,
            foreground: false,
        }
    }

    pub fn with_enable(mut self) -> Visual {
        self.disable = false;
        self
    }

    pub fn enable(&mut self) -> &mut Visual {
        self.disable = false;
        self
    }

    pub fn with_style(mut self, style: VisualStyle) -> Visual {
        self.render.set_style(style);
        self
    }

    pub fn set_style(&mut self, style: VisualStyle) {
        self.render.set_style(style);
    }


    pub fn draw(&mut self, ui: &mut Ui, disabled: bool, hovered: bool, pressed: bool, foreground: bool) {
        if self.disable || self.foreground != foreground { return; }
        self.render.draw(ui, disabled, hovered, pressed);
    }

    pub fn foreground(&self) -> bool {
        self.foreground
    }

    pub fn disable(&self) -> bool {
        self.disable
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Visual {
        self.render.rect.set_size(w, h);
        self
    }

    pub fn rect(&self) -> &Rect { self.render.rect() }

    pub fn rect_mut(&mut self) -> &mut Rect { self.render.rect_mut() }

    pub fn with_rect(mut self, rect: Rect) -> Visual {
        self.render.rect = rect;
        self
    }

    pub fn offset_to_rect(&mut self, rect: &Rect) {
        self.render.offset_to_rect(rect)
    }

    pub fn style(&self) -> &VisualStyle { &self.render.style }

    pub fn style_mut(&mut self) -> &mut VisualStyle { self.render.style_mut() }
}
