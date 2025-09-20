use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::table::header::TableHeader;
use crate::widgets::table::TableExt;
use crate::widgets::{WidgetChange, WidgetKind, WidgetSize};
use crate::{Color, FillStyle, Offset, Pos, Rect, Widget};
use crate::widgets::table::cell::TableCell;

pub struct TableRow {
    id: String,
    fill_render: RenderParam<RectParam>,
    cells: Vec<TableCell>,
    offset: Offset,
}

impl TableRow {
    pub fn new<T>(headers: &TableHeader<T>, row_height: f32) -> TableRow {
        let mut cells = vec![];
        for column in &headers.columns {
            cells.push(TableCell::new(column.width(), row_height));
        }
        TableRow {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new().with_height(row_height), ClickStyle::new())),
            cells,
            offset: Offset::new(Pos::new()),
        }
    }

    pub fn with_width(mut self, w: f32) -> TableRow {
        self.fill_render.param.rect.set_width(w);
        self
    }

    pub(crate) fn init(&mut self, ui: &mut Ui) {
        self.fill_render.init_rectangle(ui, false, false);
    }


    pub fn show_header<T: TableExt>(mut self, ui: &mut Ui, header: &TableHeader<T>) -> WidgetKind {
        self.cells = header.show(ui);
        let row = WidgetKind::new(ui, self);
        row
    }

    pub fn show<T: TableExt>(mut self, ui: &mut Ui, header: &TableHeader<T>, datum: &mut TableRowData<T>) -> WidgetKind {
        for (index, cell) in self.cells.iter_mut().enumerate() {
            datum.set_column(index);
            cell.show_body(ui, header, datum);
        }
        if datum.row % 2 == 0 { self.fill_render.param.style.fill = FillStyle::same(Color::rgb(245, 245, 245)) }

        let row = WidgetKind::new(ui, self);
        row
    }

    fn redraw(&mut self, ui: &mut Ui) {
        // if ui.widget_changed.contains(WidgetChange::Position) {
        //     self.fill_render.param.rect.offset_to_rect(&ui.draw_rect);
        //     self.fill_render.update(ui, false, false);
        // }
        // let pass = ui.pass.as_mut().unwrap();
        // ui.context.render.rectangle.render(&self.fill_render, pass);
    }
}

impl Widget for TableRow {
    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            _ => {}
        }
        let mut width = 0.0;
        let previous_rect = ui.draw_rect.clone();
        for cell in self.cells.iter_mut() {
            let resp = cell.update(ui);
            width += resp.size.dw;
            ui.draw_rect.add_min_x(resp.size.dw);
        }
        ui.draw_rect = previous_rect;
        Response::new(&self.id, WidgetSize::same(width, self.fill_render.param.rect.height()))
    }
}

pub struct TableRowData<T> {
    row: usize,
    column: usize,
    data: T,
    height: f32,
    enable: bool,
    selected: bool,
    hidden: bool,
}


impl<T> TableRowData<T> {
    pub fn set_column(&mut self, column: usize) {
        self.column = column;
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn column_index(&self) -> usize {
        self.column
    }

    pub fn row_index(&self) -> usize { self.row }
}

impl<T: TableExt> TableRowData<T> {
    pub fn new(t: T, row: usize, height: f32) -> Self {
        TableRowData {
            row,
            column: 0,
            data: t,
            height,
            enable: false,
            selected: false,
            hidden: false,
        }
    }


    pub fn from_vec(ts: Vec<T>, height: f32) -> Vec<TableRowData<T>> {
        let mut res = vec![];
        for (row, t) in ts.into_iter().enumerate() {
            let row_data = TableRowData::new(t, row, height);
            res.push(row_data);
        }
        res
    }

    pub fn column_string(&self) -> String {
        self.data.cols()[self.column].to_string()
    }
}