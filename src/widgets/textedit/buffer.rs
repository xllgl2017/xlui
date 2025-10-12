use crate::text::buffer::TextBuffer;
use crate::text::cchar::CChar;
use crate::ui::Ui;
use crate::widgets::textedit::cursor::EditCursor;
use crate::widgets::textedit::select::EditSelection;
use crate::widgets::textedit::EditKind;
use crate::{Offset, Padding, RichTextExt, TextWrap};

pub(crate) struct CharBuffer {
    pub(crate) buffer: TextBuffer,
    font_size: f32,
    line_height: f32,
    max_wrap_width: f32,
    pub(crate) offset: Offset,
    pub(super) edit_kind: EditKind,
    pub(crate) looking: bool,
}

impl CharBuffer {
    pub fn new(text: impl ToString) -> CharBuffer {
        CharBuffer {
            buffer: TextBuffer::new(text.to_string().wrap(TextWrap::WrapAny)).fix_width(200.0).padding(Padding::same(3.0)),
            font_size: 0.0,
            line_height: 0.0,
            max_wrap_width: 0.0,
            offset: Offset::new(),
            edit_kind: EditKind::Multi,
            looking: false,
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

    pub fn remove_by_range(&mut self, ui: &mut Ui, cursor: &mut EditCursor, selection: &mut EditSelection) {
        if cursor.vert > selection.start_vert { //向下删除
            let start_line = &mut self.buffer.lines[selection.start_vert];
            while start_line.chars.len() > selection.start_horiz {
                let c = start_line.chars.remove(selection.start_horiz);
                start_line.width -= c.width;
            }
            start_line.auto_wrap = true;
            let end_line = &mut self.buffer.lines[cursor.vert];
            let len = end_line.chars.len() - cursor.horiz;
            while end_line.len() > len {
                let c = end_line.chars.remove(0);
                end_line.width -= c.width;
            }
            for i in selection.start_vert + 1..cursor.vert {
                self.buffer.lines.remove(cursor.vert - i);
            }
            self.rebuild_text(ui);
            cursor.set_cursor(selection.start_horiz, selection.start_vert, self);
        } else if selection.start_vert > cursor.vert { //向上删除
            let start_line = &mut self.buffer.lines[selection.start_vert];
            let len = start_line.chars.len() - selection.start_horiz;
            while start_line.len() > len {
                let c = start_line.chars.remove(0);
                start_line.width -= c.width;
            }
            let end_line = &mut self.buffer.lines[cursor.vert];
            while end_line.chars.len() > cursor.horiz {
                let c = end_line.chars.remove(cursor.horiz);
                end_line.width -= c.width;
            }
            end_line.auto_wrap = true;
            for i in cursor.vert + 1..selection.start_vert {
                self.buffer.lines.remove(selection.start_vert - i);
            }
            self.rebuild_text(ui);
        } else { //同行删除
            let line = &mut self.buffer.lines[cursor.vert];
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
        selection.reset(cursor);
        selection.has_selected = false;
    }

    pub(crate) fn rebuild_text(&mut self, ui: &mut Ui) {
        let text: String = self.buffer.lines.iter().map(|x| x.raw_text()).collect();
        println!("rebuild text: {:?}", text);
        if let EditKind::Password = self.edit_kind && !self.looking {
            let text = vec!["●"; text.chars().count()];
            self.buffer.update_if_not(ui, &text.join(""), false);
            self.buffer.lines.iter_mut().for_each(|line| {
                let mut width = 0.0;
                line.chars.iter_mut().for_each(|x| {
                    x.width = 14.0;
                    width += 14.0;
                });
                line.width = width;
            });
        } else {
            self.buffer.update_if_not(ui, &text, true);
        }
    }

    pub fn remove_chars_before_cursor(&mut self, ui: &mut Ui, cursor: &mut EditCursor, selection: &mut EditSelection) {
        if cursor.horiz == 0 && cursor.vert == 0 { return; }
        println!("delete-before-{}-{}", selection.has_selected, cursor.horiz);
        if !selection.has_selected {
            cursor.delete_before(self);
            self.rebuild_text(ui)
        } else {
            self.remove_by_range(ui, cursor, selection)
        }
    }

    pub fn remove_chars_after_cursor(&mut self, ui: &mut Ui, cursor: &mut EditCursor, selection: &mut EditSelection) {
        println!("{} {} {} {}", cursor.vert, self.buffer.lines.len() - 1, cursor.horiz, self.buffer.lines.last().unwrap().len());
        if cursor.vert == self.buffer.lines.len() - 1 && cursor.horiz == self.buffer.lines.last().unwrap().len() && cursor.horiz == 0 { return; }
        if !selection.has_selected {
            cursor.delete_after(self);
            self.rebuild_text(ui);
        } else {
            self.remove_by_range(ui, cursor, selection);
        }
    }

    pub fn inset_char(&mut self, c: char, ui: &mut Ui, cursor: &mut EditCursor, selection: &mut EditSelection) { //返回x最大值 ，给游标偏移
        if selection.has_selected {
            self.remove_by_range(ui, cursor, selection);
        }
        let cchar = CChar::new(c, 0.0);
        let line = &mut self.buffer.lines[cursor.vert];
        line.chars.insert(cursor.horiz, cchar);
        self.rebuild_text(ui);
        let line = &mut self.buffer.lines[cursor.vert];
        println!("insert before-{}-{}", line.chars.len(), cursor.horiz);
        let width = if line.len() == 0 { 0.0 } else { line.chars[if cursor.horiz == 0 { 0 } else { cursor.horiz - 1 }].width };
        let horiz = if cursor.horiz + 1 >= line.chars.len() { line.chars.len() } else { cursor.horiz + 1 };
        if cursor.min_pos.x + line.get_width_in_char(horiz) > cursor.max_pos.x {
            self.offset.x -= width;
        }
        cursor.move_right(self);
        if cursor.offset.x == 0.0 && c != '\n' {
            cursor.move_right(self)
        }

        println!("insert char-{}-{}", cursor.vert, cursor.horiz);
    }

    pub fn next_char(&self, cursor: &EditCursor) -> Option<&CChar> {
        if cursor.horiz >= self.buffer.lines[cursor.vert].len() { return None; }
        self.buffer.lines.get(cursor.vert)?.chars.get(cursor.horiz)
    }

    pub fn previous_char(&self, cursor: &EditCursor) -> Option<&CChar> {
        if cursor.horiz == 0 { return None; }
        if cursor.horiz - 1 == 0 { return self.buffer.lines.get(cursor.vert)?.chars.get(0); }
        self.buffer.lines.get(cursor.vert)?.chars.get(cursor.horiz - 2)
    }

    #[cfg(not(feature = "winit"))]
    pub fn select_text(&self, select: &EditSelection, cursor: &EditCursor) -> String {
        if !select.has_selected { return "".to_string(); }
        let mut chars = vec![];
        if select.start_vert != cursor.vert {
            let (start_vert, start_horiz, end_vert, end_horiz) = if select.start_vert < cursor.vert {
                (select.start_vert, select.start_horiz, cursor.vert, cursor.horiz)
            } else {
                (cursor.vert, cursor.horiz, select.start_vert, select.start_horiz)
            };
            let start_line = &self.buffer.lines[start_vert];
            chars.push(start_line.get_text_by_range(start_horiz..start_line.len()));
            for i in start_vert + 1..end_vert {
                chars.push(self.buffer.lines[i].raw_text());
            }
            let end_line = &self.buffer.lines[end_vert];
            chars.push(end_line.get_text_by_range(0..end_horiz));
        } else {
            let (start_horiz, end_horiz) = if select.start_horiz < cursor.horiz {
                (select.start_horiz, cursor.horiz)
            } else {
                (cursor.horiz, select.start_horiz)
            };
            chars.push(self.buffer.lines[select.start_vert].get_text_by_range(start_horiz..end_horiz))
        }
        chars.join("")
    }
}