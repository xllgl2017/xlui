use crate::frame::context::UpdateType;
use crate::render::{RenderParam, VisualStyle, WidgetStyle};
use crate::shape::Shape;
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::ui::Ui;
use crate::widgets::textedit::buffer::CharBuffer;
use crate::widgets::textedit::cursor::EditCursor;
use crate::window::UserEvent;
use crate::{Offset, Shadow};

pub struct EditSelection {
    renders: Vec<RenderParam>,
    changed: bool,
    pub(crate) has_selected: bool,
    pub(crate) start_vert: usize,
    pub(crate) start_horiz: usize,
}

impl EditSelection {
    pub fn new() -> EditSelection {
        EditSelection {
            renders: vec![],
            changed: false,
            has_selected: false,
            start_vert: 0,
            start_horiz: 0,
        }
    }

    pub fn init(&mut self, rows: usize, line_height: f32) {
        let mut select_style = VisualStyle::same(WidgetStyle {
            fill: Color::rgba(144, 209, 255, 100),
            border: Border::same(0.0),
            radius: Radius::same(0),
            shadow: Shadow::new(),
        });
        select_style.inactive.fill = Color::rgba(255, 0, 0, 100);
        for row in 0..rows {
            let mut render = RenderParam::new(Shape::rectangle()).with_style(select_style.clone()).with_size(0.0, line_height);
            render.rect_mut().offset(&Offset::new().with_y(row as f32 * line_height).covered());
            self.renders.push(render);
        }
    }

    pub fn set_by_cursor(&mut self, cursor: &EditCursor) {
        let x = cursor.cursor_min();
        for (index, render) in self.renders.iter_mut().enumerate() {
            if index == cursor.vert {
                render.rect_mut().set_x_min(x);
                render.rect_mut().set_x_max(x);
            } else {
                let min = render.rect().dx().min;
                render.rect_mut().set_x_max(min);
            }
        }
        self.start_vert = cursor.vert;
        self.start_horiz = cursor.horiz;
        self.changed = true;
    }

    pub fn render(&mut self, ui: &mut Ui, rows: usize) {
        for (index, render) in self.renders.iter_mut().enumerate() {
            if index >= rows { continue; }
            render.draw(ui, false, false, false);
        }
    }

    pub fn reset(&mut self, cursor: &EditCursor) {
        for render in self.renders.iter_mut() {
            let min = render.rect().dx().min;
            render.rect_mut().set_x_max(min);
        }
        self.start_vert = cursor.vert;
        self.start_horiz = cursor.horiz;
        self.changed = true;
        self.has_selected = false;
    }

