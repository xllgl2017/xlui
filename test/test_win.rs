// src/main.rs
use windows::core::PCWSTR;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;

use std::ptr::null_mut;

// static mut STATE: Option<State> = None;

// struct State {
//     surface: wgpu::Surface<'static>,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     config: wgpu::SurfaceConfiguration,
//     size: (u32, u32),
// }

// impl State {
//     async fn new(hwnd: HWND, width: u32, height: u32) -> Self {
//         let instance = wgpu::Instance::default();
//         // 创建 surface (Win32 HWND)
//         let surface = unsafe { instance.create_adapter_from_hal(Exposed) }
//             .expect("create surface");
//
//         let adapter = instance
//             .request_adapter(&wgpu::RequestAdapterOptions {
//                 power_preference: wgpu::PowerPreference::HighPerformance,
//                 compatible_surface: Some(&surface),
//                 force_fallback_adapter: false,
//             })
//             .await
//             .unwrap();
//
//         let (device, queue) = adapter
//             .request_device(
//                 &wgpu::DeviceDescriptor {
//                     label: None,
//                     required_features: Default::default(),
//                     required_limits: Default::default(),
//                     memory_hints: Default::default(),
//                     trace: Default::default(),
//                 },
//             )
//             .await
//             .unwrap();
//
//         let surface_caps = surface.get_capabilities(&adapter);
//         let config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: surface_caps.formats[0],
//             width,
//             height,
//             present_mode: surface_caps.present_modes[0],
//             desired_maximum_frame_latency: 0,
//             alpha_mode: surface_caps.alpha_modes[0],
//             view_formats: vec![],
//         };
//         surface.configure(&device, &config);
//
//         Self {
//             surface,
//             device,
//             queue,
//             config,
//             size: (width, height),
//         }
//     }
//
//     fn resize(&mut self, width: u32, height: u32) {
//         if width > 0 && height > 0 {
//             self.size = (width, height);
//             self.config.width = width;
//             self.config.height = height;
//             self.surface.configure(&self.device, &self.config);
//         }
//     }
//
//     fn render(&mut self) {
//         match self.surface.get_current_texture() {
//             Ok(output) => {
//                 let view = output
//                     .texture
//                     .create_view(&wgpu::TextureViewDescriptor::default());
//
//                 let mut encoder = self
//                     .device
//                     .create_command_encoder(&wgpu::CommandEncoderDescriptor {
//                         label: Some("Render Encoder"),
//                     });
//
//                 {
//                     encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                         label: Some("Render Pass"),
//                         color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                             view: &view,
//                             resolve_target: None,
//                             ops: wgpu::Operations {
//                                 load: wgpu::LoadOp::Clear(wgpu::Color {
//                                     r: 0.2,
//                                     g: 0.3,
//                                     b: 0.4,
//                                     a: 1.0,
//                                 }),
//                                 store: wgpu::StoreOp::Store,
//                             },
//                         })],
//                         depth_stencil_attachment: None,
//                         timestamp_writes: None,
//                         occlusion_query_set: None,
//                     });
//                 }
//
//                 self.queue.submit(Some(encoder.finish()));
//                 output.present();
//             }
//             Err(e) => eprintln!("render error: {:?}", e),
//         }
//     }
// }

unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_SIZE => {
            let width = LOWORD(lparam.0 as u32) as u32;
            let height = HIWORD(lparam.0 as u32) as u32;
            // if let Some(st) = STATE.as_mut() {
            //     st.resize(width, height);
            // }
            println!("resize-{}-{}", width, height);
            LRESULT(0)
        }
        WM_PAINT => {
            // if let Some(st) = STATE.as_mut() {
            //     st.render();
            // }
            println!("paint");
            ValidateRect(Option::from(hwnd), None);
            LRESULT(0)
        }
        WM_KEYDOWN => {
            println!("Key down: {}", wparam.0);
            LRESULT(0)
        }
        WM_MOUSEMOVE => {
            let x = get_x_lparam(lparam);
            let y = get_y_lparam(lparam);
            println!("Mouse move: {},{}", x, y);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            println!("exit");
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn LOWORD(l: u32) -> u16 {
    (l & 0xffff) as u16
}
fn HIWORD(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}

#[inline]
fn get_x_lparam(lp: LPARAM) -> i32 {
    (lp.0 as i16) as i32
}

#[inline]
fn get_y_lparam(lp: LPARAM) -> i32 {
    ((lp.0 >> 16) as i16) as i32
}

fn to_wstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}



fn main() -> windows::core::Result<()> {
    unsafe {
        let hinstance = GetModuleHandleW(None).unwrap();
        let class_name = to_wstr("dfdsgsdg");
        let wc = WNDCLASSW {
            lpfnWndProc: Some(wndproc),
            hInstance: HINSTANCE::from(hinstance),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            // hbrBackground: HBRUSH(COLOR_WINDOW.0 as isize),
            ..Default::default()
        };

        RegisterClassW(&wc);
        let hwnd = CreateWindowExW(
            Default::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(to_wstr("sdfsdsdgsdfgsd").as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            Some(HINSTANCE::from(hinstance)),
            None,
        ).unwrap();
        // 初始化 wgpu
        // STATE = Some(pollster::block_on(State::new(hwnd, 800, 600)));

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
    Ok(())
}
