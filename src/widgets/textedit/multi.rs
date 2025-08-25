use crate::frame::context::{Context, UpdateType};
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


pub struct MultiEdit {
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
    focused: bool,

}

impl MultiEdit {
    pub fn new(text: String) -> MultiEdit {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::WHITE;
        fill_style.fill.hovered = Color::WHITE;
        fill_style.fill.clicked = Color::WHITE;
        fill_style.border.inactive = Border::new(0.0).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = fill_style.border.hovered.clone();


        MultiEdit {
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

    pub(crate) fn reset_size(&mut self, context: &Context) {
        self.text_buffer.reset_size(context); //计算行高
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
        if !self.changed { return; }
        self.changed = false;
        self.fill_render.update(ui, self.hovered || self.focused, ui.device.device_input.mouse.pressed);
        self.cursor_render.update(ui);
        self.select_render.update(ui);
        self.text_buffer.update_buffer_text(ui, &self.char_layout.draw_text());
    }

    fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            self.fill_render.param.rect = ui.available_rect().clone();
            self.reset_size(&ui.context);
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


    fn key_input(&mut self, key: Option<winit::keyboard::Key>, ui: &mut Ui) {
        self.changed = true;
        match key.unwrap() {
            winit::keyboard::Key::Named(name) => {
                println!("{:?}", name);
                match name {
                    //更新游标
                    winit::keyboard::NamedKey::ArrowLeft => self.cursor_render.move_left(&self.char_layout),
                    winit::keyboard::NamedKey::ArrowRight => self.cursor_render.move_right(&self.char_layout),
                    winit::keyboard::NamedKey::ArrowUp => self.cursor_render.move_up(&self.char_layout),
                    winit::keyboard::NamedKey::ArrowDown => self.cursor_render.move_down(&self.char_layout),
                    //更新游标+文本
                    winit::keyboard::NamedKey::Backspace => self.char_layout.remove_chars_before_cursor(ui, &mut self.cursor_render, &self.select_render),
                    winit::keyboard::NamedKey::Delete => self.char_layout.remove_chars_after_cursor(ui, &mut self.cursor_render, &self.select_render),
                    winit::keyboard::NamedKey::Space => self.char_layout.inset_char(' ', ui, &mut self.cursor_render, &self.select_render),
                    winit::keyboard::NamedKey::Enter => self.char_layout.inset_char('\n', ui, &mut self.cursor_render, &self.select_render),
                    winit::keyboard::NamedKey::Home => self.cursor_render.set_cursor(0, self.cursor_render.vert, &self.char_layout),
                    winit::keyboard::NamedKey::End => self.cursor_render.set_cursor(self.char_layout.lines[self.cursor_render.vert].len(), self.cursor_render.vert, &self.char_layout),
                    _ => {}
                }
            }
            //更新游标+文本
            winit::keyboard::Key::Character(c) => {
                let c = c.chars().next().unwrap();
                self.char_layout.inset_char(c, ui, &mut self.cursor_render, &self.select_render);
            }
            winit::keyboard::Key::Unidentified(_) => {}
            winit::keyboard::Key::Dead(_) => {}
        }
        self.select_render.reset();
        // if let Some(ref mut callback) = self.callback {
        //     let app = ui.app.take().unwrap();
        //     callback(app, ui, self.char_layout.text());
        //     ui.app.replace(app);
        // }
        // ui.send_updates(&self.contact_ids, ContextUpdate::String(self.char_layout.text()));
        ui.context.window.request_redraw();
    }
}

impl Widget for MultiEdit {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.select_render.render(ui, self.char_layout.lines.len());
        self.cursor_render.render(ui);
        self.text_buffer.redraw(ui);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::None => {}
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            UpdateType::MouseMove => {
                if ui.device.device_input.mouse.pressed && self.focused {
                    self.select_render.move_select(ui, &mut self.cursor_render, &self.char_layout);
                    self.changed = true;
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
                if self.focused {
                    let pos = ui.device.device_input.mouse.lastest;
                    self.cursor_render.update_by_pos(pos, &self.char_layout);
                    self.select_render.set_by_cursor(&self.cursor_render);
                }
                self.changed = true;
                ui.context.window.request_redraw();
            }
            UpdateType::MouseRelease => {}
            UpdateType::MouseWheel => {}
            UpdateType::KeyRelease(ref mut key) => {
                if self.focused { self.key_input(key.take(), ui); }
            }
            UpdateType::Offset(_) => {}
            UpdateType::Drop => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}