    pub fn move_select(&mut self, ui: &mut Ui, cursor: &mut EditCursor, cchar: &mut CharBuffer) {
        let pos = ui.device.device_input.mouse.lastest.relative;
        cursor.update_by_pos(pos, cchar);
        if cursor.vert > self.start_vert { //向下选择
            for (index, render) in self.renders.iter_mut().enumerate() {
                if index == self.start_vert {
                    let line = &cchar.buffer.lines[index];
                    render.rect_mut().set_x_min(line.get_width_in_char(self.start_horiz) + cursor.min_pos.x);
                    render.rect_mut().set_x_max(line.width + cursor.min_pos.x);
                } else if index == cursor.vert {
                    render.rect_mut().set_x_min(cursor.min_pos.x);
                    render.rect_mut().set_x_max(cursor.cursor_min());
                } else if index > self.start_vert && index < cursor.vert {
                    let line = &cchar.buffer.lines[index];
                    render.rect_mut().set_x_min(cursor.min_pos.x);
                    render.rect_mut().set_x_max(cursor.min_pos.x + line.width);
                } else {
                    let min = render.rect().dx().min;
                    render.rect_mut().set_x_max(min);
                }
            }
        } else if cursor.vert < self.start_vert { //向上选择
            for (index, render) in self.renders.iter_mut().enumerate() {
                if index == self.start_vert {
                    render.rect_mut().set_x_min(cursor.min_pos.x);
                } else if index == cursor.vert {
                    let line = &cchar.buffer.lines[index];
                    render.rect_mut().set_x_min(cursor.cursor_min());
                    render.rect_mut().set_x_max(cursor.min_pos.x + line.width);
                } else if index > cursor.vert && index < self.start_vert {
                    let line = &cchar.buffer.lines[index];
                    render.rect_mut().set_x_min(cursor.min_pos.x);
                    render.rect_mut().set_x_max(cursor.min_pos.x + line.width);
                } else {
                    let min = render.rect().dx().min;
                    render.rect_mut().set_x_max(min);
                }
            }
        } else { //同行选择
            for (index, render) in self.renders.iter_mut().enumerate() {
                if index != cursor.vert {
                    let min = render.rect().dx().min;
                    render.rect_mut().set_x_max(min);
                    continue;
                }
                let line = &cchar.buffer.lines[self.start_vert];
                if cursor.horiz > self.start_horiz { //向右选择
                    let ox = line.get_width_in_char(self.start_horiz) + cchar.offset.x;
                    let ox = if cursor.min_pos.x + ox < cursor.min_pos.x { cursor.min_pos.x } else { cursor.min_pos.x + ox };
                    render.rect_mut().set_x_min(ox);
                    render.rect_mut().set_x_max(cursor.cursor_min());
                } else if cursor.horiz < self.start_horiz { //向左选择
                    render.rect_mut().set_x_min(cursor.cursor_min());
                    let ox = line.get_width_in_char(self.start_horiz) + cchar.offset.x;
                    let ox = if cursor.min_pos.x + ox > cursor.max_pos.x { cursor.max_pos.x } else { cursor.min_pos.x + ox };
                    render.rect_mut().set_x_max(ox);
                }
                if pos.x > cursor.max_pos.x {
                    let wid = ui.context.window.id();
                    ui.context.user_update = (wid, UpdateType::None);
                    let window = ui.context.window.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        window.request_update_event(UserEvent::ReqUpdate);
                    });
                } else if pos.x < cursor.min_pos.x {
                    let wid = ui.context.window.id();
                    ui.context.user_update = (wid, UpdateType::None);
                    let window = ui.context.window.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        window.request_update_event(UserEvent::ReqUpdate);
                    });
                }
            }
        }
        self.has_selected = true;
        println!("move_select-{} {} {} {}", self.start_horiz, self.start_vert, cursor.horiz, cursor.vert);
        if self.start_horiz == cursor.horiz && self.start_vert == cursor.vert { self.has_selected = false; }
        self.changed = true;
    }

    pub fn select_by_ime(&mut self, start_horiz: usize, start_vert: usize, cchar: &CharBuffer, cursor: &EditCursor) {
        self.reset(cursor);
        self.start_horiz = start_horiz;
        self.start_vert = start_vert;
        self.has_selected = true;
        if start_vert == cursor.vert { //处于同一行中
            let line = &cchar.buffer.lines[cursor.vert];
            let mut x_min = line.get_width_in_char(start_horiz) + cursor.min_pos.x;
            // let mut x_max = line.get_width_in_char(cursor.horiz) + cursor.min_pos.x;
            // if x_max > cursor.max_pos.x { x_max = cursor.max_pos.x; }
            if x_min < cursor.min_pos.x { x_min = cursor.min_pos.x; }
            self.renders[cursor.vert].rect_mut().set_x_min(x_min);
            self.renders[cursor.vert].rect_mut().set_x_max(cursor.cursor_min() + 2.0);
        } else {
            let start_line = &cchar.buffer.lines[start_vert];
            let end_line = &cchar.buffer.lines[cursor.vert];
            let sm = start_line.get_width_in_char(start_horiz) + cursor.min_pos.x;
            self.renders[start_vert].rect_mut().set_x_min(sm);
            if cursor.max_pos.x > cursor.min_pos.x + start_line.width {
                self.renders[start_vert].rect_mut().set_x_max(cursor.min_pos.x + start_line.width);
            } else {
                self.renders[start_vert].rect_mut().set_x_max(cursor.max_pos.x);
            }

            self.renders[cursor.vert].rect_mut().set_x_min(cursor.min_pos.x);
            let em = end_line.get_width_in_char(cursor.horiz) + cursor.min_pos.x;
            self.renders[cursor.vert].rect_mut().set_x_max(em);
            for v in start_vert + 1..cursor.vert {
                self.renders[v].rect_mut().set_x_min(cursor.min_pos.x);
                if cursor.max_pos.x > cursor.min_pos.x + cchar.buffer.lines[v].width {
                    self.renders[v].rect_mut().set_x_max(cursor.min_pos.x + cchar.buffer.lines[v].width);
                } else {
                    self.renders[v].rect_mut().set_x_max(cursor.max_pos.x);
                }
            }
        }


        self.changed = true;
    }

    pub fn update_position(&mut self, mut rect: Rect) {
        for render in self.renders.iter_mut() {
            // println!("111{:?}", render.rect());
            render.rect_mut().offset_y_to(rect.dy().min);
            // println!("222{:?}", render.rect());
            rect.add_min_y(render.rect().height());
        }
    }
}