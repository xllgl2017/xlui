use std::fmt::Display;
use xlui::frame::App;
use xlui::layout::scroll_area::ScrollArea;
use xlui::ui::Ui;
use xlui::widgets::button::Button;
use xlui::widgets::checkbox::CheckBox;
use xlui::widgets::combobox::ComboBox;
use xlui::widgets::image::Image;
use xlui::widgets::label::Label;
use xlui::widgets::listview::ListView;
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

impl Display for TD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}


fn main() {
    XlUiApp::new().run();
}

struct XlUiApp {
    label: Label,
    count: i32,
    list_view: ListView<TD>,
}

impl XlUiApp {
    pub fn new() -> Self {
        let data = vec![
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
        ];
        Self {
            label: Label::new("hello".to_string()).width(200.0),
            count: 0,
            list_view: ListView::new(data).with_size(300.0, 400.0),
        }
    }

    pub fn click1(&mut self, _ui: &mut Ui) {
        self.count += 1;
        println!("count: {}", self.count);
    }

    pub fn click2(&mut self, _ui: &mut Ui) {
        self.count += 2;
        println!("count2: {}", self.count);
    }

    pub fn add(&mut self, ui: &mut Ui) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ui);
    }

    pub fn reduce(&mut self, ui: &mut Ui) {
        self.count -= 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ui);
    }

    pub fn slider(&mut self, ui: &mut Ui, value: f32) {
        self.label.set_text(format!("slider: {}", value));
        self.label.update(ui);
    }

    pub fn check(&mut self, ui: &mut Ui, checked: bool) {
        self.label.set_text(format!("check: {}", checked));
        self.label.update(ui);
    }

    pub fn edit_changed(&mut self, ui: &mut Ui, text: String) {
        self.label.set_text(format!("textedit: {}", text));
        self.label.update(ui);
    }


    fn combo_changed(&mut self, ctx: &mut Ui, item: &&str) {
        self.label.set_text(format!("combo: {}", item));
        self.label.update(ctx);
    }

    fn list_changed(&mut self, ui: &mut Ui) {
        self.label.set_text(format!("list: {}", self.list_view.current().as_ref().unwrap()));
        self.label.update(ui);
    }
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        ui.add_mut(&mut self.label);
        ui.horizontal(|ui| {
            ui.add(Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add));
            ui.add(Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce));
        });
        ui.add(TextEdit::new("sdsd".to_string()).width_id("xlui_edit").connect(Self::edit_changed));
        ui.add(SpinBox::new(1, 1, 1..10));
        ui.horizontal(|ui| {
            ui.add(Slider::new(10.0).with_range(0.0..100.0).connect(Self::slider));
            ui.slider(30.0, 0.0..100.0).set_callback(Self::slider);
        });
        ui.horizontal(|ui| {
            ui.add(CheckBox::new(false, "checkbox1").connect(Self::check));
            ui.checkbox(true, "checkbox2").set_callback(Self::check);
        });
        ui.horizontal(|ui| {
            ui.label("h1");
            ui.label("h1");
            ui.label("h1");
            let area = ScrollArea::new().with_size(300.0, 400.0);
            area.show(ui, |ui| {
                ui.label("start");
                ui.vertical(|ui| {
                    ui.label("sv1");
                    ui.label("sv2");
                    ui.button("sv3").set_callback(Self::click1);
                });
                ui.horizontal(|ui| {
                    ui.label("sh1");
                    ui.label("sh2");
                    ui.button("sh3").set_callback(Self::click2);
                });
                for i in 0..1000 {
                    ui.label(format!("i{}", i));
                }
                ui.label("end");
            });
            ui.add(ComboBox::new(vec!["item1", "item2", "item3", "item4", "item5", "item6"]).connect(Self::combo_changed).with_popup_height(150.0));
            self.list_view.set_callback(Self::list_changed);
            self.list_view.show(ui);
        });

        ui.label("hello label1");
        ui.image("logo.jpg", (200.0, 200.0));
        ui.add(Image::new("logo.jpg").with_size(200.0, 200.0));
    }

    fn update(&mut self, ui: &mut Ui) {
        self.label.update(ui);
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.label.redraw(ui);
    }
}
