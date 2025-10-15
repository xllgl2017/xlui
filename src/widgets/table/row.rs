use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::Response;
use crate::size::Geometry;
use crate::ui::Ui;
use crate::widgets::table::cell::TableCell;
use crate::widgets::table::header::TableHeader;
use crate::widgets::table::TableExt;
use crate::widgets::{WidgetKind, WidgetSize, WidgetState};
use crate::*;

pub struct TableRow {
    id: String,
    fill_render: RenderParam,
    cells: Vec<TableCell>,
    geometry: Geometry,
    state: WidgetState,
}

impl TableRow {
    pub fn new<T>(headers: &TableHeader<T>, row_height: f32) -> TableRow {
        let mut cells = vec![];
        for column in &headers.columns {
            cells.push(TableCell::new(column.width(), row_height));
        }
        TableRow {
            id: gen_unique_id(),
            fill_render: RenderParam::new(RenderKind::Rectangle(RectParam::new())),
            cells,
            geometry: Geometry::new().with_fix_height(row_height),
            state: WidgetState::default(),
        }
    }

    pub fn with_width(mut self, w: f32) -> TableRow {
        self.geometry.set_fix_width(w);
        self
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
        self.fill_render.rect_mut().set_size(self.geometry.width(), self.geometry.height());
        if datum.row % 2 == 0 { self.fill_render.style_mut().fill = FillStyle::same(Color::rgb(245, 245, 245)) }

        let row = WidgetKind::new(ui, self);
        row
    }
}

impl Widget for TableRow {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            #[cfg(feature = "gpu")]
            UpdateType::Init | UpdateType::ReInit => self.fill_render.init(ui, false, false),
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
        self.geometry.set_width(width);
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}

pub struct TableRowData<T> {
    row: usize,
    column: usize,
    data: T,
    // height: f32,
    // enable: bool,
    // selected: bool,
    // hidden: bool,
}


impl<T> TableRowData<T> {
    pub fn set_column(&mut self, column: usize) {
        self.column = column;
    }

    // pub fn height(&self) -> f32 {
    //     self.height
    // }

    // pub fn data(&self) -> &T {
    //     &self.data
    // }
    //
    // pub fn data_mut(&mut self) -> &mut T {
    //     &mut self.data
    // }

    pub fn column_index(&self) -> usize {
        self.column
    }

    pub fn row_index(&self) -> usize { self.row }
}

impl<T: TableExt> TableRowData<T> {
    pub fn new(t: T, row: usize, _height: f32) -> Self {
        TableRowData {
            row,
            column: 0,
            data: t,
            // height,
            // enable: false,
            // selected: false,
            // hidden: false,
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