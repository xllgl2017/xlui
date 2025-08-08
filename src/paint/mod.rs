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
use crate::frame::App;
use crate::frame::context::Context;
use crate::paint::combobox::PaintComboBox;
// use crate::paint::popup::PaintPopup;
use crate::paint::radio::PaintRadioButton;
use crate::paint::rectangle::PaintRectangle;
use crate::size::rect::Rect;
use crate::ui::DrawParam;

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
// pub mod popup;
pub mod combobox;

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
    Rectangle(PaintRectangle),
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
    // Popup(PaintPopup),
    ComboBox(PaintComboBox),
}

impl PaintTask {
    pub fn paint_text_mut(&mut self) -> &mut PaintText {
        match self {
            PaintTask::Text(paint_text) => paint_text,
            _ => panic!("应该是PaintTask::Text"),
        }
    }

    pub fn paint_edit(&self) -> &PaintTextEdit {
        match self {
            PaintTask::TextEdit(paint_edit) => paint_edit,
            _ => panic!("应该是PaintTask::TextEdit"),
        }
    }


    pub fn paint_btn_mut(&mut self) -> &mut PaintButton {
        match self {
            PaintTask::Button(paint_btn) => paint_btn,
            _ => panic!("应该是PaintTask::PaintButton"),
        }
    }

    pub fn paint_spinbox_mut(&mut self) -> &mut PaintSpinBox {
        match self {
            PaintTask::SpinBox(paint_spinbox) => paint_spinbox,
            _ => panic!("应该是PaintTask::PaintSpinBox"),
        }
    }

    pub fn paint_slider_mut(&mut self) -> &mut PaintSlider {
        match self {
            PaintTask::Slider(paint_slider) => paint_slider,
            _ => panic!("应该是PaintTask::PaintSpinBox"),
        }
    }

    pub fn paint_checkbox_mut(&mut self) -> &mut PaintCheckBox {
        match self {
            PaintTask::CheckBox(paint_checkbox) => paint_checkbox,
            _ => panic!("应该是PaintTask::PaintCheckBox"),
        }
    }

    pub fn paint_radio_mut(&mut self) -> &mut PaintRadioButton {
        match self {
            PaintTask::Radio(paint_radio) => paint_radio,
            _ => panic!("应该是PaintTask::PaintRadioButton"),
        }
    }

    pub fn paint_rect_mut(&mut self) -> &mut PaintRectangle {
        match self {
            PaintTask::Rectangle(paint_rect) => paint_rect,
            _ => panic!("应该是PaintTask::PaintRectangle"),
        }
    }

    pub fn rect(&self) -> &Rect {
        match self {
            PaintTask::Rectangle(t) => t.rect(),
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
            // PaintTask::Popup(t) => &t.rect,
            PaintTask::ComboBox(t) => t.rect()
        }
    }

    pub fn draw<A: App>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        match self {
            PaintTask::Rectangle(paint_rect) => paint_rect.render(param, pass),
            PaintTask::Text(paint_text) => paint_text.render(param, pass), //绘制文本
            PaintTask::Image(paint_image) => paint_image.render(param, pass),
            PaintTask::ScrollBar(paint_bar) => paint_bar.render(param, pass),
            PaintTask::TextEdit(paint_edit) => paint_edit.render(param, pass),
            PaintTask::SpinBox(paint_spin_box) => paint_spin_box.render(param, pass),
            PaintTask::Slider(paint_slider) => paint_slider.render(param, pass),
            PaintTask::CheckBox(paint_checkbox) => paint_checkbox.render(param, pass),
            PaintTask::Button(paint_button) => paint_button.render(param, pass),
            PaintTask::ScrollArea(paint_area) => paint_area.draw(param, pass),
            PaintTask::Radio(paint_radio) => paint_radio.draw(param, pass),
            // PaintTask::Popup(paint_popup) => paint_popup.draw(device, context, render_pass),
            PaintTask::ComboBox(paint_combo) => paint_combo.draw(param, pass),
            _ => {}
        }
    }

    pub(crate) fn mouse_move<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        match self {
            PaintTask::Rectangle(paint_rect) => paint_rect.mouse_move(device, context),
            PaintTask::ScrollBar(paint_bar) => paint_bar.mouse_move(&device, context),
            PaintTask::TextEdit(paint_edit) => paint_edit.mouse_move(&device, context),
            PaintTask::SpinBox(paint_spinbox) => paint_spinbox.mouse_move(device, context),
            PaintTask::Slider(paint_slider) => paint_slider.mouse_move(device, context, app),
            PaintTask::CheckBox(paint_checkbox) => paint_checkbox.mouse_move(device, context),
            PaintTask::Button(paint_button) => paint_button.mouse_move(device, context),
            PaintTask::ScrollArea(paint_area) => paint_area.mouse_move(device, context, app),
            PaintTask::Radio(paint_radio) => paint_radio.mouse_move(device, context),
            PaintTask::ComboBox(paint_combo) => paint_combo.mouse_move(device, context, app),
            _ => {}
        }
    }

    pub(crate) fn mouse_down<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        match self {
            PaintTask::ScrollBar(paint_bar) => paint_bar.mouse_down(device),
            PaintTask::TextEdit(paint_edit) => paint_edit.mouse_down(device, context),
            PaintTask::SpinBox(paint_spinbox) => paint_spinbox.mouse_down(device, context, app),
            PaintTask::Slider(paint_slider) => paint_slider.mouse_down(device),
            PaintTask::ScrollArea(paint_area) => paint_area.mouse_down(device, context, app),
            _ => {}
        }
    }

    pub(crate) fn mouse_release<A: App>(&mut self, device: &Device, context: &mut Context, app: &mut A) {
        match self {
            PaintTask::SpinBox(paint_spinbox) => paint_spinbox.click(device, context, app),
            PaintTask::CheckBox(paint_checkbox) => paint_checkbox.mouse_click(device, context, app),
            PaintTask::Radio(paint_radio) => paint_radio.click(device, context, app),
            PaintTask::ComboBox(paint_combo) => paint_combo.click(device, context),
            PaintTask::Slider(paint_slider) => paint_slider.mouse_release(device),
            PaintTask::Button(paint_btn) => paint_btn.click(device, context, app),
            _ => {}
        }
    }
}