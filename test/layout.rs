use xlui::frame::App;
use xlui::layout::scroll_area::ScrollArea;
use xlui::ui::Ui;

struct Frame {}

impl App for Frame {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label("h1");
            ui.label("h2");
            ui.label("h3");
            ui.label("h4");
            ScrollArea::new().with_size(200.0, 200.0).show(ui, |ui| {
                ui.label("a1");
                ui.label("a2");
                ui.label("a3");
                ui.label("a4");
                ui.label("a5");
                ui.label("a6");
                ui.label("a7");
                ui.label("a8");
            });
            ui.label("h5");
            ui.label("h6");
            ui.horizontal(|ui| {
                ui.label("hh1");
                ui.label("hh2");
                ui.vertical(|ui| {
                    ui.label("hhv1");
                    ui.label("hhv2");
                });
                ui.horizontal(|ui| {
                    ui.label("hhh1");
                    ui.label("hhh2");
                })
            });
            ui.vertical(|ui| {
                ui.label("hv1");
                ui.label("hv2");
                ui.horizontal(|ui| {
                    ui.label("hvh1");
                    ui.label("hvh1");
                });
                ui.vertical(|ui| {
                    ui.label("hvv1");
                    ui.label("hvv2");
                    let area = ScrollArea::new().with_size(300.0, 200.0);
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
            });
        });
        ui.vertical(|ui| {
            ui.label("v1");
            ui.label("v2");
            ui.vertical(|ui| {
                ui.label("vv1");
                ui.vertical(|ui| {
                    ui.label("vvv1");
                    ui.horizontal(|ui| {
                        ui.label("vvvh1");
                        ui.label("vvvh2");
                        ui.vertical(|ui| {
                            ui.label("vvvhv1");
                            ui.label("vvvhv2");
                        });
                    });
                    ui.label("vvv2");
                });
                ui.label("vv2");
            });
            ui.horizontal(|ui| {
                ui.label("vh1");
                ui.label("vh2");
            });
        });

        ui.label("v3");
        ui.label("v4");
        ui.label("v5");
        ui.label("v6");
    }

    fn update(&mut self, ui: &mut Ui) {
    }

    fn redraw(&mut self, ui: &mut Ui) {
    }
}


fn main() {
    Frame {}.run();
}