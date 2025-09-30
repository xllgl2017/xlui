use xlui::*;

pub struct TestShape {
    pub(crate) border: Border,
    shadow_x: f32,
    shadow_y: f32,
}
impl TestShape {
    pub fn new() -> TestShape {
        TestShape {
            border: Border::same(1.0).radius(Radius::same(5)),
            shadow_x: 5.0,
            shadow_y: 8.0,
        }
    }

    fn border_with(&mut self, _: &mut Ui, v: f32) {
        self.border.left_width = v;
        self.border.right_width = v;
        self.border.top_width = v;
        self.border.bottom_width = v;

        // self.rectangle.style_mut().border.inactive.width = v;
        // self.rectangle.style_mut().border.hovered.width = v;
        // self.rectangle.style_mut().border.clicked.width = v;
        // self.rectangle.update(ui);
        // self.triangle.style_mut().border.inactive.width = v;
        // self.circle.style_mut().border.inactive.width = v;
    }

    fn border_radius(&mut self, _: &mut Ui, v: u8) {
        self.border.radius = Radius::same(v);
        // self.rectangle.style_mut().border.inactive.radius = Radius::same(v);
        // self.rectangle.style_mut().border.hovered.radius = Radius::same(v);
        // self.rectangle.style_mut().border.clicked.radius = Radius::same(v);
        // self.rectangle.update(ui);
    }

    fn border_radius_f32(&mut self, _: &mut Ui, v: f32) {
        self.border.radius = Radius::same(v as u8)
        // self.rectangle.style_mut().border.inactive.radius = Radius::same(v as u8);
        // self.rectangle.style_mut().border.hovered.radius = Radius::same(v as u8);
        // self.rectangle.style_mut().border.clicked.radius = Radius::same(v as u8);
        // self.rectangle.update(ui);
    }

    fn shadow_offset_x(&mut self, _: &mut Ui, v: f32) {
        self.shadow_x = v;
        // self.rectangle.offset_x(v);
        // self.rectangle.update(ui);
    }

    fn shadow_offset_y(&mut self, _: &mut Ui, v: f32) {
        self.shadow_y = v;
        // self.rectangle.offset_y(v);
        // self.rectangle.update(ui);
    }
}


impl App for TestShape {
    fn draw(&mut self, ui: &mut Ui) {
        let shadow = Shadow {
            offset: [self.shadow_x, self.shadow_y],
            spread: 10.0,
            blur: 1.0,
            color: Color::rgba(0, 0, 0, 30),
        };
        let style = ui.style.borrow().widgets.popup.clone();
        self.border.color = style.border.inactive.color.clone();
        ui.horizontal(|ui| {
            let rectangle = Rectangle::new(style.clone(), 200.0, 150.0)
                .with_id("rectangle").with_shadow(shadow);
            ui.add(rectangle);
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("边框:");
                    ui.add(SpinBox::new(style.border.inactive.left_width, 1.0, 0.0..20.0).id("sbw")
                        .contact("sb").contact("tsbw").contact("tsb")
                        .connect(Self::border_with));
                    ui.add(Slider::new(style.border.inactive.left_width).with_range(0.0..20.0).id("sb")
                        .contact("sbw").contact("tsbw").contact("tsb")
                        .connect(Self::border_with));
                });
                ui.horizontal(|ui| {
                    ui.label("圆角:");
                    ui.add(SpinBox::new(style.border.inactive.radius.left_bottom, 1, 0..50).id("sbr").contact("sr").connect(Self::border_radius));
                    ui.add(Slider::new(style.border.inactive.radius.left_bottom as f32).with_range(0.0..50.0).id("sr").contact("sbr").connect(Self::border_radius_f32));
                });
                ui.horizontal(|ui| {
                    ui.label("偏移:");
                    ui.label("x:");
                    ui.slider(5.0, 0.0..100.0).set_callback(Self::shadow_offset_x);
                    ui.label("y:");
                    ui.slider(8.0, 0.0..100.0).set_callback(Self::shadow_offset_y);
                });
            });
        });
        ui.horizontal(|ui| {
            let rect = Rect::new().with_size(200.0, 150.0);
            let mut p0 = Pos::new();
            p0.x = rect.dx().min + 100.0;
            p0.y = rect.dy().min;
            let mut p1 = Pos::new();
            p1.x = rect.dx().min;
            p1.y = rect.dy().min + 150.0;
            let mut p2 = Pos::new();
            p2.x = rect.dx().min + 200.0;
            p2.y = rect.dy().min + 150.0;
            let triangle = Triangle::new().with_pos(p0, p1, p2).with_style(style.clone());
            ui.add(triangle);
            ui.add_space(20.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("边框:");
                    ui.add(SpinBox::new(style.border.inactive.left_width, 1.0, 0.0..20.0).id("tsbw")
                        .contact("tsb").contact("sb").contact("sbw")
                        .connect(Self::border_with));
                    ui.add(Slider::new(style.border.inactive.left_width).with_range(0.0..20.0).id("tsb")
                        .contact("tsbw").contact("sb").contact("sbw")
                        .connect(Self::border_with));
                });
            });
        });
        ui.horizontal(|ui| {
            // let mut style = ClickStyle::new();
            // style.fill.inactive = Color::BLUE;
            // style.border = BorderStyle::same(Border::new(1.0).color(Color::RED));
            // self.circle.set_style(style);
            let mut circle = Circle::new(50.0);
            circle.set_style(style.clone());
            ui.add(circle);
            ui.add_space(120.0);
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("边框:");
                    ui.add(SpinBox::new(style.border.inactive.left_width, 1.0, 0.0..20.0).id("csbw")
                        .contact("csb").contact("tsb").contact("sb").contact("sbw")
                        .connect(Self::border_with));
                    ui.add(Slider::new(style.border.inactive.left_width).with_range(0.0..20.0).id("csb")
                        .contact("csbw").contact("tsb").contact("sb").contact("sbw")
                        .connect(Self::border_with));
                });
            });
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        let rectangle: &mut Rectangle = ui.get_widget("rectangle").unwrap();
        rectangle.set_offset_x(self.shadow_x);
        rectangle.set_offset_y(self.shadow_y);
        rectangle.set_border(self.border.clone());
    }


    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            title: "TestShape".to_string(),
            ..Default::default()
        }
    }
}