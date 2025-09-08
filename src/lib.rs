//!
//! xlui是一个Rust的2D GUI库。目标是利用Rust语言原生构建GUI、体积小(最小第三方依赖)，简单易用，
//! 在保证性能的前提下尽量减少CPU的开销。
//! ### 目前的控件工作状态如下图
//! ![控件状态](https://github.com/xllgl2017/xlui/blob/main/res/img/doc/img_1.png?raw=true)
//! ### 下面是xlui的最小运行示例
//! ```
//! use xlui::frame::App;
//! use xlui::*;
//! use xlui::ui::Ui;
//! use xlui::frame::context::Context;
//!
//! fn main() {
//!     let app=XlUiApp::new();
//!     //直接调run()
//!     // app.run().unwrap();
//! }
//!
//! struct XlUiApp {
//!     label: Label,
//!     count: i32,
//! }
//!
//!
//! impl XlUiApp {
//!     fn new()->XlUiApp{
//!         XlUiApp{
//!             label: Label::new("hello").width(100.0),
//!             count: 0,
//!         }
//!     }
//!     fn add(&mut self,_:&mut Button,ui: &mut Ui){
//!         self.count += 1;
//!         self.label.set_text(format!("count: {}", self.count));
//!         self.label.update(ui);
//!     }
//!
//!     fn reduce(&mut self,_:&mut Button,ui: &mut Ui){
//!         self.count-=1;
//!         self.label.set_text(format!("count: {}", self.count));
//!         self.label.update(ui);
//!     }
//! }
//!
//! //实现App trait
//! impl App for XlUiApp {
//!     fn draw(&mut self, ui: &mut Ui) {
//!         ui.add_mut(&mut self.label);
//!         ui.horizontal(|ui| {
//!             ui.add(Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add));
//!             ui.add(Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce));
//!         });
//!      }
//!
//!     fn update(&mut self, ui: &mut Ui) {
//!         self.label.update(ui);
//!      }
//!
//!     fn redraw(&mut self, ui: &mut Ui) {
//!         self.label.redraw(ui);
//!     }
//!
//!     fn window_attributes(&self) -> WindowAttribute {
//!         WindowAttribute{
//!             inner_size:(800,600).into(),
//!             ..Default::default()
//!         }
//!     }
//! }
//!```
//! * xlui可以在App.update中获取Widget的可变引用，以便修改控件
//! * update函数是后台接收到系统事件时才会调用，这里不应该直接修改，应根据条件修改。
//!```
//! use xlui::ui::Ui;
//! use xlui::*;
//!
//! fn update(ui:&mut Ui){
//!    let label:&mut Label=ui.get_widget("my_label").unwrap();
//!    label.set_text("这里是修改后的文本".to_string());
//! }
//! ```

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;
use crate::ui::Ui;

mod widgets;
pub mod align;
pub mod vertex;
pub mod layout;
mod text;
mod size;
pub mod frame;
pub mod ui;

pub mod style;
mod render;
pub mod response;
pub mod map;
mod window;
mod key;
mod error;

#[cfg(all(not(feature = "winit"), target_os = "windows"))]
pub use window::win32::tray::Tray;
pub use window::{attribute::WindowAttribute, inner::InnerWindow};
pub use size::{font::Font, border::Border, padding::Padding, radius::Radius, rect::Rect, pos::Pos, Size};
pub use widgets::{button::Button, spinbox::SpinBox, select::SelectItem, listview::ListView, slider::Slider,
                  checkbox::CheckBox, radio::RadioButton, processbar::ProcessBar, rectangle::Rectangle,
                  circle::Circle, textedit::TextEdit, label::Label, combobox::ComboBox, image::Image,
                  Widget, triangle::Triangle};
pub use text::{rich::RichTextExt, TextWrap, rich::RichText};

pub trait NumCastExt: Sized {
    fn as_f32(&self) -> f32;
    fn from_num<N: Into<f64>>(n: N) -> Self;
}

#[macro_export]
macro_rules! impl_num_cast_ext {
    ($($t:ty),*) => {
        $(
            impl NumCastExt for $t {
                fn as_f32(&self) -> f32 {
                    *self as f32
                }
                fn from_num<N: Into<f64>>(n: N) -> Self {
                    n.into() as $t
                }
            }
        )*
    }
}



// 支持的类型
impl_num_cast_ext!(i8, i16, i32, i64, u8, u16, u32, u64, f32, f64);


