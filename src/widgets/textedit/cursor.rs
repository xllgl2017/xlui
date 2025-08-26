use crate::Offset;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::size::border::Border;
use crate::size::pos::Pos;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::ClickStyle;
use crate::style::color::Color;
use crate::text::text_buffer::TextBuffer;
use crate::ui::Ui;
use crate::widgets::textedit::buffer::{CharBuffer, EditChar};

pub struct EditCursor {
    pub(crate) min_pos: Pos,
    pub(crate) horiz: usize,
    pub(crate) vert: usize,
    render: RenderParam<RectParam>,
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
        cursor_style.border.inactive = Border::new(0.0).radius(Radius::same(0));
        cursor_style.border.hovered = Border::new(0.0).radius(Radius::same(0));
        cursor_style.border.clicked = Border::new(0.0).radius(Radius::same(0));
        EditCursor {
            min_pos: Pos::new(),
            horiz: 0,
            vert: 0,
            render: RenderParam::new(RectParam::new(Rect::new(), cursor_style)),
            offset: Offset::new(Pos::new()),
            line_height: 0.0,
            changed: false,

        }
    }

    pub fn init(&mut self, text: &TextBuffer, cchar: &CharBuffer, ui: &mut Ui, init: bool) {
        if init {
            self.min_pos.x = text.rect.dx().min;
            self.min_pos.y = text.rect.dy().min;
            self.line_height = text.text.height;
            self.vert = cchar.lines.len();
            let last_line = cchar.lines.last().unwrap();
            self.horiz = cchar.lines.last().unwrap().len();
            let oy = (self.vert - 1) as f32 * text.text.height;
            self.offset.x = last_line.width;
            self.offset.y = oy;
            self.render.param.rect.offset(&self.offset);
        }
        self.render.init_rectangle(ui, false, false);
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if !self.changed { return; }
        self.changed = false;
        self.render.param.rect.offset(&self.offset);
        self.render.update(ui, false, false);
    }

    pub fn offset(&mut self, offset: &Offset) {
        self.min_pos.x += offset.x;
        self.min_pos.y += offset.y;
        self.render.param.rect.offset(offset);
        self.changed = true;
    }

    pub fn render(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.render, pass);
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.render.param.rect = rect;
    }

    pub fn move_left(&mut self, cchar: &CharBuffer) {
        println!("move left-{}-{}", self.horiz, self.vert);
        if self.horiz == 0 && self.vert == 0 { return; }
        if self.horiz == 0 {
            self.vert -= 1;
            self.horiz = cchar.lines[self.vert].len();
            self.offset.y -= self.line_height;
            self.offset.x = cchar.lines[self.vert].width;
        } else {
            self.offset.x -= cchar.lines[self.vert].chars[self.horiz - 1].width;
            self.horiz -= 1;
        }
        self.changed = true;
    }

    pub fn delete_before(&mut self, cchar: &mut CharBuffer) -> EditChar {
        self.changed = true;
        if self.horiz == 0 {
            self.vert -= 1;
            let line = &mut cchar.lines[self.vert];
            self.horiz = if line.auto_wrap { line.chars.len() - 1 } else { line.chars.len() };
            let c = if line.auto_wrap { line.chars.remove(self.horiz) } else { EditChar::new('\n', 0.0) };
            line.auto_wrap = true;
            self.offset.x = line.width - c.width;
            self.offset.y -= self.line_height;
            c
        } else {
            self.horiz -= 1;
            let line = &mut cchar.lines[self.vert];
            let c = line.chars.remove(self.horiz);
            self.offset.x -= c.width;
            c
        }
    }

    pub fn delete_after(&mut self, cchar: &mut CharBuffer) -> EditChar {
        let len = cchar.lines.len();
        let line = &mut cchar.lines[self.vert];
        let c = if self.horiz == line.len() && self.vert < len && self.horiz == 0 {
            let wrap = line.auto_wrap;
            line.auto_wrap = true;
            if wrap { cchar.lines[self.vert + 1].chars.remove(0) } else { EditChar::new('\n', 0.0) }
        } else if self.horiz < line.len() {
            line.chars.remove(self.horiz)
        } else { EditChar::new(' ', 0.0) };
        self.changed = true;
        c
    }

    pub fn move_right(&mut self, cchar: &CharBuffer) {
        println!("move right-{}-{}", self.horiz, self.vert);
        let line = &cchar.lines[self.vert];
        if self.horiz == line.len() {
            if self.vert == cchar.lines.len() - 1 { return; }
            self.vert += 1;
            self.horiz = 0;
            self.offset.x = 0.0;
            self.offset.y += self.line_height;
        } else {
            self.horiz += 1;
            self.offset.x += line.chars[self.horiz - 1].width;
        }
        self.changed = true;
    }

    fn up_down_offset(&mut self, cchar: &CharBuffer) {
        let line = &cchar.lines[self.vert];
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
        if self.vert == cchar.lines.len() - 1 { return; }
        self.vert += 1;
        self.up_down_offset(cchar);
        println!("current cursor {} {}", self.vert, self.horiz);
        self.offset.y += self.line_height;
        self.changed = true;
    }

    pub fn update_by_pos(&mut self, pos: Pos, cchar: &CharBuffer) {
        let clip_height = pos.y + cchar.offset.y - self.min_pos.y;
        let mut line_index = (clip_height / self.line_height) as usize;
        if line_index >= cchar.lines.len() { line_index = cchar.lines.len() - 1; }
        self.vert = line_index;
        let line = &cchar.lines[self.vert];
        let mut sum_width = 0.0;
        self.horiz = 0;
        let mut char_width = 0.0;
        for cc in line.chars.iter() {
            let char_min = self.min_pos.x + sum_width;
            let char_max = self.min_pos.x + sum_width + cc.width;
            if pos.x + cchar.offset.x > char_min && pos.x + cchar.offset.x < char_max {
                char_width = cc.width;
                break;
            }
            sum_width += cc.width;
            self.horiz += 1;
        }
        let char_min = self.min_pos.x + sum_width;
        if pos.x + cchar.offset.x < char_min {
            self.horiz = 0;
            self.offset.x = 0.0;
        } else if pos.x + cchar.offset.x > char_min + char_width / 2.0 {
            if self.horiz >= line.len() { self.horiz = line.chars.len() } else { self.horiz += 1; }
            self.offset.x = sum_width + char_width;
        } else {
            self.offset.x = sum_width;
        }
        self.offset.y = self.line_height * line_index as f32;
        println!("1111111111111111111111111111-{}-{}", self.vert, self.horiz);
        self.changed = true;
    }

    pub fn cursor_min(&self) -> f32 {
        self.min_pos.x + self.offset.x
    }

    pub fn set_cursor(&mut self, horiz: usize, vert: usize, cchar: &CharBuffer) {
        self.horiz = horiz;
        self.vert = vert;
        self.offset.x = cchar.lines[vert].get_width_in_char(horiz);
        self.offset.y = vert as f32 * self.line_height;
        self.changed = true;
    }
}