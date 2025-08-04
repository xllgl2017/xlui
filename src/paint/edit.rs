use crate::frame::context::Context;
use crate::paint::rectangle::PaintRectangle;
use crate::paint::text::PaintText;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::{Device, Pos};

struct TextChar {
    x: Pos,
    width: f32,
    char: char,
}

impl TextChar {
    pub fn new(c: char, xm: f32, w: f32) -> TextChar {
        TextChar {
            char: c,
            x: Pos { min: xm, max: xm + w },
            width: w,
        }
    }

    pub fn half_x(&self) -> f32 {
        self.x.min + self.width / 2.0
    }
}

pub(crate) struct PaintTextEdit {
    text: PaintText,
    pub fill: PaintRectangle,
    cursor: PaintRectangle,
    select: PaintRectangle,
    chars: Vec<TextChar>,
    min_x: f32,
    max_x: f32,
    has_select: bool,
    current_char: usize, //统一计算x-max
    hovered: bool,
    pub(crate) focused: bool,
}

impl PaintTextEdit {
    pub fn new(ui: &mut Ui, rect: Rect, buffer: &TextBuffer) -> Self {
        let mut cursor_rect = rect.clone_add_padding(&Padding::same(5.0));
        cursor_rect.x.min = cursor_rect.x.min - 2.0;
        cursor_rect.x.max = cursor_rect.x.min + 2.0;
        let mut chars = vec![];
        let mut wx = buffer.rect.x.min;
        for char in buffer.text.chars() {
            let w = ui.ui_manage.context.font.char_width(char, buffer.text_size.font_size);
            chars.push(TextChar::new(char, wx, w));
            wx += w;
        }
        let mut select_rect = buffer.rect.clone();
        select_rect.set_width(0.0);
        PaintTextEdit {
            text: PaintText::new(ui, buffer),
            fill: PaintRectangle::new(ui, rect),
            cursor: PaintRectangle::new(ui, cursor_rect),
            select: PaintRectangle::new(ui, select_rect),
            chars,
            min_x: buffer.rect.x.min,
            max_x: wx,
            has_select: false,
            current_char: buffer.text.chars().count() - 1,
            hovered: false,
            focused: false,
        }
    }

    pub fn fill_style(&mut self, style: ClickStyle) {
        self.fill.set_style(style);
    }

    pub fn cursor_style(&mut self, style: ClickStyle) {
        self.cursor.set_style(style);
    }

    pub fn select_style(&mut self, style: ClickStyle) {
        self.select.set_style(style);
    }

    // pub fn prepare(&mut self, device: &Device, context: &mut Context, hovered: bool, mouse_down: bool, focused: bool) {
    //     self.fill.prepare(device, hovered || focused, mouse_down);
    //     self.text.prepare(device, context);
    //     if focused { self.cursor.prepare(device, hovered, mouse_down); }
    //
    //     if self.has_select { self.select.prepare(device, hovered, mouse_down); }
    // }


