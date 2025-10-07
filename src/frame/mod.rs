use std::any::Any;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{BeginPaint, EndPaint, GetStockObject, HBRUSH, PAINTSTRUCT, WHITE_BRUSH};
use windows::Win32::UI::WindowsAndMessaging::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, PostQuitMessage, RegisterClassW, SetWindowLongPtrW, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, GWLP_USERDATA, IDC_ARROW, MSG, WM_DESTROY, WM_PAINT, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE};
#[cfg(feature = "winit")]
use winit::event_loop::{ControlFlow, EventLoop};
use crate::error::UiResult;
#[cfg(feature = "winit")]
use crate::window::winit_app::WInitApplication;
use crate::ui::Ui;
use crate::window::win32::until::to_wstr;
use crate::window::win32::{until, Win32Window};
use crate::window::wino::EventLoopHandle;
use crate::WindowAttribute;

pub mod context;


pub trait App: Any + 'static {
    fn draw(&mut self, ui: &mut Ui);
    fn update(&mut self, _: &mut Ui) {}

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute::default()
    }

    fn run(self) -> UiResult<()>
    where
        Self: Sized,
    {
        //wasm-pack build --target web
        #[cfg(feature = "winit")]
        return start_winit_app();
        #[cfg(target_os = "windows")]
        return start_win32_app(self);
    }
}


#[cfg(feature = "winit")]
fn start_winit_app() -> UiResult<()> {
    let event_loop = EventLoop::with_user_event().build()?;
    let proxy_event = event_loop.create_proxy();
    event_loop.set_control_flow(ControlFlow::Wait);
    let mut application = WInitApplication::new();
    application.set_app(Some(self));
    application.set_proxy_event(Some(proxy_event));
    event_loop.run_app(&mut application)?;
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            // paint_rect(hdc);
            // 定义圆的矩形边界
            // 定义三角形的三个点
            // paint_triangle(hdc);
            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
fn to_wide_null(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

#[cfg(all(windows, not(feature = "winit")))]
fn start_win32_app<A: App>(app: A) -> UiResult<()> {
    let mut win32 = Win32Window::new(app)?;
    let window = win32.get_window_by_index(0);
    unsafe { SetWindowLongPtrW(window.handle().win32().hwnd, GWLP_USERDATA, &mut win32 as *mut _ as isize); }
    win32.run()?;
    Ok(())
}