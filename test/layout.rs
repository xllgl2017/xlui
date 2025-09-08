use xlui::Border;
use xlui::frame::App;
use xlui::layout::{HorizontalLayout, VerticalLayout};
use xlui::layout::scroll_area::ScrollArea;
use xlui::style::ClickStyle;
use xlui::style::color::Color;
use xlui::ui::Ui;

pub struct TestLayout {}

impl App for TestLayout {
    fn draw(&mut self, ui: &mut Ui) {
        ui.add_layout(HorizontalLayout::right_to_left(), |ui| {
            // println!("{:?}", ui.available_rect());
            ui.label("hhh1");
            ui.label("hhhh2");
        });
        ui.add_layout(VerticalLayout::bottom_to_top(), |ui| {
            println!("{:?}", ui.available_rect());
            ui.label("hhh1");
            println!("{:?}", ui.available_rect());
            ui.label("hhhh2");
        });
        // ui.horizontal(|ui| {
        //     ui.vertical(|ui| {
        //         println!("{:?}", ui.available_rect());
        //
        //         ui.horizontal(|ui| {
        //             ui.vertical(|ui| {
        //                 ui.label("ScrollArea1");
        //                 ScrollArea::new().with_size(200.0, 200.0).show(ui, |ui| {
        //                     ui.label("a1");
        //                     ui.label("a2");
        //                     ui.label("a3");
        //                     ui.label("a4");
        //                     ui.label("a5");
        //                     ui.label("a6");
        //                     ui.label("a7");
        //                     ui.label("a8");
        //                 });
        //             });
        //             ui.horizontal(|ui| {
        //                 let mut rect = ui.available_rect().clone();
        //                 rect.set_size(200.0, 150.0);
        //                 let mut style = ClickStyle::new();
        //                 style.fill.inactive = Color::rgba(170, 218, 234, 128);
        //                 style.fill.hovered = Color::rgba(170, 218, 234, 128);
        //                 style.fill.clicked = Color::rgba(170, 218, 234, 128);
        //                 style.border.inactive = Border::new(0.0);
        //                 style.border.hovered = Border::new(0.0);
        //                 style.border.clicked = Border::new(0.0);
        //                 ui.paint_rect(rect, style);
        //                 ui.add_space(10.0);
        //                 ui.label("Horizontal");
        //                 ui.vertical(|ui| {
        //                     ui.label("v1");
        //                     ui.label("v2");
        //                     ui.label("v3");
        //                 });
        //                 ui.label("h1");
        //                 ui.label("h2");
        //                 ui.label("h3");
        //                 ui.add_space(50.0);
        //             });
        //             ui.vertical(|ui| {
        //                 let mut rect = ui.available_rect().clone();
        //                 rect.set_size(200.0, 150.0);
        //                 let mut style = ClickStyle::new();
        //                 style.fill.inactive = Color::rgba(190, 140, 209, 128);
        //                 style.fill.hovered = Color::rgba(190, 140, 209, 128);
        //                 style.fill.clicked = Color::rgba(190, 140, 209, 128);
        //                 style.border.inactive = Border::new(0.0);
        //                 style.border.hovered = Border::new(0.0);
        //                 style.border.clicked = Border::new(0.0);
        //                 ui.paint_rect(rect, style);
        //                 ui.add_space(10.0);
        //                 ui.label("Vertical");
        //                 ui.horizontal(|ui| {
        //                     ui.label("v1");
        //                     ui.label("v2");
        //                     ui.label("v3");
        //                 });
        //                 ui.label("h1");
        //                 ui.label("h2");
        //                 ui.label("h3");
        //             });
        //             ui.label("right_left");
        //             ui.add_layout(HorizontalLayout::right_to_left(), |ui| {
        //                 ui.label("hhh1");
        //                 ui.label("hhhh2");
        //             });
        //         });
        //
        //         ui.horizontal(|ui| {
        //             ui.vertical(|ui| {
        //                 ui.label("ScrollArea2");
        //                 let area = ScrollArea::new().with_size(300.0, 200.0);
        //                 area.show(ui, |ui| {
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("ss");
        //                     ui.horizontal(|ui| {
        //                         ui.label("h1");
        //                         ui.label("h2");
        //                         ui.label("h3");
        //                         ui.label("h4");
        //                         ui.label("h5");
        //                         ui.label("h6");
        //                         ui.label("h7");
        //                         ui.label("h8");
        //                         ui.label("h9");
        //                         ui.label("h10");
        //                         ui.label("h11");
        //                         ui.label("h12");
        //                         ui.label("h13");
        //                         ui.label("h14");
        //                         ui.label("h15");
        //                         ui.label("h16");
        //                         ui.label("h17");
        //                     });
        //                     ui.label("se");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                     ui.label("s1");
        //                 });
        //             });
        //             ui.horizontal(|ui| {
        //                 ui.label("h1");
        //                 ui.label("h2");
        //                 ui.label("h3");
        //                 ui.label("h4");
        //                 ui.label("h5");
        //                 ui.vertical(|ui| {
        //                     ui.horizontal(|ui| {
        //                         ui.label("h1");
        //                         ui.label("h2");
        //                         ui.label("h3");
        //                         ui.label("h4");
        //                         ui.label("h5");
        //                     });
        //                     ui.horizontal(|ui| {
        //                         ui.label("h1");
        //                         ui.label("h2");
        //                         ui.label("h3");
        //                         ui.label("h4");
        //                         ui.label("h5");
        //                     });
        //                     ui.horizontal(|ui| {
        //                         ui.label("h1");
        //                         ui.label("h2");
        //                         ui.label("h3");
        //                         ui.label("h4");
        //                         ui.label("h5");
        //                     });
        //                     ui.horizontal(|ui| {
        //                         ui.label("h1");
        //                         ui.label("h2");
        //                         ui.label("h3");
        //                         ui.label("h4");
        //                         ui.label("h5");
        //                     });
        //                     ui.horizontal(|ui| {
        //                         ui.label("h1");
        //                         ui.label("h2");
        //                         ui.label("h3");
        //                         ui.label("h4");
        //                         ui.label("h5");
        //                     });
        //                     ui.horizontal(|ui| {
        //                         ui.label("h1");
        //                         ui.label("h2");
        //                         ui.label("h3");
        //                         ui.label("h4");
        //                         ui.label("h5");
        //                     });
        //                 });
        //             });
        //         });
        //     });
        // });
        // ui.label("h1");
        // ui.label("h2");
    }
}


fn main() {
    TestLayout {}.run().unwrap();
}