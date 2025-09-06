use crate::frame::context::UpdateType;
use crate::key::Key;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::text_buffer::TextBuffer;
use crate::text::TextWrap;
use crate::ui::Ui;
use crate::widgets::textedit::buffer::CharBuffer;
use crate::widgets::textedit::cursor::EditCursor;
use crate::widgets::textedit::select::EditSelection;
use crate::widgets::Widget;
pub(crate) mod buffer;
mod select;
mod cursor;

#[derive(PartialEq)]
enum EditKind {
    Single,
    Multi,
}


pub struct TextEdit {
    id: String,
    text_buffer: TextBuffer,
    fill_render: RenderParam<RectParam>,
    select_render: EditSelection,
    cursor_render: EditCursor,
    changed: bool,
    hovered: bool,
    size_mode: SizeMode,
    char_layout: CharBuffer,
    desire_lines: usize,
    pub(crate) focused: bool,

}

impl TextEdit {
    fn new(text: String) -> TextEdit {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::WHITE;
        fill_style.fill.hovered = Color::WHITE;
        fill_style.fill.clicked = Color::WHITE;
        fill_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = fill_style.border.hovered.clone();


        TextEdit {
            id: crate::gen_unique_id(),
            text_buffer: TextBuffer::new(text).with_wrap(TextWrap::WrapWorld),
            fill_render: RenderParam::new(RectParam::new(Rect::new(), fill_style)),
            select_render: EditSelection::new(),
            cursor_render: EditCursor::new(),
            changed: false,
            hovered: false,
            size_mode: SizeMode::Auto,
            char_layout: CharBuffer::new(),
            desire_lines: 8,
            focused: false,
        }
    }


    pub fn single_edit(txt: impl ToString) -> TextEdit {
        let mut res = Self::new(txt.to_string());
        res.desire_lines = 1;
        res.char_layout.edit_kind = EditKind::Single;
        res
    }

    pub fn multi_edit(txt: impl ToString) -> TextEdit {
        TextEdit::new(txt.to_string())
    }

    pub fn with_rows(mut self, row: usize) -> TextEdit {
        self.desire_lines = row;
        if row == 1 { self.char_layout.edit_kind = EditKind::Single; }
        self
    }

    pub(crate) fn set_rect(&mut self, rect: Rect) {
        self.fill_render.param.rect = rect;
        self.size_mode = SizeMode::Fix;
    }

    pub(crate) fn update_text(&mut self, ui: &mut Ui, text: String) {
        self.char_layout.set_text(&text, ui);
        self.text_buffer.set_text(text);
        self.select_render.reset(&self.cursor_render);
        self.changed = true;
    }

    pub fn text(&self) -> String {
        self.char_layout.raw_text()
    }

