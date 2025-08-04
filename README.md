# 🚀<img alt="&lt;img alt=&quot;logo&quot; height=&quot;20&quot;&gt;" height="20" src="img/logo/logo_96.png" width="20"/> xlui: 
[<img alt="github" src="https://img.shields.io/badge/github-xllgl2017/xlui-8da0cb?logo=github" height="20">](https://github.com/xllgl2017/xlui) [![Latest version](https://img.shields.io/crates/v/xlui.svg)](https://crates.io/crates/xlui) [![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/xllgl2017/main/LICENSE-APACHE)
### 示例
```rust
fn main() {
    let attr = WindowAttribute {
        inner_size: (800, 600).into(),
        ..Default::default()
    };
    let mut app = Application::new().with_attrs(attr);
    app.run(XlUiApp::new());
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

    pub fn add(&mut self, uim: &mut UiM) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(uim);
    }

    pub fn reduce(&mut self, uim: &mut UiM) {
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
}
```

## 控件(目前可用)
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

### ⬜️ ScrollBar(垂直)

### ⬜️ TextEdit

## Layout

### ⬜️ ScrollArea

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

[//]: # (❌)
