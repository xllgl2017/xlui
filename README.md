# 🚀<img alt="logo" height="30" src="res/img/logo/logo_96.png" width="30"/> xlui:

[<img alt="github" src="https://img.shields.io/badge/github-xllgl2017/xlui-8da0cb?logo=github" height="20">](https://github.com/xllgl2017/xlui)
[![Latest version](https://img.shields.io/crates/v/xlui.svg)](https://crates.io/crates/xlui)
[![Documentation](https://docs.rs/xlui/badge.svg)](https://docs.rs/xlui)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/xllgl2017/xlui/blob/main/LICENSE-APACHE)

&nbsp;&nbsp;&nbsp;&nbsp; xlui是一个用Rust语言，基于winit和wgpu实现的2D GUI库。目标是利用Rust语言原生构建GUI、最小第三方依赖，简单易用， 在保证性能的前提下尽量减少CPU的开销。

### xlui的目标

| 适配情况 |   目标系统    |    平台UI     | 备注  |
|:----:|:---------:|:-----------:|:---:|
|  ✅   |   Linux   | x11,wayland |     |
|  ✅   |  Windows  |    10,11    | 有延时 |
|  ⬜️  |   MacOS   |      -      |     |
|  ⬜️  |  Android  |     11+     |     |
|  ⬜️  |    Web    |    Wasm     |     |
|  ❌   | HarmonyOS |   暂无适配计划    |     |

### 示例

```rust
use xlui::frame::{App, WindowAttribute};
use xlui::ui::Ui;
use xlui::widgets::button::Button;
use xlui::widgets::label::Label;
use xlui::widgets::Widget;

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
}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        ui.add_mut(&mut self.label);
        ui.horizontal(|ui| {
            ui.add(Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add));
            ui.add(Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce));
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        self.label.update(ui);
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.label.redraw(ui);
    }

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (800, 600).into(),
            ..Default::default()
        }
    }
}
```

## [🎯](https://github.com/xllgl2017/xlui/wiki/%E5%B8%83%E5%B1%80)控件(目前可用) 

![控件状态](/res/img/doc/img_1.png)



[//]:  # (❌⬜️)  
