use crate::align::Align;
use crate::frame::context::{ContextUpdate, UpdateType};
use crate::key::Key;
use crate::layout::LayoutDirection;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
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
use crate::{App, FillStyle, TextWrap};
use std::mem;
use crate::size::Geometry;

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
    fill_render: RenderParam,
    select_render: EditSelection,
    cursor_render: EditCursor,
    changed: bool,
    hovered: bool,
    char_layout: CharBuffer,
    desire_lines: usize,
    pub(crate) focused: bool,
    psd_buffer: TextBuffer,

}

impl TextEdit {
    fn new(text: impl ToString) -> TextEdit {
        let mut fill_style = ClickStyle::new();
        fill_style.fill = FillStyle::same(Color::WHITE);
        fill_style.border.inactive = Border::same(0.0).radius(Radius::same(2));
        fill_style.border.hovered = Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = fill_style.border.hovered.clone();
        let param = RectParam::new().with_rect(Rect::new().with_size(200.0, 30.0)).with_style(fill_style);
        TextEdit {
            id: crate::gen_unique_id(),
            callback: None,
            contact_ids: vec![],
            fill_render: RenderParam::new(RenderKind::Rectangle(param)),
            select_render: EditSelection::new(),
            cursor_render: EditCursor::new(),
            changed: false,
            hovered: false,
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

    pub fn with_width(mut self, w: f32) -> Self {
        self.char_layout.buffer.geometry.set_fix_width(w);
        self
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }


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
    pub(crate) fn buffer(&mut self) -> &mut TextBuffer {
        &mut self.char_layout.buffer
    }

    pub(crate) fn reset_size(&mut self, ui: &mut Ui) {
        if self.char_layout.buffer.text.size.is_none() { self.char_layout.buffer.text.size = Some(ui.context.font.size()) }
        let line_height = ui.context.font.line_height(self.char_layout.buffer.text.font_size());
        let height = line_height * self.desire_lines as f32 + 6.0;
        self.char_layout.buffer.geometry.set_fix_height(height);
        self.char_layout.buffer.init(ui); //计算行高
        self.fill_render.rect_mut().set_size(self.char_layout.buffer.geometry.width(), height);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.rect_mut().offset_to_rect(&ui.draw_rect);
            #[cfg(feature = "gpu")]
            self.fill_render.update(ui, false, false);
            self.char_layout.buffer.geometry.offset_to_rect(&ui.draw_rect);
            let mut cursor_rect = self.char_layout.buffer.geometry.rect();
            cursor_rect.set_width(2.0);
            cursor_rect.set_height(self.char_layout.buffer.text.height);
            self.cursor_render.update_position(ui, cursor_rect, &self.char_layout);
            self.select_render.update_position(ui, self.char_layout.buffer.geometry.rect());
            let mut psd_rect = self.fill_render.rect_mut().clone();
            psd_rect.set_x_direction(LayoutDirection::Max);
            psd_rect.add_min_y(2.0);
            self.psd_buffer.geometry.offset_to_rect(&psd_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            #[cfg(feature = "gpu")]
            self.fill_render.update(ui, self.hovered || self.focused, ui.device.device_input.mouse.pressed);
            self.cursor_render.update();
            self.select_render.update(ui);
        }
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.reset_size(ui);
            self.char_layout.set_font_size(self.char_layout.buffer.text.font_size());
            self.char_layout.set_line_height(self.char_layout.buffer.text.height);
            self.char_layout.set_max_wrap_width(self.char_layout.buffer.geometry.width());
            if let EditKind::Password = self.char_layout.edit_kind { self.char_layout.rebuild_text(ui); }
            self.psd_buffer.init(ui);
        }
        #[cfg(feature = "gpu")]
        self.fill_render.init(ui, false, false);
        self.cursor_render.init(&self.char_layout, ui, init);
        self.select_render.init(self.desire_lines, &self.char_layout.buffer.geometry.rect(), self.char_layout.buffer.text.height, ui, init);
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
        // #[cfg(feature = "gpu")]
        // let pass = ui.pass.as_mut().unwrap();
        // #[cfg(feature = "gpu")]
        // ui.context.render.rectangle.render(&self.fill_render, pass);
        self.fill_render.draw(ui, false, false);
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

                let hovered = ui.device.device_input.hovered_at(self.fill_render.rect());
                if self.hovered != hovered {
                    self.hovered = hovered;
                    self.changed = true;
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MousePress => {
                self.focused = ui.device.device_input.pressed_at(self.fill_render.rect());
                ui.context.window.ime().request_ime(self.focused);
                if self.focused {
                    // ui.context.window.win32().request_ime();
                    let pos = ui.device.device_input.mouse.lastest.relative;
                    self.cursor_render.update_by_pos(pos, &mut self.char_layout);
                    self.select_render.set_by_cursor(&self.cursor_render);
                }
                self.changed = true;
                ui.context.window.request_redraw();
            }
            UpdateType::MouseRelease => {
                if ui.device.device_input.click_at(&self.psd_buffer.geometry.rect()) && let EditKind::Password = self.char_layout.edit_kind {
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
                            self.changed = true;
                            ui.context.window.request_redraw();
                        }
                        Key::CtrlX => {
                            let select_text = self.char_layout.select_text(&self.select_render, &self.cursor_render);
                            ui.context.window.set_clipboard(ClipboardData::Text(select_text));
                            self.char_layout.remove_by_range(ui, &mut self.cursor_render, &mut self.select_render);
                            ui.context.window.request_redraw();
                        }
                        Key::CtrlA => {
                            let horiz = self.char_layout.buffer.lines.last().unwrap().chars.len();
                            let vert = self.char_layout.buffer.lines.len() - 1;
                            self.cursor_render.set_cursor(horiz, vert, &self.char_layout);
                            self.select_render.select_by_ime(0, 0, &self.char_layout, &self.cursor_render);
                            ui.context.window.request_redraw();
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
                            ui.send_updates(&self.contact_ids, ContextUpdate::String(self.text()));
                            ui.context.window.request_redraw();
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
        Response::new(&self.id, WidgetSize::same(self.fill_render.rect().width(), self.fill_render.rect().height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.char_layout.buffer.geometry
    }
}