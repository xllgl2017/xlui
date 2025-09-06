use crate::key::Key;
use crate::window::event::WindowEvent;
use crate::window::ime::IME;
use crate::window::win32::tray::Tray;
use crate::window::{WindowId, WindowKind, WindowType};
use crate::{Pos, Size, WindowAttribute};
use std::sync::Arc;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, POINT};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAW};
use windows::Win32::UI::WindowsAndMessaging::*;

pub mod tray;
pub(crate) mod handle;
mod until;

const TRAY_ICON: u32 = WM_USER + 1;

pub struct Win32Window {
    // id: WindowId,
    // pub(crate) hwnd: HWND,
    size: Size,
    tray: Option<Tray>,
    handles: Vec<Arc<WindowType>>,
}


impl Win32Window {
    pub fn new(attr: &mut WindowAttribute, ime: Arc<IME>) -> Win32Window {
        unsafe {
            let hinstance = GetModuleHandleW(None).unwrap();
            let class_name = until::to_wstr(&attr.title);
            let wc = WNDCLASSW {
                lpfnWndProc: Some(until::wndproc),
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
                PCWSTR(until::to_wstr(&attr.title).as_ptr()),
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
            let handle = handle::Win32WindowHandle {
                hwnd,
            };
            let window_type = WindowType {
                kind: WindowKind::Win32(handle),
                id: WindowId::unique_id(),
                type_: WindowType::ROOT,
                ime,
            };
            let mut window = Win32Window {
                // id: WindowId(crate::unique_id_u32()),
                // hwnd,
                size: attr.inner_size,
                tray: attr.tray.take(),
                handles: vec![Arc::new(window_type)],
            };
            // unsafe { SetWindowLongPtrW(window.hwnd, GWLP_USERDATA, &window as *const _ as isize); }
            window.show_tray();
            window
        }
    }

    pub fn last_window(&self) -> Arc<WindowType> {
        self.handles.last().unwrap().clone()
    }

    pub fn show_tray(&self) {
        println!("show  tray-{}", self.tray.is_some());
        if let Some(ref tray) = self.tray {
            let h_icon = match tray.icon {
                None => unsafe { LoadIconW(None, IDI_APPLICATION).unwrap() }
                Some(ref ip) => unsafe { until::load_tray_icon(ip) },
            };
            // 配置托盘图标数据
            let mut tip = [0; 128];
            let tip_s = until::to_wstr(&tray.hovered_text);
            tip[..tip_s.len()].copy_from_slice(tip_s.as_ref());
            let mut nid = NOTIFYICONDATAW {
                cbSize: size_of::<NOTIFYICONDATAW>() as u32,
                hWnd: self.handles[0].win32().hwnd,
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

    pub fn create_child_window(&self, parent: &Arc<WindowType>, attr: &WindowAttribute) -> Arc<WindowType> {
        self.handles[0].clone()
    }

    pub fn run(&self) -> (WindowId, WindowEvent) {
        unsafe {
            let mut msg = std::mem::zeroed::<MSG>();
            let ret = GetMessageW(&mut msg, None, 0, 0);
            if ret.0 == 0 { return (self.handles[0].id, WindowEvent::ReqClose); }

            println!("{}", msg.message);
            match msg.message {
                WM_SIZE => {
                    let width = until::loword(msg.lParam.0 as u32) as u32;
                    let height = until::hiword(msg.lParam.0 as u32) as u32;
                    println!("resize-{}-{}", width, height);
                    (self.handles[0].id, WindowEvent::Resize(Size { width, height }))
                }
                WM_PAINT => {
                    println!("paint");
                    ValidateRect(Option::from(self.handles[0].win32().hwnd), None);
                    (self.handles[0].id, WindowEvent::Redraw)
                    // LRESULT(0)
                }
                WM_KEYDOWN => {
                    println!("Key down: {}", msg.wParam.0);
                    (self.handles[0].id, WindowEvent::KeyPress(Key::Backspace))
                }
                WM_LBUTTONDOWN => {
                    let x = until::get_x_lparam(msg.lParam) as f32;
                    let y = until::get_y_lparam(msg.lParam) as f32;
                    (self.handles[0].id, WindowEvent::MousePress(Pos { x, y }))
                }
                WM_LBUTTONUP => {
                    let x = until::get_x_lparam(msg.lParam) as f32;
                    let y = until::get_y_lparam(msg.lParam) as f32;
                    (self.handles[0].id, WindowEvent::MouseRelease(Pos { x, y }))
                }
                WM_MOUSEMOVE => {
                    let x = until::get_x_lparam(msg.lParam) as f32;
                    let y = until::get_y_lparam(msg.lParam) as f32;
                    (self.handles[0].id, WindowEvent::MouseMove(Pos { x, y }))
                }
                WM_DESTROY => {
                    // unsafe { PostQuitMessage(0); }
                    println!("exit");
                    match self.tray {
                        None => (self.handles[0].id, WindowEvent::ReqClose),
                        Some(_) => {
                            self.handles[0].win32().set_visible(false).unwrap();
                            (self.handles[0].id, WindowEvent::ReqClose)
                        }
                    }
                }
                _ => {
                    TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                    // DefWindowProcW(self.hwnd, msg.message, msg.wParam, msg.lParam);
                    (self.handles[0].id, WindowEvent::None)
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
                    AppendMenuW(h_menu, MF_STRING, menu.event, PCWSTR(until::to_wstr(&menu.label).as_ptr()));
                    if let Some(ref ip) = menu.icon {
                        let h_icon = until::load_tray_icon(ip);
                        let h_bitmap = until::icon_to_bitmap(h_icon, 16, 16); // 需要把 HICON 转成 HBITMAP
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
                    self.handles[0].win32().hwnd,
                    None,
                );
                println!("111111111111");

                DestroyMenu(h_menu);
            }
        }
    }
}


