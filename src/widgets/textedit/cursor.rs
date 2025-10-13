use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::size::border::Border;
use crate::size::pos::Pos;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::text::cchar::CChar;
use crate::ui::Ui;
use crate::widgets::textedit::buffer::CharBuffer;
use crate::Offset;

pub struct EditCursor {
    pub(crate) min_pos: Pos,
    pub(crate) max_pos: Pos,
    pub(crate) horiz: usize,
    pub(crate) vert: usize,
    render: RenderParam,
    pub(crate) offset: Offset,
    line_height: f32,
    pub(crate) changed: bool,
}

impl EditCursor {
    pub fn new() -> EditCursor {
        let mut cursor_style = ClickStyle::new();
        cursor_style.fill.inactive = Color::rgb(0, 83, 125);
        cursor_style.fill.hovered = Color::rgb(0, 83, 125);
        cursor_style.fill.clicked = Color::rgb(0, 83, 125);
        cursor_style.border.inactive = Border::same(0.0).radius(Radius::same(0));
        cursor_style.border.hovered = Border::same(0.0).radius(Radius::same(0));
        cursor_style.border.clicked = Border::same(0.0).radius(Radius::same(0));
        EditCursor {
            min_pos: Pos::new(),
            max_pos: Pos::new(),
            horiz: 0,
            vert: 0,
            render: RenderParam::new(RenderKind::Rectangle(RectParam::new().with_style(cursor_style))),
            offset: Offset::new(),
            line_height: 0.0,
            changed: false,

        }
    }

    pub fn init(&mut self, cchar: &CharBuffer, ui: &mut Ui, init: bool) {
        if init {
            self.line_height = cchar.buffer.text.height;
            self.vert = cchar.buffer.lines.len();
            self.horiz = cchar.buffer.lines.last().unwrap().len();
        }
        #[cfg(feature = "gpu")]
        self.render.init(ui, false, false);
    }

    pub fn reset_x(&mut self, cchar: &CharBuffer) {
        self.offset.x = cchar.buffer.lines[self.vert].get_width_in_char(self.horiz);
        self.changed = true;
    }

    pub fn update(&mut self) {
        if !self.changed { return; }
        self.changed = false;
        self.render.rect_mut().offset(&self.offset);
    }


    pub fn render(&mut self, ui: &mut Ui) {
        self.render.draw(ui, false, false);
    }

    pub fn update_position(&mut self, ui: &mut Ui, rect: Rect, cchar: &CharBuffer) {
        *self.render.rect_mut() = rect;
        self.render.rect_mut().offset(&self.offset);
        #[cfg(feature = "gpu")]
        self.render.update(ui, false, false);
        self.min_pos.x = cchar.buffer.geometry.x();
        self.min_pos.y = cchar.buffer.geometry.y();
        self.max_pos.x = cchar.buffer.geometry.right();
        self.max_pos.y = cchar.buffer.geometry.bottom();
    }

    pub fn move_left(&mut self, cchar: &CharBuffer) {
        println!("move left-{}-{}", self.horiz, self.vert);
        if self.horiz == 0 && self.vert == 0 { return; }
        if self.horiz == 0 {
            self.vert -= 1;
            self.horiz = cchar.buffer.lines[self.vert].len();
            self.offset.y -= self.line_height;
            self.offset.x = cchar.buffer.lines[self.vert].width + cchar.offset.x;
        } else {
            self.horiz -= 1;
            self.offset.x = cchar.buffer.lines[self.vert].get_width_in_char(self.horiz) + cchar.offset.x;

        }
        self.changed = true;
    }

    pub fn delete_before(&mut self, cchar: &mut CharBuffer) -> CChar {
        self.changed = true;
        if self.horiz == 0 {
            self.vert -= 1;
            let line = &mut cchar.buffer.lines[self.vert];
            self.horiz = if line.auto_wrap { line.chars.len() - 1 } else { line.chars.len() };
            let c = if line.auto_wrap { line.chars.remove(self.horiz) } else { CChar::new('\n', 0.0) };
            line.auto_wrap = true;
            self.offset.x = line.width - c.width;
            self.offset.y -= self.line_height;
            c
        } else {
            let line = &mut cchar.buffer.lines[self.vert];
            let c = line.chars.remove(self.horiz - 1);
            cchar.offset.x += c.width;
            if cchar.offset.x >= 0.0 { cchar.offset.x = 0.0; }
            self.horiz -= 1;
            self.offset.x = line.get_width_in_char(self.horiz) + cchar.offset.x;
            c
        }
    }

    pub fn delete_after(&mut self, cchar: &mut CharBuffer) -> CChar {
        let len = cchar.buffer.lines.len();
        let line = &mut cchar.buffer.lines[self.vert];
        let c = if self.horiz == line.len() && self.vert < len {
            let wrap = line.auto_wrap;
            line.auto_wrap = true;
            if wrap && self.vert + 1 < len { cchar.buffer.lines[self.vert + 1].chars.remove(0) } else { CChar::new('\n', 0.0) }
        } else if self.horiz < line.len() {
            let c = line.chars.remove(self.horiz);
            cchar.offset.x += c.width;
            if cchar.offset.x >= 0.0 { cchar.offset.x = 0.0; }
            c
        } else { CChar::new(' ', 0.0) };
        self.changed = true;
        c
    }

