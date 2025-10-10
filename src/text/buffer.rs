use crate::align::Align;
use crate::size::Geometry;
#[cfg(feature = "gpu")]
use crate::text::cchar::CChar;
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
use std::mem;
#[cfg(feature = "gpu")]
use wgpu::MultisampleState;

pub struct TextBuffer {
    pub(crate) text: RichText,
    pub(crate) geometry: Geometry,
    #[cfg(feature = "gpu")]
    pub(crate) buffer: Option<glyphon::Buffer>,
    #[cfg(feature = "gpu")]
    pub(crate) render: Option<glyphon::TextRenderer>,
    pub(crate) clip_x: f32,
    pub(crate) clip_y: f32,
    pub(crate) change: bool,
    // pub(crate) align: Align,
    pub(crate) lines: Vec<LineChar>,
}

impl TextBuffer {
    pub fn new(text: impl Into<RichText>) -> TextBuffer {
        TextBuffer {
            text: text.into(),
            geometry: Geometry::new(),
            #[cfg(feature = "gpu")]
            buffer: None,
            #[cfg(feature = "gpu")]
            render: None,
            clip_x: 0.0,
            clip_y: 0.0,
            change: false,
            // align: Align::LeftTop,
            lines: vec![],
        }
    }

    #[cfg(feature = "gpu")]
    fn reset(&mut self) {
        if self.buffer.is_none() { return; }
        self.text.width = 0.0;
        self.lines.clear();
        let buffer = self.buffer.as_ref().unwrap();
        for buffer_line in &buffer.lines {
            let mut line = LineChar::new();
            line.auto_wrap = false;
            for layout in buffer_line.layout_opt().unwrap() {
                for glyph in &layout.glyphs {
                    let cchar = buffer_line.text()[glyph.start..glyph.end].chars().next().unwrap();
                    if self.geometry.is_fix_width() && line.width + glyph.w >= self.geometry.width() && self.text.wrap.is_wrap() {
                        let mut line = mem::take(&mut line);
                        line.auto_wrap = true;
                        self.lines.push(line);
                    }
                    line.push(CChar::new(cchar, glyph.w));
                    self.text.width += glyph.w;
                }
            }
            self.lines.push(line);
        }
        if let Some(line) = self.lines.last_mut() { line.auto_wrap = true; }
    }

    #[cfg(not(feature = "gpu"))]
    fn reset(&mut self, ui: &mut Ui) {
        #[cfg(target_os = "windows")]
        { self.lines = ui.context.window.win32().measure_char_widths(&self.text).unwrap(); }
        #[cfg(target_os = "linux")]
        { self.lines = ui.context.window.x11().measure_char_widths(&self.text); }
        self.text.width = self.lines[0].width;
    }

    pub fn init(&mut self, ui: &mut Ui) {
        if self.text.size.is_none() { self.text.size = Some(ui.context.font.size()) }
        if self.text.family.is_none() { self.text.family = Some(ui.context.font.family().to_string()) }
        self.text.height = ui.context.font.line_height(self.text.font_size());

        #[cfg(not(feature = "gpu"))]
        self.reset(ui);
        #[cfg(feature = "gpu")]
        {
            let mut buffer = glyphon::Buffer::new(&mut ui.context.render.text.font_system, glyphon::Metrics::new(self.text.font_size(), self.text.height));
            buffer.set_wrap(&mut ui.context.render.text.font_system, self.text.wrap.as_gamma());
            buffer.set_text(&mut ui.context.render.text.font_system, &self.text.text, &self.text.font_family(), Shaping::Advanced);
            let render = glyphon::TextRenderer::new(&mut ui.context.render.text.atlas, &ui.device.device, MultisampleState {
                count: SAMPLE_COUNT,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }, None);
            self.render = Some(render);
            self.buffer = Some(buffer);
            self.reset();
        }
        self.geometry.set_size(self.text.width, self.text.height);
        #[cfg(feature = "gpu")]
        self.buffer.as_mut().unwrap().set_size(&mut ui.context.render.text.font_system, Some(self.geometry.width()), Some(self.geometry.height()));
        // let ox = self.geometry.width() - self.text.width - self.geometry.padding().horizontal();
        // let oy = self.geometry.height() - self.text.height - self.geometry.padding().vertical();
        // match self.align {
        //     //固定宽度
        //     Align::LeftCenter => {
        //         self.clip_x = 0.0;
        //         self.clip_y = oy / 2.0;
        //     }
        //     Align::Center => {
        //         self.clip_x = ox / 2.0;
        //         self.clip_y = oy / 2.0;
        //     }
        //     Align::RightCenter => {
        //         self.clip_x = ox;
        //         self.clip_y = oy / 2.0;
        //     }
        //     //固定高度
        //     Align::CenterTop => {
        //         self.clip_x = ox / 2.0;
        //         self.clip_y = 0.0;
        //     }
        //     Align::CenterBottom => {
        //         self.clip_x = ox / 2.0;
        //         self.clip_y = oy
        //     }
        //     //宽高固定
        //     Align::LeftTop => {
        //         self.clip_x = 0.0;
        //         self.clip_y = 0.0;
        //     }
        //     Align::LeftBottom => {
        //         self.clip_x = 0.0;
        //         self.clip_y = oy;
        //     }
        //     Align::RightTop => {
        //         self.clip_x = ox;
        //         self.clip_y = 0.0;
        //     }
        //     Align::RightBottom => {
        //         self.clip_x = ox;
        //         self.clip_y = oy;
        //     }
        // }
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
            buffer: self.buffer.as_ref().unwrap(),
            left: self.geometry.x() + self.clip_x,
            top: self.geometry.y() + self.clip_y,
            scale: 1.0,
            bounds,
            default_color: self.text.color.as_glyphon_color(),
            custom_glyphs: &[],
        };
        self.render.as_mut().unwrap().prepare(
            &ui.device.device, &ui.device.queue,
            &mut ui.context.render.text.font_system,
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

    #[cfg(all(target_os = "linux", not(feature = "gpu")))]
    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        let param = &mut ui.paint.as_mut().unwrap();
        ui.context.window.x11().paint_text(param, &self.text, self.geometry.rect()).unwrap();
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
        #[cfg(feature = "gpu")]
        match self.buffer {
            None => self.set_text(text.to_string()),
            Some(_) => {
                self.change = self.text.text != text;
                self.text.text = text.to_string();
                self.update_buffer(ui);
            }
        }
        #[cfg(not(feature = "gpu"))]
        {
            self.change = self.text.text != text;
            self.text.text = text.to_string();
            self.update_buffer(ui);
        }
    }

    pub fn update_if_not(&mut self, ui: &mut Ui, text: &str, reset: bool) {
        #[cfg(feature = "gpu")]
        self.buffer.as_mut().unwrap().set_text(&mut ui.context.render.text.font_system, text, &self.text.font_family(), Shaping::Advanced);
        self.text.text = text.to_string();
        if reset {
            #[cfg(feature = "gpu")]
            self.reset();
            #[cfg(all(windows, not(feature = "gpu")))]
            self.reset(ui);
        }
    }

    pub fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.change { return; }
        self.change = false;
        #[cfg(feature = "gpu")]
        self.buffer.as_mut().unwrap().set_text(
            &mut ui.context.render.text.font_system, self.text.text.as_str(),
            &self.text.font_family(), Shaping::Advanced);
        #[cfg(feature = "gpu")]
        self.reset();
        #[cfg(all(windows, not(feature = "gpu")))]
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