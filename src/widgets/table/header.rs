use crate::TableExt;
use crate::ui::Ui;
use crate::widgets::table::cell::TableCell;
use crate::widgets::table::column::TableColumn;
use crate::widgets::table::row::{TableRow, TableRowData};

pub type TableHeaderUi = Box<dyn Fn(&mut Ui, &TableColumn) + 'static>;
pub type TableBodyUi<T> = Box<dyn Fn(&mut Ui, &TableRowData<T>) + 'static>;


pub struct TableUi<T> {
    hui: TableHeaderUi,
    bui: TableBodyUi<T>,
}

impl<T> TableUi<T> {
    pub fn show_header(&self, ui: &mut Ui, column: &TableColumn) {
        (self.hui)(ui, column);
    }

    pub fn show_body(&self, ui: &mut Ui, data: &TableRowData<T>) {
        (self.bui)(ui, data);
    }
}

impl<T: TableExt> TableUi<T> {
    pub fn new() -> Self {
        TableUi {
            hui: Box::new(|ui, column| ui.label(column.name())),
            bui: Box::new(|ui, row_datum| ui.label(row_datum.column_string())),
        }
    }
}

pub struct TableHeader<T> {
    height: f32,
    pub(crate) columns: Vec<TableColumn>,
    pub(crate) uis: Vec<TableUi<T>>,
}

impl<T: TableExt> TableHeader<T> {
    pub fn from_columns(columns: Vec<TableColumn>) -> Self {
        let mut uis = vec![];
        for _ in 0..columns.len() {
            uis.push(TableUi::new());
        }
        TableHeader {
            height: 20.0,
            columns: columns,
            uis,
        }
    }

    pub fn show(&self, ui: &mut Ui) -> Vec<TableCell> {
        let mut cells = vec![];
        for (column_index, column) in self.columns.iter().enumerate() {
            let mut cell = TableCell::new(column.width(), self.height);
            cell.show_header(ui, &self.uis[column_index], column);
            cells.push(cell);
        }
        cells
    }

    pub fn set_hui(&mut self, column: usize, hui: TableHeaderUi) {
        self.uis[column].hui = hui;
    }
}