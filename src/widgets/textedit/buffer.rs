use std::mem;
use crate::size::pos::Pos;
use crate::ui::Ui;
use crate::widgets::textedit::EditKind;
use crate::Offset;
use std::ops::Range;
use crate::widgets::textedit::cursor::EditCursor;

pub(crate) struct EditChar {
    cchar: char,
    pub(crate) width: f32,
}

impl EditChar {
    pub fn new(cchar: char, width: f32) -> EditChar {
        EditChar { cchar, width }
    }
}

#[derive(Default)]
pub struct EditLine {
    pub(crate) chars: Vec<EditChar>,
    auto_wrap: bool,
    pub(crate) width: f32,
}

impl EditLine {
    pub fn new() -> EditLine {
        EditLine {
            chars: vec![],
            auto_wrap: true,
            width: 0.0,
        }
    }

    pub fn push(&mut self, cchar: EditChar) {
        self.width += cchar.width;
        self.chars.push(cchar);
    }

    pub fn raw_text(&self) -> String {
        let mut res: String = self.chars.iter().map(|x| x.cchar.to_string()).collect();
        if !self.auto_wrap { res += "\n"; }
        res
    }

    pub fn len(&self) -> usize { self.chars.len() }
}


pub(crate) struct CharBuffer {
    pub(crate) lines: Vec<EditLine>,
    // pub(crate) chars: Vec<EditChar>,
    selected: Range<usize>,
    draw_text: String,
    font_size: f32,
    // pub(crate) line_index: Vec<usize>,
    line_height: f32,
    max_wrap_width: f32,
    pub(crate) offset: Offset,
    edit_kind: EditKind,
}

impl CharBuffer {
    pub fn new() -> CharBuffer {
        CharBuffer {
            lines: vec![],
            selected: 0..0,
            draw_text: "".to_string(),
            font_size: 0.0,
            // line_index: vec![0],
            line_height: 0.0,
            max_wrap_width: 0.0,
            offset: Offset::new(Pos::new()),
            edit_kind: EditKind::Multi,
        }
    }
    pub fn set_text(&mut self, text: &str, ui: &mut Ui) {
        self.draw_text.clear();
        self.lines.clear();
        // self.chars.clear();
        // self.line_index = vec![0];
        match self.edit_kind {
            EditKind::Single => {
                let mut line = EditLine::new();
                for cchar in text.chars() {
                    let cchar = if cchar == '\n' { '↩' } else { cchar };
                    let width = ui.context.font.char_width(cchar, self.font_size);
                    self.draw_text.push(cchar);
                    line.push(EditChar::new(cchar, width));
                }
                line.auto_wrap = true;
                self.lines.push(line);
            }
            EditKind::Multi => {
                println!("{:?}", text);
                let mut line = EditLine::new();
                for cchar in text.chars() {
                    if cchar == '\n' {
                        self.draw_text.push('\n');
                        line.auto_wrap = false;
                        let line = mem::take(&mut line);
                        self.lines.push(line);
                    } else {
                        let width = ui.context.font.char_width(cchar, self.font_size);
                        if line.width + width > self.max_wrap_width { //需要换行
                            self.draw_text.push('\n');
                            line.auto_wrap = true;
                            let line = mem::take(&mut line);
                            self.lines.push(line);
                        }
                        self.draw_text.push(cchar);
                        line.push(EditChar::new(cchar, width));
                    }
                }
                line.auto_wrap = true;
                if line.chars.len() != 0 || text.len() == 0 { self.lines.push(line); }
            }
        }
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
    }

    pub fn set_line_height(&mut self, line_height: f32) {
        self.line_height = line_height;
    }

    pub fn set_max_wrap_width(&mut self, max_wrap_width: f32) {
        self.max_wrap_width = max_wrap_width;
    }

    pub fn draw_text(&self) -> &str { &self.draw_text }

    pub fn raw_text(&self) -> String {
        self.lines.iter().map(|x| x.raw_text()).collect()
        // self.chars.iter().map(|c| c.cchar.to_string()).collect()
    }

    // pub fn remove_by_range(&mut self, ui: &mut Ui, cursor: &mut EditCursor, rebuild: bool) {
    //     for i in self.selected.clone() {
    //         if self.line_index.contains(&i) && self.edit_kind == EditKind::Multi { self.offset.y -= self.line_height; }
    //         let cchar = self.chars.remove(self.selected.start);
    //         if self.edit_kind == EditKind::Single { self.offset.x -= cchar.width; }
    //         cursor.move_left(self);
    //         // self.cursor -= 1;
    //     }
    //     if self.offset.x < 0.0 { self.offset.x = 0.0; }
    //     if rebuild {
    //         let raw_text = self.raw_text();
    //         self.set_text(&raw_text, ui);
    //     }
    // }

    pub fn remove_chars_before_cursor(&mut self, ui: &mut Ui, cursor: &mut EditCursor) {
        if cursor.horiz == 0 && cursor.vert == 0 { return; }
        if self.selected.start == self.selected.end {
            let cchar = cursor.delete_before(self);
            match self.edit_kind {
                EditKind::Single => {
                    self.offset.x -= cchar.width;
                    if self.offset.x < 0.0 { self.offset.x = 0.0; }
                }
                EditKind::Multi => {
                    let raw_text = self.raw_text();
                    self.set_text(&raw_text, ui);
                }
            }
        } else {
            // self.remove_by_range(ui, cursor, true)
        }
    }

    pub fn remove_chars_after_cursor(&mut self, ui: &mut Ui, cursor: &mut EditCursor) {
        if cursor.vert == self.lines.len() - 1 && cursor.horiz == self.lines.last().unwrap().len() { return; }
        if self.selected.start == self.selected.end {
            let cchar = cursor.delete_after(self);
            match self.edit_kind {
                EditKind::Single => {
                    self.offset.x -= cchar.width;
                    if self.offset.x < 0.0 { self.offset.x = 0.0; }
                }
                EditKind::Multi => {
                    let raw_text = self.raw_text();
                    self.set_text(&raw_text, ui);
                }
            }
        } else {
            // self.remove_by_range(ui, cursor, true)
        }
    }

    // pub fn current_char(&self, cursor: usize) -> Option<&EditChar> {
    //     if cursor == 0 { return None; }
    //     self.chars.get(cursor-1)
    // }

    // pub fn next_char(&self) -> Option<&EditChar> {
    //     if self.cursor >= self.chars.len() { return None; }
    //     Some(&self.chars[self.cursor])
    // }
    //
    // pub fn previous_char(&self) -> Option<&EditChar> {
    //     if self.cursor == 0 { return None; }
    //     if self.cursor - 1 == 0 { return Some(&self.chars[0]); }
    //     Some(&self.chars[self.cursor - 2])
    // }
    //
    pub fn inset_char(&mut self, c: char, ui: &mut Ui, cursor: &mut EditCursor) { //返回x最大值 ，给游标偏移
        if self.selected.start != self.selected.end {
            // self.remove_by_range(ui, cursor, true);
        }
        let width = ui.context.font.char_width(c, self.font_size);
        let cchar = EditChar::new(c, width);
        let line = &mut self.lines[cursor.vert];
        line.chars.insert(cursor.horiz, cchar);
        let raw_text = self.raw_text();
        self.set_text(&raw_text, ui);
        cursor.move_right(self);
        if cursor.offset.x == 0.0 {
            cursor.move_right(self)
        }
        println!("insert char-{}-{}", cursor.vert, cursor.horiz);
    }
}