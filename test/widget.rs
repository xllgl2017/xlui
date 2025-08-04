use xlui::frame::{App, Application, WindowAttribute};
use xlui::layout::scroll_area::ScrollArea;
use xlui::ui::{Ui, UiM};
use xlui::widgets::button::Button;
use xlui::widgets::checkbox::CheckBox;
use xlui::widgets::label::Label;
use xlui::widgets::slider::Slider;
use xlui::widgets::spinbox::SpinBox;
use xlui::widgets::textedit::TextEdit;
use xlui::widgets::Widget;

fn main() {
    let attr = WindowAttribute {
        inner_size: (800, 600).into(),
        ..Default::default()
    };
    let mut app = Application::new().with_attrs(attr);
    app.run(XlUiApp::new());
}

struct XlUiApp {
    label: Label,
    count: i32,
}

impl XlUiApp {
    pub fn new() -> Self {
        Self {
            label: Label::new("hello".to_string()).width(100.0),
            count: 0,
        }
    }

    pub fn click1(&mut self, uim: &mut UiM) {
        self.count += 1;
        println!("count: {}", self.count);
    }

    pub fn click2(&mut self, uim: &mut UiM) {
        self.count += 2;
        println!("count2: {}", self.count);
    }

    pub fn add(&mut self, uim: &mut UiM) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(uim);
    }

    pub fn reduce(&mut self, uim: &mut UiM) {
        self.count -= 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(uim);
    }

    pub fn slider(&mut self, uim: &mut UiM, value: f32) {
        self.label.set_text(format!("slider: {}", value));
        self.label.update(uim);
    }

    pub fn check(&mut self, uim: &mut UiM, checked: bool) {
        self.label.set_text(format!("check: {}", checked));
        self.label.update(uim);
    }

    pub fn spinbox(&mut self, uim: &mut UiM, value: i32) {
        self.label.set_text(format!("spinbox: {}", value));
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


        TextEdit::new("sdsd".to_string()).draw(ui);
    }
}
