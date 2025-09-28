use std::fmt::{Display, Formatter};
#[cfg(all(not(feature = "winit"), target_os = "windows"))]
use std::process::exit;
#[cfg(all(not(feature = "winit"), target_os = "windows"))]
use xlui::Tray;
use xlui::*;

fn main() {
    TestWidget::new().run().unwrap();
}

pub struct TestWidget {
    status: String,
    count: i32,
    change_image: bool,
}

impl TestWidget {
    pub fn new() -> Self {
        Self {
            status: "".to_string(),
            count: 0,
            change_image: false,
        }
    }

    fn add(&mut self, _: &mut Button, _: &mut Ui) {
        self.count += 1;
        self.status = format!("count: {}", self.count);
    }

    fn reduce(&mut self, _: &mut Button, _: &mut Ui) {
        self.count -= 1;
        self.status = format!("count: {}", self.count);
    }

    fn slider(&mut self, _: &mut Ui, value: f32) {
        self.status = format!("slider: {}", value);
    }

    fn check(&mut self, _: &mut Ui, checked: bool) {
        self.status = format!("check: {}", checked);
    }

    fn spinbox_i32(&mut self, _: &mut Ui, value: i32) {
        self.status = format!("spinbox: {}", value);
    }

    fn spinbox_f32(&mut self, _: &mut Ui, value: f32) {
        self.status = format!("spinbox: {}", value);
    }

    fn radio(&mut self, _: &mut Ui, checked: bool) {
        self.status = format!("radio: {}", checked);
    }

    fn edit_changed(&mut self, _: &mut Ui, value: String) {
        self.status = format!("edit: {}", value);
    }

    fn image_button_click(&mut self, btn: &mut Button, ui: &mut Ui) {
        self.status = format!("image button: {}", self.count);
        self.change_image = true;
        btn.set_image("/home/xl/下载/2f2da786-1326-42ee-9d14-a13946d05e7f.png");
        btn.update(ui);
    }

    fn combo_changed(&mut self, _: &mut Ui, data: &SV) {
        self.status = format!("combo: {}", data);
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
            ui.add(Label::new("当前控件的工作状态".size(24.0).color(Color::ORANGE)));
        });
        ui.horizontal(|ui| {
            ui.radio(true, "Label");
            ui.add_space(50.0);
            ui.add(Label::new("这里是Label+").with_id("status").max_width(100.0));
            ui.add_space(190.0);
            ui.checkbox(true, "文本更新");
            ui.checkbox(true, "多样文本");
        });

        ui.horizontal(|ui| {
            ui.radio(true, "Button");
            ui.add_space(43.0);
            ui.add(Button::new("+").width(30.0).height(30.0).connect(Self::add));
            ui.add(Button::new("-").width(30.0).height(30.0).connect(Self::reduce));
            ui.add_space(225.0);
            ui.checkbox(true, "回调事件");
            ui.checkbox(true, "文本对齐");
        });

        ui.horizontal(|ui| {
            ui.radio(true, "Slider");
            ui.add_space(43.0);
            ui.add(Slider::new(50.0).id("s").contact("sb").contact("pbr").contact("status").with_range(0.0..100.0).connect(Self::slider));
            ui.slider(30.0, 0.0..100.0).set_callback(Self::slider);
            ui.add_space(24.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(true, "颜色分离");
        });
        ui.horizontal(|ui| {
            ui.radio(true, "CheckBox");
            ui.add_space(30.0);
            ui.add(CheckBox::new(false, "checkbox1").id("cb").contact("rb").connect(Self::check).with_size(100.0, 25.0));
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
            ui.add(TextEdit::single_edit("abcdefghijklmnopqrstuvwsyz1234567890".to_string()).connect(Self::edit_changed));
            ui.add_space(87.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(true, "选择");
            ui.checkbox(true, "多行");
            ui.checkbox(true, "滚动");
            ui.checkbox(true, "复制");
            ui.checkbox(true, "粘贴");
            ui.checkbox(true, "密码");
        });
        let cb = ComboBox::new(vec![SV::Item1, SV::Item2, SV::Item3, SV::Item4, SV::Item5, SV::Item6]).with_popup_height(150.0).connect(Self::combo_changed);
        let p = cb.parent();
        ui.horizontal(|ui| {
            ui.radio(true, "ComboBox");
            ui.add_space(30.0);
            ui.add(cb);
            ui.add(CheckComboBox::new(vec![1, 2, 3, 4]).with_popup_height(150.0));
            ui.add_space(186.0);
            ui.checkbox(true, "变动监测");
            ui.checkbox(true, "滚动");
        });
        ui.horizontal(|ui| {
            ui.radio(true, "Image");
            ui.add_space(50.0);
            ui.image("logo.jpg", (50.0, 50.0));
            ui.add(Image::new(include_bytes!("../logo.jpg")).with_size(50.0, 50.0).with_id("test_image"));
            ui.add_space(182.0);
            ui.checkbox(true, "bytes图片流");
        });
        ui.horizontal(|ui| {
            ui.radio(true, "ImageButton");
            ui.add_space(10.0);
            ui.add(Button::image_and_text("logo.jpg", "Image").width(100.0).height(40.0).connect(Self::image_button_click));
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
            ui.radio(true, "ProcessBar");
            ui.add_space(11.0);
            ui.add(ProcessBar::new(50.0).with_id("pbr"));
        });
        ui.horizontal(|ui| {
            let edit = TextEdit::multi_edit("abcdefghijk\nlmnopqrstuvwsyz12\n34567890"); //\nsdfsdfsd\nfgfdgdfdfg\ndfgdfgdfg\nsdfsdf\nsdfdsjfsdf
            ui.add(edit);
        });
        ui.horizontal(|ui| {
            let edit = TextEdit::single_edit("dsfdsf").password();
            ui.add(edit);
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        let status: &mut Label = ui.get_widget("status").unwrap();
        status.set_text(&self.status);
        if self.change_image {
            self.change_image = false;
            let image: &mut Image = ui.get_widget("test_image").unwrap();
            image.set_image("/home/xl/下载/2f2da786-1326-42ee-9d14-a13946d05e7f.png");
        }
    }


    fn window_attributes(&self) -> WindowAttribute {
        #[cfg(all(not(feature = "winit"), target_os = "windows"))]
        let mut tray = Tray::new().hovered_text("Rust Icon");
        #[cfg(all(not(feature = "winit"), target_os = "windows"))]
        tray.add_menu("退出", None).set_callback(|| exit(0));
        #[cfg(all(not(feature = "winit"), target_os = "windows"))]
        let other = tray.add_menu("其他", None);
        #[cfg(all(not(feature = "winit"), target_os = "windows"))]
        other.set_icon("C:\\Users\\xl\\Downloads\\aknxx-37a47-001.ico"); //ico格式
        #[cfg(all(not(feature = "winit"), target_os = "windows"))]
        other.add_child("item1", None);
        #[cfg(all(not(feature = "winit"), target_os = "windows"))]
        other.add_child("item2", None);
        WindowAttribute {
            inner_size: (1000, 800).into(),
            #[cfg(all(not(feature = "winit"), target_os = "windows"))]
            tray: Some(tray),
            decorations: true,
            ..Default::default()
        }
    }
}
