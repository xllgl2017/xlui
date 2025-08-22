use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::size::border::Border;
use crate::size::pos::Pos;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::Offset;
use crate::widgets::textedit::buffer::CharBuffer;
use crate::widgets::textedit::cursor::EditCursor;

pub struct EditSelection {
    renders: Vec<RenderParam<RectParam>>,
    changed: bool,
}

impl EditSelection {
    pub fn new() -> EditSelection {
        EditSelection {
            renders: vec![],
            changed: false,
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
                rect.offset(&Offset::new(Pos::new()).with_y(row as f32 * line_height).delete_offset());
                let render = RenderParam::new(RectParam::new(rect, select_style.clone()));
                self.renders.push(render);
            }
        }
        self.renders.iter_mut().for_each(|x| x.init_rectangle(ui, false, false));
    }

    pub fn set_by_cursor(&mut self, line: usize, x: f32) {
        self.renders[line].param.rect.set_x_min(x);
        self.renders[line].param.rect.set_x_max(x);
        self.changed = true;
    }

    pub fn update(&mut self, ui: &mut Ui) {
        if !self.changed { return; }
        self.changed = false;
        for render in self.renders.iter_mut() {
            render.update(ui, false, false);
        }
    }

    pub fn render(&mut self, ui: &mut Ui, rows: usize) {
        let pass = ui.pass.as_mut().unwrap();
        for (index, render) in self.renders.iter().enumerate() {
            if index >= rows { continue; }
            ui.context.render.rectangle.render(render, pass);
        }
    }

    pub fn reset(&mut self) {
        for render in self.renders.iter_mut() {
            render.param.rect.set_x_max(render.param.rect.dx().min);
        }
        self.changed = true;
    }

    // pub fn move_select(&mut self, ui: &mut Ui, cursor: &mut EditCursor, cchar: &CharBuffer) {
    //     let pos = ui.device.device_input.mouse.lastest;
    //     let pre_line = cursor.line;
    //     let pre_cursor = cursor.current;
    //     cursor.update_by_pos(pos, cchar);
    //     let press_pos = ui.device.device_input.mouse.pressed_pos;
    //     if pre_line < cursor.line { //向下选择
    //         let start = cchar.line_index[pre_line];
    //         let end = cchar.line_index[cursor.line];
    //         let mut offset = 0.0;
    //         cchar.chars[start..end].iter().for_each(|x| offset += x.width);
    //         self.renders[pre_line].param.rect.set_x_max(offset + cursor.min_pos.x);
    //     } else if pre_line > cursor.line { //向上选择
    //         self.renders[cursor.line].param.rect.set_x_min(cursor.cursor_min());
    //         self.renders[cursor.line].param.rect.set_x_max(200.0);
    //         self.renders[pre_line].param.rect.set_x_min(cursor.min_pos.x);
    //     } else {
    //         if pos.x > press_pos.x { //向右选择
    //             self.renders[cursor.line].param.rect.set_x_max(cursor.cursor_min());
    //         } else {
    //             self.renders[cursor.line].param.rect.set_x_min(cursor.cursor_min());
    //         }
    //     }
    //     self.changed = true;
    // }
}