const SAMPLE_COUNT: u32 = 4;

#[derive(Clone, Debug)]
enum OffsetDirection {
    Down,
    Left,
    Right,
    Up,
}

#[derive(Clone, Debug)]
pub struct Offset {
    pos: Pos,
    x: f32,
    y: f32,
    covered: bool,
    direction: OffsetDirection,
    target_id: String,
}

impl Offset {
    pub fn new(pos: Pos) -> Offset {
        Offset {
            pos,
            x: 0.0,
            y: 0.0,
            covered: false,
            direction: OffsetDirection::Down,
            target_id: "".to_string(),
        }
    }

    pub fn with_pos(mut self, pos: Pos) -> Offset {
        self.pos = pos;
        self
    }

    pub fn with_y(mut self, y: f32) -> Offset {
        self.y = y;
        self
    }

    pub fn with_x(mut self, x: f32) -> Offset {
        self.x = x;
        self
    }

    pub fn delete_offset(mut self) -> Offset {
        self.covered = true;
        self
    }
}


pub struct Device {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub cache: glyphon::Cache,
    pub texture_format: wgpu::TextureFormat,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device_input: DeviceInput,
    pub surface: wgpu::Surface<'static>,
}

pub struct MouseInput {
    lastest: Pos,
    previous: Pos,
    delta: (f32, f32),

    pressed: bool,
    pressed_pos: Pos,
    pressed_time: u128,

    clicked: AtomicBool,
    a: f32,
}

impl MouseInput {
    pub fn offset(&self) -> (f32, f32) {
        (self.offset_x(), self.offset_y())
    }

    pub fn offset_x(&self) -> f32 {
        self.lastest.x - self.previous.x
    }

    pub fn offset_y(&self) -> f32 {
        self.lastest.y - self.previous.y
    }

    pub fn x(&self) -> f32 {
        self.lastest.x
    }

    pub fn y(&self) -> f32 {
        self.lastest.y
    }

    pub fn update(&mut self, pos: Pos) {
        self.previous = self.lastest.clone();
        self.lastest = pos;
    }

    pub fn lastest(&self) -> &Pos {
        &self.lastest
    }

    pub fn delta_x(&self) -> f32 { self.delta.0 }

    pub fn delta_y(&self) -> f32 { self.delta.1 }

    pub fn pressed(&self) -> bool { self.pressed }

    pub fn mouse_press(&mut self) {
        self.previous = self.lastest.clone();
        self.pressed_pos = self.lastest.clone();
        self.pressed_time = time_ms();
        self.pressed = true;
    }

    pub fn mouse_release(&mut self) {
        let et = time_ms();
        self.a = self.offset_y() * 120.0 / (et - self.pressed_time) as f32 / (et - self.pressed_time) as f32;
        println!("{} m/s2", self.a);
        self.clicked.store(true, Ordering::SeqCst);
        self.pressed = false;
        // self.pressed_pos.clear()
    }
}

pub struct DeviceInput {
    pub mouse: MouseInput,

}

impl DeviceInput {
    pub fn new() -> DeviceInput {
        DeviceInput {
            mouse: MouseInput {
                lastest: Pos::new(),
                previous: Pos::new(),
                delta: (0.0, 0.0),
                pressed_pos: Pos::new(),
                pressed: false,
                clicked: AtomicBool::new(false),
                pressed_time: 0,
                a: 0.0,
            }
        }
    }

    pub fn click_at(&self, rect: &Rect) -> bool {
        if !self.mouse.clicked.load(Ordering::SeqCst) { return false; }

        let press = rect.has_position(self.mouse.pressed_pos);
        let release = rect.has_position(self.mouse.lastest);
        self.mouse.clicked.store(!(press && release), Ordering::SeqCst);
        press && release
    }

    pub fn pressed_at(&self, rect: &Rect) -> bool {
        if !self.mouse.pressed { return false; }
        rect.has_position(self.mouse.pressed_pos)
    }

    pub fn hovered_at(&self, rect: &Rect) -> bool {
        rect.has_position(self.mouse.lastest)
    }
}

pub(crate) fn time_ms() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
}


pub fn unique_id_u32() -> u32 {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    println!("unique {}", t.to_string()[10..].to_string());
    t.to_string()[10..].to_string().parse::<u32>().unwrap()
}

pub fn gen_unique_id() -> String {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}", t)
}

pub fn _run_test(_: fn(&mut Ui)) {}