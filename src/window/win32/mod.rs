use crate::error::UiResult;
use crate::key::Key;
use crate::map::Map;
use crate::window::event::WindowEvent;
use crate::window::ime::IME;
use crate::window::win32::handle::Win32WindowHandle;
use crate::window::win32::tray::Tray;
use crate::window::{WindowId, WindowKind, WindowType};
use crate::{Pos, Size, WindowAttribute};
use std::sync::Arc;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, POINT};
use windows::Win32::Graphics::Gdi::ValidateRect;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Input::Ime::{ImmGetCompositionStringW, ImmGetContext, GCS_COMPSTR, GCS_RESULTSTR};
use windows::Win32::UI::Shell::{Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAW};
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::window::win32::clipboard::Win32Clipboard;

pub mod tray;
pub(crate) mod handle;
mod until;
mod clipboard;

const TRAY_ICON: u32 = WM_USER + 1;
const REQ_UPDATE: u32 = WM_USER + 2;
const CREATE_CHILD: u32 = WM_USER + 3;
const RE_INIT: u32 = WM_USER + 4;
const IME: u32 = WM_USER + 5;
const REQ_CLOSE: u32 = 99999;

pub struct Win32Window {
    tray: Option<Tray>,
    handles: Map<WindowId, Arc<WindowType>>,
}


impl Win32Window {
    pub fn new(attr: &mut WindowAttribute, ime: Arc<IME>) -> UiResult<Win32Window> {
        let handle = Win32Window::create_window(attr)?;
        let window_type = WindowType {
            kind: WindowKind::Win32(handle),
            id: WindowId::unique_id(),
            type_: WindowType::ROOT,
            ime,
        };
        let mut handles = Map::new();
        handles.insert(window_type.id, Arc::new(window_type));

        let window = Win32Window {
            tray: attr.tray.take(),
            handles,
        };
        window.show_tray()?;
        Ok(window)
    }

    pub fn last_window(&self) -> Arc<WindowType> {
        self.handles.last().unwrap().clone()
    }

