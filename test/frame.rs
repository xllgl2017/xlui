use xlui::frame::App;
use xlui::layout::popup::Popup;
use xlui::radius::Radius;
use xlui::size::rect::Rect;
use xlui::style::{ClickStyle, Shadow};
use xlui::style::color::Color;
use xlui::ui::Ui;
use xlui::widgets::rectangle::Rectangle;
use xlui::widgets::Widget;

fn main() {
    XlUi::new().run();
}

struct XlUi {
    frame: Rectangle,
}
impl XlUi {
    fn new() -> XlUi {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            color: Color::rgba(0, 0, 0, 30),
        };
        let mut rect = Rect::new().with_size(300.0, 200.0);
        rect.offset(10.0, 10.0);
        XlUi {
            frame: Rectangle::new(rect, Popup::popup_style()).with_shadow(shadow)
        }
    }

    fn border_with(&mut self, ui: &mut Ui, v: f32) {
        self.frame.style_mut().border.inactive.width = v;
        self.frame.update(ui);
    }

    fn border_radius(&mut self, ui: &mut Ui, v: u8) {
        self.frame.style_mut().border.inactive.radius = Radius::same(v);
        self.frame.update(ui);
    }

    fn border_radius_f32(&mut self, ui: &mut Ui, v: f32) {
        self.frame.style_mut().border.inactive.radius = Radius::same(v as u8);
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


impl App for XlUi {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_mut(&mut self.frame);
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("边框:");
                    ui.spinbox(1.0, 1.0, 0.0..20.0).set_callback(Self::border_with);
                    ui.slider(1.0, 0.0..20.0).set_callback(Self::border_with);
                });
                ui.horizontal(|ui| {
                    ui.label("圆角:");
                    ui.spinbox(5, 1, 0..50).set_callback(Self::border_radius);
                    ui.slider(5.0, 0.0..50.0).set_callback(Self::border_radius_f32);
                });
                ui.horizontal(|ui| {
                    ui.label("偏移:");
                    ui.label("x:");
                    ui.slider(5.0,0.0..100.0).set_callback(Self::shadow_offset_x);
                    ui.add_space(20.0);
                    ui.label("y:");
                    ui.slider(5.0,0.0..100.0).set_callback(Self::shadow_offset_y);
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