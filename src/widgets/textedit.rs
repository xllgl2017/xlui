use crate::frame::context::{Context, ContextUpdate, UpdateType};
use crate::render::rectangle::param::RectParam;
use crate::render::WrcRender;
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::pos::Axis;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;
use std::any::Any;
use crate::size::radius::Radius;

struct TextChar {
    x: Axis,
    width: f32,
    char: char,
}

impl TextChar {
    pub fn new(c: char, xm: f32, w: f32) -> TextChar {
        TextChar {
            char: c,
            x: (xm..xm + w).into(),
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
    fn new() -> CharLayout {
        CharLayout {
            chars: vec![],
            font_size: 0.0,
            width: 0.0,
            x_min: 0.0,
            cursor: 0,
        }
    }
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

    fn remove_last(&mut self) -> f32 {
        //游标在最前端，无字符，不需要删除
        if self.cursor == 0 { return self.x_min; }
        let c = self.chars.remove(self.cursor - 1);
        self.width -= c.width;
        self.cursor -= 1;
        //将删除后面的字符进行位移
        self.chars[self.cursor..].iter_mut().for_each(|cc| cc.x -= c.width);
        c.x.min
    }

    fn remove_after(&mut self) {
        if self.cursor == self.chars.len() { return; }
        let c = self.chars.remove(self.cursor);
        self.width -= c.width;
        self.chars[self.cursor..].iter_mut().for_each(|cc| cc.x += c.width);
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
        self.chars[self.cursor..].iter_mut().for_each(|cc| cc.x += w);
        xm
    }
}

pub struct TextEdit {
    pub(crate) id: String,
    text_buffer: TextBuffer,
    size_mode: SizeMode,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui, String)>>,
    char_layout: CharLayout,

    fill_param: RectParam,
    fill_index: usize,
    fill_buffer: Option<wgpu::Buffer>,

    select_param: RectParam,
    select_index: usize,
    select_buffer: Option<wgpu::Buffer>,

    cursor_param: RectParam,
    cursor_index: usize,
    cursor_buffer: Option<wgpu::Buffer>,

    hovered: bool,
    pub(crate) focused: bool,
    mouse_press: bool,

    contact_ids: Vec<String>,
}

impl TextEdit {
    pub fn new(context: String) -> TextEdit {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::WHITE;
        fill_style.fill.hovered = Color::WHITE;
        fill_style.fill.clicked = Color::WHITE;
        fill_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = fill_style.border.hovered.clone();

        let mut select_style = ClickStyle::new();
        select_style.fill.inactive = Color::rgba(144, 209, 255, 100);
        select_style.fill.hovered = Color::rgba(144, 209, 255, 100);
        select_style.fill.clicked = Color::rgba(144, 209, 255, 100);
        select_style.border.inactive = Border::new(0.0).radius(Radius::same(0));
        select_style.border.hovered = Border::new(0.0).radius(Radius::same(0));
        select_style.border.clicked = Border::new(0.0).radius(Radius::same(0));

        let mut cursor_style = ClickStyle::new();
        cursor_style.fill.inactive = Color::rgb(0, 83, 125);
        cursor_style.fill.hovered = Color::rgb(0, 83, 125);
        cursor_style.fill.clicked = Color::rgb(0, 83, 125);
        cursor_style.border.inactive = Border::new(0.0).radius(Radius::same(0));
        cursor_style.border.hovered = Border::new(0.0).radius(Radius::same(0));
        cursor_style.border.clicked = Border::new(0.0).radius(Radius::same(0));
        TextEdit {
            id: crate::gen_unique_id(),
            text_buffer: TextBuffer::new(context),
            size_mode: SizeMode::Auto,
            callback: None,
            char_layout: CharLayout::new(),

            fill_param: RectParam::new(Rect::new(), fill_style),
            fill_index: 0,
            fill_buffer: None,

            select_param: RectParam::new(Rect::new(), select_style),
            select_index: 0,
            select_buffer: None,

            cursor_param: RectParam::new(Rect::new(), cursor_style),
            cursor_index: 0,
            cursor_buffer: None,
            hovered: false,
            focused: false,
            mouse_press: false,
            contact_ids: vec![],
        }
    }

    pub(crate) fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(context); //计算行高
        match self.size_mode {
            SizeMode::Auto => self.fill_param.rect.set_size(200.0, 25.0),
            SizeMode::FixWidth => self.fill_param.rect.set_height(25.0),
            SizeMode::FixHeight => self.fill_param.rect.set_width(200.0),
            SizeMode::Fix => {}
        }
        let mut rect = self.fill_param.rect.clone_add_padding(&Padding::same(3.0));
        rect.add_min_x(5.0);
        self.text_buffer.rect = rect;
    }

