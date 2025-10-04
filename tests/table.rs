use xlui::*;


pub struct TableData {}

impl TableExt for TableData {
    fn cols(&self) -> Vec<impl ToString> {
        vec![1, 2, 3, 4, 5]
    }
}

pub struct TestTable {
    table_view: TableView<TableData>,
}

impl TestTable {
    pub fn new() -> TestTable {
        let columns = vec![
            TableColumn::new_name("column1").with_width(80.0),
            TableColumn::new_name("column2").with_width(100.0),
            TableColumn::new_name("column3").with_width(150.0),
            TableColumn::new_name("column4").with_width(150.0),
            TableColumn::new_name("column5").with_width(200.0),
        ];
        let mut table_view = TableView::new(columns).with_size(780.0, 600.0);
        let mut data = vec![];
        for _ in 0..5 {
            data.push(TableData {});
        }
        table_view.set_data(data);
        TestTable {
            table_view
        }
    }
}

impl App for TestTable {
    fn draw(&mut self, ui: &mut Ui) {
        self.table_view.show(ui);
    }
}

fn main() {
    TestTable::new().run().unwrap();
}