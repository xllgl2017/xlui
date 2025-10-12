use crate::align::Align;
use crate::size::Geometry;
use crate::text::cchar::LineChar;
use crate::text::rich::RichText;
use crate::text::TextWrap;
use crate::ui::Ui;
use crate::Padding;
#[cfg(feature = "gpu")]
use crate::SAMPLE_COUNT;
#[cfg(feature = "gpu")]
use glyphon::Shaping;
#[cfg(feature = "gpu")]
use wgpu::MultisampleState;
use crate::error::UiResult;

pub struct TextBuffer {
    pub(crate) text: RichText,
    pub(crate) geometry: Geometry,
    #[cfg(feature = "gpu")]
    pub(crate) buffer: glyphon::Buffer,
    #[cfg(feature = "gpu")]
    pub(crate) render: Option<glyphon::TextRenderer>,
    pub(crate) clip_x: f32,
    pub(crate) clip_y: f32,
    pub(crate) change: bool,
    pub(crate) lines: Vec<LineChar>,
}

impl TextBuffer {
    pub fn new(text: impl Into<RichText>) -> TextBuffer {
        TextBuffer {
            text: text.into(),
            geometry: Geometry::new(),
            #[cfg(feature = "gpu")]
            buffer: glyphon::Buffer::new_empty(glyphon::Metrics::new(14.0, 10.0)),
            #[cfg(feature = "gpu")]
            render: None,
            clip_x: 0.0,
            clip_y: 0.0,
            change: false,
            lines: vec![],
        }
    }

    #[cfg(feature = "gpu")]
    fn reset(&mut self, ui: &mut Ui) {
        let wrap = self.geometry.is_fix_width() && self.text.wrap.is_wrap();
        self.lines = ui.context.font.measure_text(&self.buffer, wrap, self.geometry.width());
        self.text.width = self.lines.iter().map(|x| x.width).reduce(f32::max).unwrap_or(self.geometry.width());
    }

    #[cfg(not(feature = "gpu"))]
    fn reset(&mut self, ui: &mut Ui) {
        self.lines=ui.context.font.measure_text(&ui.context.window,&self.text).unwrap();
        // #[cfg(target_os = "windows")]
        // { self.lines = ui.context.window.win32().measure_char_widths(&self.text).unwrap(); }
        // #[cfg(target_os = "linux")]
        // { self.lines = ui.context.window.x11().measure_text(&self.text).unwrap(); }
        // self.text.width = self.lines[0].width;
        self.text.width = self.lines.iter().map(|x| x.width).reduce(f32::max).unwrap_or(self.geometry.width());
    }

    pub fn init(&mut self, ui: &mut Ui) {
        self.text.height = self.line_height(ui).unwrap();
        #[cfg(not(feature = "gpu"))]
        self.reset(ui);

        #[cfg(feature = "gpu")]
        {
            let font_system = ui.context.font.system_mut();
            self.buffer.set_metrics(font_system, glyphon::Metrics::new(self.text.font_size(), self.text.height));
            self.buffer.set_wrap(font_system, self.text.wrap.as_gamma());
            self.buffer.set_text(font_system, &self.text.text, &self.text.font_family(), Shaping::Advanced);
            let render = glyphon::TextRenderer::new(&mut ui.context.render.text.atlas, &ui.device.device, MultisampleState {
                count: SAMPLE_COUNT,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }, None);
            self.render = Some(render);
            // self.buffer = Some(buffer);
            self.reset(ui);
        }
        self.geometry.set_size(self.text.width, self.text.height);
        #[cfg(feature = "gpu")]
        self.buffer.set_size(ui.context.font.system_mut(), Some(self.geometry.width()), Some(self.geometry.height()));
    }

    #[cfg(feature = "gpu")]
    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        let bounds = glyphon::TextBounds {
            left: self.geometry.x_i32(),
            top: 0,
            right: self.geometry.right_i32(),
            bottom: self.geometry.bottom_i32(),
        };
        let area = glyphon::TextArea {
            buffer: &self.buffer,
            left: self.geometry.x() + self.clip_x,
            top: self.geometry.y() + self.clip_y,
            scale: 1.0,
            bounds,
            default_color: self.text.color.as_glyphon_color(),
            custom_glyphs: &[],
        };
        self.render.as_mut().unwrap().prepare(
            &ui.device.device, &ui.device.queue,
            ui.context.font.system_mut(),
            &mut ui.context.render.text.atlas,
            &ui.context.viewport, vec![area],
            &mut ui.context.render.text.cache).unwrap();
        let pass = ui.pass.as_mut().unwrap();
        self.render.as_mut().unwrap().render(&mut ui.context.render.text.atlas, &ui.context.viewport, pass).unwrap()
    }

    #[cfg(all(windows, not(feature = "gpu")))]
    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        let hdc = ui.paint.as_mut().unwrap().hdc;
        ui.context.window.win32().paint_text(hdc, &self.text, self.geometry.rect());
    }

    pub(crate) fn line_height(&mut self, ui: &mut Ui) -> UiResult<f32> {
        #[cfg(not(feature = "gpu"))]
        return ui.context.font.line_height(&ui.context.window, &mut self.text);
        #[cfg(feature = "gpu")]
        return ui.context.font.line_height(&mut self.text);
    }

    #[cfg(all(target_os = "linux", not(feature = "gpu")))]
    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        let param = &mut ui.paint.as_mut().unwrap();
        ui.context.window.x11().paint_text(param, &self.text, &self.lines, self.geometry.rect(), self.clip_x, self.clip_y).unwrap();
    }

    pub fn set_text(&mut self, text: String) {
        self.change = self.text.text != text;
        self.text.text = text;
    }

    pub fn with_align(mut self, align: Align) -> Self {
        self.geometry.set_align(align);
        self
    }

    pub fn update_buffer_text(&mut self, ui: &mut Ui, text: &str) {
        self.change = self.text.text != text;
        self.text.text = text.to_string();
        self.update_buffer(ui);
    }

    pub fn update_if_not(&mut self, ui: &mut Ui, text: &str, reset: bool) {
        #[cfg(feature = "gpu")]
        self.buffer.set_text(ui.context.font.system_mut(), text, &self.text.font_family(), Shaping::Advanced);
        self.text.text = text.to_string();
        if reset {
            #[cfg(feature = "gpu")]
            self.reset(ui);
            #[cfg(not(feature = "gpu"))]
            self.reset(ui);
        }
    }

    pub fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.change { return; }
        self.change = false;
        #[cfg(feature = "gpu")]
        self.buffer.set_text(
            ui.context.font.system_mut(), self.text.text.as_str(),
            &self.text.font_family(), Shaping::Advanced);
        #[cfg(feature = "gpu")]
        self.reset(ui);
        #[cfg(not(feature = "gpu"))]
        self.reset(ui);
        self.geometry.set_width(self.lines[0].width)
    }

    pub fn set_wrap(&mut self, wrap: TextWrap) {
        self.change = self.text.wrap == wrap;
        self.text.wrap = wrap;
    }

    // pub fn height(mut self, height: f32) -> Self {
    //     self.geometry.set_height(height);
    //     self
    // }

    pub fn fix_width(mut self, w: f32) -> Self {
        self.geometry.set_fix_width(w);
        self
    }

    pub fn fix_height(mut self, h: f32) -> Self {
        self.geometry.set_fix_height(h);
        self
    }

    pub fn padding(mut self, padding: Padding) -> Self {
        self.geometry.set_padding(padding);
        self
    }

    pub fn min_width(mut self, w: f32) -> Self {
        self.geometry.set_min_width(w);
        self
    }
}