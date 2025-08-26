//! ### 文本编辑器（单行）
//! ```
//! use xlui::frame::App;
//! use xlui::ui::Ui;
//! use xlui::widgets::textedit::single::SingleEdit;
//!
//! fn edit_changed<A:App>(_:&mut A,_:&mut Ui,text:String){
//!     println!("文本变动:{}",text);
//! }
//!
//!
//! fn draw<A:App>(ui:&mut Ui){
//!    let edit=SingleEdit::new("".to_string())
//!         //连接文本变动监听函数
//!         .connect(edit_changed::<A>)
//!         //关联文本，当文本变动时修改id为my_label的Label
//!         .contact("my_label")
//!         //设置Widget的ID
//!         .width_id("my_edit");
//!     ui.add(edit);
//! }
//! ```

use crate::frame::context::{Context, ContextUpdate, UpdateType};
use crate::frame::App;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::pos::Axis;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::Widget;
use std::ops::Range;
#[deprecated]
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


#[deprecated]
struct CharLayout {
    chars: Vec<TextChar>,
    font_size: f32,
    width: f32,
    x_min: f32,
    cursor: usize, //游标位置，范围[0..=chars.len()]
    selected: Range<usize>,
    offset: f32,
}


impl CharLayout {
    fn new() -> CharLayout {
        CharLayout {
            chars: vec![],
            font_size: 0.0,
            width: 0.0,
            x_min: 0.0,
            cursor: 0,
            selected: 0..0,
            offset: 0.0,
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
            selected: 0..0,
            offset: 0.0,
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

    fn remove_range(&mut self) -> f32 {
        let c = self.chars.remove(self.selected.start);
        let mut w = c.width;
        self.chars[self.selected.start..].iter_mut().for_each(|cc| cc.x -= c.width);
        for _ in self.selected.start + 1..self.selected.end {
            let c = self.chars.remove(self.selected.start);
            w += c.width;
        }
        self.width -= w;
        self.offset += w;
        if self.offset > 0.0 { self.offset = 0.0 }
        self.chars[self.selected.start..].iter_mut().for_each(|cc| cc.x -= w);
        self.reset_cursor(self.selected.start);
        self.selected.end = self.selected.start;
        c.x.min + self.offset
    }

    fn remove_char_in_cursor(&mut self) -> f32 {
        if self.selected.start == self.selected.end {
            //游标在最前端，无字符，不需要删除
            if self.cursor == 0 { return self.x_min; }
            let c = self.chars.remove(self.cursor - 1);
            self.width -= c.width;
            self.cursor -= 1;
            self.offset += c.width;
            if self.offset > 0.0 { self.offset = 0.0 }
            //将删除后面的字符进行位移
            self.chars[self.cursor..].iter_mut().for_each(|cc| cc.x -= c.width);
            c.x.min + self.offset
        } else {
            self.remove_range()
        }
    }

    fn remove_after(&mut self) -> f32 {
        if self.selected.start == self.selected.end {
            if self.cursor == self.chars.len() { return self.x_min + self.width; }
            let c = self.chars.remove(self.cursor);
            self.width -= c.width;
            self.offset += c.width;
            if self.offset > 0.0 { self.offset = 0.0 }
            self.chars[self.cursor..].iter_mut().for_each(|cc| cc.x -= c.width);
            c.x.max + self.offset
        } else {
            self.remove_range()
        }
    }

    fn current_char(&self) -> Option<&TextChar> {
        if self.cursor == 0 { return None; }
        Some(&self.chars[self.cursor - 1])
    }

    fn next_char(&self) -> Option<&TextChar> {
        if self.cursor >= self.chars.len() { return None; }
        Some(&self.chars[self.cursor])
    }

    fn previous_char(&self) -> Option<&TextChar> {
        if self.cursor == 0 { return None; }
        if self.cursor - 1 == 0 { return Some(&self.chars[0]); }
        Some(&self.chars[self.cursor - 2])
    }

    fn push_char(&mut self, c: char, context: &Context, lx: f32) -> f32 { //返回x最大值 ，给游标偏移
        if self.selected.start != self.selected.end {
            self.remove_range();
        }
        let w = context.font.char_width(c, self.font_size);
        let cx = if let Some(c) = self.current_char() {
            c.x.max
        } else { self.x_min };
        let c = TextChar::new(c, cx, w);
        let xm = c.x.max;
        if c.x.max + self.offset > lx {
            self.offset -= c.width;
        }
        self.chars.insert(self.cursor, c);
        self.cursor += 1;
        self.width += w;
        self.chars[self.cursor..].iter_mut().for_each(|cc| cc.x += w);
        xm + self.offset
    }
}

#[deprecated = "use MultiEdit::new("".to_string()).with_rows(1)"]
pub struct SingleEdit {
    pub(crate) id: String,
    text_buffer: TextBuffer,
    size_mode: SizeMode,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, String)>>,
    char_layout: CharLayout,

    fill_render: RenderParam<RectParam>,
    select_render: RenderParam<RectParam>,
    cursor_render: RenderParam<RectParam>,

    hovered: bool,
    pub(crate) focused: bool,
    mouse_press: bool,

    contact_ids: Vec<String>,
    changed: bool,
}

impl SingleEdit {
    pub fn new(context: String) -> SingleEdit {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::WHITE;
        fill_style.fill.hovered = Color::WHITE;
        fill_style.fill.clicked = Color::WHITE;
        fill_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = fill_style.border.hovered.clone();

        let mut select_style = ClickStyle::new();
        select_style.fill.inactive = Color::rgba(144, 209, 255, 100); //Color::rgba(255, 0, 0, 100); //
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
        SingleEdit {
            id: crate::gen_unique_id(),
            text_buffer: TextBuffer::new(context),
            size_mode: SizeMode::Auto,
            callback: None,
            char_layout: CharLayout::new(),

            fill_render: RenderParam::new(RectParam::new(Rect::new(), fill_style)),
            select_render: RenderParam::new(RectParam::new(Rect::new(), select_style)),
            cursor_render: RenderParam::new(RectParam::new(Rect::new(), cursor_style)),

            hovered: false,
            focused: false,
            mouse_press: false,
            contact_ids: vec![],
            changed: false,
        }
    }

