mod shape;

use std::fmt::{Display, Formatter};
use xlui::frame::{App, WindowAttribute};
use xlui::layout::inner::InnerWindow;
use xlui::ui::Ui;
use xlui::widgets::button::Button;
use crate::shape::TestShape;

#[derive(PartialEq)]
enum TestKind {
    Shape,
}

impl Display for TestKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestKind::Shape => f.write_str("Shape"),
        }
    }
}

fn main() {
    // TestTriangle::new().run();
    XlUi::new().run();
}

struct XlUi {}

impl XlUi {
    pub fn new() -> XlUi {
        XlUi {}
    }

    fn on_rect_close(&mut self, window: InnerWindow, _: &mut Ui) {
        let frame: TestShape = window.to_();
        println!("{} {}", frame.border_radius, frame.border_width);
    }


    fn open_test_rectangle(&mut self, _: &mut Button, ui: &mut Ui) {
        ui.create_inner_window(TestShape::new()).on_close(Self::on_rect_close);
    }

}

impl App for XlUi {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.button(TestKind::Shape).set_callback(Self::open_test_rectangle);
        });
    }
    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (1920, 1080).into(),
            ..Default::default()
        }
    }
}