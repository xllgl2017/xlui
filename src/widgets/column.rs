use crate::frame::context::UpdateType;
use crate::map::Map;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::ui::Ui;
use crate::widgets::cell::Cell;
use crate::widgets::WidgetSize;
use crate::{Border, LayoutKind, Rect, VerticalLayout, Widget};

pub struct Column {
    id: String,
    resize: bool,
    width: f32,
    right_line: RenderParam<RectParam>,
    cells: Map<String, Cell>,
    draw_rect: Rect,
}

impl Column {
    pub fn new() -> Self {
        let mut line_style = ClickStyle::new();
        line_style.fill = FillStyle::same(Color::RED);
        line_style.border = BorderStyle::same(Border::new(0.0));
        Self {
            id: crate::gen_unique_id(),
            resize: false,
            width: 0.0,
            right_line: RenderParam::new(RectParam::new(Rect::new(), line_style)),
            cells: Map::new(),
            draw_rect: Rect::new(),
        }
    }

    pub fn add(&mut self, cell: Cell) {
        self.cells.insert(cell.id.clone(), cell);
    }

    pub fn cell(&mut self, context: impl Fn(&mut Ui) + 'static) {
        let cell = Cell::new();
        self.add(cell);
    }

    pub fn resize(mut self, resize: bool) -> Self {
        self.resize = resize;
        self
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub(crate) fn init(&mut self, ui: &mut Ui, init: bool) {
        if init {
            let layout = VerticalLayout::top_to_bottom().with_space(0.0);
            let previous_layout = ui.layout.replace(LayoutKind::new(layout)).unwrap();
            for cell in self.cells.iter_mut() {
                let resp = cell.update(ui);
            }
            self.draw_rect = ui.layout().drawn_rect();
            if self.width != 0.0 { self.draw_rect.set_width(self.width); }
            self.draw_rect.add_max_x(1.0);
            self.right_line.param.rect = self.draw_rect.clone();
            self.right_line.param.rect.set_x_min(self.right_line.param.rect.dx().max - 1.0);
            ui.layout.replace(previous_layout).unwrap();
        } else {
            for cell in self.cells.iter_mut() {
                cell.update(ui);
            }
        }
        self.right_line.init_rectangle(ui, false, false);
    }

    pub(crate) fn get_cell(&mut self, id: &String) -> Option<&mut Cell> {
        self.cells.get_mut(id)
    }

    fn redraw(&mut self, ui: &mut Ui) {
        if self.resize {
            let pass = ui.pass.as_mut().unwrap();
            ui.context.render.rectangle.render(&self.right_line, pass);
        }
        self.cells.iter_mut().for_each(|cell| cell.redraw(ui));
    }
}

impl Widget for Column {
    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::None => {}
            UpdateType::Init => self.init(ui, true),
            UpdateType::ReInit => self.init(ui, false),
            UpdateType::MouseMove => {}
            UpdateType::MousePress => {}
            UpdateType::MouseRelease => {}
            UpdateType::MouseWheel => {}
            UpdateType::KeyRelease(_) => {}
            UpdateType::IME(_) => {}
            UpdateType::CreateWindow => {}
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.draw_rect.width(), self.draw_rect.height()))
    }
}