    pub(crate) fn set_rect(&mut self, rect: Rect) {
        self.fill_param.rect = rect;
        self.size_mode = SizeMode::Fix;
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, String)) -> Self {
        self.callback = Some(Callback::create_textedit(f));
        self
    }

    pub fn width_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn contact(mut self, id: impl ToString) -> Self {
        self.contact_ids.push(id.to_string());
        self
    }

    fn update_cursor(&mut self, ui: &mut Ui, xm: f32) {
        self.cursor_param.rect.offset_x_to(xm);
        let data = self.cursor_param.as_draw_param(false, false);
        ui.device.queue.write_buffer(self.cursor_buffer.as_ref().unwrap(), 0, data);
    }

    fn text_select(&mut self, ui: &mut Ui) {
        let lx = ui.device.device_input.mouse.lastest().x;
        let pos = self.char_layout.chars.iter().position(|tc| tc.x.min < lx && lx < tc.x.max);
        if let Some(pos) = pos {
            let ct = &self.char_layout.chars[pos];
            if lx > ui.device.device_input.mouse.pressed_pos.x { //向右选择
                self.select_param.rect.set_x_max(if lx >= ct.half_x() { ct.x.max } else { ct.x.min });
            } else { //向左选择
                self.select_param.rect.set_x_min(if lx >= ct.half_x() { ct.x.max } else { ct.x.min });
            }

            self.char_layout.reset_cursor(if lx >= ct.half_x() { pos + 1 } else { pos });
            let xm = if lx > ui.device.device_input.mouse.pressed_pos.x { self.select_param.rect.dx().max } else { self.select_param.rect.dx().min };
            self.update_cursor(ui, xm);
        }
        let data = self.select_param.as_draw_param(false, false);
        ui.device.queue.write_buffer(self.select_buffer.as_ref().unwrap(), 0, data);
        ui.context.window.request_redraw();
    }

    fn key_input(&mut self, key: Option<winit::keyboard::Key>, ui: &mut Ui) {
        match key.unwrap() {
            winit::keyboard::Key::Named(name) => {
                match name {
                    winit::keyboard::NamedKey::Backspace => {
                        let xm = self.char_layout.remove_last();
                        self.update_cursor(ui, xm);
                        let text = self.char_layout.text();
                        self.text_buffer.set_text(text, ui);
                    }
                    winit::keyboard::NamedKey::ArrowLeft => {
                        let xm = self.char_layout.cursor_reduce();
                        self.update_cursor(ui, xm);
                    }
                    winit::keyboard::NamedKey::ArrowRight => {
                        let xm = self.char_layout.cursor_add();
                        self.update_cursor(ui, xm);
                    }
                    winit::keyboard::NamedKey::Delete => {
                        self.char_layout.remove_after();
                        let text = self.char_layout.text();
                        self.text_buffer.set_text(text, ui);
                    }
                    winit::keyboard::NamedKey::Space => {
                        let xm = self.char_layout.push_char(' ', &ui.context);
                        self.update_cursor(ui, xm);
                        let text = self.char_layout.text();
                        self.text_buffer.set_text(text, ui);
                    }
                    _ => {}
                }
            }
            winit::keyboard::Key::Character(c) => {
                let c = c.chars().next().unwrap();
                let xm = self.char_layout.push_char(c, &ui.context);
                self.update_cursor(ui, xm);
                let text = self.char_layout.text();
                self.text_buffer.set_text(text, ui);
            }
            winit::keyboard::Key::Unidentified(_) => {}
            winit::keyboard::Key::Dead(_) => {}
        }
        if let Some(ref mut callback) = self.callback {
            let app = ui.app.take().unwrap();
            callback(*app, ui, self.char_layout.text());
            ui.app.replace(app);
        }
        ui.send_updates(&self.contact_ids, ContextUpdate::String(self.char_layout.text()));
        ui.context.window.request_redraw();
    }

    pub(crate) fn update_text(&mut self, ui: &mut Ui, text: String) {
        self.text_buffer.set_text(text, ui);
        let wx = self.text_buffer.rect.dx().min;
        self.char_layout = CharLayout::from_text(wx, &self.text_buffer.text, self.text_buffer.text_size.font_size, &ui.context);
        self.cursor_param.rect.offset_x_to(self.char_layout.x_min + self.char_layout.width);
        let data = self.cursor_param.as_draw_param(false, false);
        ui.device.queue.write_buffer(self.cursor_buffer.as_ref().unwrap(), 0, data);
    }

    pub(crate) fn text(&self) -> String {
        self.char_layout.text()
    }

    fn init(&mut self, ui: &mut Ui) {
        self.fill_param.rect = ui.layout().available_rect().clone_with_size(&self.fill_param.rect);
        self.reset_size(&ui.context);
        //背景
        let data = self.fill_param.as_draw_param(false, false);
        let fill_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.fill_index = ui.context.render.rectangle.create_bind_group(&ui.device, &fill_buffer);
        self.fill_buffer = Some(fill_buffer);
        //文本选择
        self.select_param.rect = self.text_buffer.rect.clone();
        self.select_param.rect.set_width(0.0);
        let data = self.select_param.as_draw_param(false, false);
        let select_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.select_index = ui.context.render.rectangle.create_bind_group(&ui.device, &select_buffer);
        self.select_buffer = Some(select_buffer);
        //字符管理
        let wx = self.text_buffer.rect.dx().min;
        self.char_layout = CharLayout::from_text(wx, &self.text_buffer.text, self.text_buffer.text_size.font_size, &ui.context);
        //游标
        self.cursor_param.rect = self.fill_param.rect.clone_add_padding(&Padding::same(5.0));
        self.cursor_param.rect.set_x_min(self.cursor_param.rect.dx().min - 2.0);
        self.cursor_param.rect.set_x_max(self.cursor_param.rect.dx().min + 2.0);
        self.cursor_param.rect.offset_x_to(self.char_layout.x_min + self.char_layout.width);
        let data = self.cursor_param.as_draw_param(false, false);
        let cursor_buffer = ui.context.render.rectangle.create_buffer(&ui.device, data);
        self.cursor_index = ui.context.render.rectangle.create_bind_group(&ui.device, &cursor_buffer);
        self.cursor_buffer = Some(cursor_buffer);
        //文本
        self.text_buffer.draw(ui);
    }
}


