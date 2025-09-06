use crate::window::win32::{Win32Window, IME, REQ_CLOSE, TRAY_ICON};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, GetDC, ReleaseDC, SelectObject, HBITMAP, HGDIOBJ};
use windows::Win32::UI::Input::Ime::*;
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::error::UiResult;

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

pub fn icon_to_bitmap(h_icon: HICON, width: i32, height: i32) -> UiResult<HBITMAP> {
    let hdc = unsafe { GetDC(None) };
    let hdc_mem = unsafe { CreateCompatibleDC(Some(hdc)) };
    let hbm = unsafe { CreateCompatibleBitmap(hdc, width, height) };
    unsafe { SelectObject(hdc_mem, HGDIOBJ::from(hbm)) };

    // 绘制图标到位图
    unsafe { DrawIconEx(hdc_mem, 0, 0, h_icon, width, height, 0, None, DI_NORMAL)? };

    unsafe { DeleteDC(hdc_mem).ok()? };
    unsafe { ReleaseDC(None, hdc) };
    Ok(hbm)
}

pub unsafe fn load_tray_icon(ip: &str) -> HICON {
    let icon_path = to_wstr(ip);
    let h_icon = unsafe { LoadImageW(None, PCWSTR(icon_path.as_ptr()), IMAGE_ICON, 32, 32, LR_LOADFROMFILE).unwrap() };
    HICON(h_icon.0)
}

pub unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    println!("ime2-----------{}", msg);
    match msg {
        WM_CLOSE => {
            println!("req quit-{:?}", hwnd);
            unsafe { PostMessageW(Some(hwnd), REQ_CLOSE, WPARAM(0), LPARAM(0)).unwrap() };
            LRESULT(0)
        }
        TRAY_ICON => {
            match lparam.0 as u32 {
                WM_RBUTTONUP => {
                    let app: &Win32Window = unsafe { (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *const Win32Window).as_ref() }.unwrap();
                    app.show_tray_menu().unwrap();
                }
                _ => {}
            }
            LRESULT(0)
        }
        WM_IME_STARTCOMPOSITION | WM_IME_ENDCOMPOSITION | WM_IME_COMPOSITION => {
            unsafe { PostMessageW(Some(hwnd), IME, WPARAM(msg as usize), lparam).unwrap() };
            LRESULT(0)
        }
        WM_IME_NOTIFY => {

            match wparam.0 as u32 {
                IMN_OPENCANDIDATE | IMN_CHANGECANDIDATE | IMN_CLOSECANDIDATE => {
                    // 鼠标点击候选词会触发这些
                    unsafe {
                        let himc = ImmGetContext(hwnd);
                        let size = ImmGetCompositionStringW(himc, GCS_RESULTSTR, None, 0);
                        if size > 0 {
                            let len = size as usize / 2;
                            let mut buf: Vec<u16> = vec![0; len];
                            ImmGetCompositionStringW(himc, GCS_RESULTSTR, Some(buf.as_mut_ptr() as *mut _), size as u32);
                            let s = String::from_utf16_lossy(&buf);
                            println!("Mouse select Result: {}", s);
                        }
                        ImmReleaseContext(hwnd, himc).unwrap();
                    }
                }
                _ => {}
            }
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}