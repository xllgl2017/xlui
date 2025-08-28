use std::sync::Arc;
use xlui::frame::App;
use xlui::layout::scroll_area::ScrollArea;
use xlui::ui::Ui;
use xlui::widgets::button::Button;
use xlui::widgets::combobox::ComboBox;
use xlui::{Font, WindowAttribute};
use xlui::widgets::textedit::TextEdit;

fn main() {
    XlUi {}.run();
}

struct XlUi {}

impl App for XlUi {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("sc");
            ui.button("btn");
            ui.checkbox(false, "1");
            ui.spinbox(1, 1, 0..100);
            ui.radio(false, "2");
        });
        ui.image("logo.jpg", (50.0, 50.0));
        ui.slider(0.0, 0.0..100.0);
        ui.select_value(1);
        ui.add(Button::image_and_text("logo.jpg", "btn").width(100.0).height(50.0));
        ui.add(TextEdit::single_edit("".to_string()));
        ui.horizontal(|ui| {
            ui.button("btn");
        });
        ui.vertical(|ui| {
            ui.button("btn1");
        });

        let area = ScrollArea::new().with_size(100.0, 100.0);
        area.show(ui, |ui| {
            ui.label("sd");
            ui.button("btn2");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
            ui.label("sd");
        });
        ui.add(ComboBox::new(vec![1, 2, 3, 4, 5, 6, 7, 8]).with_popup_height(150.0));
    }
    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            font: Arc::new(Font::from_file("../xrs/target/res/font/微软雅黑.ttf").with_size(24.0)),
            ..Default::default()
        }
    }
}