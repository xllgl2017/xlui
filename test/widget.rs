use xlui::frame::context::Context;
use xlui::frame::{App, WindowAttribute};
use xlui::ui::Ui;
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

    fn add(&mut self, ctx: &mut Context) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ctx);
    }

    fn reduce(&mut self, ctx: &mut Context) {
        self.count -= 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ctx);
    }

    fn slider(&mut self, ctx: &mut Context, value: f32) {
        self.label.set_text(format!("slider: {}", value));
        self.label.update(ctx);
    }

    fn check(&mut self, ctx: &mut Context, checked: bool) {
        self.label.set_text(format!("check: {}", checked));
        self.label.update(ctx);
    }

    fn spinbox(&mut self, ctx: &mut Context, value: i32) {
        self.label.set_text(format!("spinbox: {}", value));
        self.label.update(ctx);
    }

    fn radio(&mut self, ctx: &mut Context, checked: bool) {
        self.label.set_text(format!("radio: {}", checked));
        self.label.update(ctx);
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
        ui.horizontal(|ui| {
            ComboBox::new(vec![1, 2, 3]).with_popup_height(150.0).draw(ui);
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
