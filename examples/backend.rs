use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::{sleep, spawn};
use std::time::Duration;
use xlui::*;

fn main() {
    XlUiApp::new().run().unwrap();
}

struct XlUiApp {
    status: String,
    count: i32,
    list_view: ListView<String>,
    channel: (Sender<String>, Receiver<String>),
}

impl XlUiApp {
    pub fn new() -> Self {
        let mut data = vec![];
        for i in 0..5 {
            data.push(i.to_string());
        }
        Self {
            status: "".to_string(),
            count: 0,
            list_view: ListView::new(data).with_size(280.0, 300.0).with_item_height(38.0),
            channel: channel(),
        }
    }

    fn list_changed(&mut self, _: &mut Ui) {
        println!("change");
        // self.label.set_text(format!("list: {}", self.list_view.current().as_ref().unwrap()));
        // self.label.update(ui);
    }

    fn list_add(&mut self, _: &mut Button, _: &mut Ui) {
        println!("add list item");
        self.list_view.push(self.count.to_string());
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
        self.list_view.on_scrolling(|datum, layout| {
            let label: &mut Label = layout.get_widget(&"list_item".to_string()).unwrap();
            label.set_text(datum.to_string());
        });
        self.list_view.show(ui);
        let window = ui.window();
        let sender = self.channel.0.clone();
        spawn(move || {
            let mut num = 0;
            loop {
                sleep(Duration::from_nanos(100));
                sender.send(num.to_string()).unwrap();
                window.request_update();
                num += 1;
            }
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        if let Ok(datum) = self.channel.1.try_recv() {
            self.list_view.push(datum)
        }
        self.list_view.update(ui);
    }

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (300, 330).into(),
            ..Default::default()
        }
    }
}
