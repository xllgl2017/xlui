pub struct TableColumn {
    name: String,
    width: f32,
    filter: bool,
    sort: bool,
}

impl TableColumn {
    pub fn new_name(name: impl ToString) -> Self {
        TableColumn {
            name: name.to_string(),
            width: 100.0,
            filter: false,
            sort: false,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn width(&self) -> f32 {
        self.width
    }

    pub(crate) fn filter(&self) -> bool {
        self.filter
    }

    pub(crate) fn sort(&self) -> bool {
        self.sort
    }

    pub fn with_filter(mut self, filter: bool) -> Self {
        self.filter=filter;
        self
    }

    pub fn with_sort(mut self, sort: bool) -> Self {
        self.sort=sort;
        self
    }

    pub fn with_width(mut self, width: f32) -> Self {
        self.width=width;
        self
    }
}