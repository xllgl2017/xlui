use crate::align::Align;
use crate::frame::context::{ContextUpdate, UpdateType};
use crate::key::Key;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::textedit::buffer::CharBuffer;
use crate::widgets::textedit::cursor::EditCursor;
use crate::widgets::textedit::select::EditSelection;
use crate::widgets::{Widget, WidgetChange, WidgetSize};
use crate::window::ime::IMEData;
use crate::window::ClipboardData;
use crate::{App, TextWrap};
use std::mem;

pub(crate) mod buffer;
mod select;
mod cursor;

#[derive(PartialEq)]
enum EditKind {
    Single,
    Multi,
    Password,
}


pub struct TextEdit {
    id: String,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, String)>>,
    contact_ids: Vec<String>,
    fill_render: RenderParam<RectParam>,
    select_render: EditSelection,
    cursor_render: EditCursor,
    changed: bool,
    hovered: bool,
    size_mode: SizeMode,
    char_layout: CharBuffer,
    desire_lines: usize,
    pub(crate) focused: bool,
    psd_buffer: TextBuffer,

}

impl TextEdit {
    fn new(text: impl ToString) -> TextEdit {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::WHITE;
        fill_style.fill.hovered = Color::WHITE;
        fill_style.fill.clicked = Color::WHITE;
        fill_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = fill_style.border.hovered.clone();
        TextEdit {
            id: crate::gen_unique_id(),
            callback: None,
            contact_ids: vec![],
            fill_render: RenderParam::new(RectParam::new(Rect::new().with_size(200.0, 30.0), fill_style)),
            select_render: EditSelection::new(),
            cursor_render: EditCursor::new(),
            changed: false,
            hovered: false,
            size_mode: SizeMode::FixWidth(200.0),
            char_layout: CharBuffer::new(text),
            desire_lines: 8,
            focused: false,
            psd_buffer: TextBuffer::new("🔒").with_align(Align::Center), //👁🔓
        }
    }


    pub fn single_edit(txt: impl ToString) -> TextEdit {
        let mut res = Self::new(txt.to_string());
        res.desire_lines = 1;
        res.char_layout.edit_kind = EditKind::Single;
        res.char_layout.buffer.set_wrap(TextWrap::NoWrap);
        res
    }

    pub fn password(mut self) -> TextEdit {
        self.char_layout.edit_kind = EditKind::Password;
        self
    }

    pub fn multi_edit(txt: impl ToString) -> TextEdit {
        TextEdit::new(txt.to_string())
    }

    pub fn with_rows(mut self, row: usize) -> TextEdit {
        self.desire_lines = row;
        if row == 1 { self.char_layout.edit_kind = EditKind::Single; }
        self
    }

    pub fn set_width(&mut self, width: f32) {
        self.size_mode.fix_width(width);
    }

    // pub(crate) fn set_rect(&mut self, rect: Rect) {
    //     self.fill_render.param.rect = rect;
    //     self.size_mode = SizeMode::Fix;
    // }

    pub(crate) fn update_text(&mut self, ui: &mut Ui, text: String) {
        self.char_layout.buffer.update_buffer_text(ui, &text);
        self.select_render.reset(&self.cursor_render);
        self.changed = true;
    }

    pub fn text(&self) -> String {
        self.char_layout.buffer.lines.iter().map(|x| x.raw_text()).collect()
    }

