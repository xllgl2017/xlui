//! ### 最小运行示例
//! ![控件状态](https://github.com/xllgl2017/xlui/blob/main/res/img/doc/img.png)
//! ```
//! use xlui::frame::{WindowAttribute,App};
//! use xlui::widgets::Widget;
//! use xlui::widgets::label::Label;
//! use xlui::widgets::button::Button;
//! use xlui::ui::Ui;
//! use xlui::frame::context::Context;
//!
//! fn main() {
//!     let app=XlUiApp::new();
//!     //直接调run()
//!     app.run();
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
//!             label: Label::new("hello"),
//!             count: 0,
//!         }
//!     }
//!     fn add(&mut self,ui: &mut Ui){
//!         self.count += 1;
//!         self.label.set_text(format!("count: {}", self.count));
//!         self.label.update(ui);
//!     }
//!
//!     fn reduce(&mut self,ui: &mut Ui){
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
//!             Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add).redraw(ui);
//!             Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce).redraw(ui);
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
//!             inner_size:(800.0,600.0).into(),
//!             ..Default::default()
//!         }
//!     }
//! }
//!```

use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;
use crate::size::pos::Pos;
use crate::size::rect::Rect;
use crate::ui::Ui;

pub mod widgets;
mod align;
pub mod vertex;
pub mod radius;
pub mod layout;
pub mod text;
pub mod font;
pub mod size;
pub mod frame;
pub mod ui;

pub mod style;
mod render;
pub mod response;
pub mod map;

pub trait NumCastExt: Sized {
    fn as_f32(&self) -> f32;
    fn from_num<N: Into<f64>>(n: N) -> Self;
}

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

pub struct Offset {
    x: f32,
    y: f32,
}

impl Offset {
    pub fn new() -> Offset {
        Offset {
            x: 0.0,
            y: 0.0,
        }
    }

    pub fn new_y(y: f32) -> Offset {
        Offset {
            x: 0.0,
            y,
        }
    }
}


pub struct Device {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub cache: glyphon::Cache,
    pub texture_format: wgpu::TextureFormat,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub device_input: DeviceInput,
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

    pub fn update(&mut self, pos: winit::dpi::PhysicalPosition<f64>) {
        self.previous = self.lastest.clone();
        self.lastest.x = pos.x as f32;
        self.lastest.y = pos.y as f32;
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

        let press = rect.has_position(self.mouse.pressed_pos.x, self.mouse.pressed_pos.y);
        let release = rect.has_position(self.mouse.lastest.x, self.mouse.lastest.y);
        self.mouse.clicked.store(!(press && release), Ordering::SeqCst);
        press && release
    }

    pub fn pressed_at(&self, rect: &Rect) -> bool {
        if !self.mouse.pressed { return false; }
        rect.has_position(self.mouse.pressed_pos.x, self.mouse.pressed_pos.y)
    }

    pub fn hovered_at(&self, rect: &Rect) -> bool {
        rect.has_position(self.mouse.lastest.x, self.mouse.lastest.y)
    }
}

pub(crate) fn time_ms() -> u128 {
    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis()
}


pub fn gen_unique_id() -> String {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}", t)
}

pub fn _run_test(_: fn(&mut Ui)) {}