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
        let style = FrameStyle {
            fill: Color::rgb(240, 240, 240),
            radius: Radius::same(5),
            shadow: Shadow {
                offset: [0.0, 0.0],
                spread: 10.0,
                color: Color::rgba(0, 0, 0, 30),
            },
        };
        let layout: &mut VerticalLayout = ui.layout().as_mut_().unwrap();
        layout.set_padding(Padding::same(10.0));
        layout.set_style(style);
        ui.add(Label::new(self.status.as_str().color(Color::ORANGE)).with_id("status"));
        for i in 10..=28 {
            ui.add(Label::new(format!("当前字号: {}px", i).size(i as f32).color(Color::GREEN)));
        }
    }

    fn update(&mut self, _: &mut Ui) {}


    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            font: Arc::new(Font::from_family("微软雅黑").with_size(24.0)),
            inner_size: (300, 500).into(),
            transparent: true,
            decorations: false,
            fill: Color::TRANSPARENT,
            ..Default::default()
        }
    }
}