    pub(crate) fn reset_size(&mut self, ui: &mut Ui) {
        self.text_buffer.reset_size(ui); //计算行高
        let height = self.text_buffer.text.height * self.desire_lines as f32 + 6.0;
        match self.size_mode {
            SizeMode::Auto => self.fill_render.param.rect.set_size(200.0, height),
            SizeMode::FixWidth => self.fill_render.param.rect.set_height(height),
            _ => {}
        }
        let mut rect = self.fill_render.param.rect.clone_add_padding(&Padding::same(3.0));
        rect.contract_x(2.0);
        self.text_buffer.rect = rect;
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.changed && !ui.can_offset { return; }
        self.changed = false;
        if ui.can_offset {
            self.text_buffer.rect.offset(&ui.offset);
            self.cursor_render.offset(&ui.offset);
            self.select_render.offset(&ui.offset);
            self.fill_render.param.rect.offset(&ui.offset);
        }
        self.fill_render.update(ui, self.hovered || self.focused, ui.device.device_input.mouse.pressed);
        self.cursor_render.update(ui);
        self.select_render.update(ui);
        self.text_buffer.update_buffer_text(ui, &self.char_layout.draw_text());
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.fill_render.param.rect = ui.available_rect().clone_with_size(&self.fill_render.param.rect);
            self.reset_size(ui);
            self.char_layout.set_font_size(self.text_buffer.text.font_size());
            self.char_layout.set_line_height(self.text_buffer.text.height);
            println!("111111111111111-{}-{}", self.text_buffer.rect.width(), self.fill_render.param.rect.width());
            self.char_layout.set_max_wrap_width(self.text_buffer.rect.width());
            self.char_layout.set_text(&self.text_buffer.text.text, ui);
            self.text_buffer.update_buffer_text(ui, self.char_layout.draw_text());
            let mut cursor_rect = self.text_buffer.rect.clone();
            cursor_rect.set_width(2.0);
            cursor_rect.set_height(self.text_buffer.text.height);
            self.cursor_render.set_rect(cursor_rect);
        }
        self.fill_render.init_rectangle(ui, false, false);
        self.cursor_render.init(&self.text_buffer, &self.char_layout, ui, init);
        self.select_render.init(self.desire_lines, &self.text_buffer.rect, self.text_buffer.text.height, ui, init);
        self.text_buffer.draw(ui);
    }

    fn key_input(&mut self, key: Option<Key>, ui: &mut Ui) {
        if key.is_none() { return; }
        self.changed = true;
        match key.unwrap() {
            Key::Backspace => {
                self.char_layout.remove_chars_before_cursor(ui, &mut self.cursor_render, &mut self.select_render);
                self.text_buffer.clip_x = self.char_layout.offset.x;
            }
            Key::Enter => self.char_layout.inset_char('\n', ui, &mut self.cursor_render, &mut self.select_render),
            Key::Space => {
                self.char_layout.inset_char(' ', ui, &mut self.cursor_render, &mut self.select_render);
                self.text_buffer.clip_x = self.char_layout.offset.x;
            }
            Key::Home => {
                self.text_buffer.clip_x = 0.0;
                self.char_layout.offset.x = 0.0;
                self.cursor_render.set_cursor(0, self.cursor_render.vert, &self.char_layout)
            }
            Key::End => {
                let line = &self.char_layout.lines[self.cursor_render.vert];
                if line.width + self.cursor_render.min_pos.x > self.cursor_render.max_pos.x {
                    self.text_buffer.clip_x = self.cursor_render.max_pos.x - line.width - self.cursor_render.min_pos.x;
                    self.char_layout.offset.x = self.text_buffer.clip_x;
                }
                self.cursor_render.set_cursor(line.len(), self.cursor_render.vert, &self.char_layout)
            }
            Key::Delete => self.char_layout.remove_chars_after_cursor(ui, &mut self.cursor_render, &mut self.select_render),
            Key::Char(c) => {
                self.char_layout.inset_char(c, ui, &mut self.cursor_render, &mut self.select_render);
                self.text_buffer.clip_x = self.char_layout.offset.x;
            }
            Key::LeftArrow => {
                if self.cursor_render.cursor_min() <= self.cursor_render.min_pos.x && let Some(cchar) = self.char_layout.previous_char(&self.cursor_render) {
                    self.text_buffer.clip_x += cchar.width;
                    if self.text_buffer.clip_x > 0.0 { self.text_buffer.clip_x = 0.0; }
                    self.char_layout.offset.x = self.text_buffer.clip_x;
                }
                self.cursor_render.move_left(&self.char_layout);
            }
            Key::RightArrow => {
                if self.cursor_render.cursor_min() + 2.0 >= self.cursor_render.max_pos.x && let Some(cchar) = self.char_layout.next_char(&self.cursor_render) {
                    self.text_buffer.clip_x -= cchar.width;
                    self.char_layout.offset.x -= cchar.width;
                }
                self.cursor_render.move_right(&self.char_layout)
            }
            Key::UpArrow => self.cursor_render.move_up(&self.char_layout),
            Key::DownArrow => self.cursor_render.move_down(&self.char_layout),
            _ => {}
        }
        self.select_render.reset(&self.cursor_render);
        // if let Some(ref mut callback) = self.callback {
        //     let app = ui.app.take().unwrap();
        //     callback(app, ui, self.char_layout.text());
        //     ui.app.replace(app);
        // }
        // ui.send_updates(&self.contact_ids, ContextUpdate::String(self.char_layout.text()));
        ui.context.window.request_redraw();
    }
}

impl Widget for TextEdit {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.select_render.render(ui, self.char_layout.lines.len());
        if self.focused { self.cursor_render.render(ui); }
        self.text_buffer.redraw(ui);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            UpdateType::MouseMove => {
                if ui.device.device_input.mouse.pressed && self.focused {
                    self.select_render.move_select(ui, &mut self.cursor_render, &mut self.char_layout);
                    self.changed = true;
                    self.text_buffer.clip_x = self.char_layout.offset.x;
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
                    let pos = ui.device.device_input.mouse.lastest;
                    self.cursor_render.update_by_pos(pos, &mut self.char_layout);
                    self.select_render.set_by_cursor(&self.cursor_render);
                }
                self.changed = true;
                ui.context.window.request_redraw();
            }
            UpdateType::KeyRelease(ref mut key) => {
                if self.focused { self.key_input(key.take(), ui); }
            }
            UpdateType::IME => {
                if self.focused {
                    let chars = ui.context.window.ime().chars();
                    let start_horiz = self.select_render.start_horiz;
                    let start_vert = self.select_render.start_vert;
                    for c in chars {
                        self.char_layout.inset_char(c, ui, &mut self.cursor_render, &mut self.select_render);
                    }
                    self.text_buffer.update_buffer_text(ui, self.char_layout.draw_text());
                    if !ui.context.window.ime().is_commited() {
                        self.select_render.select_by_ime(start_horiz, start_vert, &self.char_layout, &self.cursor_render);
                        ui.context.window.set_ime_position(self.cursor_render.cursor_min(), self.cursor_render.min_pos.y + self.cursor_render.offset.y + self.text_buffer.text.height);
                    } else {
                        ui.context.window.ime().request_ime(true);
                        self.select_render.reset(&self.cursor_render);
                        ui.context.window.ime().ime_done();
                    }

                    self.changed = true;
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}