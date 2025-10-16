use crate::frame::context::UpdateType;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::Response;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::ui::Ui;
use crate::widgets::table::column::TableColumn;
use crate::widgets::table::header::{TableHeader, TableUi};
use crate::widgets::table::row::TableRowData;
use crate::widgets::{WidgetChange, WidgetSize, WidgetState};
use crate::{Border, HorizontalLayout, LayoutKind, Padding, Radius, Rect, TableExt, Widget};
use crate::size::Geometry;

pub struct TableCell {
    pub(crate) id: String,
    fill_render: RenderParam,
    cell_line: RenderParam,
    layout: Option<LayoutKind>,
    geometry: Geometry,
    state: WidgetState,
}

impl TableCell {
    pub fn new(width: f32, height: f32) -> TableCell {
        let mut cell_style = ClickStyle::new();
        cell_style.fill = FillStyle::same(Color::rgb(235, 235, 235));
        cell_style.border = BorderStyle::same(Border::same(0.0).color(Color::BLUE).radius(Radius::same(0)));
        let layout = HorizontalLayout::left_to_right().with_size(width, height)
            .with_padding(Padding::same(0.0).left(5.0));
        let mut cell_line_style = ClickStyle::new();
        cell_line_style.fill = FillStyle::same(Color::rgb(160, 160, 160));
        cell_line_style.border = BorderStyle::same(Border::same(0.0).radius(Radius::same(0)));
        let fill_param = RectParam::new().with_rect(Rect::new().with_size(width, height)).with_style(cell_style);
        let cell_param = RectParam::new().with_rect(Rect::new().with_size(1.0, height)).with_style(cell_line_style);
        TableCell {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RenderKind::Rectangle(fill_param)),
            cell_line: RenderParam::new(RenderKind::Rectangle(cell_param)),
            layout: Some(LayoutKind::new(layout)),
            geometry: Geometry::new().with_context_size(width, height),
            state: WidgetState::default(),
        }
    }

    pub fn show_header<T: TableExt>(&mut self, ui: &mut Ui, tui: &TableUi<T>, column: &TableColumn) {
        let current_layout = self.layout.take().unwrap();
        let previous_layout = ui.layout.replace(current_layout).unwrap();
        tui.show_header(ui, column);
        self.layout = ui.layout.replace(previous_layout);
        #[cfg(feature = "gpu")]
        self.fill_render.init(ui, false, false);
        #[cfg(feature = "gpu")]
        self.cell_line.init(ui, false, false);
    }

    pub fn show_body<T: TableExt>(&mut self, ui: &mut Ui, header: &TableHeader<T>, row_datum: &TableRowData<T>) {
        let current_layout = self.layout.take().unwrap();
        let previous_layout = ui.layout.replace(current_layout).unwrap();
        let tui = &header.uis[row_datum.column_index()];
        tui.show_body(ui, row_datum);
        self.layout = ui.layout.replace(previous_layout);
        if (row_datum.column_index() % 2 == 0 && row_datum.row_index() % 2 != 0) || (row_datum.column_index() % 2 != 0 && row_datum.row_index() % 2 == 0) {
            self.fill_render.style_mut().fill = FillStyle::same(Color::rgb(245, 245, 245))
        }
        #[cfg(feature = "gpu")]
        self.cell_line.init(ui, false, false);
        #[cfg(feature = "gpu")]
        self.fill_render.init(ui, false, false);
    }
    fn redraw(&mut self, ui: &mut Ui) {
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.fill_render.rect_mut().offset_to_rect(&ui.draw_rect);
            let mut cell_rect = self.fill_render.rect_mut().clone();
            cell_rect.set_x_min(cell_rect.dx().max - 2.0);
            self.cell_line.rect_mut().offset_to_rect(&cell_rect);
        }
        self.fill_render.draw(ui, false, false);
        self.cell_line.draw(ui, false, false);
        self.layout.as_mut().unwrap().update(ui);
    }
}

impl Widget for TableCell {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::MouseMove => {
                let mut rect = self.cell_line.rect_mut().clone();
                rect.add_min_x(-2.0);
                rect.add_max_x(2.0);
                if ui.device.device_input.hovered_at(&rect) {
                    self.cell_line.style_mut().border.inactive.set_same(2.0);
                    ui.context.window.request_redraw();
                }
            }
            _ => {}
        }
        self.layout.as_mut().unwrap().update(ui);
        Response::new(&self.id, WidgetSize::same(self.geometry.margin_width(), self.geometry.margin_height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}