impl Widget for TextEdit {
    fn redraw(&mut self, ui: &mut Ui) -> Response {
        if self.fill_buffer.is_none() { self.init(ui); }
        let resp = Response::new(&self.id, &self.fill_param.rect);
        if ui.pass.is_none() { return resp; }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(self.fill_index, pass);
        self.text_buffer.redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        if self.focused { ui.context.render.rectangle.render(self.cursor_index, pass); }
        ui.context.render.rectangle.render(self.select_index, pass);
        resp
    }

    fn update(&mut self, ui: &mut Ui) {
        match ui.update_type {
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.fill_param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    let data = self.fill_param.as_draw_param(self.hovered || self.focused, false);
                    ui.device.queue.write_buffer(self.fill_buffer.as_ref().unwrap(), 0, data);
                    ui.context.window.request_redraw();
                }
                if self.focused && ui.device.device_input.mouse.pressed { self.text_select(ui); }
            }
            UpdateType::MousePress => {
                self.mouse_press = true;
                self.focused = ui.device.device_input.pressed_at(&self.fill_param.rect);
                if ui.device.device_input.pressed_at(&self.fill_param.rect) {
                    //鼠标按下
                    let x = ui.device.device_input.mouse.lastest().x;
                    if x < self.char_layout.x_min {
                        self.select_param.rect.set_x_min(self.char_layout.x_min);
                        self.select_param.rect.set_x_max(self.char_layout.x_min);
                        self.update_cursor(ui, self.char_layout.x_min);
                        self.char_layout.reset_cursor(0);
                        ui.context.window.request_redraw();
                    } else if x > self.char_layout.x_min + self.char_layout.width {
                        self.select_param.rect.set_x_min(self.char_layout.x_min + self.char_layout.width);
                        self.select_param.rect.set_x_max(self.char_layout.x_min + self.char_layout.width);
                        self.update_cursor(ui, self.char_layout.x_min + self.char_layout.width);
                        self.char_layout.reset_cursor(self.char_layout.chars.len());
                        ui.context.window.request_redraw();
                    } else {
                        let pos = self.char_layout.chars.iter().position(|tc| tc.x.min < x && x < tc.x.max);
                        if let Some(pos) = pos {
                            let ct = &self.char_layout.chars[pos];
                            self.select_param.rect.set_x_min(if x >= ct.half_x() { ct.x.max } else { ct.x.min });
                            self.select_param.rect.set_x_max(if x >= ct.half_x() { ct.x.max } else { ct.x.min });
                            self.char_layout.reset_cursor(if x >= ct.half_x() { pos + 1 } else { pos });
                            self.update_cursor(ui, self.select_param.rect.dx().min);
                            ui.context.window.request_redraw();
                        }
                    }
                }
                let data = self.select_param.as_draw_param(false, false);
                ui.device.queue.write_buffer(self.select_buffer.as_ref().unwrap(), 0, data);
            }
            UpdateType::MouseRelease => self.mouse_press = false,
            UpdateType::KeyRelease(ref mut key) => {
                if !self.focused || key.is_none() { return; }
                self.key_input(key.take(), ui)
            }
            _ => {}
        }
    }
}