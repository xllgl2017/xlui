mod frame;

use std::fmt::{Display, Formatter};
use xlui::frame::{App, WindowAttribute};
use xlui::ui::Ui;
use xlui::widgets::button::Button;
use crate::frame::TestFrame;

#[derive(PartialEq)]
enum TestKind {
    Widgets
}

impl Display for TestKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestKind::Widgets => f.write_str("Widgets"),
        }
    }
}

fn main() {
    XlUi::new().run();
}

struct XlUi {}

impl XlUi {
    pub fn new() -> XlUi {
        XlUi {}
    }
    pub fn open_test_widget(&mut self, _: &mut Button, ui: &mut Ui) {
        ui.create_inner_window(TestFrame::new());
    }
}

impl App for XlUi {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.button(TestKind::Widgets).set_callback(Self::open_test_widget);
        });
    }
    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (1920, 1080).into(),
            ..Default::default()
        }
    }
}