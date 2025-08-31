use std::num::NonZeroIsize;
use crate::window::WindowId;
use crate::{Pos, Size, WindowAttribute};
use raw_window_handle::{DisplayHandle, RawDisplayHandle, RawWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::window::event::WindowEvent;

pub struct Win32Window {
    id: WindowId,
    hwnd: HWND,
    size: Size,

}

fn to_wstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
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

unsafe extern "system" fn wndproc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_DESTROY => {
            unsafe { PostQuitMessage(0) };
            LRESULT(0)
        }
        _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }
}

impl Win32Window {
    pub fn new(attr: &WindowAttribute) -> Win32Window {
        unsafe {
            let hinstance = GetModuleHandleW(None).unwrap();
            let class_name = to_wstr(&attr.title);
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
                PCWSTR(to_wstr(&attr.title).as_ptr()),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                attr.inner_size.width as i32,
                attr.inner_size.height as i32,
                None,
                None,
                Some(HINSTANCE::from(hinstance)),
                None,
            ).unwrap();
            Win32Window {
                id: WindowId(crate::unique_id_u32()),
                hwnd,
                size: attr.inner_size,
            }
        }
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn id(&self) -> WindowId {
        self.id
    }

    pub fn request_redraw(&self) {
        unsafe { PostMessageW(Option::from(self.hwnd), WM_PAINT, WPARAM(0), LPARAM(0)); }
    }

    pub fn run(&self) -> WindowEvent {
        unsafe {
            let mut msg = std::mem::zeroed::<MSG>();
            let ret = GetMessageW(&mut msg, None, 0, 0);
            if ret.0 == 0 { return WindowEvent::ReqClose; }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
            match msg.message {
                WM_SIZE => {
                    let width = LOWORD(msg.lParam.0 as u32) as u32;
                    let height = HIWORD(msg.lParam.0 as u32) as u32;
                    println!("resize-{}-{}", width, height);
                    WindowEvent::Resize(Size { width, height })
                }
                WM_PAINT => {
                    println!("paint");
                    ValidateRect(Option::from(self.hwnd), None);
                    WindowEvent::Redraw
                    // LRESULT(0)
                }
                WM_KEYDOWN => {
                    println!("Key down: {}", msg.wParam.0);
                    WindowEvent::KeyPress
                }
                WM_LBUTTONDOWN => {
                    let x = get_x_lparam(msg.lParam) as f32;
                    let y = get_y_lparam(msg.lParam) as f32;
                    WindowEvent::MousePress(Pos { x, y })
                }
                WM_LBUTTONUP => {
                    let x = get_x_lparam(msg.lParam) as f32;
                    let y = get_y_lparam(msg.lParam) as f32;
                    WindowEvent::MouseRelease(Pos { x, y })
                }
                WM_MOUSEMOVE => {
                    let x = get_x_lparam(msg.lParam) as f32;
                    let y = get_y_lparam(msg.lParam) as f32;
                    WindowEvent::MouseMove(Pos { x, y })
                }
                WM_DESTROY => {
                    // unsafe { PostQuitMessage(0); }
                    println!("exit");
                    WindowEvent::ReqClose
                }
                _ => {
                    // DefWindowProcW(self.hwnd, msg.message, msg.wParam, msg.lParam);
                    WindowEvent::None
                }
            }
        }
    }

    pub fn window_handle(&self) -> WindowHandle {
        let hwnd_nz = NonZeroIsize::new(self.hwnd.0 as isize).unwrap();
        let mut win32_window_handle = Win32WindowHandle::new(hwnd_nz);
        let hinst = unsafe { GetWindowLongPtrW(self.hwnd, GWLP_HINSTANCE) };
        if let Some(nz) = NonZeroIsize::new(hinst) {
            win32_window_handle.hinstance = Some(nz);
        }

        let raw_window_handle = RawWindowHandle::Win32(win32_window_handle);
        unsafe { WindowHandle::borrow_raw(raw_window_handle) }
    }

    pub fn display_handle(&self) -> DisplayHandle {
        let win32_display_handle = WindowsDisplayHandle::new();
        let raw_display_handle = RawDisplayHandle::Windows(win32_display_handle);
        unsafe { DisplayHandle::borrow_raw(raw_display_handle) }
    }
}


unsafe impl Sync for Win32Window {}

unsafe impl Send for Win32Window {}