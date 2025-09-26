use xlui::{App, TabWidget, Ui};

pub struct TestTabWidget {}

impl TestTabWidget {
    pub fn new() -> TestTabWidget {
        TestTabWidget {}
    }
}

impl App for TestTabWidget {
    fn draw(&mut self, ui: &mut Ui) {
        let mut widget = TabWidget::new();
        widget.add_tab(ui, "1", |ui| {
            ui.label("tab1");
        });
        widget.add_tab(ui, "2", |ui| {
            ui.label("tab2");
        });
        ui.add(widget);
    }
}