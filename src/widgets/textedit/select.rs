use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::size::border::Border;
use crate::size::pos::Pos;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::textedit::buffer::CharBuffer;
use crate::widgets::textedit::cursor::EditCursor;
use crate::Offset;
use crate::window::UserEvent;

pub struct EditSelection {
    renders: Vec<RenderParam<RectParam>>,
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

    pub fn init(&mut self, rows: usize, rect: &Rect, line_height: f32, ui: &mut Ui, init: bool) {
        if init {
            let mut select_style = ClickStyle::new();
            select_style.fill.inactive = Color::rgba(255, 0, 0, 100); //Color::rgba(144, 209, 255, 100); //
            select_style.fill.hovered = Color::rgba(144, 209, 255, 100);
            select_style.fill.clicked = Color::rgba(144, 209, 255, 100);
            select_style.border.inactive = Border::new(0.0).radius(Radius::same(0));
            select_style.border.hovered = Border::new(0.0).radius(Radius::same(0));
            select_style.border.clicked = Border::new(0.0).radius(Radius::same(0));
            for row in 0..rows {
                let mut rect = rect.clone();
                rect.set_x_max(rect.dx().min);
                rect.set_height(line_height);
                rect.offset(&Offset::new(Pos::new()).with_y(row as f32 * line_height).covered());
                let render = RenderParam::new(RectParam::new(rect, select_style.clone()));
                self.renders.push(render);
            }
        }
        self.renders.iter_mut().for_each(|x| x.init_rectangle(ui, false, false));
    }

    pub fn set_by_cursor(&mut self, cursor: &EditCursor) {
        let x = cursor.cursor_min();
        for (index, render) in self.renders.iter_mut().enumerate() {
            if index == cursor.vert {
                render.param.rect.set_x_min(x);
                render.param.rect.set_x_max(x);
            } else {
                render.param.rect.set_x_max(render.param.rect.dx().min);
            }
        }
        self.start_vert = cursor.vert;
        self.start_horiz = cursor.horiz;
        self.changed = true;
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if !self.changed { return; }
        self.changed = false;
        for render in self.renders.iter_mut() {
            render.update(ui, false, false);
        }
    }

    pub fn offset(&mut self, offset: &Offset) {
        self.renders.iter_mut().for_each(|x| x.param.rect.offset(offset));
        self.changed = true;
    }

    pub fn render(&mut self, ui: &mut Ui, rows: usize) {
        let pass = ui.pass.as_mut().unwrap();
        for (index, render) in self.renders.iter().enumerate() {
            if index >= rows { continue; }
            ui.context.render.rectangle.render(render, pass);
        }
    }

    pub fn reset(&mut self, cursor: &EditCursor) {
        for render in self.renders.iter_mut() {
            render.param.rect.set_x_max(render.param.rect.dx().min);
        }
        self.start_vert = cursor.vert;
        self.start_horiz = cursor.horiz;
        self.changed = true;
        self.has_selected = false;
    }

