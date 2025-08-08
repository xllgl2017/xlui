use std::any::Any;
use crate::frame::context::Context;
use crate::paint::rectangle::PaintRectangle;
use crate::paint::text::PaintText;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::ui::{DrawParam, Ui};
use crate::widgets::textedit::TextEdit;
use crate::{Device, Pos};
use crate::frame::App;
use crate::paint::color::Color;
use crate::radius::Radius;
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::size::border::Border;

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


struct CharLayout {
    chars: Vec<TextChar>,
    font_size: f32,
    width: f32,
    x_min: f32,
    cursor: usize, //游标位置，范围[0..=chars.len()]
}

impl CharLayout {
    fn from_text(x_min: f32, txt: &str, font_size: f32, context: &Context) -> CharLayout {
        let mut chars = vec![];
        let mut wx = x_min;
        let mut width = 0.0;
        for char in txt.chars() {
            let w = context.font.char_width(char, font_size);
            chars.push(TextChar::new(char, wx, w));
            wx += w;
            width += w;
        }
        CharLayout {
            //将游标设置为最后一个字符后面
            cursor: chars.len(),
            chars,
            font_size,
            width,
            x_min,
        }
    }

    fn reset_cursor(&mut self, index: usize) {
        self.cursor = index;
    }

    fn cursor_add(&mut self) -> f32 {
        if self.cursor >= self.chars.len() {
            self.cursor = self.chars.len();
            self.x_min + self.width
        } else {
            self.cursor += 1;
            self.chars[self.cursor - 1].x.max
        }
    }

    fn cursor_reduce(&mut self) -> f32 {
        if self.cursor == 0 { return self.x_min; }
        self.cursor -= 1;
        if self.cursor == 0 { return self.x_min; }
        self.chars[self.cursor - 1].x.max
    }

    fn text(&self) -> String {
        self.chars.iter().map(|c| c.char.to_string()).collect()
    }

    fn remove_char(&mut self) -> f32 {
        //游标在最前端，无字符，不需要删除
        if self.cursor == 0 { return self.x_min; }
        let c = self.chars.remove(self.cursor - 1);
        self.width -= c.width;
        self.cursor -= 1;
        //将删除后面的字符进行位移
        self.chars[self.cursor..].iter_mut().for_each(|cc| cc.x.offset(-c.width));
        c.x.min
    }

    fn remove_after(&mut self) {
        if self.cursor == self.chars.len() { return; }
        let c = self.chars.remove(self.cursor);
        self.width -= c.width;
        self.chars[self.cursor..].iter_mut().for_each(|cc| cc.x.offset(c.width));
    }

    fn current_char(&self) -> Option<&TextChar> {
        if self.cursor == 0 { return None; }
        Some(&self.chars[self.cursor - 1])
    }

    fn push_char(&mut self, c: char, context: &Context) -> f32 { //返回x最大值 ，给游标偏移
        let w = context.font.char_width(c, self.font_size);
        let cx = if let Some(c) = self.current_char() {
            c.x.max
        } else { self.x_min };
        let c = TextChar::new(c, cx, w);
        let xm = c.x.max;
        self.chars.insert(self.cursor, c);
        self.cursor += 1;
        self.width += w;
        self.chars[self.cursor..].iter_mut().for_each(|cc| cc.x.offset(w));
        xm
    }
}


pub(crate) struct PaintTextEdit {
    id: String,
    text: PaintText,
    pub fill: PaintRectangle,
    cursor_index: usize,
    cursor_param: RectParam,
    cursor_buffer: wgpu::Buffer,
    select: PaintRectangle,
    char_layout: CharLayout,
    has_select: bool,
    hovered: bool,
    pub(crate) focused: bool,
    mouse_down_x: f32,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, String)>>,
}

