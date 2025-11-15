use std::fmt::Display;
use xlui::*;

struct TD {
    name: String,
}

impl TD {
    fn new(name: impl ToString) -> TD {
        TD { name: name.to_string() }
    }

    fn on_scroll(&self, layout: &mut LayoutKind) {
        println!("{}  {}", "onscroll", self.name);
        let name: &mut Label = layout.get_widget(&"item_id".to_string()).unwrap();
        name.set_text(&self.name);
    }
}

impl Display for TD {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}


fn main() {
    XlUiApp::new().run().unwrap();
}

struct XlUiApp {
    status: String,
    count: i32,
    list_view: ListView<TD>,
}

impl XlUiApp {
    pub fn new() -> Self {
        let mut data = vec![];
        for i in 0..5 {
            data.push(TD::new((i + 1).to_string()));
        }
        Self {
            status: "".to_string(),
            count: 0,
            list_view: ListView::new(data).with_size(280.0, 300.0).with_item_height(38.0),
        }
    }

    pub fn click1(&mut self, _: &mut Button, _: &mut Ui) {
        self.count += 1;
        println!("count: {}", self.count);
    }

    pub fn click2(&mut self, _: &mut Button, _: &mut Ui) {
        self.count += 2;
        println!("count2: {}", self.count);
    }

    pub fn add(&mut self, btn: &mut Button, _: &mut Ui) {
        self.count += 1;
        self.status = format!("count: {}", self.count);
        btn.set_text(&self.status);
    }

    pub fn reduce(&mut self, _: &mut Button, _: &mut Ui) {
        self.count -= 1;
        self.status = format!("count: {}", self.count);
    }

    pub fn slider(&mut self, _: &mut Ui, value: f32) {
        self.status = format!("slider: {}", value);
    }

    pub fn check(&mut self, _: &mut Ui, checked: bool) {
        self.status = format!("check: {}", checked);
    }

    pub fn edit_changed(&mut self, _: &mut Ui, text: String) {
        self.status = format!("textedit: {}", text);
    }


    fn combo_changed(&mut self, _: &mut Ui, item: &&str) {
        self.status = format!("combo: {}", item);
    }

    fn list_changed(&mut self, _: &mut Ui) {
        // self.label.set_text(format!("list: {}", self.list_view.current().as_ref().unwrap()));
        // self.label.update(ui);
    }

    fn list_add(&mut self, _: &mut Button, _: &mut Ui) {
        println!("add list item");
        self.list_view.push(TD::new(self.count));
        self.count += 1;
    }

    fn list_delete(&mut self, _: &mut Button, _: &mut Ui) {
        let current = self.list_view.current_index().unwrap();
        self.list_view.remove(current);
        // ui.request_update(UpdateType::None);
    }
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add(Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add));
            ui.add(Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce));
        });
        ui.add(TextEdit::single_edit("sdsd".to_string()));
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
            let area = ScrollWidget::vertical().enable_hscroll().with_size(300.0, 400.0);
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
                    // for i in 0..1000 {
                    //     ui.label(format!("h{}", i));
                    // }
                });
                for i in 0..1000 {
                    ui.label(format!("i{}", i));
                }
                ui.label("end");
            });
            ui.add(ComboBox::new(vec!["item1", "item2", "item3", "item4", "item5", "item6"]).connect(Self::combo_changed).with_popup_height(150.0));
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.button("添加").set_callback(Self::list_add);
                    ui.button("删除").set_callback(Self::list_delete);
                });
                self.list_view.set_callback(Self::list_changed);
                self.list_view.on_scrolling(TD::on_scroll);
                self.list_view.set_item_widget(|ui, datum| {
                    ui.image("logo.jpg", (30.0, 30.0));
                    ui.vertical(|ui| {
                        ui.add(Label::new(datum.to_string()).with_id("item_id"));
                        ui.horizontal(|ui| {
                            ui.label("00:00");
                            ui.label("200");
                            ui.label("HTTP/1.1");
                            ui.label("10 KB");
                            ui.label("10 KB");
                        });
                    });
                });
                self.list_view.show(ui);
            });
        });

        ui.label("hello label1");
        ui.image("logo.jpg", (200.0, 200.0));
        ui.add(Image::new("logo.jpg").with_size(200.0, 200.0));
    }

    fn update(&mut self, ui: &mut Ui) {
        self.list_view.update(ui);
    }
}
