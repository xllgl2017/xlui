use std::fmt::Display;
use xlui::frame::context::UpdateType;
use xlui::frame::App;
use xlui::{HorizontalLayout, Label, ListView, Padding, ScrollWidget, VerticalLayout};
// use xlui::{Button, CheckBox, ComboBox, Image, Label, ListView, Slider, SpinBox, TextEdit, Widget};
// use xlui::layout::scroll_area::ScrollArea;
use xlui::ui::Ui;

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
    // label: Label,
    count: i32,
    list_view: ListView<TD>,
}

impl XlUiApp {
    pub fn new() -> Self {
        let mut data = vec![];
        for i in 0..100 {
            data.push(TD::new(i + 1));
        }
        Self {
            // label: Label::new("hello".to_string()).width(200.0),
            count: 0,
            list_view: ListView::new(data).with_size(250.0, 300.0),
        }
    }

    // pub fn click1(&mut self, _: &mut Button, _ui: &mut Ui) {
    //     self.count += 1;
    //     println!("count: {}", self.count);
    // }
    //
    // pub fn click2(&mut self, _: &mut Button, _ui: &mut Ui) {
    //     self.count += 2;
    //     println!("count2: {}", self.count);
    // }
    //
    // pub fn add(&mut self, btn: &mut Button, ui: &mut Ui) {
    //     self.count += 1;
    //     self.label.set_text(format!("count: {}", self.count));
    //     self.label.update(ui);
    //     btn.set_text(self.label.text());
    // }
    //
    // pub fn reduce(&mut self, _: &mut Button, ui: &mut Ui) {
    //     self.count -= 1;
    //     self.label.set_text(format!("count: {}", self.count));
    //     self.label.update(ui);
    // }
    //
    // pub fn slider(&mut self, ui: &mut Ui, value: f32) {
    //     self.label.set_text(format!("slider: {}", value));
    //     self.label.update(ui);
    // }
    //
    // pub fn check(&mut self, ui: &mut Ui, checked: bool) {
    //     self.label.set_text(format!("check: {}", checked));
    //     self.label.update(ui);
    // }
    //
    // pub fn edit_changed(&mut self, ui: &mut Ui, text: String) {
    //     self.label.set_text(format!("textedit: {}", text));
    //     self.label.update(ui);
    // }
    //
    //
    // fn combo_changed(&mut self, ctx: &mut Ui, item: &&str) {
    //     self.label.set_text(format!("combo: {}", item));
    //     self.label.update(ctx);
    // }
    //
    // fn list_changed(&mut self, ui: &mut Ui) {
    //     self.label.set_text(format!("list: {}", self.list_view.current().as_ref().unwrap()));
    //     self.label.update(ui);
    // }
    //
    // fn list_add(&mut self, _: &mut Button, ui: &mut Ui) {
    //     self.list_view.push(TD::new(self.count));
    //     self.count += 1;
    //     ui.request_update(UpdateType::None);
    // }
    //
    // fn list_delete(&mut self, _: &mut Button, ui: &mut Ui) {
    //     let current = self.list_view.current_index().unwrap();
    //     self.list_view.remove(current);
    //     ui.request_update(UpdateType::None);
    // }
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        ui.label("123456");
        let label = Label::new("456789");
        ui.add(label);
        let label = Label::new("456789");
        ui.add(label);
        let label = Label::new("456789");
        ui.add(label);
        let label = Label::new("456789");
        ui.add(label);
        ui.horizontal(|ui| {
            ui.label("1");
            ui.label("2");
            ui.label("3");
            ui.label("4");
            ui.label("5");
            ui.vertical(|ui| {
                ui.label("6");
                ui.label("7");
                ui.label("8");
                ui.label("9");
                ui.horizontal(|ui| {
                    let area = ScrollWidget::vertical().enable_hscroll().with_size(400.0, 300.0);
                    area.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.label("1");
                            ui.label("2");
                            ui.label("3");
                            for i in 0..100 {
                                ui.label(i);
                            }
                        });
                        for i in 0..100 {
                            ui.label(i);
                        }
                    });
                    self.list_view.set_item_widget(|ui,datum|{
                       ui.label("34");
                    });
                    self.list_view.show(ui);
                });
                ui.label("sdfdsf");
                let layout = HorizontalLayout::right_to_left();
                ui.add_layout(layout, |ui| {
                    ui.label("0");
                    ui.label("1");
                    ui.label("2");
                    ui.label("3");
                    ui.label("4");
                });
            });
        });

        let layout = VerticalLayout::bottom_to_top();
        ui.add_layout(layout, |ui| {
            ui.label("1");
            ui.label("2");
            ui.label("3");
            ui.label("4");
            ui.label("5");
            ui.label("6");
        });
        // ui.horizontal(|ui| {
        //     ui.add(Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add));
        //     ui.add(Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce));
        // });
        // ui.add(TextEdit::single_edit("sdsd".to_string()));
        // ui.add(SpinBox::new(1, 1, 1..10));
        // ui.horizontal(|ui| {
        //     ui.add(Slider::new(10.0).with_range(0.0..100.0).connect(Self::slider));
        //     ui.slider(30.0, 0.0..100.0).set_callback(Self::slider);
        // });
        // ui.horizontal(|ui| {
        //     ui.add(CheckBox::new(false, "checkbox1").connect(Self::check));
        //     ui.checkbox(true, "checkbox2").set_callback(Self::check);
        // });
        // ui.horizontal(|ui| {
        //     ui.label("h1");
        //     ui.label("h1");
        //     ui.label("h1");
        //     let area = ScrollArea::new().with_size(300.0, 400.0);
        //     area.show(ui, |ui| {
        //         ui.label("start");
        //         ui.vertical(|ui| {
        //             ui.label("sv1");
        //             ui.label("sv2");
        //             ui.button("sv3").set_callback(Self::click1);
        //         });
        //         ui.horizontal(|ui| {
        //             ui.label("sh1");
        //             ui.label("sh2");
        //             ui.button("sh3").set_callback(Self::click2);
        //         });
        //         for i in 0..1000 {
        //             ui.label(format!("i{}", i));
        //         }
        //         ui.label("end");
        //     });
        //     ui.add(ComboBox::new(vec!["item1", "item2", "item3", "item4", "item5", "item6"]).connect(Self::combo_changed).with_popup_height(150.0));
        //     ui.vertical(|ui| {
        //         ui.horizontal(|ui| {
        //             ui.button("添加").set_callback(Self::list_add);
        //             ui.button("删除").set_callback(Self::list_delete);
        //         });
        //         self.list_view.set_callback(Self::list_changed);
        //         self.list_view.set_item_widget(|ui, datum| {
        //             ui.image("logo.jpg", (30.0, 30.0));
        //             ui.vertical(|ui| {
        //                 ui.label(datum.to_string());
        //                 ui.horizontal(|ui| {
        //                     ui.label("00:00");
        //                     ui.label("200");
        //                     ui.label("HTTP/1.1");
        //                     ui.label("10 KB");
        //                     ui.label("10 KB");
        //                 });
        //             });
        //         });
        //         self.list_view.show(ui);
        //     });
        // });
        //
        // ui.label("hello label1");
        // ui.image("logo.jpg", (200.0, 200.0));
        // ui.add(Image::new("logo.jpg").with_size(200.0, 200.0));
    }

    fn update(&mut self, ui: &mut Ui) {
        // self.label.update(ui);
        // self.list_view.update(ui);
    }

    fn redraw(&mut self, ui: &mut Ui) {
        // self.label.redraw(ui);
    }
}
