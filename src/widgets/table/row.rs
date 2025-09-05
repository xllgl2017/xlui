use crate::frame::context::UpdateType;
use crate::layout::{HorizontalLayout, LayoutKind};
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::response::Response;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::table::TableExt;
use crate::{Padding, Rect, Widget};
use crate::column::Column;
use crate::map::Map;
use crate::table::header::TableHeader;

pub struct TableRow {
    id: String,
    fill_render: RenderParam<RectParam>,
    columns: Map<Column>,
}

impl TableRow {
    pub fn new<T>(headers: &TableHeader<T>) -> TableRow {
        let mut columns = vec![];
        for column in &headers.columns {
            columns.push(Column::new().width(column.width()).resize(true));
        }
        TableRow {
            id: crate::gen_unique_id(),
            fill_render: RenderParam::new(RectParam::new(Rect::new(), ClickStyle::new())),
            columns: Map::new(),
        }
    }

    pub fn add_row(&mut self){

    }

    pub(crate) fn init(&mut self, ui: &mut Ui, init: bool) {
        let mut current_layout = self.layout.take().unwrap();
        if init {
            let mut rect = ui.available_rect().clone(); //.clone_with_size(current_layout.max_rect());
            rect.set_height(current_layout.max_rect().height());
            current_layout.set_rect(rect.clone(), &Padding::same(0.0));
            self.fill_render.param.rect = rect;
        }
        self.fill_render.init_rectangle(ui, false, false);
        self.layout = Some(current_layout);
    }
}

impl Widget for TableRow {
    fn redraw(&mut self, ui: &mut Ui) {
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.layout.as_mut().unwrap().redraw(ui);
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::None => {}
            UpdateType::Init => {}
            UpdateType::ReInit => self.init(ui, false),
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

pub struct TableRowData<T> {
    row: usize,
    column: usize,
    data: T,
    height: f32,
    enable: bool,
    selected: bool,
    hidden: bool,
}


impl<T> TableRowData<T> {
    pub fn set_column(&mut self, column: usize) {
        self.column = column;
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    pub fn column_index(&self) -> usize {
        self.column
    }
}

impl<T: TableExt> TableRowData<T> {
    pub fn new(t: T, row: usize, height: f32) -> Self {
        TableRowData {
            row,
            column: 0,
            data: t,
            height,
            enable: false,
            selected: false,
            hidden: false,
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