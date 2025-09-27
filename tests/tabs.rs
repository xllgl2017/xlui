use xlui::{App, TabWidget, Ui};

pub struct TestTabWidget {}

impl TestTabWidget {
    pub fn new() -> TestTabWidget {
        TestTabWidget {}
    }
}

impl App for TestTabWidget {
    fn draw(&mut self, ui: &mut Ui) {
        let mut widget = TabWidget::new().with_size(400.0, 300.0);
        widget.add_tab(ui, "1", |ui| {
            ui.label("tab1");
        });
        widget.add_tab(ui, "2", |ui| {
            ui.label("tab2");
        });
        ui.add(widget);
    }
}