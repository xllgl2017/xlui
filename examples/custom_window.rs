// Cargo.toml 必须
// wgpu = "0.25"
// raw-window-handle = "0.4"
// pollster = "0.3"
// windows = { version = "0.48", features = ["Win32_Foundation", "Win32_UI_WindowsAndMessaging", "Win32_Graphics_Dwm"] }

use std::ptr::null_mut;
use windows::Win32::{
    Foundation::*,
    UI::WindowsAndMessaging::*,
    Graphics::Dwm::*,
};
use raw_window_handle::Win32WindowHandle;
use wgpu::util::DeviceExt;
use windows::core::PCWSTR;
use windows::Win32::Graphics::Gdi::HBRUSH;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Controls::MARGINS;

// 简单窗口属性
struct WindowAttr {
    title: String,
    width: u32,
    height: u32,
    x: i32,
    y: i32,
}

// helper: Rust str -> UTF16
fn to_wstr(s: &str) -> Vec<u16> {
    use std::os::windows::ffi::OsStrExt;
    std::ffi::OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

fn main() -> windows::core::Result<()> {
    unsafe {
        let hinstance = GetModuleHandleW(None)?;

        // 注册窗口类
        let class_name = to_wstr("transparent_wgpu");
        let wc = WNDCLASSW {
            lpfnWndProc: Some(wnd_proc),
            hInstance: HINSTANCE::from(hinstance),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: HBRUSH(null_mut()), // 不擦除背景
            ..Default::default()
        };
        RegisterClassW(&wc);

        let attr = WindowAttr {
            title: "透明 wgpu 窗口".to_string(),
            width: 800,
            height: 600,
            x: 100,
            y: 100,
        };

        // 创建透明窗口
        let hwnd = CreateWindowExW(
            WS_EX_LAYERED | WS_EX_TOPMOST, // 关键透明
            PCWSTR(class_name.as_ptr()),
            PCWSTR(to_wstr(&attr.title).as_ptr()),
            WS_POPUP | WS_VISIBLE,
            attr.x,
            attr.y,
            attr.width as i32,
            attr.height as i32,
            None,
            None,
            Some(HINSTANCE::from(hinstance)),
            None,
        ).unwrap();

        SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA); // 全透明背景
        let margins = MARGINS { cxLeftWidth: -1, cxRightWidth: -1, cyTopHeight: -1, cyBottomHeight: -1 };
        DwmExtendFrameIntoClientArea(hwnd, &margins)?;

        ShowWindow(hwnd, SW_SHOW);
    }

    Ok(())
}

// Win32 消息处理
extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match msg {
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            WM_ERASEBKGND => LRESULT(1), // 不擦背景
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}
