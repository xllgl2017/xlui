use std::sync::Arc;
use xlui::*;

fn main() {
    let app = XlUiApp::new();
    //直接调run()
    app.run().unwrap();
}

struct XlUiApp {
    status: String,
}


impl XlUiApp {
    fn new() -> XlUiApp {
        XlUiApp {
            status: "使用`微软雅黑`字体".to_string(),
        }
    }
}

//实现App trait
impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        ui.window().request_update();
        ui.add(Label::new(self.status.as_str().color(Color::ORANGE)).with_id("status"));
        for i in 10..=28 {
            ui.add(Label::new(format!("当前字号: {}px", i).size(i as f32).color(Color::GREEN)));
        }
    }

    fn update(&mut self, _: &mut Ui) {}


    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            font: Arc::new(Font::from_family("微软雅黑")
                .with_size(24.0)),
            inner_size: (300, 500).into(),
            ..Default::default()
        }
    }
}
