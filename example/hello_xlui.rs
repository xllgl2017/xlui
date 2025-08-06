use xlui::frame::App;
use xlui::layout::popup::Popup;
use xlui::layout::scroll_area::ScrollArea;
use xlui::ui::{Ui, UiM};
use xlui::widgets::button::Button;
use xlui::widgets::checkbox::CheckBox;
use xlui::widgets::combobox::ComboBox;
use xlui::widgets::label::Label;
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
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        self.label.draw(ui);
        ui.horizontal(|ui| {
            Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add).draw(ui);
            Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce).draw(ui);
        });
        // println!("draw");
        // ScrollBar::new().size(20.0, 200.0).draw(ui);
        TextEdit::new("sdsd".to_string()).draw(ui);
        SpinBox::new(1).with_range(0..10).draw(ui);
        ui.horizontal(|ui| {
            Slider::new(10.0).with_range(0.0..100.0).connect(Self::slider).draw(ui);
            ui.slider(30.0, 0.0..100.0).connect(Self::slider);
        });
        ui.horizontal(|ui| {
            CheckBox::new(false, "checkbox1").connect(Self::check).draw(ui);
            ui.checkbox(true, "checkbox2").connect(Self::check);
        });
        ui.horizontal(|ui| {
            let area = ScrollArea::new().with_size(300.0, 400.0);
            area.show(ui, |ui| {
                ui.label("start");
                ui.vertical(|ui| {
                    ui.label("sv1");
                    ui.label("sv2");
                    ui.button("sv3").connect(Self::click1);
                });
                ui.horizontal(|ui| {
                    ui.label("sh1");
                    ui.label("sh2");
                    ui.button("sh3");
                });
                for i in 0..1000 {
                    ui.label(format!("i{}", i));
                }
                ui.label("end");
            });
            ComboBox::new().with_popup_height(150.0).with_widgets(|ui| {
                ui.label("c1");
                ui.label("c2");
                ui.label("c3");
                ui.label("c4");
                ui.label("c5");

            }).draw(ui);
        });

        ui.label("hello label1");
        // ui.horizontal(|ui| {
        //     ui.label("hello label6");
        //     ui.label("hello label7");
        //     ui.button("+").connect(Self::click2);
        //     ui.button("-").connect(Self::click1);
        // });
        // ui.label("hello label2");
        // ui.label("hello label3");
        // ui.label("hello label4");
        // ui.label("hello label5");
        // // let mut button = Button::new("hello button1".to_string()).connect(Self::click1);
        //
        // ui.button("hello button1").connect(Self::click1);
        // ui.button("hello button2");
        // ui.button("hello button3");
        // ui.button("hello button4");
        // ui.button("hello button5");
        // ui.image("logo.jpg", (200.0, 200.0));
        // Image::new("logo.jpg").with_size(200.0, 200.0).draw(ui);
        // ui.image("logo.jpg", (200.0, 200.0));
    }

    // fn as_any(&mut self) -> &mut dyn Any {
    //     todo!()
    // }
}
