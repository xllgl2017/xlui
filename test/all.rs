mod rectangle;
mod triangle;

use std::fmt::{Display, Formatter};
use xlui::frame::{App, WindowAttribute};
use xlui::layout::inner::InnerWindow;
use xlui::ui::Ui;
use xlui::widgets::button::Button;
use crate::rectangle::TestRectangle;

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

    fn on_inner_close(&mut self, window: InnerWindow, _: &mut Ui) {
        let frame: TestRectangle = window.to_();
        println!("{} {}", frame.border_radius, frame.border_width);
    }

    fn open_test_widget(&mut self, _: &mut Button, ui: &mut Ui) {
        ui.create_inner_window(TestRectangle::new()).on_close(Self::on_inner_close);
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