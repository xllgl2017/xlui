# 🚀<img alt="logo" height="30" src="res/img/logo/logo_96.png" width="30"/> xlui:

[<img alt="github" src="https://img.shields.io/badge/github-xllgl2017/xlui-8da0cb?logo=github" height="20">](https://github.com/xllgl2017/xlui)
[![Latest version](https://img.shields.io/crates/v/xlui.svg)](https://crates.io/crates/xlui)
[![Documentation](https://docs.rs/xlui/badge.svg)](https://docs.rs/xlui)
[![Apache](https://img.shields.io/badge/license-Apache-blue.svg)](https://github.com/xllgl2017/xlui/blob/main/LICENSE-APACHE)

&nbsp;&nbsp;&nbsp;&nbsp; xlui是一个用Rust语言，基于winit和wgpu实现的2D
GUI库。目标是利用Rust语言原生构建GUI、最小第三方依赖,体积比winit+wgpu少30%左右，简单易用， 在保证性能的前提下尽量减少CPU的开销。

## xlui的目标

| 适配情况 |   目标系统    |    平台UI     |    备注    |
|:----:|:---------:|:-----------:|:--------:|
|  ✅   |   Linux   | x11,wayland | x11为原生窗口 |
|  ✅   |  Windows  |    10,11    |   原生窗口   |
|  ⬜️  |   MacOS   |      -      |  winit   |
|  ⬜️  |  Android  |     11+     |  winit   |
|  ⬜️  |    Web    |    Wasm     |  winit   |
|  ❌   | HarmonyOS |   暂无适配计划    |          |

## [🎯](https://github.com/xllgl2017/xlui/wiki/%E5%B8%83%E5%B1%80)控件(目前可用)

![控件状态](/res/img/doc/img_1.png)

## 快速开始xlui

### xlui的最小运行示例

```rust
use xlui::*;

fn main() {
    let app=XlUiApp::new();
    //直接调run()
    app.run().unwrap();
}

struct XlUiApp {
    status:String,
    count: i32,
}


impl XlUiApp {
    fn new()->XlUiApp{
        XlUiApp{
            count: 0,
            status:"这里是Label".to_string()
        }
    }
    fn add(&mut self,_:&mut Button,ui: &mut Ui){
        self.count += 1;
        self.status=format!("count: {}", self.count);
    }

    fn reduce(&mut self,_:&mut Button,ui: &mut Ui){
        self.count-=1;
        self.status=format!("count: {}", self.count);
    }
}

//实现App trait
impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        ui.add(Label::new("hello").with_id("status"));
        ui.horizontal(|ui| {
            ui.add(Button::new("+").width(30.0).height(30.0).connect(Self::add));
            ui.add(Button::new("-").width(30.0).height(30.0).connect(Self::reduce));
        });
    }

    fn update(&mut self, ui: &mut Ui) {
        let status:&mut Label=ui.get_widget("status").unwrap();
        status.set_text(&self.status);
    }


    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute{
            inner_size:(800,600).into(),
            ..Default::default()
        }
    }
}

```

### 详细用法，参考Wiki

* [控件使用🦖](https://github.com/xllgl2017/xlui/wiki/%E6%8E%A7%E4%BB%B6)
* [布局使用🦖](https://github.com/xllgl2017/xlui/wiki/%E5%B8%83%E5%B1%80)
* [窗口设置🦖](https://github.com/xllgl2017/xlui/wiki/%E7%AA%97%E5%8F%A3)

[//]:  # (❌⬜️)
