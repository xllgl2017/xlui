use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::ui::Ui;
use crate::widgets::table::column::TableColumn;
use crate::widgets::table::header::{TableHeader, TableUi};
use crate::widgets::table::row::TableRowData;
use crate::widgets::{WidgetChange, WidgetSize};
use crate::{Border, HorizontalLayout, LayoutKind, Padding, Radius, Rect, TableExt, Widget};

pub struct TableCell {
    pub(crate) id: String,
    fill_render: RenderParam<RectParam>,
    cell_line: RenderParam<RectParam>,
    layout: Option<LayoutKind>,
}

impl TableCell {
    pub fn new(width: f32, height: f32) -> TableCell {
        let mut cell_style = ClickStyle::new();
        cell_style.fill = FillStyle::same(Color::rgb(235, 235, 235));
        cell_style.border = BorderStyle::same(Border::new(0.0).color(Color::BLUE).radius(Radius::same(0)));
        let layout = HorizontalLayout::left_to_right().with_size(width, height)
            .with_padding(Padding::same(0.0).left(5.0));
        let mut cell_line_style = ClickStyle::new();
        cell_line_style.fill = FillStyle::same(Color::BLACK);
        cell_line_style.border = BorderStyle::same(Border::new(0.0).radius(Radius::same(0)));
        TableCell {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new().with_size(width, height), cell_style)),
            cell_line: RenderParam::new(RectParam::new(Rect::new().with_size(1.0, height), cell_line_style)),
            layout: Some(LayoutKind::new(layout)),
        }
    }

    pub fn show_header<T: TableExt>(&mut self, ui: &mut Ui, tui: &TableUi<T>, column: &TableColumn) {
        let current_layout = self.layout.take().unwrap();
        let previous_layout = ui.layout.replace(current_layout).unwrap();
        tui.show_header(ui, column);
        self.layout = ui.layout.replace(previous_layout);
        self.fill_render.init_rectangle(ui, false, false);
        self.cell_line.init_rectangle(ui, false, false);
    }

    pub fn show_body<T: TableExt>(&mut self, ui: &mut Ui, header: &TableHeader<T>, row_datum: &TableRowData<T>) {
        let current_layout = self.layout.take().unwrap();
        let previous_layout = ui.layout.replace(current_layout).unwrap();
        let tui = &header.uis[row_datum.column_index()];
        tui.show_body(ui, row_datum);
        self.layout = ui.layout.replace(previous_layout);
        if (row_datum.column_index() % 2 == 0 && row_datum.row_index() % 2 != 0) || (row_datum.column_index() % 2 != 0 && row_datum.row_index() % 2 == 0) {
            self.fill_render.param.style.fill = FillStyle::same(Color::rgb(245, 245, 245))
        }
        self.cell_line.init_rectangle(ui, false, false);
        self.fill_render.init_rectangle(ui, false, false);
    }
    fn redraw(&mut self, ui: &mut Ui) {
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.param.rect.offset_to_rect(&ui.draw_rect);
            self.fill_render.update(ui, false, false);
            let mut cell_rect = self.fill_render.param.rect.clone();
            cell_rect.set_x_min(cell_rect.dx().max - 2.0);
            self.cell_line.param.rect.offset_to_rect(&cell_rect);
            self.cell_line.update(ui, false, false);
        }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        ui.context.render.rectangle.render(&self.cell_line, pass);
        self.layout.as_mut().unwrap().update(ui);
    }
}

impl Widget for TableCell {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            _ => {}
        }
        self.layout.as_mut().unwrap().update(ui);
        Response::new(&self.id, WidgetSize::same(self.fill_render.param.rect.width(), self.fill_render.param.rect.height()))
    }
}