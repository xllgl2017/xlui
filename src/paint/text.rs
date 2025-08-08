use crate::frame::context::Context;
use crate::paint::color::Color;
use crate::size::rect::Rect;
use crate::text::text_buffer::TextBuffer;
use crate::text::{TextSize, TextWrap};
use crate::ui::{DrawParam, Ui};
use crate::{Device, SAMPLE_COUNT};
use glyphon::Shaping;
use wgpu::MultisampleState;

pub struct PaintText {
    id: String,
    pub(crate) buffer: glyphon::Buffer,
    pub(crate) render: glyphon::TextRenderer,
    pub(crate) rect: Rect,
    color: Color,
    change: bool,
}

impl PaintText {
    pub fn new(ui: &mut Ui, text_buffer: &TextBuffer) -> PaintText {
        let mut buffer = glyphon::Buffer::new(&mut ui.ui_manage.context.render.text.font_system, glyphon::Metrics::new(text_buffer.text_size.font_size, text_buffer.text_size.line_height));
        buffer.set_wrap(&mut ui.ui_manage.context.render.text.font_system, text_buffer.text_wrap.as_gamma());
        buffer.set_text(&mut ui.ui_manage.context.render.text.font_system, &text_buffer.text, &ui.ui_manage.context.font.font_attr(), Shaping::Advanced);
        let render = glyphon::TextRenderer::new(&mut ui.ui_manage.context.render.text.atlas, &ui.device.device, MultisampleState {
            count: SAMPLE_COUNT,
            mask: !0,
            alpha_to_coverage_enabled: false,
        }, None);
        PaintText {
            id: text_buffer.id.clone(),
            render,
            rect: text_buffer.rect.clone(),
            buffer,
            color: text_buffer.color.clone(),
            change: false,
        }
    }

    pub(crate) fn prepare(&mut self, device: &Device, context: &mut Context) {
        let area = glyphon::TextArea {
            buffer: &self.buffer,
            left: self.rect.x.min,
            top: self.rect.y.min,
            scale: 1.0,
            bounds: glyphon::TextBounds {
                left: 0,
                top: 0,
                right: self.rect.right(),
                bottom: self.rect.bottom(),
            },
            default_color: self.color.as_glyphon_color(),
            custom_glyphs: &[],
        };
        self.render.prepare(&device.device, &device.queue, &mut context.render.text.font_system, &mut context.render.text.atlas,
                            &context.viewport, vec![area], &mut context.render.text.cache).unwrap();
    }

    pub(crate) fn render<A>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        if let Some(update) = param.context.updates.remove(&self.id) {
            self.set_text(param.context, update.text());
        }
        self.prepare(param.device, param.context);
        self.render.render(&mut param.context.render.text.atlas, &param.context.viewport, pass).unwrap()
    }

    pub fn set_text(&mut self, context: &mut Context, text: impl AsRef<str>) {
        self.buffer.set_text(&mut context.render.text.font_system, text.as_ref(), &context.font.font_attr(), Shaping::Advanced);
        self.change = true;
    }

    pub fn set_wrap(&mut self, context: &mut Context, wrap: &TextWrap) {
        self.buffer.set_wrap(&mut context.render.text.font_system, wrap.as_gamma());
    }

    pub fn set_font_size(&mut self, context: &mut Context, text_size: &TextSize) {
        self.buffer.set_metrics(&mut context.render.text.font_system, glyphon::Metrics::new(text_size.font_size, text_size.line_height));
    }

    pub fn offset(&mut self, ox: f32, oy: f32) {
        self.rect.offset(ox, oy);
    }
}