    pub fn move_select(&mut self, ui: &mut Ui, cursor: &mut EditCursor, cchar: &mut CharBuffer) {
        let pos = ui.device.device_input.mouse.lastest;
        cursor.update_by_pos(pos, cchar);
        // println!("move_select-{:?}-{}-{}-{}-{}", pos, cursor.horiz, cursor.vert, self.start_horiz, self.start_vert);
        if cursor.vert > self.start_vert { //向下选择
            for (index, render) in self.renders.iter_mut().enumerate() {
                if index == self.start_vert {
                    let line = &cchar.buffer.lines[index];
                    render.param.rect.set_x_min(line.get_width_in_char(self.start_horiz) + cursor.min_pos.x);
                    render.param.rect.set_x_max(line.width + cursor.min_pos.x);
                } else if index == cursor.vert {
                    render.param.rect.set_x_min(cursor.min_pos.x);
                    render.param.rect.set_x_max(cursor.cursor_min());
                } else if index > self.start_vert && index < cursor.vert {
                    let line = &cchar.buffer.lines[index];
                    render.param.rect.set_x_min(cursor.min_pos.x);
                    render.param.rect.set_x_max(cursor.min_pos.x + line.width);
                } else {
                    render.param.rect.set_x_max(render.param.rect.dx().min);
                }
            }
        } else if cursor.vert < self.start_vert { //向上选择
            for (index, render) in self.renders.iter_mut().enumerate() {
                if index == self.start_vert {
                    render.param.rect.set_x_min(cursor.min_pos.x);
                } else if index == cursor.vert {
                    let line = &cchar.buffer.lines[index];
                    render.param.rect.set_x_min(cursor.cursor_min());
                    render.param.rect.set_x_max(cursor.min_pos.x + line.width);
                } else if index > cursor.vert && index < self.start_vert {
                    let line = &cchar.buffer.lines[index];
                    render.param.rect.set_x_min(cursor.min_pos.x);
                    render.param.rect.set_x_max(cursor.min_pos.x + line.width);
                } else {
                    render.param.rect.set_x_max(render.param.rect.dx().min);
                }
            }
        } else { //同行选择
            for (index, render) in self.renders.iter_mut().enumerate() {
                if index != cursor.vert {
                    render.param.rect.set_x_max(render.param.rect.dx().min);
                    continue;
                }
                let line = &cchar.buffer.lines[self.start_vert];
                if cursor.horiz > self.start_horiz { //向右选择
                    let ox = line.get_width_in_char(self.start_horiz) + cchar.offset.x;
                    let ox = if cursor.min_pos.x + ox < cursor.min_pos.x { cursor.min_pos.x } else { cursor.min_pos.x + ox };
                    render.param.rect.set_x_min(ox);
                    render.param.rect.set_x_max(cursor.cursor_min());
                } else if cursor.horiz < self.start_horiz { //向左选择
                    render.param.rect.set_x_min(cursor.cursor_min());
                    let ox = line.get_width_in_char(self.start_horiz) + cchar.offset.x;
                    let ox = if cursor.min_pos.x + ox > cursor.max_pos.x { cursor.max_pos.x } else { cursor.min_pos.x + ox };
                    render.param.rect.set_x_max(ox);
                }
                if pos.x > cursor.max_pos.x {
                    let wid = ui.context.window.id();
                    ui.context.user_update = (wid, UpdateType::None);
                    let window = ui.context.window.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        window.request_update(UserEvent::ReqUpdate);
                    });
                    // #[cfg(feature = "winit")]
                    // {
                    //     let window = ui.context.window.clone();
                    //     std::thread::spawn(move || {
                    //         std::thread::sleep(std::time::Duration::from_millis(100));
                    //         window.request_update(UserEvent::ReqUpdate);
                    //     });
                    // }
                    // #[cfg(not(feature = "winit"))]
                    // {
                    //     ui.context.user_update = (wid, UpdateType::None);
                    //     let window = ui.context.window.clone();
                    //     std::thread::spawn(move || {
                    //         std::thread::sleep(std::time::Duration::from_millis(100));
                    //         window.request_update(UserEvent::ReqUpdate);
                    //     });
                    // }
                } else if pos.x < cursor.min_pos.x {
                    let wid = ui.context.window.id();
                    ui.context.user_update = (wid, UpdateType::None);
                    let window = ui.context.window.clone();
                    std::thread::spawn(move || {
                        std::thread::sleep(std::time::Duration::from_millis(100));
                        window.request_update(UserEvent::ReqUpdate);
                    });
                    // #[cfg(feature = "winit")]
                    // {
                    //     let w = ui.context.window.clone();
                    //     std::thread::spawn(move || {
                    //         std::thread::sleep(std::time::Duration::from_millis(100));
                    //         w.request_update(UserEvent::ReqUpdate);
                    //     });
                    // }
                    // #[cfg(not(feature = "winit"))]
                    // {
                    //     ui.context.user_update = (wid, UpdateType::None);
                    //     let window = ui.context.window.clone();
                    //     std::thread::spawn(move || {
                    //         std::thread::sleep(std::time::Duration::from_millis(100));
                    //         window.request_update(UserEvent::ReqUpdate);
                    //     });
                    // }
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
            let x_min = line.get_width_in_char(start_horiz) + cursor.min_pos.x;
            let x_max = line.get_width_in_char(cursor.horiz) + cursor.min_pos.x;
            self.renders[cursor.vert].param.rect.set_x_min(x_min);
            self.renders[cursor.vert].param.rect.set_x_max(x_max);
        } else {
            let start_line = &cchar.buffer.lines[start_vert];
            let end_line = &cchar.buffer.lines[cursor.vert];
            let sm = start_line.get_width_in_char(start_horiz) + cursor.min_pos.x;
            self.renders[start_vert].param.rect.set_x_min(sm);
            self.renders[start_vert].param.rect.set_x_max(cursor.max_pos.x);
            self.renders[cursor.vert].param.rect.set_x_min(cursor.min_pos.x);
            let em = end_line.get_width_in_char(cursor.horiz) + cursor.min_pos.x;
            self.renders[cursor.vert].param.rect.set_x_max(em);
            for v in start_vert + 1..cursor.vert {
                self.renders[v].param.rect.set_x_min(cursor.min_pos.x);
                self.renders[v].param.rect.set_x_max(cursor.max_pos.x);
            }
        }


        self.changed = true;
    }

    pub fn update_position(&mut self, ui: &mut Ui, mut rect: Rect) {
        for render in self.renders.iter_mut() {
            render.param.rect.offset_y_to(rect.dy().min);
            render.update(ui, false, false);
            rect.add_min_y(render.param.rect.height());
        }
    }
}