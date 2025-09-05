use crate::layout::scroll_area::ScrollArea;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::table::cell::TableCell;
use crate::widgets::table::column::TableColumn;
use crate::widgets::table::header::TableHeader;
use crate::widgets::table::param::TableParams;
use crate::widgets::table::row::TableRow;
use crate::{Border, Padding, Radius, Rect};
use crate::cell::Cell;
use crate::table::header::TableHeaderUi;

mod param;
pub mod column;
mod row;
mod header;
mod cell;

pub struct TableView<T> {
    id: String,
    lid: String,
    rect: Rect,
    header: TableHeader<T>,
    params: TableParams<T>,
}


impl<T: TableExt> TableView<T> {
    pub fn new(columns: Vec<TableColumn>) -> Self {
        let header = TableHeader::from_columns(columns);
        let params = TableParams::new();
        TableView {
            id: crate::gen_unique_id(),
            lid: "".to_string(),
            rect: Rect::new(),
            header,
            params,
        }
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.rect.set_width(width);
        self
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.rect.set_height(height);
        self
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.rect.set_width(width);
        self.rect.set_height(height);
        self
    }

    pub fn set_header_ui(&mut self, column: usize, hui: TableHeaderUi) {
        self.header.set_hui(column, hui);
    }
}

impl<T: TableExt> TableView<T> {
    pub fn set_data(&mut self, data: Vec<T>) {
        self.params.set_data(data);
    }

    pub fn show_row(&mut self, ui: &mut Ui, row: usize) {
        let data = &mut self.params.row_data_mut()[row];
        let mut row = TableRow::new(data.height());
        row.init(ui, true);
        let previous_layout = ui.layout.replace(row.layout.take().unwrap()).unwrap();
        let r = 0..data.data().cols().len();
        for column_index in r {
            data.set_column(column_index);
            let cell = TableCell::new();
            cell.show_body(ui, &self.header, data);
        }
        row.layout = ui.layout.replace(previous_layout);
        ui.add(row);
    }

    pub fn show(&mut self, ui: &mut Ui) {
        self.rect = ui.available_rect().clone_with_size(&self.rect);
        let mut area = ScrollArea::new();
        self.lid = area.id.clone();
        area.set_rect(self.rect.clone());
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::TRANSPARENT;
        fill_style.fill.hovered = Color::TRANSPARENT;
        fill_style.fill.clicked = Color::TRANSPARENT;
        fill_style.border.inactive = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        area.set_style(fill_style);
        area.show(ui, |ui| {
            ui.layout().set_item_space(0.0);
            ui.layout().set_padding(Padding::same(0.0));
            let mut table_rows = TableRow::new(&self.header);
            let mut rows =vec![];
            for (column_index,column) in self.header.columns.iter().enumerate() {
                rows.push(Cell::new().with_context(Box::new(|ui|{}));
            }
            self.header.show(ui);
            for row in 0..self.params.row_data().len() {
                self.show_row(ui, row);
            }
        });
    }

    pub fn update(&mut self, ui: &mut Ui) {}
}

pub trait TableExt {
    fn cols(&self) -> Vec<impl ToString>;
}