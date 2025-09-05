use crate::frame::context::UpdateType;
use crate::layout::{HorizontalLayout, LayoutKind};
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::table::column::TableColumn;
use crate::table::header::{TableHeader, TableUi};
use crate::table::row::TableRowData;
use crate::table::TableExt;
use crate::ui::Ui;
use crate::{Border, Padding, Radius, Rect, Widget};

pub struct TableCell {
    pub(crate) id: String,
    fill_render: RenderParam<RectParam>,
    layout: Option<LayoutKind>,
}

impl TableCell {
    pub fn new() -> TableCell {
        let mut cell_style = ClickStyle::new();
        cell_style.fill = FillStyle::same(Color::WHITE);
        cell_style.border = BorderStyle::same(Border::new(0.0).color(Color::BLUE));
        TableCell {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new(), cell_style)),
            layout: Some(LayoutKind::Horizontal(HorizontalLayout::left_to_right())),
        }
    }

    pub fn show_header<T: TableExt>(mut self, ui: &mut Ui, tui: &TableUi<T>, column: &TableColumn) {
        let mut current_layout = self.layout.take().unwrap();
        current_layout.set_rect(ui.available_rect().clone(), &Padding::same(0.0));
        let previous_layout = ui.layout.replace(current_layout).unwrap();
        tui.show_header(ui, column);
        self.layout = ui.layout.replace(previous_layout);
        self.fill_render.param.rect = self.layout.as_ref().unwrap().drawn_rect();
        self.fill_render.param.rect.set_width(column.width());
        self.fill_render.init_rectangle(ui, false, false);
        ui.add(self);
    }

    pub fn show_body<T: TableExt>(mut self, ui: &mut Ui, header: &TableHeader<T>, row_datum: &TableRowData<T>) {
        let mut current_layout = self.layout.take().unwrap();
        current_layout.set_rect(ui.available_rect().clone(), &Padding::same(0.0));
        let previous_layout = ui.layout.replace(current_layout).unwrap();
        let tui = &header.uis[row_datum.column_index()];
        tui.show_body(ui, row_datum);
        self.layout = ui.layout.replace(previous_layout);
        self.fill_render.param.rect = self.layout.as_ref().unwrap().drawn_rect();
        self.fill_render.param.rect.set_width(header.columns[row_datum.column_index()].width());
        self.fill_render.init_rectangle(ui, false, false);
        ui.add(self);
    }
}

impl Widget for TableCell {
    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.layout.as_mut().unwrap().redraw(ui);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::None => {}
            UpdateType::Init => {}
            UpdateType::ReInit => {}
            UpdateType::MouseMove => {}
            UpdateType::MousePress => {}
            UpdateType::MouseRelease => {}
            UpdateType::MouseWheel => {}
            UpdateType::KeyRelease(_) => {}
            UpdateType::Offset(_) => {}
            UpdateType::Drop => {}
            UpdateType::IME(_) => {}
            UpdateType::CreateWindow => {}
        }
        Response::new(&self.id, &self.fill_render.param.rect)
    }
}