use crate::paint::button::PaintButton;
use crate::paint::checkbox::PaintCheckBox;
use crate::paint::edit::PaintTextEdit;
use crate::paint::image::PaintImage;
use crate::paint::scroll_area::PaintScrollArea;
use crate::paint::scroll_bar::PaintScrollBar;
use crate::paint::slider::PaintSlider;
use crate::paint::spinbox::PaintSpinBox;
use crate::paint::text::PaintText;
use crate::vertex::Vertex;
use crate::{Device, SAMPLE_COUNT};
use crate::paint::radio::PaintRadioButton;
use crate::size::rect::Rect;

pub mod text;
pub mod image;
pub mod rectangle;
pub mod line;
pub mod color;
pub mod edit;
pub mod spinbox;
pub mod triangle;
pub mod slider;
pub mod scroll_bar;
pub mod checkbox;
pub mod scroll_area;
pub mod button;
pub mod radio;

fn gen_render_pipeline(device: &Device, topology: wgpu::PrimitiveTopology) -> wgpu::RenderPipeline {
    let shader = device.device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(include_str!("../render/triangle.wgsl").into()),
    });
    let pipeline_layout = device.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    let render_pipeline = device.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[Vertex::desc()],
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: Default::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format: device.surface_config.format,
                blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology,
            ..Default::default()
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


pub(crate) enum PaintTask {
    Text(PaintText),
    // Rectangle(PaintRectangle),
    // Line(PaintLine),
    Image(PaintImage),
    ScrollBar(PaintScrollBar),
    TextEdit(PaintTextEdit),
    SpinBox(PaintSpinBox),
    // Triangle(PaintTriangle),
    Slider(PaintSlider),
    CheckBox(PaintCheckBox),
    Button(PaintButton),
    ScrollArea(PaintScrollArea),
    Radio(PaintRadioButton),
}

impl PaintTask {
    pub fn paint_text(&mut self) -> &mut PaintText {
        match self {
            PaintTask::Text(paint_text) => paint_text,
            _ => panic!("应该是PaintTask::Text"),
        }
    }

    pub fn rect(&self) -> &Rect {
        match self {
            PaintTask::Text(t) => &t.rect,
            PaintTask::Image(t) => &t.rect,
            PaintTask::ScrollBar(t) => &t.rect(),
            PaintTask::TextEdit(t) => &t.rect(),
            PaintTask::SpinBox(t) => &t.rect(),
            PaintTask::Slider(t) => &t.rect(),
            PaintTask::CheckBox(t) => &t.rect(),
            PaintTask::Button(t) => &t.rect(),
            PaintTask::ScrollArea(t) => &t.rect,
            PaintTask::Radio(t) => &t.rect,
        }
    }

    // pub fn paint_rectangle(&mut self) -> &mut PaintRectangle {
    //     match self {
    //         PaintTask::Rectangle(paint_rectangle) => paint_rectangle,
    //         _ => panic!("应该是PaintTask::Rectangle"),
    //     }
    // }
    //
    // pub fn paint_line(&mut self) -> &mut PaintLine {
    //     match self {
    //         PaintTask::Line(paint_line) => paint_line,
    //         _ => panic!("应该是PaintTask::Line"),
    //     }
    // }
}