    pub fn contact(mut self, id: impl ToString) -> Self {
        self.contact_ids.push(id.to_string());
        self
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, String)) -> Self {
        self.callback = Some(Callback::create_textedit(f));
        self
    }

    pub(crate) fn reset_size(&mut self, ui: &mut Ui) {
        self.char_layout.buffer.set_width(self.size_mode.width(200.0) - 6.0);
        // self.char_layout.buffer.rect.set_width(194.0);
        // self.char_layout.buffer.size_mode = SizeMode::FixWidth;
        self.char_layout.buffer.init(ui); //计算行高
        let height = self.char_layout.buffer.text.height * self.desire_lines as f32 + 6.0;
        let (w, h) = self.size_mode.size(200.0, height);
        // match self.size_mode {
        //     SizeMode::Auto => self.fill_render.param.rect.set_size(200.0, height),
        //     SizeMode::FixWidth => self.fill_render.param.rect.set_height(height),
        //     _ => {}
        // }
        // let mut rect = self.fill_render.param.rect.clone_add_padding(&Padding::same(3.0));
        self.fill_render.param.rect.set_size(w, h);
        self.char_layout.buffer.set_height(h);
        // rect.contract_x(2.0);
        // self.char_layout.buffer.rect = rect;
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        // if !self.changed && !ui.can_offset { return; }
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.param.rect.offset_to_rect(&ui.draw_rect);
            self.fill_render.update(ui, false, false);
            let mut text_rect = ui.draw_rect.clone();
            text_rect.contract(3.0, 3.0);
            // text_rect.add_min_x(3.0);
            // text_rect.add_min_y(3.0);
            self.char_layout.buffer.rect.offset_to_rect(&text_rect);
            // println!("{:?}", self.char_layout.buffer.rect);
            // self.cursor_render.min_pos.x = self.char_layout.buffer.rect.dx().min;
            // self.cursor_render.min_pos.y = self.char_layout.buffer.rect.dy().min;
            // self.cursor_render.max_pos.x = self.char_layout.buffer.rect.dx().max;
            // self.cursor_render.max_pos.y = self.char_layout.buffer.rect.dy().max;
            let mut cursor_rect = self.char_layout.buffer.rect.clone();
            cursor_rect.set_width(2.0);
            cursor_rect.set_height(self.char_layout.buffer.text.height);
            self.cursor_render.update_position(ui, cursor_rect, &self.char_layout);
            self.select_render.update_position(ui, self.char_layout.buffer.rect.clone());

            self.psd_buffer.rect = self.fill_render.param.rect.clone();
            self.psd_buffer.rect.set_x_min(self.fill_render.param.rect.dx().max - 20.0);
            // self.psd_buffer.init(ui);
            self.psd_buffer.rect.add_min_y(1.0);
            // self.cursor_render.set_rect(cursor_rect);
            // self.cursor_render.update(ui);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            self.fill_render.update(ui, self.hovered || self.focused, ui.device.device_input.mouse.pressed);
            self.cursor_render.update(ui);
            self.select_render.update(ui);
        }
        // if ui.can_offset {
        //     self.char_layout.buffer.rect.offset(&ui.offset);
        //     self.cursor_render.offset(&ui.offset);
        //     self.select_render.offset(&ui.offset);
        //     self.psd_buffer.rect.offset(&ui.offset);
        //     self.fill_render.param.rect.offset(&ui.offset);
        // }

    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            // self.fill_render.param.rect = ui.available_rect().clone_with_size(&self.fill_render.param.rect);
            self.reset_size(ui);
            self.char_layout.set_font_size(self.char_layout.buffer.text.font_size());
            self.char_layout.set_line_height(self.char_layout.buffer.text.height);
            println!("111111111111111-{}-{}", self.char_layout.buffer.rect.width(), self.fill_render.param.rect.width());
            self.char_layout.set_max_wrap_width(self.char_layout.buffer.rect.width());
            if let EditKind::Password = self.char_layout.edit_kind { self.char_layout.rebuild_text(ui); }

            // self.psd_buffer.align = Align::Center;
            // self.psd_buffer.rect = self.fill_render.param.rect.clone();
            // self.psd_buffer.rect.set_x_min(self.fill_render.param.rect.dx().max - 20.0);
            self.psd_buffer.init(ui);
            // self.psd_buffer.rect.add_min_y(1.0);
        }
        self.fill_render.init_rectangle(ui, false, false);
        self.cursor_render.init(&self.char_layout, ui, init);
        self.select_render.init(self.desire_lines, &self.char_layout.buffer.rect, self.char_layout.buffer.text.height, ui, init);
    }

    fn key_input(&mut self, key: Key, ui: &mut Ui) {
        self.changed = true;
        match key {
            Key::Backspace => {
                self.char_layout.remove_chars_before_cursor(ui, &mut self.cursor_render, &mut self.select_render);
                self.char_layout.buffer.clip_x = self.char_layout.offset.x;
            }
            Key::Enter => self.char_layout.inset_char('\n', ui, &mut self.cursor_render, &mut self.select_render),
            Key::Space => {
                self.char_layout.inset_char(' ', ui, &mut self.cursor_render, &mut self.select_render);
                self.char_layout.buffer.clip_x = self.char_layout.offset.x;
            }
            Key::Home => {
                self.char_layout.buffer.clip_x = 0.0;
                self.char_layout.offset.x = 0.0;
                self.cursor_render.set_cursor(0, self.cursor_render.vert, &self.char_layout)
            }
            Key::End => {
                let line = &self.char_layout.buffer.lines[self.cursor_render.vert];
                if line.width + self.cursor_render.min_pos.x > self.cursor_render.max_pos.x {
                    self.char_layout.buffer.clip_x = self.cursor_render.max_pos.x - line.width - self.cursor_render.min_pos.x;
                    self.char_layout.offset.x = self.char_layout.buffer.clip_x;
                }
                self.cursor_render.set_cursor(line.len(), self.cursor_render.vert, &self.char_layout)
            }
            Key::Delete => self.char_layout.remove_chars_after_cursor(ui, &mut self.cursor_render, &mut self.select_render),
            Key::Char(c) => {
                self.char_layout.inset_char(c, ui, &mut self.cursor_render, &mut self.select_render);
                self.char_layout.buffer.clip_x = self.char_layout.offset.x;
            }
            Key::LeftArrow => {
                if self.cursor_render.cursor_min() <= self.cursor_render.min_pos.x && let Some(cchar) = self.char_layout.previous_char(&self.cursor_render) {
                    self.char_layout.buffer.clip_x += cchar.width;
                    if self.char_layout.buffer.clip_x > 0.0 { self.char_layout.buffer.clip_x = 0.0; }
                    self.char_layout.offset.x = self.char_layout.buffer.clip_x;
                }
                self.cursor_render.move_left(&self.char_layout);
            }
            Key::RightArrow => {
                if self.cursor_render.cursor_min() + 2.0 >= self.cursor_render.max_pos.x && let Some(cchar) = self.char_layout.next_char(&self.cursor_render) {
                    self.char_layout.buffer.clip_x -= cchar.width;
                    self.char_layout.offset.x = self.char_layout.buffer.clip_x;
                }
                self.cursor_render.move_right(&self.char_layout)
            }
            Key::UpArrow => self.cursor_render.move_up(&self.char_layout),
            Key::DownArrow => self.cursor_render.move_down(&self.char_layout),
            _ => {}
        }
        self.select_render.reset(&self.cursor_render);
        let text: String = self.char_layout.buffer.lines.iter().map(|x| x.raw_text()).collect();
        if let Some(ref mut callback) = self.callback {
            let app = ui.app.take().unwrap();
            callback(app, ui, text.clone());
            ui.app.replace(app);
        }
        ui.send_updates(&self.contact_ids, ContextUpdate::String(text));
        ui.context.window.request_redraw();
    }

    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.select_render.render(ui, self.char_layout.buffer.lines.len());
        if self.focused { self.cursor_render.render(ui); }
        self.char_layout.buffer.redraw(ui);
        if let EditKind::Password = self.char_layout.edit_kind {
            self.psd_buffer.redraw(ui);
        }
    }
}

