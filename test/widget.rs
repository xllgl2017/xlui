use std::fmt::{Display, Formatter};
use xlui::frame::App;
use xlui::text::rich::RichText;
use xlui::ui::Ui;
#[cfg(target_os = "windows")]
use xlui::Tray;
use xlui::{Button, CheckBox, ComboBox, Image, Label, ProcessBar, RadioButton, SelectItem, Slider, SpinBox, TextEdit, Widget, WindowAttribute};

#[allow(dead_code)]
fn main() {
    TestWidget::new().run();
}

pub struct TestWidget {
    label: Label,
    count: i32,
    source: String,
}

impl TestWidget {
    pub fn new() -> Self {
        Self {
            label: Label::new("这里是Label".to_string()).width(100.0),
            count: 0,
            source: "".to_string(),
        }
    }

    fn add(&mut self, _: &mut Button, ui: &mut Ui) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ui);
    }

    fn reduce(&mut self, _: &mut Button, ui: &mut Ui) {
        self.count -= 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(ui);
    }

    fn slider(&mut self, ui: &mut Ui, value: f32) {
        self.label.set_text(format!("slider: {}", value));
        self.label.update(ui);
    }

    fn check(&mut self, ui: &mut Ui, checked: bool) {
        self.label.set_text(format!("check: {}", checked));
        self.label.update(ui);
    }

    fn spinbox_i32(&mut self, ui: &mut Ui, value: i32) {
        self.label.set_text(format!("spinbox: {}", value));
        self.label.update(ui);
    }

    fn spinbox_f32(&mut self, ui: &mut Ui, value: f32) {
        self.label.set_text(format!("spinbox: {}", value));
        self.label.update(ui);
    }

    fn radio(&mut self, ui: &mut Ui, checked: bool) {
        self.label.set_text(format!("radio: {}", checked));
        self.label.update(ui);
    }

    // fn edit_changed(&mut self, ui: &mut Ui, value: String) {
    //     self.label.set_text(format!("edit: {}", value));
    //     self.label.update(ui);
    // }

    fn image_button_click(&mut self, btn: &mut Button, ui: &mut Ui) {
        self.label.set_text(format!("image button: {}", self.count));
        self.label.update(ui);
        self.source = "/home/xl/下载/2f2da786-1326-42ee-9d14-a13946d05e7f.png".to_string();
        btn.set_image("/home/xl/下载/2f2da786-1326-42ee-9d14-a13946d05e7f.png", ui);
        btn.update(ui);
    }

    fn combo_changed(&mut self, ui: &mut Ui, data: &SV) {
        self.label.set_text(format!("combo: {}", data));
        self.label.update(ui);
    }
}

#[derive(PartialEq)]
pub enum SV {
    Item1,
    Item2,
    Item3,
    Item4,
    Item5,
    Item6,
}

impl Display for SV {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SV::Item1 => f.write_str("Item1"),
            SV::Item2 => f.write_str("Item2"),
            SV::Item3 => f.write_str("Item3"),
            SV::Item4 => f.write_str("Item4"),
            SV::Item5 => f.write_str("Item5"),
            SV::Item6 => f.write_str("Item6"),
        }
    }
}

