use xlui::align::Align;
use xlui::frame::App;
use xlui::style::color::Color;
use xlui::style::{ClickStyle, FillStyle};
use xlui::text::rich::RichTextExt;
use xlui::ui::Ui;
use xlui::{Button, Label, WindowAttribute};

pub struct TestAlign {}

impl TestAlign {
    pub fn new() -> Self {
        TestAlign {}
    }
}


impl App for TestAlign {
    fn draw(&mut self, ui: &mut Ui) {
        let mut style = ClickStyle::new();
        style.fill = FillStyle::same(Color::rgb(235, 152, 235));
        ui.horizontal(|ui| {
            ui.add(Label::new("AlignTop🦀".color(Color::GREEN).size(18.0)).height(50.0).width(100.0).align(Align::Center));
            ui.add(Button::new("LT").width(50.0).height(50.0).align(Align::LeftTop).with_style(style.clone()));
            ui.add(Button::new("CT").width(50.0).height(50.0).align(Align::CenterTop).with_style(style.clone()));
            ui.add(Button::new("RT").width(50.0).height(50.0).align(Align::RightTop).with_style(style.clone()));
        });
        ui.horizontal(|ui| {
            ui.add(Label::new("AlignCenter").height(50.0).width(100.0).align(Align::Center));
            ui.add(Button::new("CL").width(50.0).height(50.0).align(Align::LeftCenter).with_style(style.clone()));
            ui.add(Button::new("CC").width(50.0).height(50.0).align(Align::Center).with_style(style.clone()));
            ui.add(Button::new("CR").width(50.0).height(50.0).align(Align::RightCenter).with_style(style.clone()));
        });
        ui.horizontal(|ui| {
            ui.add(Label::new("AlignBottom").height(50.0).width(100.0).align(Align::Center));
            ui.add(Button::new("BL").width(50.0).height(50.0).align(Align::LeftBottom).with_style(style.clone()));
            ui.add(Button::new("BC").width(50.0).height(50.0).align(Align::CenterBottom).with_style(style.clone()));
            ui.add(Button::new("BR").width(50.0).height(50.0).align(Align::RightBottom).with_style(style.clone()));
        });
    }

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (400, 300).into(),
            ..Default::default()
        }
    }
}