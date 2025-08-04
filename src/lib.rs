use std::time::SystemTime;

pub mod widgets;
mod align;
pub mod vertex;
mod radius;
pub mod layout;
pub mod text;
pub mod font;
pub mod size;
pub mod frame;
pub mod ui;

pub mod style;
mod paint;
mod render;
mod response;
mod map;

const SAMPLE_COUNT: u32 = 4;

#[derive(Clone, PartialEq, Debug)]
pub struct Pos {
    pub min: f32,
    pub max: f32,
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
    pressed: bool,
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
                pressed: false,
            }
        }
    }
}


pub fn gen_unique_id() -> String {
    let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    format!("{:x}", t)
}

