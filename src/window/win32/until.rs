use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, GetDC, ReleaseDC, SelectObject, HBITMAP, HGDIOBJ};
use windows::Win32::UI::WindowsAndMessaging::{DefWindowProcW, DrawIconEx, GetWindowLongPtrW, LoadImageW, PostQuitMessage, DI_NORMAL, GWLP_USERDATA, HICON, IMAGE_ICON, LR_LOADFROMFILE, WM_DESTROY, WM_RBUTTONUP};
use crate::window::win32::{Win32Window, TRAY_ICON};

pub fn to_wstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}

pub fn loword(l: u32) -> u16 {
    (l & 0xffff) as u16
}
pub fn hiword(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}

#[inline]
pub fn get_x_lparam(lp: LPARAM) -> i32 {
    (lp.0 as i16) as i32
}

#[inline]
pub fn get_y_lparam(lp: LPARAM) -> i32 {
    ((lp.0 >> 16) as i16) as i32
}

pub unsafe fn icon_to_bitmap(h_icon: HICON, width: i32, height: i32) -> HBITMAP {
    let hdc = GetDC(None);
    let hdc_mem = CreateCompatibleDC(Some(hdc));
    let hbm = CreateCompatibleBitmap(hdc, width, height);
    SelectObject(hdc_mem, HGDIOBJ::from(hbm));

    // 绘制图标到位图
    DrawIconEx(hdc_mem, 0, 0, h_icon, width, height, 0, None, DI_NORMAL);

    DeleteDC(hdc_mem);
    ReleaseDC(None, hdc);
    hbm
}

pub unsafe fn load_tray_icon(ip: &str) -> HICON {
    let icon_path = to_wstr(ip);
    let h_icon = unsafe { LoadImageW(None, PCWSTR(icon_path.as_ptr()), IMAGE_ICON, 32, 32, LR_LOADFROMFILE).unwrap() };
    HICON(h_icon.0)
}

pub unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        TRAY_ICON => {
            match lparam.0 as u32 {
                WM_RBUTTONUP => {
                    let app: &Win32Window = &*unsafe { GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Win32Window };
                    app.show_tray_menu();
                }
                _ => {}
            }
            // PostMessageW(Some(hwnd), TRAY_ICON, wparam, lparam);
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}