    pub fn show_tray(&self) -> UiResult<()> {
        println!("show  tray-{}", self.tray.is_some());
        if let Some(ref tray) = self.tray {
            let h_icon = match tray.icon {
                None => unsafe { LoadIconW(None, IDI_APPLICATION)? }
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
            unsafe { Shell_NotifyIconW(NIM_ADD, &mut nid).ok()?; }
        }
        Ok(())
    }

    fn create_window(attr: &WindowAttribute) -> UiResult<Win32WindowHandle> {
        let hinstance = unsafe { GetModuleHandleW(None) }?;
        let class_name = until::to_wstr(&(attr.title.clone()));
        let wc = WNDCLASSW {
            lpfnWndProc: Some(until::wndproc),
            hInstance: HINSTANCE::from(hinstance),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hCursor: unsafe { LoadCursorW(None, IDC_ARROW)? },
            ..Default::default()
        };
        unsafe { RegisterClassW(&wc); }
        let hwnd = unsafe {
            CreateWindowExW(
                Default::default(),
                PCWSTR(class_name.as_ptr()),
                PCWSTR(until::to_wstr(&attr.title).as_ptr()),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                attr.position[0], attr.position[1],
                attr.inner_size.width as i32, attr.inner_size.height as i32,
                None,
                None,
                Some(HINSTANCE::from(hinstance)),
                None,
            )
        }?;
        Ok(Win32WindowHandle { hwnd })
    }

    pub fn create_child_window(&mut self, parent: &Arc<WindowType>, attr: &WindowAttribute) -> UiResult<Arc<WindowType>> {
        let handle = Win32Window::create_window(attr)?;
        let window_type = Arc::new(WindowType {
            kind: WindowKind::Win32(handle),
            id: WindowId::unique_id(),
            type_: WindowType::CHILD,
            ime: parent.ime.clone(),
        });
        self.handles.insert(window_type.id, window_type.clone());
        Ok(window_type)
    }

    pub fn run(&mut self) -> (WindowId, WindowEvent) {
        unsafe {
            let mut msg = std::mem::zeroed::<MSG>();
            let ret = GetMessageW(&mut msg, None, 0, 0);
            if ret.0 == 0 { return (self.handles[0].id, WindowEvent::ReqClose); }
            let window = self.handles.iter().find(|x| x.win32().hwnd == msg.hwnd);
            if window.is_none() { return (WindowId(0), WindowEvent::None); }
            let window = window.unwrap();
            match msg.message {
                WM_SIZE => {
                    let width = until::loword(msg.lParam.0 as u32) as u32;
                    let height = until::hiword(msg.lParam.0 as u32) as u32;
                    println!("resize-{}-{}", width, height);
                    (window.id, WindowEvent::Resize(Size { width, height }))
                }
                WM_PAINT => {
                    println!("paint");
                    ValidateRect(Option::from(window.win32().hwnd), None).unwrap();
                    (window.id, WindowEvent::Redraw)
                    // LRESULT(0)
                }
                WM_KEYDOWN => {
                    println!("Key down: {}", msg.wParam.0);
                    println!("1111111111111={:?}", Win32Clipboard {}.get_clipboard_data());
                    (window.id, WindowEvent::KeyPress(Key::Backspace))
                }
                WM_LBUTTONDOWN => {
                    //切换输入法
                    // let h_ime = ImmGetContext(window.win32().hwnd);
                    // let open=ImmGetOpenStatus(h_ime);
                    // ImmSetOpenStatus(h_ime, !open.as_bool());
                    // ImmReleaseContext(window.win32().hwnd, h_ime);
                    let x = until::get_x_lparam(msg.lParam) as f32;
                    let y = until::get_y_lparam(msg.lParam) as f32;
                    (window.id, WindowEvent::MousePress(Pos { x, y }))
                }
                WM_LBUTTONUP => {
                    let x = until::get_x_lparam(msg.lParam) as f32;
                    let y = until::get_y_lparam(msg.lParam) as f32;
                    (window.id, WindowEvent::MouseRelease(Pos { x, y }))
                }
                WM_MOUSEMOVE => {
                    let x = until::get_x_lparam(msg.lParam) as f32;
                    let y = until::get_y_lparam(msg.lParam) as f32;
                    (window.id, WindowEvent::MouseMove(Pos { x, y }))
                }
                REQ_UPDATE => (window.id, WindowEvent::ReqUpdate),
                CREATE_CHILD => (window.id, WindowEvent::CreateChild),
                RE_INIT => {
                    println!("re_init");
                    (window.id, WindowEvent::Reinit)
                }
                IME => {
                    let himc = ImmGetContext(window.win32().hwnd);
                    if msg.lParam.0 == 7168 {
                        let size = ImmGetCompositionStringW(himc, GCS_RESULTSTR, None, 0);
                        if size > 0 {
                            let len = size as usize / 2;
                            let mut buf: Vec<u16> = vec![0; len];
                            ImmGetCompositionStringW(himc, GCS_RESULTSTR, Some(buf.as_mut_ptr() as *mut _), size as u32);
                            let s = String::from_utf16_lossy(&buf);
                            window.ime().ime_commit(s.chars().collect());
                            println!("ime2: {}", s);
                            return (window.id, WindowEvent::IME);
                        }
                    }
                    if msg.lParam.0 == 440 {
                        let size = ImmGetCompositionStringW(himc, GCS_COMPSTR, None, 0);
                        if size > 0 {
                            let len = (size as usize) / 2;
                            let mut buf: Vec<u16> = vec![0; len];
                            ImmGetCompositionStringW(himc, GCS_COMPSTR, Some(buf.as_mut_ptr() as *mut _), size as u32);
                            let s = String::from_utf16_lossy(&buf);
                            println!("ime1: {}", s);
                            window.ime().ime_draw(s.chars().collect());
                            return (window.id, WindowEvent::IME);
                        }
                    }

                    (window.id, WindowEvent::None)
                }
                REQ_CLOSE => {
                    let wid = window.id;
                    let window = self.handles.remove(&wid).unwrap();
                    (window.id, WindowEvent::ReqClose)
                }
                _ => {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageW(&msg);
                    (window.id, WindowEvent::None)
                }
            }
        }
    }

    pub fn show_tray_menu(&self) -> UiResult<()> {
        unsafe {
            if let Some(ref tray) = self.tray {
                let h_menu = CreatePopupMenu()?;
                for menu in &tray.menus {
                    // 添加普通菜单项
                    AppendMenuW(h_menu, MF_STRING, menu.event, PCWSTR(until::to_wstr(&menu.label).as_ptr()))?;
                    if let Some(ref ip) = menu.icon {
                        let h_icon = until::load_tray_icon(ip);
                        let h_bitmap = until::icon_to_bitmap(h_icon, 16, 16)?; // 需要把 HICON 转成 HBITMAP
                        let mut mii = MENUITEMINFOW::default();
                        mii.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
                        mii.fMask = MIIM_BITMAP;
                        mii.hbmpItem = h_bitmap; // HBITMAP 或 HBMMENU_CALLBACK
                        SetMenuItemInfoW(h_menu, menu.event as u32, false, &mii)?;
                    }
                }
                // 获取鼠标位置
                let mut pt = POINT::default();
                GetCursorPos(&mut pt)?;

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
                ).ok()?;
                println!("111111111111");

                DestroyMenu(h_menu)?;
            }
            Ok(())
        }
    }
}