    pub(crate) fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(context); //计算行高
        match self.size_mode {
            SizeMode::Auto => self.fill_render.param.rect.set_size(200.0, 25.0),
            SizeMode::FixWidth => self.fill_render.param.rect.set_height(25.0),
            SizeMode::FixHeight => self.fill_render.param.rect.set_width(200.0),
            SizeMode::Fix => {}
        }
        let mut rect = self.fill_render.param.rect.clone_add_padding(&Padding::same(3.0));
        rect.add_min_x(2.0);
        rect.add_max_x(-2.0);
        self.text_buffer.rect = rect;
    }

    pub(crate) fn set_rect(&mut self, rect: Rect) {
        self.fill_render.param.rect = rect;
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
        self.cursor_render.param.rect.offset_x_to(xm);
        self.cursor_render.update(ui, false, false);
    }

    fn text_select(&mut self, ui: &mut Ui) {
        let lx = ui.device.device_input.mouse.lastest().x;
        if lx < self.text_buffer.rect.dx().min {
            self.select_render.param.rect.set_x_min(self.text_buffer.rect.dx().min);
            let event = ui.context.event.clone();
            let wid = ui.context.window.id();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                event.send_event((wid, UpdateType::None)).unwrap();
            });
        } else if lx > self.text_buffer.rect.dx().max {
            self.select_render.param.rect.set_x_max(self.text_buffer.rect.dx().max);
            let event = ui.context.event.clone();
            let wid = ui.context.window.id();
            std::thread::spawn(move || {
                std::thread::sleep(std::time::Duration::from_millis(100));
                event.send_event((wid, UpdateType::None)).unwrap();
            });
        } else {
            let pos = self.char_layout.chars.iter().position(|tc| tc.x.min + self.char_layout.offset < lx && lx <= tc.x.max + self.char_layout.offset);
            if let Some(pos) = pos {
                let ct = &mut self.char_layout.chars[pos];
                if lx > ui.device.device_input.mouse.pressed_pos.x { //向右选择
                    self.select_render.param.rect.set_x_max(if lx >= ct.half_x() + self.char_layout.offset { ct.x.max } else { ct.x.min } + self.char_layout.offset);
                    self.char_layout.selected.end = if lx >= ct.half_x() + self.char_layout.offset { pos + 1 } else { pos };
                } else { //向左选择
                    self.select_render.param.rect.set_x_min(if lx >= ct.half_x() + self.char_layout.offset { ct.x.max } else { ct.x.min } + self.char_layout.offset);
                    self.char_layout.selected.start = if lx >= ct.half_x() + self.char_layout.offset { pos + 1 } else { pos };
                }
                let ct = &self.char_layout.chars[pos];
                self.char_layout.reset_cursor(if lx >= ct.half_x() + self.char_layout.offset { pos + 1 } else { pos });
                let xm = if lx > ui.device.device_input.mouse.pressed_pos.x { self.select_render.param.rect.dx().max } else { self.select_render.param.rect.dx().min };
                self.update_cursor(ui, xm);
            }
            self.changed = true;
            ui.context.window.request_redraw();
        }
    }

    fn key_input(&mut self, key: Option<winit::keyboard::Key>, ui: &mut Ui) {
        self.select_render.param.rect.set_x_min(0.0);
        self.select_render.param.rect.set_x_max(0.0);
        self.changed = true;
        match key.unwrap() {
            winit::keyboard::Key::Named(name) => {
                println!("{:?}", name);
                match name {
                    winit::keyboard::NamedKey::Backspace => {
                        let xm = self.char_layout.remove_char_in_cursor();
                        self.update_cursor(ui, xm);
                        self.text_buffer.clip_x = self.char_layout.offset;
                        let text = self.char_layout.text();
                        self.text_buffer.set_text(text);
                    }
                    winit::keyboard::NamedKey::ArrowLeft => {
                        let xm = self.char_layout.cursor_reduce();
                        let xm = if xm + self.char_layout.offset < self.text_buffer.rect.dx().min {
                            self.char_layout.offset = -(xm - self.text_buffer.rect.dx().min);
                            self.text_buffer.rect.dx().min
                        } else {
                            xm + self.char_layout.offset
                        };
                        self.update_cursor(ui, xm);
                        self.text_buffer.clip_x = self.char_layout.offset;
                    }
                    winit::keyboard::NamedKey::ArrowRight => {
                        let xm = self.char_layout.cursor_add();
                        let xm = if xm > self.text_buffer.rect.dx().max {
                            self.char_layout.offset = -(xm - self.text_buffer.rect.dx().max);
                            self.text_buffer.rect.dx().max
                        } else { xm };
                        self.update_cursor(ui, xm);
                        self.text_buffer.clip_x = self.char_layout.offset;
                    }
                    winit::keyboard::NamedKey::Delete => {
                        let xm = self.char_layout.remove_after();
                        self.update_cursor(ui, xm);
                        self.text_buffer.clip_x = self.char_layout.offset;
                        let text = self.char_layout.text();
                        self.text_buffer.set_text(text);
                    }
                    winit::keyboard::NamedKey::Space => {
                        let xm = self.char_layout.push_char(' ', &ui.context, self.text_buffer.rect.dx().max);
                        self.update_cursor(ui, xm);
                        let text = self.char_layout.text();
                        self.text_buffer.set_text(text);
                    }
                    _ => {}
                }
            }
            winit::keyboard::Key::Character(c) => {
                let c = c.chars().next().unwrap();
                let xm = self.char_layout.push_char(c, &ui.context, self.text_buffer.rect.dx().max);
                self.update_cursor(ui, xm);
                self.text_buffer.clip_x = self.char_layout.offset;
                let text = self.char_layout.text();

                self.text_buffer.set_text(text);
            }
            winit::keyboard::Key::Unidentified(_) => {}
            winit::keyboard::Key::Dead(_) => {}
        }
        self.char_layout.selected = 0..0;
        if let Some(ref mut callback) = self.callback {
            let app = ui.app.take().unwrap();
            callback(app, ui, self.char_layout.text());
            ui.app.replace(app);
        }
        ui.send_updates(&self.contact_ids, ContextUpdate::String(self.char_layout.text()));
        ui.context.window.request_redraw();
    }

    pub(crate) fn update_text(&mut self, ui: &mut Ui, text: String) {
        self.text_buffer.set_text(text);
        let wx = self.text_buffer.rect.dx().min;
        self.char_layout = CharLayout::from_text(wx, &self.text_buffer.text.text, self.text_buffer.text.font_size(), &ui.context);
        self.cursor_render.param.rect.offset_x_to(self.char_layout.x_min + self.char_layout.width);
        self.changed = true;
    }

    pub(crate) fn text(&self) -> String {
        self.char_layout.text()
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.fill_render.param.rect = ui.layout().available_rect().clone_with_size(&self.fill_render.param.rect);
            self.reset_size(&ui.context);
        }
        //背景
        self.fill_render.init_rectangle(ui, false, false);
        //文本选择
        self.select_render.param.rect = self.text_buffer.rect.clone();
        self.select_render.param.rect.set_width(0.0);
        self.select_render.init_rectangle(ui, false, false);
        //字符管理
        let wx = self.text_buffer.rect.dx().min;
        self.char_layout = CharLayout::from_text(wx, &self.text_buffer.text.text, self.text_buffer.text.font_size(), &ui.context);
        //游标
        self.cursor_render.param.rect = self.fill_render.param.rect.clone_add_padding(&Padding::same(5.0));
        self.cursor_render.param.rect.set_x_min(self.cursor_render.param.rect.dx().min - 2.0);
        self.cursor_render.param.rect.set_x_max(self.cursor_render.param.rect.dx().min + 2.0);
        self.cursor_render.param.rect.offset_x_to(self.char_layout.x_min + self.char_layout.width);
        self.cursor_render.init_rectangle(ui, false, false);
        //文本
        self.text_buffer.draw(ui);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.changed && !ui.can_offset { return; }
        self.changed = false;
        if ui.can_offset {
            self.fill_render.param.rect.offset(&ui.offset);
            self.text_buffer.rect.offset(&ui.offset);
            self.select_render.param.rect.offset(&ui.offset);
            self.cursor_render.param.rect.offset(&ui.offset);
        }
        self.cursor_render.update(ui, false, false);
        self.select_render.update(ui, false, false);
        self.fill_render.update(ui, false, false);
        self.text_buffer.update_buffer(ui);
    }
}