impl Widget for TextEdit {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            UpdateType::MouseMove => {
                if ui.device.device_input.mouse.pressed && self.focused {
                    self.select_render.move_select(ui, &mut self.cursor_render, &mut self.char_layout);
                    self.changed = true;
                    self.char_layout.buffer.clip_x = self.char_layout.offset.x;
                    ui.context.window.request_redraw();
                }

                let hovered = ui.device.device_input.hovered_at(&self.fill_render.param.rect);
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.changed = true;
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MousePress => {
                self.focused = ui.device.device_input.pressed_at(&self.fill_render.param.rect);
                ui.context.window.ime().request_ime(self.focused);
                if self.focused {
                    // ui.context.window.win32().request_ime();
                    let pos = ui.device.device_input.mouse.lastest;
                    self.cursor_render.update_by_pos(pos, &mut self.char_layout);
                    self.select_render.set_by_cursor(&self.cursor_render);
                }
                self.changed = true;
                ui.context.window.request_redraw();
            }
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.psd_buffer.rect) && let EditKind::Password = self.char_layout.edit_kind {
                    self.char_layout.looking = !self.char_layout.looking;
                    self.psd_buffer.update_buffer_text(ui, if self.char_layout.looking { "🔓" } else { "🔒" });
                    self.char_layout.rebuild_text(ui);
                    self.cursor_render.reset_x(&self.char_layout);
                    ui.context.window.request_redraw();
                    self.changed = true;
                }
            }
            #[cfg(not(feature = "winit"))]
            UpdateType::KeyPress(ref mut key) => {
                if self.focused {
                    match key {
                        Key::CtrlC => {
                            println!("copy");
                            let select_text = self.char_layout.select_text(&self.select_render, &self.cursor_render);
                            ui.context.window.set_clipboard(ClipboardData::Text(select_text));
                        }
                        Key::CtrlV => {
                            #[cfg(target_os = "linux")]
                            ui.context.window.request_clipboard(ClipboardData::Text(String::new()));
                            #[cfg(target_os = "windows")]
                            let res = ui.context.window.win32().clipboard.get_clipboard_data(ClipboardData::Text(String::new())).unwrap_or(ClipboardData::Unsupported);
                            #[cfg(target_os = "windows")]
                            match res {
                                ClipboardData::Unsupported => {}
                                ClipboardData::Text(t) => {
                                    for c in t.chars() {
                                        self.char_layout.inset_char(c, ui, &mut self.cursor_render, &mut self.select_render);
                                    }
                                }
                                ClipboardData::Image(_) => {}
                                ClipboardData::Url(_) => {}
                            }
                            #[cfg(target_os = "windows")]
                            { self.changed = true; }
                            #[cfg(target_os = "windows")]
                            { ui.context.window.request_redraw(); }
                        }
                        _ => {}
                    }
                }
            }
            UpdateType::Clipboard(ref mut clipboard) => {
                if self.focused {
                    match mem::take(clipboard) {
                        ClipboardData::Text(t) => {
                            for c in t.chars() {
                                self.char_layout.inset_char(c, ui, &mut self.cursor_render, &mut self.select_render);
                            }
                        }
                        _ => {}
                    }
                }
            }
            UpdateType::KeyRelease(ref mut key) => {
                if self.focused { self.key_input(mem::take(key), ui); }
            }
            UpdateType::IME(ref mut data) => {
                if self.focused {
                    let start_horiz = self.select_render.start_horiz;
                    let start_vert = self.select_render.start_vert;
                    match data {
                        IMEData::Preedit(cs) => {
                            for c in mem::take(cs) {
                                self.char_layout.inset_char(c, ui, &mut self.cursor_render, &mut self.select_render);
                            }
                            self.select_render.select_by_ime(start_horiz, start_vert, &self.char_layout, &self.cursor_render);
                            ui.context.window.set_ime_position(self.cursor_render.cursor_min(), self.cursor_render.min_pos.y + self.cursor_render.offset.y, self.char_layout.buffer.text.height);
                        }
                        IMEData::Commit(cs) => {
                            for c in mem::take(cs) {
                                self.char_layout.inset_char(c, ui, &mut self.cursor_render, &mut self.select_render);
                            }
                            ui.context.window.ime().request_ime(true);
                            self.select_render.reset(&self.cursor_render);
                        }
                    }
                    self.changed = true;
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.fill_render.param.rect.width(), self.fill_render.param.rect.height()))
    }
}