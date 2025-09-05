use crate::size::pos::Pos;
use crate::ui::Ui;
use crate::widgets::textedit::cursor::EditCursor;
use crate::widgets::textedit::select::EditSelection;
use crate::widgets::textedit::EditKind;
use crate::Offset;
use std::mem;

#[derive(Debug)]
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
    pub(crate) auto_wrap: bool,
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

    pub fn get_width_in_char(&self, index: usize) -> f32 {
        let mut width = 0.0;
        // let index = if index >= self.chars.len() { return self.width } else { index };
        self.chars[..index].iter().for_each(|x| width += x.width);
        width
    }

    pub fn len(&self) -> usize { self.chars.len() }
}


pub(crate) struct CharBuffer {
    pub(crate) lines: Vec<EditLine>,
    draw_text: String,
    font_size: f32,
    line_height: f32,
    max_wrap_width: f32,
    pub(crate) offset: Offset,
    pub(super) edit_kind: EditKind,
}

impl CharBuffer {
    pub fn new() -> CharBuffer {
        CharBuffer {
            lines: vec![],
            draw_text: "".to_string(),
            font_size: 0.0,
            line_height: 0.0,
            max_wrap_width: 0.0,
            offset: Offset::new(Pos::new()),
            edit_kind: EditKind::Multi,
        }
    }
    pub fn set_text(&mut self, text: &str, ui: &mut Ui) {
        self.draw_text.clear();
        self.lines.clear();
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
    }

    pub fn remove_by_range(&mut self, ui: &mut Ui, cursor: &mut EditCursor, selection: &EditSelection) {
        if cursor.vert > selection.start_vert { //向下删除
            let start_line = &mut self.lines[selection.start_vert];
            while start_line.chars.len() > selection.start_horiz {
                let c = start_line.chars.remove(selection.start_horiz);
                start_line.width -= c.width;
            }
            start_line.auto_wrap = true;
            let end_line = &mut self.lines[cursor.vert];
            let len = end_line.chars.len() - cursor.horiz;
            while end_line.len() > len {
                let c = end_line.chars.remove(0);
                end_line.width -= c.width;
            }
            for i in selection.start_vert + 1..cursor.vert {
                self.lines.remove(cursor.vert - i);
            }
            self.rebuild_text(ui);
            cursor.set_cursor(selection.start_horiz, selection.start_vert, self);
        } else if selection.start_vert > cursor.vert { //向上删除
            let start_line = &mut self.lines[selection.start_vert];
            let len = start_line.chars.len() - selection.start_horiz;
            while start_line.len() > len {
                let c = start_line.chars.remove(0);
                start_line.width -= c.width;
            }
            let end_line = &mut self.lines[cursor.vert];
            while end_line.chars.len() > cursor.horiz {
                let c = end_line.chars.remove(cursor.horiz);
                end_line.width -= c.width;
            }
            end_line.auto_wrap = true;
            for i in cursor.vert + 1..selection.start_vert {
                self.lines.remove(selection.start_vert - i);
            }
            self.rebuild_text(ui);
        } else { //同行删除
            let line = &mut self.lines[cursor.vert];
            if selection.start_horiz < cursor.horiz { //左到右删除
                for _ in selection.start_horiz..cursor.horiz {
                    let c = line.chars.remove(selection.start_horiz);
                    self.offset.x += c.width;
                    if self.offset.x >= 0.0 { self.offset.x = 0.0; }
                    cursor.horiz -= 1;
                }
                cursor.set_cursor(cursor.horiz, cursor.vert, self);
                cursor.changed = true;
            } else { //右到左删除
                for _ in cursor.horiz..selection.start_horiz {
                    let c = line.chars.remove(cursor.horiz);
                    self.offset.x += c.width;
                    if self.offset.x >= 0.0 { self.offset.x = 0.0; }
                }
                cursor.set_cursor(cursor.horiz, cursor.vert, self);
                cursor.changed = true;
            }
            self.rebuild_text(ui);
        }
    }

    fn rebuild_text(&mut self, ui: &mut Ui) {
        let raw_text = self.raw_text();
        self.set_text(&raw_text, ui);
    }

    pub fn remove_chars_before_cursor(&mut self, ui: &mut Ui, cursor: &mut EditCursor, selection: &EditSelection) {
        if cursor.horiz == 0 && cursor.vert == 0 { return; }
        println!("delete-before-{}-{}", selection.has_selected, cursor.horiz);
        if !selection.has_selected {
            cursor.delete_before(self);
            self.rebuild_text(ui)
        } else {
            self.remove_by_range(ui, cursor, selection)
        }
    }

    pub fn remove_chars_after_cursor(&mut self, ui: &mut Ui, cursor: &mut EditCursor, selection: &EditSelection) {
        println!("{} {} {} {}", cursor.vert, self.lines.len() - 1, cursor.horiz, self.lines.last().unwrap().len());
        if cursor.vert == self.lines.len() - 1 && cursor.horiz == self.lines.last().unwrap().len() && cursor.horiz == 0 { return; }
        if !selection.has_selected {
            cursor.delete_after(self);
            self.rebuild_text(ui);
        } else {
            self.remove_by_range(ui, cursor, selection);
        }
    }

    pub fn inset_char(&mut self, c: char, ui: &mut Ui, cursor: &mut EditCursor, selection: &EditSelection) { //返回x最大值 ，给游标偏移
        if selection.has_selected {
            self.remove_by_range(ui, cursor, selection);
        }
        let width = ui.context.font.char_width(c, self.font_size);
        let cchar = EditChar::new(c, width);
        let line = &mut self.lines[cursor.vert];
        line.chars.insert(cursor.horiz, cchar);
        self.rebuild_text(ui);
        let line = &mut self.lines[cursor.vert];
        if cursor.min_pos.x + line.get_width_in_char(cursor.horiz + 1) > cursor.max_pos.x {
            self.offset.x -= width;
        }
        cursor.move_right(self);
        if cursor.offset.x == 0.0 && c != '\n' {
            cursor.move_right(self)
        }

        println!("insert char-{}-{}", cursor.vert, cursor.horiz);
    }

    pub fn next_char(&self, cursor: &EditCursor) -> Option<&EditChar> {
        if cursor.horiz >= self.lines[cursor.vert].len() { return None; }
        self.lines.get(cursor.vert)?.chars.get(cursor.horiz)
    }

    pub fn previous_char(&self, cursor: &EditCursor) -> Option<&EditChar> {
        if cursor.horiz == 0 { return None; }
        if cursor.horiz - 1 == 0 { return self.lines.get(cursor.vert)?.chars.get(0); }
        self.lines.get(cursor.vert)?.chars.get(cursor.horiz - 2)
    }
}