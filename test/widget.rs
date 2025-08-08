use xlui::frame::context::Context;
use xlui::frame::{App, WindowAttribute};
use xlui::ui::Ui;
use xlui::widgets::button::Button;
use xlui::widgets::checkbox::CheckBox;
use xlui::widgets::combobox::ComboBox;
use xlui::widgets::label::Label;
use xlui::widgets::radio::RadioButton;
use xlui::widgets::slider::Slider;
use xlui::widgets::spinbox::SpinBox;
use xlui::widgets::textedit::TextEdit;
use xlui::widgets::Widget;

fn main() {
    XlUiApp::new().run();
}

struct XlUiApp {
    label: Label,
    count: i32,
    combo_data: Vec<&'static str>,
}

impl XlUiApp {
    fn new() -> Self {
        Self {
            label: Label::new("这里是Label".to_string()).width(100.0),
            count: 0,
            combo_data: vec!["item1", "item2", "item3", "item4"],
        }
    }

    fn add(&mut self, ctx: &mut Context) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ctx);
    }

    fn reduce(&mut self, ctx: &mut Context) {
        self.count -= 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ctx);
    }

    fn slider(&mut self, ctx: &mut Context, value: f32) {
        self.label.set_text(format!("slider: {}", value));
        self.label.update(ctx);
    }

    fn check(&mut self, ctx: &mut Context, checked: bool) {
        self.label.set_text(format!("check: {}", checked));
        self.label.update(ctx);
    }

    fn spinbox(&mut self, ctx: &mut Context, value: i32) {
        self.label.set_text(format!("spinbox: {}", value));
        self.label.update(ctx);
    }

    fn radio(&mut self, ctx: &mut Context, checked: bool) {
        self.label.set_text(format!("radio: {}", checked));
        self.label.update(ctx);
    }

    fn edit_changed(&mut self, ctx: &mut Context, value: String) {
        self.label.set_text(format!("edit: {}", value));
        self.label.update(ctx);
    }
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_space(100.0);
            Label::new("当前控件的工作状态").size(24.0).draw(ui);
        });
        ui.horizontal(|ui| {
            RadioButton::new(true, "Label").draw(ui);
            ui.add_space(50.0);
            self.label.draw(ui);
            ui.add_space(190.0);
            ui.checkbox(true, "文本更新");
            ui.checkbox(false, "多样文本");
        });

        ui.horizontal(|ui| {
            RadioButton::new(true, "Button").draw(ui);
            ui.add_space(43.0);
            Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add).draw(ui);
            Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce).draw(ui);
            ui.add_space(225.0);
            ui.checkbox(true, "回调事件");
            ui.checkbox(false, "文本对齐");
        });

        ui.horizontal(|ui| {
            RadioButton::new(true, "Slider").draw(ui);
            ui.add_space(43.0);
            Slider::new(10.0).with_range(0.0..100.0).connect(Self::slider).draw(ui);
            ui.slider(30.0, 0.0..100.0).connect(Self::slider);
            ui.add_space(24.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(false, "颜色分离");
        });
        ui.horizontal(|ui| {
            RadioButton::new(true, "CheckBox").draw(ui);
            ui.add_space(30.0);
            CheckBox::new(false, "checkbox1").connect(Self::check).draw(ui);
            ui.checkbox(true, "checkbox2").connect(Self::check);
            ui.add_space(129.0);
            ui.checkbox(true, "变动监测");
        });
        ui.horizontal(|ui| {
            RadioButton::new(true, "SpinBox").draw(ui);
            ui.add_space(38.0);
            SpinBox::new(1).with_range(0..10).connect(Self::spinbox).draw(ui);
            ui.spinbox(1, 0..10).connect(Self::spinbox);
            ui.add_space(83.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(false, "泛类");
        });
        ui.horizontal(|ui| {
            RadioButton::new(true, "RadioButton").draw(ui);
            ui.add_space(10.0);
            RadioButton::new(false, "radiobutton").connect(Self::radio).draw(ui);
            ui.radio(true, "radiobutton").connect(Self::radio);
            ui.add_space(93.0);
            ui.checkbox(true, "变动监测");
        });
        ui.horizontal(|ui| {
            RadioButton::new(true, "TextEdit").draw(ui);
            ui.add_space(30.0);
            TextEdit::new("sdsd".to_string()).connect(Self::edit_changed).draw(ui);
            ui.add_space(87.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(false, "选择");
            ui.checkbox(false, "多行");
            ui.checkbox(false, "滚动");
            ui.checkbox(false, "复制");
            ui.checkbox(false, "粘贴");
            ui.checkbox(false, "密码");
        });
        ui.horizontal(|ui| {
            RadioButton::new(false, "ComboBox").draw(ui);
            ui.add_space(30.0);
            let mut combo = ComboBox::new(&self.combo_data).with_popup_height(150.0);
            combo.draw(ui);
            combo.draw(ui);
            ui.add_space(186.0);
            ui.checkbox(false, "变动监测");
        });
        ui.horizontal(|ui| {
            RadioButton::new(false, "ImageButton").draw(ui);
            ui.add_space(10.0);
            Button::image_and_text("logo.jpg", "按钮").width(50.0).height(40.0).draw(ui);
        });
    }

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (800, 600).into(),
            ..Default::default()
        }
    }
}
