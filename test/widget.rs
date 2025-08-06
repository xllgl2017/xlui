use xlui::frame::{App, WindowAttribute};
use xlui::ui::{Ui, UiM};
use xlui::widgets::button::Button;
use xlui::widgets::checkbox::CheckBox;
use xlui::widgets::combobox::ComboBox;
use xlui::widgets::label::Label;
use xlui::widgets::radio::RadioButton;
use xlui::widgets::slider::Slider;
use xlui::widgets::spinbox::SpinBox;
use xlui::widgets::textedit::TextEdit;
use xlui::widgets::Widget;

fn main() {
    XlUiApp::new().run();
}

struct XlUiApp {
    label: Label,
    count: i32,
}

impl XlUiApp {
    fn new() -> Self {
        Self {
            label: Label::new("hello".to_string()).width(100.0),
            count: 0,
        }
    }

    fn add(&mut self, uim: &mut UiM) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(uim);
    }

    fn reduce(&mut self, uim: &mut UiM) {
        self.count -= 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(uim);
    }

    fn slider(&mut self, uim: &mut UiM, value: f32) {
        self.label.set_text(format!("slider: {}", value));
        self.label.update(uim);
    }

    fn check(&mut self, uim: &mut UiM, checked: bool) {
        self.label.set_text(format!("check: {}", checked));
        self.label.update(uim);
    }

    fn spinbox(&mut self, uim: &mut UiM, value: i32) {
        self.label.set_text(format!("spinbox: {}", value));
        self.label.update(uim);
    }

    fn radio(&mut self, uim: &mut UiM, checked: bool) {
        self.label.set_text(format!("radio: {}", checked));
        self.label.update(uim);
    }
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        self.label.draw(ui);
        ui.label("hello label1");
        ui.horizontal(|ui| {
            Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add).draw(ui);
            Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce).draw(ui);
        });

        ui.horizontal(|ui| {
            Slider::new(10.0).with_range(0.0..100.0).connect(Self::slider).draw(ui);
            ui.slider(30.0, 0.0..100.0).connect(Self::slider);
        });
        ui.horizontal(|ui| {
            CheckBox::new(false, "checkbox1").connect(Self::check).draw(ui);
            ui.checkbox(true, "checkbox2").connect(Self::check);
        });
        ui.horizontal(|ui| {
            SpinBox::new(1).with_range(0..10).connect(Self::spinbox).draw(ui);
            ui.spinbox(1, 0..10).connect(Self::spinbox);
        });
        ui.horizontal(|ui| {
            RadioButton::new(false, "radiobutton").connect(Self::radio).draw(ui);
            ui.radio(true, "radiobutton").connect(Self::radio);
        });
        ui.horizontal(|ui|{
            ComboBox::new().with_popup_height(150.0).with_widgets(|ui|{
                ui.label("c1");
                ui.label("c2");
                ui.label("c3");
                ui.label("c4");
                ui.label("c5");
            }).draw(ui);
        });


        TextEdit::new("sdsd".to_string()).draw(ui);
    }

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (800, 600).into(),
            ..Default::default()
        }
    }
}
