use crate::error::UiResult;
use crate::map::Map;
use crate::window::ime::IME;
use crate::window::win32::clipboard::Win32Clipboard;
use crate::window::win32::handle::Win32WindowHandle;
use crate::window::win32::tray::Tray;
use crate::window::wino::{EventLoopHandle, LoopWindow};
use crate::window::{WindowId, WindowKind, WindowType};
use crate::{App, TrayMenu, WindowAttribute};
use std::ops::Index;
use std::process::exit;
use std::sync::Arc;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{HINSTANCE, HWND, POINT};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::{Shell_NotifyIconW, NIF_ICON, NIF_MESSAGE, NIF_TIP, NIM_ADD, NOTIFYICONDATAW};
use windows::Win32::UI::WindowsAndMessaging::*;
use crate::window::event::WindowEvent;

pub mod tray;
pub(crate) mod handle;
pub(crate) mod until;
mod clipboard;

const TRAY_ICON: u32 = WM_USER + 1;
const REQ_UPDATE: u32 = WM_USER + 2;
const CREATE_CHILD: u32 = WM_USER + 3;
const RE_INIT: u32 = WM_USER + 4;
const IME: u32 = WM_USER + 5;
const REQ_CLOSE: u32 = WM_USER + 6;
const USER_UPDATE: u32 = WM_USER + 7;
const RESIZE: u32 = WM_USER + 8;


pub struct Win32Window {
    tray: Option<Tray>,
    windows: Map<WindowId, LoopWindow>,
}


impl Win32Window {
    pub fn new<A: App>(app: A) -> UiResult<Win32Window> {
        let mut attr = app.window_attributes();
        let handle = Win32Window::create_window(&attr)?;
        let window_type = WindowType {
            kind: WindowKind::Win32(handle),
            id: WindowId::unique_id(),
            type_: WindowType::ROOT,
            ime: Arc::new(IME::new_win32()),
        };
        let app = Box::new(app);
        let tray = attr.tray.take();
        let mut window = pollster::block_on(async { LoopWindow::create_window(app, Arc::new(window_type), attr).await });
        window.handle_event(WindowEvent::Redraw);
        let mut windows = Map::new();
        windows.insert(window.window_id(), window);
        let window = Win32Window {
            tray,
            windows,
        };
        window.show_tray()?;
        Ok(window)
    }

    pub fn get_window_by_index(&self, index: usize) -> &LoopWindow {
        self.windows.index(index)
    }

    pub fn get_window_mut_by_hand(&mut self, hwnd: HWND) -> Option<&mut LoopWindow> {
        self.windows.iter_mut().find(|x| x.handle().win32().hwnd == hwnd)
    }

    pub fn close_window(&mut self, hwnd: HWND) -> Option<LoopWindow> {
        let wid = self.get_window_mut_by_hand(hwnd)?.window_id();
        let window = self.windows.remove(&wid);
        if self.windows.len() == 0 { exit(0); }
        window
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
                hWnd: self.windows[0].handle().win32().hwnd,
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
        Ok(Win32WindowHandle { hwnd, clipboard: Win32Clipboard })
    }

    pub fn create_child_window(&mut self, parent: &Arc<WindowType>, app: Box<dyn App>) -> UiResult<()> {
        let attr = app.window_attributes();
        let handle = Win32Window::create_window(&attr)?;
        let window_type = Arc::new(WindowType {
            kind: WindowKind::Win32(handle),
            id: WindowId::unique_id(),
            type_: WindowType::CHILD,
            ime: parent.ime.clone(),
        });
        let windows = pollster::block_on(async { LoopWindow::create_window(app, window_type, attr).await });
        self.windows.insert(windows.window_id(), windows);
        Ok(())
    }

    pub fn run(&mut self) -> UiResult<()> {
        loop {
            unsafe {
                let mut msg = std::mem::zeroed::<MSG>();
                let ret = GetMessageW(&mut msg, None, 0, 0);
                if ret.0 == 0 { return Err("get message event error".into()); }
                let _ = TranslateMessage(&msg);
                DispatchMessageW(&msg);
            }
        }
    }

    fn add_tray_menu(&self, h_menu: HMENU, id: u32, menu: &TrayMenu, flag: MENU_ITEM_FLAGS) -> UiResult<()> {
        unsafe {
            AppendMenuW(h_menu, flag, id as usize, PCWSTR(until::to_wstr(&menu.label).as_ptr()))?;
            if let Some(ref ip) = menu.icon {
                let h_icon = until::load_tray_icon(ip);
                let h_bitmap = until::icon_to_bitmap(h_icon, 16, 16)?; // 需要把 HICON 转成 HBITMAP
                let mut mii = MENUITEMINFOW::default();
                mii.cbSize = size_of::<MENUITEMINFOW>() as u32;
                mii.fMask = MIIM_BITMAP;
                mii.hbmpItem = h_bitmap; // HBITMAP 或 HBMMENU_CALLBACK
                SetMenuItemInfoW(h_menu, id, false, &mii)?;
            }
        }
        Ok(())
    }

    pub fn show_tray_menu(&self) -> UiResult<()> {
        unsafe {
            if let Some(ref tray) = self.tray {
                let h_menu = CreatePopupMenu()?;
                for menu in &tray.menus {
                    // 添加普通菜单项
                    if menu.children.len() == 0 {
                        self.add_tray_menu(h_menu, menu.id, menu, MF_STRING)?;
                    } else {
                        let sub_menu = CreatePopupMenu()?;
                        for child in &menu.children {
                            self.add_tray_menu(sub_menu, child.id, child, MF_STRING)?;
                        }
                        self.add_tray_menu(h_menu, sub_menu.0 as u32, menu, MF_POPUP)?;
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
                    self.windows[0].handle().win32().hwnd,
                    None,
                ).ok()?;
                println!("111111111111");

                DestroyMenu(h_menu)?;
            }
            Ok(())
        }
    }
}


