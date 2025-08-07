use xlui::frame::App;
use xlui::layout::scroll_area::ScrollArea;
use xlui::paint::color::Color;
use xlui::radius::Radius;
use xlui::size::border::Border;
use xlui::style::{BorderStyle, ClickStyle, FillStyle};
use xlui::ui::{Ui, UiM};
use xlui::widgets::button::Button;
use xlui::widgets::checkbox::CheckBox;
use xlui::widgets::combobox::ComboBox;
use xlui::widgets::image::Image;
use xlui::widgets::label::Label;
use xlui::widgets::slider::Slider;
use xlui::widgets::spinbox::SpinBox;
use xlui::widgets::textedit::TextEdit;
use xlui::widgets::Widget;

struct TD {
    name: String,
}

impl TD {
    fn new(name: impl ToString) -> TD {
        TD { name: name.to_string() }
    }
}

struct ListView {
    data: Vec<TD>,
}

impl ListView {
    fn new() -> ListView {
        ListView {
            data: vec![
                TD::new("1"),
                TD::new("2"),
                TD::new("3"),
                TD::new("4"),
                TD::new("5"),
                TD::new("6"),
                TD::new("7"),
                TD::new("8"),
                TD::new("9"),
                TD::new("0"),
                TD::new("11")
            ],
        }
    }
    fn item_click(app: &mut XlUiApp, uim: &mut UiM) {
        println!("item click");
    }

    fn item_widget(&self, ui: &mut Ui, datum: &TD) {
        let mut rect = ui.available_rect();
        rect.set_height(38.0);
        let style = ClickStyle {
            fill: FillStyle {
                inactive: Color::TRANSPARENT,
                hovered: Color::TRANSPARENT,
                clicked: Color::TRANSPARENT,
            },
            border: BorderStyle {
                inactive: Border::new(1.0).radius(Radius::same(3)).color(Color::BLUE),
                hovered: Border::new(1.0).radius(Radius::same(3)).color(Color::RED),
                clicked: Border::new(1.0).radius(Radius::same(3)).color(Color::YELLOW),
            },
        };
        ui.paint_rect(rect, style).connect(Self::item_click);
        ui.horizontal(|ui| {
            ui.image("logo.jpg", (30.0, 30.0));
            ui.vertical(|ui| {
                ui.label(datum.name.as_str());
                ui.horizontal(|ui| {
                    ui.label("00:00");
                    ui.label("200");
                    ui.label("HTTP/1.1");
                    ui.label("10 KB");
                    ui.label("10 KB");
                });
            });
        });
    }
}


impl Widget for ListView {
    fn draw(&mut self, ui: &mut Ui) {
        ScrollArea::new().with_size(300.0, 300.0).show(ui, |ui| {
            for datum in self.data.iter() {
                self.item_widget(ui, datum);
            }
        });
    }

    fn update(&mut self, uim: &mut UiM) {
        todo!()
    }
}

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
            label: Label::new("hello".to_string()).width(200.0),
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
        let text = uim.get_edit_text("xlui_edit");
        println!("text: {}", text);
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

    pub fn edit_changed(&mut self, uim: &mut UiM, text: &str) {
        self.label.set_text(format!("textedit: {}", text));
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
        TextEdit::new("sdsd".to_string()).width_id("xlui_edit").connect(Self::edit_changed).draw(ui);
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
            ui.add(ListView::new());
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
        Image::new("logo.jpg").with_size(200.0, 200.0).draw(ui);
        ui.image("logo.jpg", (200.0, 200.0));
    }

    // fn as_any(&mut self) -> &mut dyn Any {
    //     todo!()
    // }
}
