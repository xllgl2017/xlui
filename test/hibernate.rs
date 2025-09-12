// use std::sync::Arc;
// use xlui::frame::App;
// use xlui::ui::Ui;
// use xlui::{Button, ComboBox, Font, Label, RichTextExt, ScrollWidget, TextEdit, TextWrap, WindowAttribute};
//
// fn main() {
//     XlUi {}.run();
// }
//
// struct XlUi {}
//
// impl App for XlUi {
//     fn draw(&mut self, ui: &mut Ui) {
//         ui.horizontal(|ui| {
//             ui.label("sc");
//             ui.add(Label::new("hello world xlui".wrap(TextWrap::WrapWorld)).width(100.0).height(100.0));
//             ui.button("btn");
//             ui.checkbox(false, "1");
//             ui.spinbox(1, 1, 0..100);
//             ui.radio(false, "2");
//         });
//         ui.image("logo.jpg", (50.0, 50.0));
//         ui.slider(0.0, 0.0..100.0);
//         ui.select_value(1);
//         ui.add(Button::image_and_text("logo.jpg", "btn").width(100.0).height(50.0));
//         ui.add(TextEdit::single_edit("fdfgfdhg的\n风格的fjhgkjhl".to_string()));
//         ui.horizontal(|ui| {
//             ui.button("btn");
//         });
//         ui.vertical(|ui| {
//             ui.button("btn1");
//         });
//
//         let area = ScrollWidget::vertical().with_size(100.0, 100.0);
//         area.show(ui, |ui| {
//             ui.label("sd");
//             ui.button("btn2");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//             ui.label("sd");
//         });
//         ui.add(ComboBox::new(vec![1, 2, 3, 4, 5, 6, 7, 8]).with_popup_height(150.0));
//     }
//     fn window_attributes(&self) -> WindowAttribute {
//         WindowAttribute {
//             font: Arc::new(Font::from_file("../xrs/target/res/font/微软雅黑.ttf").with_size(24.0)),
//             ..Default::default()
//         }
//     }
// }

use xlui::frame::App;
use xlui::*;
use xlui::ui::Ui;

fn main() {
    let app=XlUiApp::new();
    //直接调run()
    app.run().unwrap();
}

struct XlUiApp {
    status:String,
    count: i32,
}


impl XlUiApp {
    fn new()->XlUiApp{
        XlUiApp{
            count: 0,
            status:"这里是Label".to_string()
        }
    }
    fn add(&mut self,_:&mut Button,ui: &mut Ui){
        self.count += 1;
        self.status=format!("count: {}", self.count);
    }

    fn reduce(&mut self,_:&mut Button,ui: &mut Ui){
        self.count-=1;
        self.status=format!("count: {}", self.count);
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
        let status:&mut Label=ui.get_widget("status").unwrap();
        status.set_text(&self.status);
    }


    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute{
            inner_size:(800,600).into(),
            ..Default::default()
        }
    }
}