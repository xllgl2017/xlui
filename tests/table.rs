use xlui::column::Column;
use xlui::frame::App;
use xlui::table::{TableExt, TableView};
use xlui::table::column::TableColumn;
use xlui::ui::Ui;


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
            TableColumn::new_name("column1").with_width(50.0),
            TableColumn::new_name("column2").with_width(100.0),
            TableColumn::new_name("column3").with_width(150.0),
            TableColumn::new_name("column4").with_width(150.0),
            TableColumn::new_name("column5").with_width(200.0),
        ];
        let mut table_view = TableView::new(columns).with_size(780.0, 600.0);
        table_view.set_data(vec![TableData {}, TableData {}]);
        TestTable {
            table_view
        }
    }
}

impl App for TestTable {
    fn draw(&mut self, ui: &mut Ui) {
        // self.table_view.show(ui);
        ui.horizontal(|ui| {
            let mut column = Column::new().width(200.0).resize(true);
            column.cell(|ui| ui.label("row1"));
            column.cell(|ui| ui.label("row1"));
            column.cell(|ui| ui.label("row2"));
            column.cell(|ui| ui.label("row3"));
            column.cell(|ui| ui.label("row4"));
            column.cell(|ui| ui.label("row5"));
            column.cell(|ui| ui.label("row6"));
            column.cell(|ui| ui.label("row7"));
            ui.add(column);

            let mut column = Column::new().width(200.0).resize(false);
            column.cell(|ui| ui.label("row1"));
            column.cell(|ui| ui.label("row1"));
            column.cell(|ui| ui.label("row2"));
            column.cell(|ui| ui.label("row3"));
            column.cell(|ui| ui.label("row4"));
            column.cell(|ui| ui.label("row5"));
            column.cell(|ui| ui.label("row6"));
            column.cell(|ui| ui.label("row7"));
            ui.add(column);
        });
    }
}