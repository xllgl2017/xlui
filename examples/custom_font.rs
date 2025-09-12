use std::sync::Arc;
use xlui::*;

fn main() {
    let app = XlUiApp::new();
    //直接调run()
    app.run().unwrap();
}

struct XlUiApp {
    status: String,
    count: i32,
}


impl XlUiApp {
    fn new() -> XlUiApp {
        XlUiApp {
            count: 0,
            status: "这里是Label".to_string(),
        }
    }
    fn add(&mut self, _: &mut Button, _: &mut Ui) {
        self.count += 1;
        self.status = format!("count: {}", self.count);
    }

    fn reduce(&mut self, _: &mut Button, ui: &mut Ui) {
        self.count -= 1;
        self.status = format!("count: {}", self.count);
    }
}

//实现App trait
impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        ui.add(Label::new("hello").with_id("status"));
        ui.horizontal(|ui| {
            ui.add(Button::new("+").width(30.0).height(30.0).connect(Self::add));
            ui.add(Button::new("-").width(30.0).height(30.0).connect(Self::reduce));
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        let status: &mut Label = ui.get_widget("status").unwrap();
        status.set_text(&self.status);
    }


    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            font: Arc::new(Font::from_file("../xrs/target/res/font/微软雅黑.ttf")
                .with_size(24.0)),
            inner_size: (300, 200).into(),
            ..Default::default()
        }
    }
}
