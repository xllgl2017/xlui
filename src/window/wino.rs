use crate::frame::context::{Context, Render, UpdateType};
use crate::frame::App;
use crate::map::Map;
use crate::ui::AppContext;
use crate::window::event::WindowEvent;
use crate::window::{UserEvent, WindowId, WindowType};
use crate::{Device, DeviceInput, Size, WindowAttribute};
use glyphon::{Cache, Resolution, Viewport};
use std::error::Error;
use std::sync::Arc;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GWLP_USERDATA};

pub trait EventLoopHandle {
    fn event(&mut self, event: WindowEvent);
}

pub struct LoopWindow {
    pub(crate) app_ctx: AppContext,
    pub(crate) app: Box<dyn App>,
}

impl LoopWindow {
    pub async fn create_window(mut app: Box<dyn App>, wt: Arc<WindowType>, attr: &WindowAttribute) -> LoopWindow {
        let device = Self::rebuild_device(&wt, attr.inner_size).await.unwrap();
        device.surface.configure(&device.device, &device.surface_config);
        let viewport = Viewport::new(&device.device, &device.cache);
        let context = Context {
            size: attr.inner_size,
            font: attr.font.clone(),
            viewport,
            window: wt,
            resize: false,
            render: Render::new(&device),
            updates: Map::new(),
            user_update: (WindowId(crate::unique_id_u32()), UpdateType::None),
            new_window: None,
        };
        let mut app_ctx = AppContext::new(device, context);
        app_ctx.draw(&mut app);
        LoopWindow {
            app_ctx,
            app,
        }
    }

    pub(crate) async fn rebuild_device(window: &Arc<WindowType>, size: Size) -> Result<Device, Box<dyn Error>> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions::default()).await?;
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor::default()).await?;
        let cache = Cache::new(&device);
        let surface = instance.create_surface(window.clone())?;
        let cap = surface.get_capabilities(&adapter);
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: cap.formats[0],
            view_formats: vec![cap.formats[0].add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: size.width,
            height: size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        let w = window.clone();
        device.on_uncaptured_error(Box::new(move |err: wgpu::Error| {
            w.request_update(UserEvent::ReInit);
            println!("Error: {:#?}", err);
            println!("{}", err.to_string());
        }));
        Ok(Device {
            device,
            queue,
            cache,
            surface,
            texture_format: cap.formats[0],
            surface_config,
            device_input: DeviceInput::new(),
        })
    }
}

impl EventLoopHandle for LoopWindow {
    fn event(&mut self, event: WindowEvent) {
        println!("{:?}", event);
        match event {
            WindowEvent::None => {}
            WindowEvent::KeyPress(_) => {}
            WindowEvent::KeyRelease(key) => {
                self.app_ctx.key_input(UpdateType::KeyRelease(Some(key)), &mut self.app);
            }
            WindowEvent::MouseMove(pos) => {
                self.app_ctx.device.device_input.mouse.update(pos);
                self.app_ctx.update(UpdateType::MouseMove, &mut self.app);
            }
            WindowEvent::MouseWheel => {}
            WindowEvent::MousePress(_) => {
                self.app_ctx.device.device_input.mouse.mouse_press();
                self.app_ctx.update(UpdateType::MousePress, &mut self.app);
            }
            WindowEvent::MouseRelease(_) => {
                self.app_ctx.device.device_input.mouse.mouse_release();
                self.app_ctx.update(UpdateType::MouseRelease, &mut self.app);
                self.app_ctx.device.device_input.mouse.a = 0.0;
            }
            WindowEvent::Redraw => {
                self.app_ctx.context.viewport.update(&self.app_ctx.device.queue, Resolution {
                    width: self.app_ctx.device.surface_config.width,
                    height: self.app_ctx.device.surface_config.height,
                });
                self.app_ctx.redraw(&mut self.app)
            }
            WindowEvent::Reinit => {}
            WindowEvent::Resize(size) => {
                self.app_ctx.context.size = size;
                self.app_ctx.device.surface_config.width = size.width;
                self.app_ctx.device.surface_config.height = size.height;
                let device = &self.app_ctx.device.device;
                let config = &self.app_ctx.device.surface_config;
                self.app_ctx.device.surface.configure(device, config);
            }
            // WindowEvent::ReqClose => self.sender.send((self.app_ctx.context.window.id(), WindowEvent::ReqClose)).unwrap(),
            WindowEvent::ReqUpdate => self.app_ctx.update(self.app_ctx.context.user_update.1.clone(), &mut self.app),
            WindowEvent::IME => self.app_ctx.update(UpdateType::IME, &mut self.app),
            _ => {}
        }
    }
}

// unsafe impl Send for LoopWindow {}
// unsafe impl Sync for LoopWindow {}
