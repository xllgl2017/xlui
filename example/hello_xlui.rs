use xlui::frame::context::Context;
use xlui::frame::App;
use xlui::layout::scroll_area::ScrollArea;
use xlui::response::Response;
use xlui::size::rect::Rect;
use xlui::ui::Ui;
use xlui::widgets::button::Button;
use xlui::widgets::checkbox::CheckBox;
use xlui::widgets::combobox::ComboBox;
use xlui::widgets::image::Image;
use xlui::widgets::label::Label;
use xlui::widgets::slider::Slider;
use xlui::widgets::spinbox::SpinBox;
use xlui::widgets::textedit::TextEdit;
use xlui::widgets::Widget;

struct TD {
    name: String,
}

impl TD {
    fn new(name: impl ToString) -> TD {
        TD { name: name.to_string() }
    }
}

struct ListView {
    id: String,
    data: Vec<TD>,
    current_index: Option<usize>,
}

impl ListView {
    fn new() -> ListView {
        ListView {
            id: xlui::gen_unique_id(),
            data: vec![
                TD::new("1"),
                TD::new("2"),
                TD::new("3"),
                TD::new("4"),
                TD::new("5"),
                TD::new("6"),
                TD::new("7"),
                TD::new("8"),
                TD::new("9"),
                TD::new("0"),
                TD::new("11")
            ],
            current_index: None,
        }
    }


    // fn item_widget(&self, ui: &mut Ui, datum: &TD, row: usize) {
    //     let mut rect = ui.available_rect();
    //     rect.set_height(38.0);
    //     let style = ClickStyle {
    //         fill: FillStyle {
    //             inactive: Color::TRANSPARENT,
    //             hovered: Color::TRANSPARENT,
    //             clicked: Color::TRANSPARENT,
    //         },
    //         border: BorderStyle {
    //             inactive: Border::new(1.0).radius(Radius::same(3)).color(Color::BLUE),
    //             hovered: Border::new(1.0).radius(Radius::same(3)).color(Color::RED),
    //             clicked: Border::new(1.0).radius(Radius::same(3)).color(Color::YELLOW),
    //         },
    //     };
    //     ui.paint_rect(rect, style).connect(row, XlUiApp::item_click);
    //     ui.horizontal(|ui| {
    //         // ui.image("logo.jpg", (30.0, 30.0));
    //         ui.vertical(|ui| {
    //             ui.label(datum.name.as_str());
    //             ui.horizontal(|ui| {
    //                 ui.label("00:00");
    //                 ui.label("200");
    //                 ui.label("HTTP/1.1");
    //                 ui.label("10 KB");
    //                 ui.label("10 KB");
    //             });
    //         });
    //     });
    // }

    fn current(&self, ctx: &mut Context) -> Option<&TD> {
        let current = self.current_index?;
        Some(&self.data[current])
    }

    pub fn remove(&mut self, index: usize) {
        self.data.remove(index);
    }

    pub fn push(&mut self, datum: TD, ctx: &mut Context) {
        self.data.push(datum);
    }
}


impl Widget for ListView {
    fn draw(&mut self, ui: &mut Ui) -> Response {
        // ScrollArea::new().with_size(300.0, 300.0).show(ui, |ui| {
        //     for (row, datum) in self.data.iter().enumerate() {
        //         self.item_widget(ui, datum, row);
        //     }
        // });
        Response {
            id: self.id.clone(),
            rect: Rect::new(),
        }
    }

    fn update(&mut self, ui: &mut Ui) {}

    fn redraw(&mut self, ui: &mut Ui) {}
}

fn main() {
    XlUiApp::new().run();
}

struct XlUiApp {
    label: Label,
    count: i32,
    // list_view: ListView,
}

impl XlUiApp {
    pub fn new() -> Self {
        Self {
            label: Label::new("hello".to_string()).width(200.0),
            count: 0,
            // list_view: ListView::new(),
        }
    }

    pub fn click1(&mut self, _ui: &mut Ui) {
        self.count += 1;
        println!("count: {}", self.count);
    }

    pub fn click2(&mut self, _ui: &mut Ui) {
        self.count += 2;
        println!("count2: {}", self.count);
    }

    pub fn add(&mut self, ui: &mut Ui) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ui);
    }

    pub fn reduce(&mut self, ui: &mut Ui) {
        self.count -= 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ui);
    }

    pub fn slider(&mut self, ui: &mut Ui, value: f32) {
        self.label.set_text(format!("slider: {}", value));
        self.label.update(ui);
    }

    pub fn check(&mut self, ui: &mut Ui, checked: bool) {
        self.label.set_text(format!("check: {}", checked));
        self.label.update(ui);
    }

    pub fn edit_changed(&mut self, ui: &mut Ui, text: String) {
        self.label.set_text(format!("textedit: {}", text));
        self.label.update(ui);
    }

    fn item_click(&mut self, ctx: &mut Ui, row: usize) {
        println!("item click {}", row);
    }

    fn combo_changed(&mut self, ctx: &mut Ui, item: &&str) {
        self.label.set_text(format!("combo: {}", item));
        self.label.update(ctx);
    }
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        ui.add_mut(&mut self.label);
        ui.horizontal(|ui| {
            ui.add(Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add));
            ui.add(Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce));
        });
        // // println!("draw");
        // // ScrollBar::new().size(20.0, 200.0).draw(ui);
        ui.add(TextEdit::new("sdsd".to_string()).width_id("xlui_edit").connect(Self::edit_changed));
        ui.add(SpinBox::new(1, 1, 1..10));
        ui.horizontal(|ui| {
            ui.add(Slider::new(10.0).with_range(0.0..100.0).connect(Self::slider));
            ui.slider(30.0, 0.0..100.0).set_callback(Self::slider);
        });
        ui.horizontal(|ui| {
            ui.add(CheckBox::new(false, "checkbox1").connect(Self::check));
            ui.checkbox(true, "checkbox2").set_callback(Self::check);
        });
        ui.horizontal(|ui| {
            ui.label("h1");
            ui.label("h1");
            ui.label("h1");
            let area = ScrollArea::new().with_size(300.0, 400.0);
            area.show(ui, |ui| {
                ui.label("start");
                ui.vertical(|ui| {
                    ui.label("sv1");
                    ui.label("sv2");
                    ui.button("sv3").set_callback(Self::click1);
                });
                ui.horizontal(|ui| {
                    ui.label("sh1");
                    ui.label("sh2");
                    ui.button("sh3").set_callback(Self::click2);
                });
                for i in 0..1000 {
                    ui.label(format!("i{}", i));
                }
                ui.label("end");
            });
            ui.add(ComboBox::new(vec!["item1", "item2", "item3", "item4", "item5", "item6"]).connect(Self::combo_changed).with_popup_height(150.0));
            // self.list_view.draw(ui);
        });

        ui.label("hello label1");
        ui.image("logo.jpg", (200.0, 200.0));
        ui.add(Image::new("logo.jpg").with_size(200.0, 200.0));
        // ui.image("logo.jpg", (200.0, 200.0));
    }

    fn update(&mut self, ui: &mut Ui) {
        self.label.update(ui);
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.label.redraw(ui);
    }
}
