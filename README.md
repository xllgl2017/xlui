# 🚀<img alt="logo" height="30" src="res/img/logo/logo_96.png" width="30"/> xlui:

[<img alt="github" src="https://img.shields.io/badge/github-xllgl2017/xlui-8da0cb?logo=github" height="20">](https://github.com/xllgl2017/xlui)
[![Latest version](https://img.shields.io/crates/v/xlui.svg)](https://crates.io/crates/xlui)
[![Documentation](https://docs.rs/xlui/badge.svg)](https://docs.rs/xlui)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/xllgl2017/xlui/blob/main/LICENSE-APACHE)

&nbsp;&nbsp;&nbsp;&nbsp; xlui是一个Rust的2D GUI库，体积小(最小第三方依赖)，简单易用，在保证性能的前提下尽量减少CPU的开销。

### xlui的目标

| 适配情况 |   目标系统    |    平台UI     |
|:----:|:---------:|:-----------:|
|  ✅   |   Linux   | x11,wayland |
|  ⬜️  |  Windows  |    10,11    |
|  ⬜️  |   MacOS   |      -      |
|  ⬜️  |  Android  |     11+     |
|  ⬜️  |    Web    |    Wasm     |
|  ❌   | HarmonyOS |   暂无适配计划    |

### 示例

```rust
fn main() {
    XlUiApp::new().run();
}

struct XlUiApp {
    label: Label,
    count: i32,
}

impl XlUiApp {
    pub fn new() -> Self {
        Self {
            label: Label::new("hello".to_string()).width(100.0),
            count: 0,
        }
    }

    pub fn add(&mut self, ctx: &mut Context) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(uim);
    }

    pub fn reduce(&mut self, ctx: &mut Context) {
        self.count -= 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(uim);
    }
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        self.label.draw(ui);
        ui.horizontal(|ui| {
            Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add).draw(ui);
            Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce).draw(ui);
        });
    }

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (800, 600).into(),
            ..Default::default()
        }
    }
}
```

## 控件(目前可用)

![控件状态](/res/img/doc/img.png)

### ✅ Label

```rust
fn draw(&mut self, ui: &mut Ui) {
    ui.label("hello label");
    //或者
    Label::new("hello label").draw(ui);
}
```

### ✅ Button

```rust
fn draw(&mut self, ui: &mut Ui) {
    ui.button("hello button").connect(Self::clicked);
    //或者
    Button::new("hello label").connect(Self::clicked).draw(ui);
}
```

### ✅ Slider

```rust
fn draw(&mut self, ui: &mut Ui) {
    ui.slider(30.0, 0.0..100.0).connect(Self::slider);
    //或者
    Slider::new(10.0).with_range(0.0..100.0).connect(Self::slider).draw(ui);
}
```

### ✅ SpinBox

```rust
fn draw(&mut self, ui: &mut Ui) {
    ui.spinbox(1, 0..10).connect(Self::changed);
    //或者
    SpinBox::new(1).with_range(0..10).connect(Self::changed).draw(ui);
}
```

### ✅ CheckBox

```rust
fn draw(&mut self, ui: &mut Ui) {
    ui.checkbox(true, "checkbox1").connect(Self::checked);
    //或者
    CheckBox::new(false, "checkbox2").connect(Self::checked).draw(ui);
}
```

### ✅ Image

```rust
fn draw(&mut self, ui: &mut Ui) {
    ui.image("logo.jpg", (200.0, 200.0));
    //或者
    Image::new("logo.jpg").with_size(200.0, 200.0).draw(ui);
}
```

### ✅ RadioButton

```rust
fn draw(&mut self, ui: &mut Ui) {
    ui.radio(true, "radiobutton").connect(Self::radio);
    //或者
    RadioButton::new(false, "radiobutton").connect(Self::radio).draw(ui);
}
```

### ✅ ComboBox

```rust
fn draw(&mut self, ui: &mut Ui) {
    // ui.radio(true, "radiobutton").connect(Self::radio);
    //或者
    let combo_data = vec!["item1", "item2", "item3", "item4"];
    let combo = ComboBox::new(&combo_data).connect(Self::combo_changed).with_popup_height(150.0);
    combo.draw(ui);
    combo.draw(ui);//再次添加一个
}
```

### ✅ ScrollBar(垂直)

```rust
fn draw(&mut self, ui: &mut Ui) {
    ScrollBar::new().with_size(20.0, 100.0).draw(ui);
}
```

### ✅ TextEdit

```rust

//文本变动监测
fn edit_changed(&mut self, ctx: &mut Context, text: &str) {
    self.label.set_text(format!("textedit: {}", text));
    self.label.update(ctx);
}

fn draw(&mut self, ui: &mut Ui) {
    //创建TextEdit并添加ID，以便后续获取其文本
    TextEdit::new("sdsd".to_string()).width_id("xlui_edit")
        .connect(Self::edit_changed).draw(ui);
}
```

### ⬜️ ListView(debug下流畅显示10w条数据)

### ⬜️ TableView(debug下流畅显示10w条数据)

## 布局

### ✅ Layout(垂直、水平)

```rust
fn draw(&mut self, ui: &mut Ui) {
    ui.horizontal(|ui| {
        //...
    });
    ui.vertical(|ui| {
        //...
    });
}
```

### ✅ ScrollArea(垂直)

```rust
fn draw(&mut self, ui: &mut Ui) {
    let area = ScrollArea::new().with_size(300.0, 400.0);
    area.show(ui, |ui| {
        ui.label("start");
        ui.vertical(|ui| {
            ui.label("sv1");
            ui.label("sv2");
            ui.button("sv3").connect(Self::click1);
        });
        ui.horizontal(|ui| {
            ui.label("sh1");
            ui.label("sh2");
            ui.button("sh3");
        });
        for i in 0..1000 {
            ui.label(format!("i{}", i));
        }
        ui.label("end");
    });
}
```

### ⬜️ Popup

## ⬜️ 自定义窗口

[//]:  # (❌⬜️)  