impl PaintTextEdit {
    pub fn new(ui: &mut Ui, edit: &mut TextEdit) -> Self {
        let wx = edit.text_buffer.rect.x.min;
        let char_layout = CharLayout::from_text(wx, &edit.text_buffer.text, edit.text_buffer.text_size.font_size, &ui.ui_manage.context);

        let mut cursor_rect = edit.rect.clone_add_padding(&Padding::same(5.0));
        cursor_rect.x.min = cursor_rect.x.min - 2.0;
        cursor_rect.x.max = cursor_rect.x.min + 2.0;
        cursor_rect.offset_x_to(char_layout.x_min + char_layout.width);

        let mut cursor_style = ui.style.widget.click.clone();
        cursor_style.fill.inactive = Color::rgb(0, 83, 125);
        cursor_style.fill.hovered = Color::rgb(0, 83, 125);
        cursor_style.fill.clicked = Color::rgb(0, 83, 125);
        cursor_style.border.inactive = Border::new(0.0).radius(Radius::same(0));
        cursor_style.border.hovered = Border::new(0.0).radius(Radius::same(0));
        cursor_style.border.clicked = Border::new(0.0).radius(Radius::same(0));
        let mut cursor_param = RectParam::new(cursor_rect, cursor_style);
        let data = cursor_param.as_draw_param(false, false);
        let cursor_buffer = ui.ui_manage.context.render.rectangle.create_buffer(&ui.device, data);
        let cursor_index = ui.ui_manage.context.render.rectangle.create_bind_group(&ui.device, &cursor_buffer);


        let mut select_rect = edit.text_buffer.rect.clone();
        select_rect.set_width(0.0);

        PaintTextEdit {
            id: edit.id.clone(),
            text: PaintText::new(ui, &edit.text_buffer),
            fill: PaintRectangle::new(ui, edit.rect.clone()),
            cursor_index,
            cursor_param,
            cursor_buffer,
            select: PaintRectangle::new(ui, select_rect),
            char_layout,
            has_select: false,
            hovered: false,
            focused: false,
            mouse_down_x: 0.0,
            callback: edit.callback.take(),
        }
    }

    pub fn fill_style(&mut self, style: ClickStyle) {
        self.fill.set_style(style);
    }

    pub fn select_style(&mut self, style: ClickStyle) {
        self.select.set_style(style);
    }