impl App for TestWidget {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.add_space(100.0);
            ui.add(Label::new(RichText::new("当前控件的工作状态").size(24.0)));
        });
        ui.horizontal(|ui| {
            ui.radio(true, "Label");
            ui.add_space(50.0);
            ui.add_mut(&mut self.label);
            ui.add_space(190.0);
            ui.checkbox(true, "文本更新");
            ui.checkbox(false, "多样文本");
        });

        ui.horizontal(|ui| {
            ui.add(RadioButton::new(true, "Button"));
            ui.add_space(43.0);
            ui.add(Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add));
            ui.add(Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce));
            ui.add_space(225.0);
            ui.checkbox(true, "回调事件");
            ui.checkbox(false, "文本对齐");
        });

        ui.horizontal(|ui| {
            ui.radio(true, "Slider");
            ui.add_space(43.0);
            ui.add(Slider::new(50.0).id("s").contact("sb").contact("pbr").with_range(0.0..100.0).connect(Self::slider));
            ui.slider(30.0, 0.0..100.0).set_callback(Self::slider);
            ui.add_space(24.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(true, "颜色分离");
        });
        ui.horizontal(|ui| {
            ui.radio(true, "CheckBox");
            ui.add_space(30.0);
            ui.add(CheckBox::new(false, "checkbox1").id("cb").contact("rb").connect(Self::check));
            ui.checkbox(true, "checkbox2").set_callback(Self::check);
            ui.add_space(129.0);
            ui.checkbox(true, "变动监测");
        });
        ui.horizontal(|ui| {
            ui.radio(true, "SpinBox");
            ui.add_space(38.0);
            ui.add(SpinBox::new(50, 1, 1..100).id("sb").contact("s").contact("pbr").connect(Self::spinbox_i32));
            ui.spinbox(1.0, 0.5, 0.0..10.0).set_callback(Self::spinbox_f32);
            ui.add_space(83.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(true, "泛类");
        });
        ui.horizontal(|ui| {
            ui.radio(true, "RadioButton");
            ui.add_space(10.0);
            ui.add(RadioButton::new(false, "radiobutton").id("rb").contact("cb").connect(Self::radio));
            ui.radio(true, "radiobutton").set_callback(Self::radio);
            ui.add_space(93.0);
            ui.checkbox(true, "变动监测");
        });
        ui.horizontal(|ui| {
            ui.radio(true, "TextEdit");
            ui.add_space(30.0);
            ui.add(TextEdit::single_edit("abcdefghijklmnopqrstuvwsyz1234567890".to_string()).with_rows(1));
            ui.add_space(87.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(true, "选择");
            ui.checkbox(false, "多行");
            ui.checkbox(false, "滚动");
            ui.checkbox(false, "复制");
            ui.checkbox(false, "粘贴");
            ui.checkbox(false, "密码");
        });
        let cb = ComboBox::new(vec![SV::Item1, SV::Item2, SV::Item3, SV::Item4, SV::Item5, SV::Item6]).with_popup_height(150.0).connect(Self::combo_changed);
        let p = cb.parent();
        ui.horizontal(|ui| {
            ui.radio(true, "ComboBox");
            ui.add_space(30.0);
            ui.add(cb);
            ui.add_space(186.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(true, "滚动");
        });
        ui.horizontal(|ui| {
            ui.radio(true, "Image");
            ui.add_space(50.0);
            ui.image("logo.jpg", (50.0, 50.0));
            ui.add(Image::new("logo.jpg").with_size(50.0, 50.0).with_id("test_image"));
            ui.add_space(182.0);
            ui.checkbox(false, "网络图片");
            ui.checkbox(false, "bytes图片流");
        });
        ui.horizontal(|ui| {
            ui.radio(true, "ImageButton");
            ui.add_space(10.0);
            ui.add(Button::image_and_text("logo.jpg", "按钮").width(50.0).height(40.0).connect(Self::image_button_click));
        });
        ui.horizontal(|ui| {
            ui.radio(true, "SelectValue");
            ui.add_space(10.0);
            ui.add(SelectItem::new(SV::Item1).with_size(40.0, 25.0).contact(p.clone()));
            ui.add(SelectItem::new(SV::Item2).with_size(40.0, 25.0).contact(p.clone()));
            ui.add(SelectItem::new(SV::Item3).with_size(40.0, 25.0).contact(p.clone()));
            ui.add(SelectItem::new(SV::Item4).with_size(40.0, 25.0).contact(p.clone()));
        });
        ui.horizontal(|ui| {
            ui.radio(false, "ProcessBar");
            ui.add_space(11.0);
            ui.add(ProcessBar::new(50.0).with_id("pbr"));
        });
        ui.horizontal(|ui| {
            let edit = TextEdit::multi_edit("abcdefghijk\nlmnopqrstuvwsyz12\n34567890"); //\nsdfsdfsd\nfgfdgdfdfg\ndfgdfgdfg\nsdfsdf\nsdfdsjfsdf
            ui.add(edit);
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        if !self.source.is_empty() {
            ui.set_image_handle(&self.source);
            let image: &mut Image = ui.get_widget("test_image").unwrap();
            image.set_image(self.source.clone());
            self.source = "".to_string();
        }
        self.label.update(ui);
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.label.redraw(ui);
    }

    fn window_attributes(&self) -> WindowAttribute {
        #[cfg(target_os = "windows")]
        let mut tray = Tray::new().hovered_text("Rust Icon");
        #[cfg(target_os = "windows")]
        tray.add_menu("退出", None);
        #[cfg(target_os = "windows")]
        tray.add_menu("其他", None);
        WindowAttribute {
            inner_size: (1000, 800).into(),
            #[cfg(target_os = "windows")]
            tray: Some(tray),
            ..Default::default()
        }
    }
}
