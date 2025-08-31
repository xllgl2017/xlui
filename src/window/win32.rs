use std::num::NonZeroIsize;
use std::thread::spawn;
use crate::window::WindowId;
use crate::{Pos, Size, WindowAttribute};
use raw_window_handle::{DisplayHandle, RawDisplayHandle, RawWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle};
use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, GetDC, ReleaseDC, SelectObject, ValidateRect, HBITMAP, HGDIOBJ};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAW};
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::window::event::WindowEvent;
use crate::window::tray::Tray;

const TRAY_ICON: u32 = WM_USER + 1;

pub struct Win32Window {
    id: WindowId,
    pub(crate) hwnd: HWND,
    size: Size,
    tray: Option<Tray>,
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

unsafe fn icon_to_bitmap(h_icon: HICON, width: i32, height: i32) -> HBITMAP {
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

unsafe fn load_tray_icon(ip: &str) -> HICON {
    let icon_path = to_wstr(ip);
    let h_icon = unsafe { LoadImageW(None, PCWSTR(icon_path.as_ptr()), IMAGE_ICON, 32, 32, LR_LOADFROMFILE).unwrap() };
    HICON(h_icon.0)
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

impl Win32Window {
    pub fn new(attr: &mut WindowAttribute) -> Win32Window {
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
            let mut window = Win32Window {
                id: WindowId(crate::unique_id_u32()),
                hwnd,
                size: attr.inner_size,
                tray: attr.tray.take(),
            };
            // unsafe { SetWindowLongPtrW(window.hwnd, GWLP_USERDATA, &window as *const _ as isize); }
            window.show_tray();
            window
        }
    }

    pub fn show_tray(&self) {
        println!("show  tray-{}", self.tray.is_some());
        if let Some(ref tray) = self.tray {
            let h_icon = match tray.icon {
                None => unsafe { LoadIconW(None, IDI_APPLICATION).unwrap() }
                Some(ref ip) => unsafe { load_tray_icon(ip) },
            };
            // 配置托盘图标数据
            let mut tip = [0; 128];
            let tip_s = to_wstr(&tray.hovered_text);
            tip[..tip_s.len()].copy_from_slice(tip_s.as_ref());
            let mut nid = NOTIFYICONDATAW {
                cbSize: size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: self.hwnd,
                uID: 1,
                uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP,
                uCallbackMessage: TRAY_ICON,
                hIcon: h_icon,
                szTip: tip,
                ..Default::default()
            };
            // 添加托盘图标
            unsafe { Shell_NotifyIconW(NIM_ADD, &mut nid); }
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

    pub fn set_visible(&self, visible: bool) {
        match visible {
            true => unsafe { ShowWindow(self.hwnd, SW_SHOW); },
            false => unsafe { ShowWindow(self.hwnd, SW_HIDE); },
        }
    }

    pub fn run(&self) -> WindowEvent {
        unsafe {
            let mut msg = std::mem::zeroed::<MSG>();
            let ret = GetMessageW(&mut msg, None, 0, 0);
            if ret.0 == 0 { return WindowEvent::ReqClose; }

            println!("{}", msg.message);
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
                    match self.tray {
                        None => WindowEvent::ReqClose,
                        Some(_) => {
                            self.set_visible(false);
                            WindowEvent::ReqClose
                        }
                    }
                }
                _ => {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                    // DefWindowProcW(self.hwnd, msg.message, msg.wParam, msg.lParam);
                    WindowEvent::None
                }
            }
        }
    }

    pub fn show_tray_menu(&self) {
        unsafe {
            if let Some(ref tray) = self.tray {
                let h_menu = CreatePopupMenu().unwrap();
                for menu in &tray.menus {
                    // // AppendMenuW(h_menu, MF_STRING, ID_TRAY_EXIT as usize, w!("退出"));
                    // let h_sub_menu = CreatePopupMenu().unwrap();
                    // AppendMenuW(h_sub_menu, MF_STRING, ID_TRAY_SUB_OPTION1 as usize, w!("子菜单1"));
                    // AppendMenuW(h_sub_menu, MF_STRING, ID_TRAY_SUB_OPTION2 as usize, w!("子菜单2"));
                    // 添加普通菜单项
                    AppendMenuW(h_menu, MF_STRING, menu.event, PCWSTR(to_wstr(&menu.label).as_ptr()));
                    if let Some(ref ip) = menu.icon {
                        let h_icon = load_tray_icon(ip);
                        let h_bitmap = icon_to_bitmap(h_icon, 16, 16); // 需要把 HICON 转成 HBITMAP
                        let mut mii = MENUITEMINFOW::default();
                        mii.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
                        mii.fMask = MIIM_BITMAP;
                        mii.hbmpItem = h_bitmap; // HBITMAP 或 HBMMENU_CALLBACK
                        SetMenuItemInfoW(h_menu, menu.event as u32, false, &mii);
                    }
                }
                // 获取鼠标位置
                let mut pt = POINT::default();
                GetCursorPos(&mut pt);

                // 必须先把窗口设为前台，否则菜单可能不会自动消失
                // SetForegroundWindow(self.hwnd);

                // 显示菜单（右键菜单）
                TrackPopupMenu(
                    h_menu,
                    TPM_RIGHTBUTTON,
                    pt.x,
                    pt.y,
                    Some(0),
                    self.hwnd,
                    None,
                );
                println!("111111111111");

                DestroyMenu(h_menu);
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