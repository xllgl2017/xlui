
use xlui::*;

pub struct TestLayout {}

impl App for TestLayout {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("ScrollArea1");
                        ScrollWidget::vertical().with_size(200.0, 200.0).show(ui, |ui| {
                            ui.label("a1");
                            ui.label("a2");
                            ui.label("a3");
                            ui.label("a4");
                            ui.label("a5");
                            ui.label("a6");
                            ui.label("a7");
                            ui.label("a8");
                        });
                    });
                    let layout = HorizontalLayout::left_to_right().with_size(200.0, 150.0)
                        .with_fill(Color::rgba(170, 218, 234, 128)).with_padding(Padding::same(5.0));
                    ui.add_layout(layout, |ui| {
                        ui.label("Horizontal");
                        ui.vertical(|ui| {
                            ui.label("v1");
                            ui.label("v2");
                            ui.label("v3");
                        });
                        ui.label("h1");
                        ui.label("h2");
                        ui.label("h3");
                        ui.add_layout(VerticalLayout::bottom_to_top(), |ui| {
                            ui.label("b1");
                            ui.label("b2");
                            ui.label("b3");
                        });
                    });
                    let layout = VerticalLayout::top_to_bottom().with_size(200.0, 150.0)
                        .with_fill(Color::rgba(190, 140, 209, 128)).with_padding(Padding::same(5.0));
                    ui.add_layout(layout, |ui| {
                        ui.label("Vertical");
                        ui.horizontal(|ui| {
                            ui.label("v1");
                            ui.label("v2");
                            ui.label("v3");
                        });
                        ui.label("h1");
                        ui.label("h2");
                        ui.label("h3");
                        ui.add_layout(HorizontalLayout::right_to_left(), |ui| {
                            ui.label("r1");
                            ui.label("r2");
                            ui.label("r3");
                        });
                    });
                    ui.label("right_left");
                    ui.add_layout(HorizontalLayout::right_to_left(), |ui| {
                        ui.label("hhh1");
                        ui.label("hhhh2");
                    });
                });

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("ScrollArea2");
                        let area = ScrollWidget::vertical().with_size(300.0, 200.0);
                        area.show(ui, |ui| {
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("ss");
                            ui.horizontal(|ui| {
                                ui.label("h1");
                                ui.label("h2");
                                ui.label("h3");
                                ui.label("h4");
                                ui.label("h5");
                                ui.label("h6");
                                ui.label("h7");
                                ui.label("h8");
                                ui.label("h9");
                                ui.label("h10");
                                ui.label("h11");
                                ui.label("h12");
                                ui.label("h13");
                                ui.label("h14");
                                ui.label("h15");
                                ui.label("h16");
                                ui.label("h17");
                            });
                            ui.label("se");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                            ui.label("s1");
                        });
                    });
                    ui.horizontal(|ui| {
                        ui.label("h1");
                        ui.label("h2");
                        ui.label("h3");
                        ui.label("h4");
                        ui.label("h5");
                        ui.vertical(|ui| {
                            ui.horizontal(|ui| {
                                ui.label("h1");
                                ui.label("h2");
                                ui.label("h3");
                                ui.label("h4");
                                ui.label("h5");
                            });
                            ui.horizontal(|ui| {
                                ui.label("h1");
                                ui.label("h2");
                                ui.label("h3");
                                ui.label("h4");
                                ui.label("h5");
                            });
                            ui.horizontal(|ui| {
                                ui.label("h1");
                                ui.label("h2");
                                ui.label("h3");
                                ui.label("h4");
                                ui.label("h5");
                            });
                            ui.horizontal(|ui| {
                                ui.label("h1");
                                ui.label("h2");
                                ui.label("h3");
                                ui.label("h4");
                                ui.label("h5");
                            });
                            ui.horizontal(|ui| {
                                ui.label("h1");
                                ui.label("h2");
                                ui.label("h3");
                                ui.label("h4");
                                ui.label("h5");
                            });
                            ui.horizontal(|ui| {
                                ui.label("h1");
                                ui.label("h2");
                                ui.label("h3");
                                ui.label("h4");
                                ui.label("h5");
                            });
                        });
                    });
                });
            });
        });
        ui.label("h1");
        ui.label("h2");
    }
}


fn main() {
    TestLayout {}.run().unwrap();
}