use crate::widgets::table::row::TableRowData;
use crate::widgets::table::TableExt;

pub struct TableParams<T> {
    row_height: f32,
    row_data: Vec<TableRowData<T>>,

}

impl<T> TableParams<T> {
    pub fn new() -> Self {
        TableParams {
            row_height: 20.0,
            row_data: vec![],
        }
    }

    pub fn row_data(&self) -> &Vec<TableRowData<T>> {
        &self.row_data
    }

    // pub fn row_data_mut(&mut self) -> &mut Vec<TableRowData<T>> {
    //     &mut self.row_data
    // }

    pub fn row_mut(&mut self, row: usize) -> &mut TableRowData<T> {
        &mut self.row_data[row]
    }

    pub fn row_height(&self) -> f32 {
        self.row_height
    }
}

impl<T: TableExt> TableParams<T> {
    pub fn set_data(&mut self, data: Vec<T>) {
        self.row_data = TableRowData::from_vec(data, self.row_height);
    }
}