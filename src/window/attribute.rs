use std::sync::Arc;
#[cfg(feature = "winit")]
use winit::window::{Icon, WindowLevel};
use crate::Font;
use crate::size::Size;
#[cfg(all(target_os = "windows", not(feature = "winit")))]
use crate::window::win32::tray::Tray;

pub struct WindowAttribute {
    pub inner_size: Size,
    pub min_inner_size: Size,
    pub max_inner_size: Size,
    pub position: [i32; 2],
    pub resizable: bool,
    pub title: String,
    pub maximized: bool,
    pub visible: bool,
    pub transparent: bool,
    pub blur: bool,
    pub decorations: bool,
    pub window_icon: Arc<Vec<u8>>,
    #[cfg(feature = "winit")]
    pub window_level: WindowLevel,
    pub font: Arc<Font>,
    #[cfg(all(not(feature = "winit"), target_os = "windows"))]
    pub tray: Option<Tray>,
}

impl WindowAttribute {
    #[cfg(feature = "winit")]
    pub fn as_winit_attributes(&self) -> winit::window::WindowAttributes {
        let attr = winit::window::WindowAttributes::default();
        let (rgba, size) = super::super::render::image::load_image_bytes(self.window_icon.as_ref()).unwrap();
        // let img = image::load_from_memory(self.window_icon.as_ref()).unwrap();
        // let rgb8 = img.to_rgba8();
        let icon = Icon::from_rgba(rgba, size.width, size.height).unwrap();
        attr.with_inner_size(self.inner_size.as_physical_size())
            .with_min_inner_size(self.min_inner_size.as_physical_size())
            .with_max_inner_size(self.max_inner_size.as_physical_size())
            .with_position(winit::dpi::Position::Physical(winit::dpi::PhysicalPosition::new(self.position[0], self.position[1])))
            .with_resizable(self.resizable)
            .with_title(self.title.as_str())
            .with_maximized(self.maximized)
            .with_visible(self.visible)
            .with_transparent(self.transparent)
            .with_blur(self.blur)
            .with_decorations(self.decorations)
            .with_window_icon(Some(icon))
            .with_window_level(self.window_level)
    }

    pub(crate) fn inner_width_f32(&self) -> f32 {
        self.inner_size.width as f32
    }

    pub(crate) fn inner_height_f32(&self) -> f32 {
        self.inner_size.height as f32
    }

    pub(crate) fn pos_x_f32(&self) -> f32 {
        self.position[0] as f32
    }

    pub(crate) fn pos_y_f32(&self) -> f32 {
        self.position[1] as f32
    }
}

impl Default for WindowAttribute {
    fn default() -> WindowAttribute {
        WindowAttribute {
            inner_size: Size { width: 800, height: 600 },
            min_inner_size: Size { width: 0, height: 0 },
            max_inner_size: Size { width: 2560, height: 1440 },
            position: [100, 100],
            resizable: true,
            title: "xlui".to_string(),
            maximized: false,
            visible: true, //是否隐藏窗口
            transparent: false, //窗口透明，配合LoadOp::Clear
            blur: true, //未知
            decorations: true, //标题栏
            window_icon: Arc::new(include_bytes!("../../logo.jpg").to_vec()),
            #[cfg(feature = "winit")]
            window_level: WindowLevel::Normal,
            font: Arc::new(Font::default()),
            #[cfg(all(not(feature = "winit"), target_os = "windows"))]
            tray: None,
        }
    }
}