    pub fn mouse_move(&mut self, device: &Device, context: &Context) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.fill.param.rect.has_position(x, y);
        // println!("{} {} {} {} {}", self.focused, self.hovered, self.mouse_down, has_pos, resp.mouse_down);
        // println!("{} {} {}", has_pos, self.mouse_down, self.focused);
        if self.hovered != has_pos || device.device_input.mouse.pressed != device.device_input.mouse.pressed {
            self.fill.prepare(device, has_pos || self.focused, device.device_input.mouse.pressed);
            self.hovered = has_pos;
            // device.device_input.mouse.request_cursor(Cursor::Icon(CursorIcon::Text))
            // context.window.set_cursor(Cursor::Icon(if has_pos || self.mouse_down { CursorIcon::Text } else { CursorIcon::Default }));
            context.window.request_redraw();
        }
        if self.focused && device.device_input.mouse.pressed {
            // context.window.set_cursor(Cursor::Icon(CursorIcon::Text));
            self.text_select(device);
            context.window.request_redraw();
        }
    }

    pub fn mouse_down(&mut self, device: &Device, context: &Context) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.fill.param.rect.has_position(x, y);
        if self.focused != has_pos || device.device_input.mouse.pressed != has_pos {
            self.focused = has_pos;
            // println!("down {} {} {}", has_pos, self.focused, self.mouse_down);
            self.fill.prepare(device, has_pos || self.focused, device.device_input.mouse.pressed);
            context.window.request_redraw();
        }
    }

    pub fn click(&mut self, device: &Device, context: &Context) {
        self.has_select = false;
        self.select.rect_mut().set_width(0.0);
        let lx = device.device_input.mouse.lastest.0;
        if lx < self.min_x {
            self.cursor.offset(self.min_x);
            self.current_char = 0;
        } else if lx > self.max_x {
            self.cursor.offset(self.max_x);
            self.current_char = if self.chars.len() != 0 { self.chars.len() - 1 } else { 0 };
        } else {
            let pos = self.chars.iter().position(|x| x.x.min < lx && lx < x.x.max);
            if let Some(pos) = pos {
                let ct = &self.chars[pos];
                self.cursor.offset(if lx >= ct.half_x() { ct.x.max } else { ct.x.min });
                self.current_char = pos;
            }
        }
        self.cursor.prepare(device, true, false);
        context.window.request_redraw();
    }

    pub fn text_select(&mut self, device: &Device) {
        self.has_select = true;
        let rect = self.select.rect_mut();
        rect.x.max = rect.x.max + device.device_input.mouse.offset_x();
        if rect.x.max > self.max_x { rect.x.max = self.max_x; }
        self.select.prepare(device, false, false)
    }


    pub fn render(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        self.fill.render(&context.render, render_pass);
        self.text.render(device, context, render_pass);
        if self.focused { self.cursor.render(&context.render, render_pass); }
        if self.has_select { self.select.render(&context.render, render_pass); }
    }

    pub fn text(&self) -> String {
        self.chars.iter().map(|c| c.char.to_string()).collect()
    }

    pub fn set_text(&mut self, text: &str, context: &mut Context) {
        self.current_char = 0;
        self.chars.clear();
        self.text.set_text(context, text);
        for c in text.chars() {
            let w = context.font.char_width(c, 14.0);
            let ct = TextChar::new(c, self.min_x, w);
            self.max_x = ct.x.max;
            self.chars.push(ct);
        }
    }

    pub fn rect(&self) -> &Rect {
        &self.fill.param.rect
    }

    pub fn key_input(&mut self, device: &Device, context: &mut Context, c: winit::keyboard::Key) {
        if !self.focused { return; }
        match c {
            winit::keyboard::Key::Named(name) => {
                match name {
                    winit::keyboard::NamedKey::Backspace => {
                        if self.chars.len() == 0 { return; }
                        let ct = self.chars.remove(self.current_char);
                        if self.current_char == 0 {
                            self.cursor.offset(self.min_x)
                        } else {
                            self.cursor.offset(ct.x.min);
                            self.current_char -= 1;
                        }
                        self.max_x = ct.x.min;
                        let text: String = self.chars.iter().map(|x| x.char.to_string()).collect();
                        self.text.set_text(context, text.as_str());
                    }
                    winit::keyboard::NamedKey::ArrowLeft => {
                        if self.chars.len() == 0 { return; }
                        if self.current_char == 0 {
                            self.cursor.offset(self.min_x);
                        } else {
                            self.current_char -= 1;
                            self.cursor.offset(self.chars[self.current_char].x.max);
                        }
                    }
                    winit::keyboard::NamedKey::ArrowRight => {
                        if self.chars.len() == 0 { return; }
                        if self.current_char == self.chars.len() - 1 {
                            self.cursor.offset(self.max_x);
                        } else {
                            self.current_char += 1;
                            self.cursor.offset(self.chars[self.current_char].x.max);
                        }
                    }
                    _ => {}
                }
            }
            winit::keyboard::Key::Character(c) => {
                let c = c.chars().next().unwrap();
                let w = context.font.char_width(c, 14.0);
                let ct = match self.chars.last() {
                    None => TextChar::new(c, self.min_x, w),
                    Some(ct) => TextChar::new(c, ct.x.max, w)
                };
                if self.chars.len() != 0 { self.current_char += 1; }
                self.max_x = ct.x.max;
                self.cursor.offset(ct.x.max);
                self.chars.push(ct);
                let text: String = self.chars.iter().map(|x| x.char.to_string()).collect();
                self.text.set_text(context, text.as_str());
            }
            winit::keyboard::Key::Unidentified(_) => {}
            winit::keyboard::Key::Dead(_) => {}
        }
        self.cursor.prepare(device, false, false);
    }
}