    pub fn mouse_move(&mut self, device: &Device, context: &Context) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.fill.param.rect.has_position(x, y);
        if self.hovered != has_pos || device.device_input.mouse.pressed != device.device_input.mouse.pressed {
            self.fill.prepare(device, has_pos || self.focused, device.device_input.mouse.pressed);
            self.hovered = has_pos;
            context.window.request_redraw();
        }
        if self.focused && device.device_input.mouse.pressed {
            self.text_select(device);
            context.window.request_redraw();
        }
    }

    fn update_cursor(&mut self, device: &Device, xm: f32) {
        self.cursor_param.rect.offset_x_to(xm);
        let data = self.cursor_param.as_draw_param(false, false);
        device.queue.write_buffer(&self.cursor_buffer, 0, data);
    }

    pub fn mouse_down(&mut self, device: &Device, context: &Context) {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.fill.param.rect.has_position(x, y);
        self.focused = has_pos;
        if !self.fill.param.rect.has_position(x, y) { return; }
        self.has_select = false;
        self.select.rect_mut().set_width(0.0);
        if x < self.char_layout.x_min {
            self.select.param.rect.x.min = self.char_layout.x_min;
            self.select.param.rect.x.max = self.char_layout.x_min;
            self.update_cursor(device, self.char_layout.x_min);
            self.char_layout.reset_cursor(0);
            context.window.request_redraw();
        } else if x > self.char_layout.x_min + self.char_layout.width {
            self.select.param.rect.x.min = self.char_layout.x_min + self.char_layout.width;
            self.select.param.rect.x.max = self.char_layout.x_min + self.char_layout.width;
            self.update_cursor(device, self.char_layout.x_min + self.char_layout.width);
            self.char_layout.reset_cursor(self.char_layout.chars.len());
            context.window.request_redraw();
        } else {
            let pos = self.char_layout.chars.iter().position(|tc| tc.x.min < x && x < tc.x.max);
            if let Some(pos) = pos {
                let ct = &self.char_layout.chars[pos];
                self.select.param.rect.x.min = if x >= ct.half_x() { ct.x.max } else { ct.x.min };
                self.select.param.rect.x.max = if x >= ct.half_x() { ct.x.max } else { ct.x.min };
                self.char_layout.reset_cursor(if x >= ct.half_x() { pos + 1 } else { pos });
                self.update_cursor(device, self.select.param.rect.x.min);
                context.window.request_redraw();
            }
        }
        self.mouse_down_x = x;
    }

    pub fn text_select(&mut self, device: &Device) {
        let lx = device.device_input.mouse.lastest().0;
        self.has_select = true;
        let pos = self.char_layout.chars.iter().position(|tc| tc.x.min < lx && lx < tc.x.max);
        if let Some(pos) = pos {
            let ct = &self.char_layout.chars[pos];
            if lx > self.mouse_down_x { //向右选择
                self.select.param.rect.x.max = if lx >= ct.half_x() { ct.x.max } else { ct.x.min };
            } else { //向左选择
                self.select.param.rect.x.min = if lx >= ct.half_x() { ct.x.max } else { ct.x.min };
            }

            self.char_layout.reset_cursor(if lx >= ct.half_x() { pos + 1 } else { pos });
            self.update_cursor(device, if lx > self.mouse_down_x { self.select.param.rect.x.max } else { self.select.param.rect.x.min });
        }
        self.select.prepare(device, false, false)
    }


    pub fn render<A>(&mut self, param: &mut DrawParam<A>, pass: &mut wgpu::RenderPass) {
        self.fill.render(param, pass);
        self.text.render(param, pass);
        if self.focused { param.context.render.rectangle.render(self.cursor_index, pass); }
        if self.has_select { self.select.render(param, pass); }
    }

    pub fn text(&self) -> String {
        self.char_layout.text()
    }

    pub fn set_text(&mut self, text: &str, context: &mut Context) {
        self.char_layout = CharLayout::from_text(self.char_layout.x_min, text, self.char_layout.font_size, context);
        self.text.set_text(context, text);
    }

    pub fn rect(&self) -> &Rect {
        &self.fill.param.rect
    }

    pub fn key_input<A: App>(&mut self, device: &Device, context: &mut Context, c: winit::keyboard::Key, app: &mut A) {
        if !self.focused { return; }
        match c {
            winit::keyboard::Key::Named(name) => {
                println!("{:?}", name);
                match name {
                    winit::keyboard::NamedKey::Backspace => {
                        let xm = self.char_layout.remove_char();
                        self.update_cursor(device, xm);
                        let text = self.char_layout.text();
                        self.text.set_text(context, text);
                    }
                    winit::keyboard::NamedKey::ArrowLeft => {
                        let xm = self.char_layout.cursor_reduce();
                        self.update_cursor(device, xm);
                    }
                    winit::keyboard::NamedKey::ArrowRight => {
                        let xm = self.char_layout.cursor_add();
                        self.update_cursor(device, xm);
                    }
                    winit::keyboard::NamedKey::Delete => {
                        self.char_layout.remove_after();
                        let text = self.char_layout.text();
                        self.text.set_text(context, text);
                    }
                    winit::keyboard::NamedKey::Space => {
                        let xm = self.char_layout.push_char(' ', context);
                        self.update_cursor(device, xm);
                        let text = self.char_layout.text();
                        self.text.set_text(context, text);
                    }
                    _ => {}
                }
            }
            winit::keyboard::Key::Character(c) => {
                let c = c.chars().next().unwrap();
                let xm = self.char_layout.push_char(c, context);
                self.update_cursor(device, xm);
                let text = self.char_layout.text();
                self.text.set_text(context, text);
            }
            winit::keyboard::Key::Unidentified(_) => {}
            winit::keyboard::Key::Dead(_) => {}
        }
        if let Some(ref mut callback) = self.callback {
            callback(app, context, self.char_layout.text());
        }
    }
}