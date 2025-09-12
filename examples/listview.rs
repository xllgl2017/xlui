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
    }
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
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
    }

    fn update(&mut self, ui: &mut Ui) {
        self.list_view.update(ui);
    }

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (300, 330).into(),
            ..Default::default()
        }
    }
}
