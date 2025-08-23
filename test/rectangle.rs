use xlui::frame::App;
use xlui::layout::popup::Popup;
use xlui::size::radius::Radius;
use xlui::size::rect::Rect;
use xlui::style::color::Color;
use xlui::style::Shadow;
use xlui::ui::Ui;
use xlui::widgets::rectangle::Rectangle;
use xlui::widgets::slider::Slider;
use xlui::widgets::spinbox::SpinBox;
use xlui::widgets::Widget;

pub struct TestRectangle {
    frame: Rectangle,
    pub border_width: f32,
    pub border_radius: u8,
}
impl TestRectangle {
    pub fn new() -> TestRectangle {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            color: Color::rgba(0, 0, 0, 30),
        };

        TestRectangle {
            frame: Rectangle::new(Rect::new(), Popup::popup_style()).with_shadow(shadow),
            border_width: 1.0,
            border_radius: 5,
        }
    }

    fn border_with(&mut self, ui: &mut Ui, v: f32) {
        self.border_width = v;
        self.frame.style_mut().border.inactive.width = v;
        self.frame.style_mut().border.hovered.width = v;
        self.frame.style_mut().border.clicked.width = v;
        self.frame.update(ui);
    }

    fn border_radius(&mut self, ui: &mut Ui, v: u8) {
        self.border_radius = v;
        self.frame.style_mut().border.inactive.radius = Radius::same(v);
        self.frame.style_mut().border.hovered.radius = Radius::same(v);
        self.frame.style_mut().border.clicked.radius = Radius::same(v);
        self.frame.update(ui);
    }

    fn border_radius_f32(&mut self, ui: &mut Ui, v: f32) {
        self.border_radius = v as u8;
        self.frame.style_mut().border.inactive.radius = Radius::same(v as u8);
        self.frame.style_mut().border.hovered.radius = Radius::same(v as u8);
        self.frame.style_mut().border.clicked.radius = Radius::same(v as u8);
        self.frame.update(ui);
    }

    fn shadow_offset_x(&mut self, ui: &mut Ui, v: f32) {
        self.frame.offset_x(v);
        self.frame.update(ui);
    }

    fn shadow_offset_y(&mut self, ui: &mut Ui, v: f32) {
        self.frame.offset_y(v);
        self.frame.update(ui);
    }
}


impl App for TestRectangle {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let rect = ui.available_rect().clone().with_size(300.0, 200.0);
            println!("{:?}", rect);
            self.frame.set_rect(rect);
            ui.add_mut(&mut self.frame);
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("边框:");
                    ui.add(SpinBox::new(1.0, 1.0, 0.0..20.0).id("sbw").contact("sb").connect(Self::border_with));
                    ui.add(Slider::new(1.0).with_range(0.0..20.0).id("sb").contact("sbw").connect(Self::border_with));
                });
                ui.horizontal(|ui| {
                    ui.label("圆角:");
                    ui.add(SpinBox::new(5, 1, 0..50).id("sbr").contact("sr").connect(Self::border_radius));
                    ui.add(Slider::new(5.0).with_range(0.0..50.0).id("sr").contact("sbr").connect(Self::border_radius_f32));
                });
                ui.horizontal(|ui| {
                    ui.label("偏移:");
                    ui.label("x:");
                    ui.slider(5.0, 0.0..100.0).set_callback(Self::shadow_offset_x);
                    ui.label("y:");
                    ui.slider(5.0, 0.0..100.0).set_callback(Self::shadow_offset_y);
                });
            });
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        self.frame.update(ui);
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.frame.redraw(ui);
    }
}