impl Widget for SingleEdit {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.text_buffer.redraw(ui);
        let pass = ui.pass.as_mut().unwrap();
        if self.focused { ui.context.render.rectangle.render(&self.cursor_render, pass); }
        ui.context.render.rectangle.render(&self.select_render, pass);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&self.fill_render.param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.changed = true;
                    ui.context.window.request_redraw();
                }
                if self.focused && ui.device.device_input.mouse.pressed { self.text_select(ui); }
            }
            UpdateType::MousePress => {
                self.mouse_press = true;
                if self.focused && !ui.device.device_input.pressed_at(&self.fill_render.param.rect) {
                    self.focused = false;
                    self.changed = true;
                    ui.context.window.request_redraw();
                }
                if ui.device.device_input.pressed_at(&self.fill_render.param.rect) {
                    self.focused = true;
                    //鼠标按下
                    let x = ui.device.device_input.mouse.lastest().x;
                    if x < self.char_layout.x_min {
                        self.select_render.param.rect.set_x_min(self.char_layout.x_min);
                        self.select_render.param.rect.set_x_max(self.char_layout.x_min);
                        self.char_layout.selected = 0..0;
                        self.update_cursor(ui, self.char_layout.x_min + self.char_layout.offset);
                        self.char_layout.reset_cursor(0);
                        ui.context.window.request_redraw();
                    } else if x > self.char_layout.x_min + self.char_layout.width {
                        self.select_render.param.rect.set_x_min(self.char_layout.x_min + self.char_layout.width);
                        self.select_render.param.rect.set_x_max(self.char_layout.x_min + self.char_layout.width);
                        self.char_layout.selected = self.char_layout.chars.len()..self.char_layout.chars.len();
                        self.update_cursor(ui, self.char_layout.x_min + self.char_layout.width + self.char_layout.offset);
                        self.char_layout.reset_cursor(self.char_layout.chars.len());
                        ui.context.window.request_redraw();
                    } else {
                        let pos = self.char_layout.chars.iter().position(|tc| tc.x.min + self.char_layout.offset < x && x < tc.x.max + self.char_layout.offset);
                        if let Some(pos) = pos {
                            let ct = &self.char_layout.chars[pos];
                            println!("r {} {}", pos, ct.char);
                            self.char_layout.selected = if x >= ct.half_x() + self.char_layout.offset { pos + 1..pos + 1 } else { pos..pos };
                            self.select_render.param.rect.set_x_min(if x >= ct.half_x() + self.char_layout.offset { ct.x.max + self.char_layout.offset } else { ct.x.min + self.char_layout.offset });
                            self.select_render.param.rect.set_x_max(if x >= ct.half_x() + self.char_layout.offset { ct.x.max + self.char_layout.offset } else { ct.x.min + self.char_layout.offset });
                            self.char_layout.reset_cursor(if x >= ct.half_x() + self.char_layout.offset { pos + 1 } else { pos });
                            self.update_cursor(ui, self.select_render.param.rect.dx().max);
                            ui.context.window.request_redraw();
                        }
                    }
                }
                self.changed = true;
            }
            UpdateType::MouseRelease => self.mouse_press = false,
            UpdateType::KeyRelease(ref mut key) => {
                if !self.focused || key.is_none() { return Response::new(&self.id, &self.fill_render.param.rect); }
                self.key_input(key.take(), ui)
            }
            // UpdateType::Offset(ref o) => {
            //     if !ui.can_offset { return Response::new(&self.id, &self.fill_render.param.rect); }
            //     self.fill_render.param.rect.offset(o);
            //     self.text_buffer.rect.offset(o);
            //     self.select_render.param.rect.offset(o);
            //     self.cursor_render.param.rect.offset(o);
            // }
            UpdateType::None => {
                if !self.focused { return Response::new(&self.id, &self.fill_render.param.rect); }
                if let Some(c) = self.char_layout.next_char() && ui.device.device_input.mouse.lastest.x > self.text_buffer.rect.dx().max
                    && ui.device.device_input.mouse.lastest.x > ui.device.device_input.mouse.pressed_pos.x {
                    println!("p {} {}", self.char_layout.cursor, self.char_layout.chars.len());
                    self.select_render.param.rect.add_min_x(-c.width);
                    self.char_layout.offset = -(c.x.max - self.text_buffer.rect.dx().max);
                    self.char_layout.cursor_add();
                    self.char_layout.selected.end = self.char_layout.chars.len();

                    if self.select_render.param.rect.dx().min < self.text_buffer.rect.dx().min {
                        self.select_render.param.rect.set_x_min(self.text_buffer.rect.dx().min);
                    }
                    self.changed = true;
                    self.text_buffer.clip_x = self.char_layout.offset;
                    ui.context.window.request_redraw();
                } else if let Some(c) = self.char_layout.previous_char() && ui.device.device_input.mouse.lastest.x < self.text_buffer.rect.dx().min
                    && ui.device.device_input.mouse.lastest.x < ui.device.device_input.mouse.pressed_pos.x {
                    self.select_render.param.rect.add_max_x(c.width);
                    println!("n {} {} {}", self.char_layout.cursor, self.char_layout.chars.len(), self.char_layout.offset);
                    self.char_layout.offset += c.width;
                    if self.char_layout.offset >= 0.0 { self.char_layout.offset = 0.0 }
                    self.char_layout.cursor_reduce();
                    self.char_layout.selected.start = 0;
                    if self.select_render.param.rect.dx().max > self.text_buffer.rect.dx().max {
                        self.select_render.param.rect.set_x_max(self.text_buffer.rect.dx().max);
                    }
                    self.changed = true;
                    self.text_buffer.clip_x = self.char_layout.offset;
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}