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
//!             Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add).draw(ui);
//!             Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce).draw(ui);
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

use std::time::SystemTime;
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

#[derive(Clone, PartialEq, Debug)]
pub struct Pos {
    // x:f32,
    // y:f32
    pub min: f32,
    pub max: f32,
}

impl Pos {
    pub fn center(&self) -> f32 {
        (self.min + self.max) / 2.0
    }

    pub fn offset(&mut self, offset: f32) {
        self.min += offset;
        self.max += offset;
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
    lastest: (f32, f32),
    previous: (f32, f32),
    delta: (f32, f32),
    pressed_pos: (f32, f32),
    pressed: bool,
    clicked: bool,
}

impl MouseInput {
    pub fn offset(&self) -> (f32, f32) {
        (self.offset_x(), self.offset_y())
    }

    pub fn offset_x(&self) -> f32 {
        self.lastest.0 - self.previous.0
    }

    pub fn offset_y(&self) -> f32 {
        self.lastest.1 - self.previous.1
    }

    pub fn x(&self) -> f32 {
        self.lastest.0
    }

    pub fn y(&self) -> f32 {
        self.lastest.1
    }

    pub fn update(&mut self, pos: winit::dpi::PhysicalPosition<f64>) {
        self.previous = self.lastest;
        self.lastest.0 = pos.x as f32;
        self.lastest.1 = pos.y as f32;
    }

    pub fn lastest(&self) -> (f32, f32) {
        self.lastest
    }

    pub fn delta_x(&self) -> f32 { self.delta.0 }

    pub fn delta_y(&self) -> f32 { self.delta.1 }

    pub fn pressed(&self) -> bool { self.pressed }
}

pub struct DeviceInput {
    pub mouse: MouseInput,

}

impl DeviceInput {
    pub fn new() -> DeviceInput {
        DeviceInput {
            mouse: MouseInput {
                lastest: (0.0, 0.0),
                previous: (0.0, 0.0),
                delta: (0.0, 0.0),
                pressed_pos: (0.0, 0.0),
                pressed: false,
                clicked: false,
            }
        }
    }

    pub fn click_at(&self, rect: &Rect) -> bool {
        let (lx, ly) = self.mouse.pressed_pos;
        let (x, y) = self.mouse.lastest;
        rect.has_position(lx, ly) && rect.has_position(x, y) && self.mouse.clicked
    }
    pub fn pressed_at(&self, rect: &Rect) -> bool {
        // println!("{:?} {:?}", rect, self.mouse.pressed_pos);
        rect.has_position(self.mouse.pressed_pos.0, self.mouse.pressed_pos.1)
    }

    pub fn hovered_at(&self, rect: &Rect) -> bool {
        rect.has_position(self.mouse.lastest.0, self.mouse.lastest.1)
    }
    // pub fn mouse_at_extend(&self, rect: &Rect) -> bool {
    //     let (x, y) = self.mouse.lastest;
    //     rect.has_position_extend(x, y)
    // }
}


pub fn gen_unique_id() -> String {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}", t)
}

pub fn _run_test(_: fn(&mut Ui)) {}