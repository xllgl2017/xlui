use xlui::frame::App;
use xlui::layout::scroll_area::ScrollArea;
use xlui::ui::Ui;
use xlui::widgets::button::Button;
use xlui::widgets::combobox::ComboBox;
use xlui::widgets::singleline::TextEdit;

fn main() {
    XlUi {}.run();
}

struct XlUi {}

impl App for XlUi {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("dsfdsf");
            ui.button("btn");
            ui.checkbox(false, "1");
            ui.spinbox(1, 1, 0..100);
            ui.radio(false, "2");
        });
        ui.image("logo.jpg", (50.0, 50.0));
        ui.slider(0.0, 0.0..100.0);
        ui.select_value(1);
        ui.add(Button::image_and_text("logo.jpg", "btn").width(100.0).height(50.0));
        ui.add(TextEdit::new("".to_string()));
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

    fn update(&mut self, ui: &mut Ui) {}

    fn redraw(&mut self, ui: &mut Ui) {}
}