    pub fn move_right(&mut self, cchar: &CharBuffer) {
        println!("move right-{}-{}", self.horiz, self.vert);
        let line = &cchar.buffer.lines[self.vert];
        if self.horiz == line.len() {
            if self.vert == cchar.buffer.lines.len() - 1 { return; }
            self.vert += 1;
            self.horiz = 0;
            self.offset.x = 0.0;
            self.offset.y += self.line_height;
        } else {
            self.horiz += 1;
            self.offset.x = line.get_width_in_char(self.horiz) + cchar.offset.x;
        }
        self.changed = true;
    }

    fn up_down_offset(&mut self, cchar: &CharBuffer) {
        let line = &cchar.buffer.lines[self.vert];
        let mut sum_width = 0.0;
        self.horiz = 0;
        for c in line.chars.iter() {
            self.horiz += 1;
            if self.offset.x < sum_width || self.offset.x > sum_width + c.width {
                sum_width += c.width;
                continue;
            }
            self.offset.x = if self.offset.x > sum_width + c.width / 2.0 { sum_width + c.width } else { sum_width };
            self.changed = true;
            break;
        }
        if self.offset.x == 0.0 {
            self.horiz -= 1;
        }
        if self.horiz >= line.len() {
            self.offset.x = line.width;
        }
    }

    pub fn move_up(&mut self, cchar: &CharBuffer) {
        if self.vert == 0 { return; }
        self.vert -= 1;
        self.up_down_offset(cchar);
        println!("current cursor {}-{}", self.vert, self.horiz);
        self.offset.y -= self.line_height;
        self.changed = true;
    }

    pub fn move_down(&mut self, cchar: &CharBuffer) {
        if self.vert == cchar.buffer.lines.len() - 1 { return; }
        self.vert += 1;
        self.up_down_offset(cchar);
        println!("current cursor {} {}", self.vert, self.horiz);
        self.offset.y += self.line_height;
        self.changed = true;
    }

    pub fn update_by_pos(&mut self, pos: Pos, cchar: &mut CharBuffer) {
        let clip_height = pos.y + cchar.offset.y - self.min_pos.y;
        let mut line_index = (clip_height / self.line_height) as usize;
        if line_index >= cchar.buffer.lines.len() { line_index = cchar.buffer.lines.len() - 1; }
        self.vert = line_index;
        let line = &cchar.buffer.lines[self.vert];


        if pos.x > self.max_pos.x && self.min_pos.x + line.width > self.max_pos.x {
            if self.horiz < line.len() {
                self.horiz += 1;
                let char = &line.chars[self.horiz - 1];
                cchar.offset.x -= char.width;
                self.offset.x = line.get_width_in_char(self.horiz) + cchar.offset.x;
            } else {
                self.horiz = line.len();
                let mo = self.min_pos.x + line.width - self.max_pos.x;
                cchar.offset.x = -mo;
                self.offset.x = line.width + cchar.offset.x;
            }
            return;
        } else if pos.x < self.min_pos.x && self.min_pos.x + line.width > self.max_pos.x {
            if self.horiz > 0 { self.horiz -= 1; }
            let char = &line.chars[self.horiz];
            self.offset.x -= char.width;
            if self.offset.x <= 0.0 { self.offset.x = 0.0; }
            cchar.offset.x += char.width;
            if cchar.offset.x >= 0.0 { cchar.offset.x = 0.0; }
            return;
        }


        let mut sum_width = 0.0;
        let mut horiz = 0;
        let mut char_width = 0.0;
        for cc in line.chars.iter() {
            let char_min = self.min_pos.x + sum_width + cchar.offset.x;
            let char_max = self.min_pos.x + sum_width + cchar.offset.x + cc.width;
            if pos.x >= char_min && pos.x <= char_max {
                char_width = cc.width;
                break;
            }
            sum_width += cc.width;
            horiz += 1;
        }
        let char_min = self.min_pos.x + sum_width + cchar.offset.x;
        println!("{}-{}-{}-{}-{}", pos.x, self.min_pos.x, char_min + char_width / 2.0, horiz, sum_width);
        if pos.x < self.min_pos.x {
            self.horiz = 0;
            self.offset.x = 0.0;
        } else if pos.x > char_min + char_width / 2.0 {
            if horiz >= line.len() { self.horiz = line.chars.len() } else { self.horiz = horiz + 1; }
            self.offset.x = sum_width + char_width + cchar.offset.x;
        } else {
            self.horiz = horiz;
            self.offset.x = sum_width + cchar.offset.x;
        }
        self.offset.y = self.line_height * line_index as f32;
        if self.offset.x + self.min_pos.x > self.max_pos.x { self.offset.x = self.max_pos.x - self.min_pos.x; }
        if self.offset.y + self.min_pos.y > self.max_pos.y { self.offset.y = self.max_pos.y - self.min_pos.y; }
        println!("1111111111111111111111111111-{}-{}", self.vert, self.horiz);
        self.changed = true;
    }

    pub fn cursor_min(&self) -> f32 {
        self.min_pos.x + self.offset.x
    }

    pub fn set_cursor(&mut self, horiz: usize, vert: usize, cchar: &CharBuffer) {
        println!("remove-cursor-{}-{}", horiz, vert);
        self.horiz = horiz;
        self.vert = vert;
        self.offset.x = cchar.buffer.lines[vert].get_width_in_char(horiz) + cchar.offset.x;
        self.offset.y = vert as f32 * self.line_height;
        self.changed = true;
    }
}