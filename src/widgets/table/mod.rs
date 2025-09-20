use crate::layout::{Layout, LayoutItem};
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::table::column::TableColumn;
use crate::widgets::table::header::{TableHeader, TableHeaderUi};
use crate::widgets::table::param::TableParams;
use crate::widgets::table::row::TableRow;
use crate::{Border, Radius, Rect, RecycleLayout, ScrollWidget};

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
        self.with_width(width).with_height(height)
    }

    pub fn set_header_ui(&mut self, column: usize, hui: TableHeaderUi) {
        self.header.set_hui(column, hui);
    }
}

impl<T: TableExt> TableView<T> {
    pub fn set_data(&mut self, data: Vec<T>) {
        self.params.set_data(data);
    }

    pub fn show_rows(&mut self, ui: &mut Ui) {
        let layout: &mut RecycleLayout = ui.layout().as_mut_().unwrap();
        let draw_count = layout.draw_count();
        let header_row = TableRow::new(&self.header, self.params.row_height())
            .with_width(self.rect.width());
        let header_item = header_row.show_header(ui, &self.header);
        println!("2323-{}-{}", header_item.width, header_item.height);
        let layout: &mut RecycleLayout = ui.layout().as_mut_().unwrap();
        layout.add_item(LayoutItem::Widget(header_item));
        for i in 0..self.params.row_data().len() {
            if i <= draw_count {
                let row = TableRow::new(&self.header, self.params.row_height())
                    .with_width(self.rect.width()).show(ui, &self.header, &mut self.params.row_mut(i));
                let layout: &mut RecycleLayout = ui.layout().as_mut_().unwrap();
                layout.add_item(LayoutItem::Widget(row));
            } else {
                let recycle: &mut RecycleLayout = ui.layout().as_mut_().unwrap();
                recycle.add_item_empty();
            }
        }
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let layout = RecycleLayout::new().with_item_height(self.params.row_height())
            .with_size(self.rect.width(), self.rect.height()).with_space(0.0);
        let mut area = ScrollWidget::vertical().enable_hscroll().with_layout(layout);
        self.lid = area.id.clone();
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::TRANSPARENT;
        fill_style.fill.hovered = Color::TRANSPARENT;
        fill_style.fill.clicked = Color::TRANSPARENT;
        fill_style.border.inactive = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        area.set_style(fill_style);
        area.show(ui, |ui| {
            self.show_rows(ui);
        });
    }

    pub fn update(&mut self, ui: &mut Ui) {}
}

pub trait TableExt {
    fn cols(&self) -> Vec<impl ToString>;
}