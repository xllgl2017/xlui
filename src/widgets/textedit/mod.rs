use crate::align::Align;
use crate::frame::context::{ContextUpdate, UpdateType};
use crate::key::Key;
use crate::layout::LayoutDirection;
use crate::style::{Visual, VisualStyle, WidgetStyle};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::text::buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::textedit::buffer::CharBuffer;
use crate::widgets::textedit::cursor::EditCursor;
use crate::widgets::textedit::select::EditSelection;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::window::ime::IMEData;
use crate::window::ClipboardData;
use crate::{App, Shadow, TextWrap};
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
    visual: Visual,
    select_render: EditSelection,
    cursor_render: EditCursor,
    char_layout: CharBuffer,
    desire_lines: usize,
    psd_buffer: TextBuffer,
    state: WidgetState,
}

impl TextEdit {
    fn new(text: impl ToString) -> TextEdit {
        let mut fill_style = VisualStyle::same(WidgetStyle {
            fill: Color::WHITE,
            border: Border::same(1.0).color(Color::rgba(144, 209, 255, 255)),
            radius: Radius::same(2),
            shadow: Shadow::new(),
        });
        fill_style.inactive.border.set_same(0.0);

        // fill_style.fill = FillStyle::same(Color::WHITE);
        // fill_style.border.inactive = Border::same(0.0).radius(Radius::same(2));
        // fill_style.border.hovered = Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        // fill_style.border.clicked = fill_style.border.hovered.clone();
        // let param = RectParam::new().with_rect(Rect::new().with_size(200.0, 30.0)).with_style(fill_style);
        TextEdit {
            id: crate::gen_unique_id(),
            callback: None,
            contact_ids: vec![],
            visual: Visual::new().with_enable().with_style(fill_style),
            select_render: EditSelection::new(),
            cursor_render: EditCursor::new(),
            char_layout: CharBuffer::new(text),
            desire_lines: 8,
            psd_buffer: TextBuffer::new("🔒").with_align(Align::Center), //👁🔓
            state: WidgetState::default(),
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
        self.state.changed = true;
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


    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.state.changed { ui.widget_changed |= WidgetChange::Value; }
        self.state.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.visual.rect_mut().offset_to_rect(&ui.draw_rect);
            self.char_layout.buffer.geometry.offset_to_rect(&ui.draw_rect);
            let mut cursor_rect = self.char_layout.buffer.geometry.context_rect();
            cursor_rect.set_width(2.0);
            cursor_rect.set_height(self.char_layout.buffer.text.height);
            self.cursor_render.update_position(cursor_rect, &self.char_layout);
            self.select_render.update_position(self.char_layout.buffer.geometry.context_rect());
            let mut psd_rect = self.visual.rect_mut().clone();
            psd_rect.set_x_direction(LayoutDirection::Max);
            psd_rect.add_min_y(2.0);
            self.psd_buffer.geometry.offset_to_rect(&psd_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            self.cursor_render.update();
        }
    }

    pub(crate) fn reset_size(&mut self, ui: &mut Ui) {
        let line_height = self.char_layout.buffer.line_height(ui).unwrap();
        let height = line_height * self.desire_lines as f32 + 6.0;
        self.char_layout.buffer.geometry.set_fix_height(height);
        self.char_layout.buffer.init(ui); //计算行高
        self.visual.rect_mut().set_size(self.char_layout.buffer.geometry.padding_width(), height);
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.reset_size(ui);
            self.char_layout.set_font_size(self.char_layout.buffer.text.font_size());
            self.char_layout.set_line_height(self.char_layout.buffer.text.height);
            self.char_layout.set_max_wrap_width(self.char_layout.buffer.geometry.context_width());
            if let EditKind::Password = self.char_layout.edit_kind { self.char_layout.rebuild_text(ui); }
            self.psd_buffer.init(ui);
        }
        // #[cfg(feature = "gpu")]
        // self.fill_render.init(ui, false, false);
        self.cursor_render.init(&self.char_layout, init);
        self.select_render.init(self.desire_lines, self.char_layout.buffer.text.height);
    }

    fn key_input(&mut self, key: Key, ui: &mut Ui) {
        self.state.changed = true;
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
                println!("1输入字符: {:?}", c);
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
        self.visual.draw(ui, self.state.disabled, self.state.hovered, self.state.focused, false);
        self.select_render.render(ui, self.char_layout.buffer.lines.len());
        if self.state.focused { self.cursor_render.render(ui); }
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
                if self.state.hovered_moving() {
                    self.select_render.move_select(ui, &mut self.cursor_render, &mut self.char_layout);
                    self.char_layout.buffer.clip_x = self.char_layout.offset.x;
                    ui.context.window.request_redraw();
                }
                let hovered = ui.device.device_input.hovered_at(self.visual.rect());
                if self.state.on_hovered(hovered) { ui.context.window.request_redraw(); }
            }
            UpdateType::MousePress => {
                let pressed = ui.device.device_input.pressed_at(self.visual.rect());
                if self.state.on_pressed(pressed) { ui.context.window.request_redraw(); }
                ui.context.window.ime().request_ime(self.state.focused);
                if self.state.focused {
                    let pos = ui.device.device_input.mouse.lastest.relative;
                    self.cursor_render.update_by_pos(pos, &mut self.char_layout);
                    self.select_render.set_by_cursor(&self.cursor_render);
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::MouseRelease => {
                let clicked = ui.device.device_input.click_at(self.visual.rect());
                if self.state.on_clicked(clicked) && let EditKind::Password = self.char_layout.edit_kind {
                    self.char_layout.looking = !self.char_layout.looking;
                    self.psd_buffer.update_buffer_text(ui, if self.char_layout.looking { "🔓" } else { "🔒" });
                    self.char_layout.rebuild_text(ui);
                    self.cursor_render.reset_x(&self.char_layout);
                    ui.context.window.request_redraw();
                }
            }
            #[cfg(not(feature = "winit"))]
            UpdateType::KeyPress(ref mut key) => {
                if self.state.focused {
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
                            self.state.changed = true;
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
                            if self.char_layout.buffer.lines.len() == 1 {
                                let mut width = self.char_layout.buffer.geometry.context_left();
                                self.char_layout.offset.x = 0.0;
                                for char in self.char_layout.buffer.lines[0].chars.iter() {
                                    if char.width + width > self.cursor_render.max_pos.x {
                                        self.char_layout.offset.x -= char.width;
                                    }
                                    width += char.width;
                                }
                                self.char_layout.buffer.clip_x = self.char_layout.offset.x;
                            }
                            self.cursor_render.set_cursor(horiz, vert, &self.char_layout);
                            self.select_render.select_by_ime(0, 0, &self.char_layout, &self.cursor_render);
                            ui.context.window.request_redraw();
                        }
                        _ => {}
                    }
                }
            }
            UpdateType::Clipboard(ref mut clipboard) => {
                if self.state.focused {
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
                if self.state.focused { self.key_input(mem::take(key), ui); }
            }
            UpdateType::IME(ref mut data) => {
                if self.state.focused {
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
                    self.state.changed = true;
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.visual.rect().width(), self.visual.rect().height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.char_layout.buffer.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}