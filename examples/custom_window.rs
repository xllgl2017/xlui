use std::process::exit;
use xlui::*;

fn main() {
    let app = XlUiApp::new();
    //直接调run()
    app.run().unwrap();
}

struct XlUiApp {}


impl XlUiApp {
    fn new() -> XlUiApp {
        XlUiApp {}
    }

    fn close(&mut self, _: &mut Button, _: &mut Ui) {
        exit(0);
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
                spread: 5.0,
                color: Color::rgba(0, 0, 0, 100),
            },
            border: Border::new(2.0).color(Color::rgb(150, 210, 255)),
        };
        let layout: &mut VerticalLayout = ui.layout().as_mut_().unwrap();
        layout.set_padding(Padding::same(2.0));
        layout.set_margin(Margin::same(0.0));
        layout.set_style(style);
        let title_layout = HorizontalLayout::left_to_right().with_size(296.0, 25.0)
            .with_fill(Color::rgb(210, 210, 210)).moving();
        ui.add_layout(title_layout, |ui| {
            ui.image("logo.jpg", (25.0, 25.0));
            ui.add(Label::new("自定义标题栏").align(Align::Center).height(25.0));
            ui.add_layout(HorizontalLayout::right_to_left(), |ui| {
                let mut style = ClickStyle::new();
                style.fill.inactive = Color::TRANSPARENT;
                style.fill.hovered = Color::rgba(255, 0, 0, 100);
                style.fill.clicked = Color::rgba(255, 0, 0, 150);
                style.border = BorderStyle::same(Border::new(0.0).radius(Radius::same(0)));
                let mut btn = Button::new("×").width(20.0).height(20.0).connect(Self::close);
                btn.set_style(style.clone());
                ui.add(btn);
                let mut btn = Button::new("□").width(20.0).height(20.0);
                style.fill.hovered = Color::rgba(160, 160, 160, 100);
                style.fill.clicked = Color::rgba(160, 160, 160, 150);
                btn.set_style(style.clone());
                ui.add(btn);
            });
        });
        ui.vertical(|ui| {
            ui.label("sdfgdfg");
        })
    }

    fn update(&mut self, _: &mut Ui) {}


    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            // font: Arc::new(Font::from_family("微软雅黑").with_size(24.0)),
            inner_size: (300, 520).into(),
            transparent: true,
            decorations: false,
            fill: Color::TRANSPARENT,
            ..Default::default()
        }
    }
}
