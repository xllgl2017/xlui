mod shape;
mod widget;
mod align;

use std::fmt::{Display, Formatter};
use xlui::frame::App;
use xlui::{Button, InnerWindow, WindowAttribute};
use xlui::ui::Ui;
use crate::align::TestAlign;
use crate::shape::TestShape;
use crate::widget::TestWidget;

#[derive(PartialEq)]
enum TestKind {
    Shape,
    Widgets,
    Align,
}

impl Display for TestKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TestKind::Shape => f.write_str("Shape"),
            TestKind::Widgets => f.write_str("Widgets"),
            TestKind::Align => f.write_str("Align"),
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

    fn on_rect_close(&mut self, window: InnerWindow, _: &mut Ui) {
        let frame: TestShape = window.to_();
        println!("{} {}", frame.border_radius, frame.border_width);
    }


    fn open_test_shape(&mut self, _: &mut Button, ui: &mut Ui) {
        ui.create_inner_window(TestShape::new()).on_close(Self::on_rect_close);
    }

    fn open_test_widgets(&mut self, _: &mut Button, ui: &mut Ui) {
        ui.create_inner_window(TestWidget::new());
    }

    fn open_test_align(&mut self, _: &mut Button, ui: &mut Ui) {
        ui.create_inner_window(TestAlign::new());
    }
}

impl App for XlUi {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.button(TestKind::Shape).set_callback(Self::open_test_shape);
            ui.button(TestKind::Widgets).set_callback(Self::open_test_widgets);
            ui.button(TestKind::Align).set_callback(Self::open_test_align);
        });
    }
    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (1920, 1080).into(),
            ..Default::default()
        }
    }
}