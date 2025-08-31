use windows::core::{w, PCWSTR};
use windows::Win32::Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, POINT, WPARAM};
use windows::Win32::Graphics::Gdi::{CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, GetDC, ReleaseDC, SelectObject, HBITMAP, HGDIOBJ};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::Shell::*;
use windows::Win32::UI::WindowsAndMessaging::*;

const WM_USER_TRAYICON: u32 = WM_USER + 1;
const ID_TRAY_EXIT: u16 = 1001;

fn to_wstr(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(Some(0)).collect()
}


fn main() {
    unsafe {
        let h_instance = GetModuleHandleW(None).unwrap();
        let class_name = w!("tray_demo_class");

        let wnd_class = WNDCLASSW {
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            hInstance: HINSTANCE::from(h_instance),
            lpszClassName: class_name,
            lpfnWndProc: Some(wnd_proc),
            ..Default::default()
        };
        RegisterClassW(&wnd_class);

        let hwnd = CreateWindowExW(
            Default::default(),
            class_name,
            w!("Tray Demo"),
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            None,
            None,
            Some(HINSTANCE::from(h_instance)),
            None,
        ).unwrap();

        // 加载一个图标（你可以换成自己的）
        let icon_path = to_wstr("C:\\Users\\xl\\Downloads\\aknxx-37a47-001.ico");
        let h_icon = LoadImageW(
            None,
            PCWSTR(icon_path.as_ptr()),
            IMAGE_ICON,
            32,
            32,
            LR_LOADFROMFILE,
        ).unwrap();
        // let h_icon = LoadIconW(None, IDI_APPLICATION).unwrap();
        // let h_icon=LoadIconW()
        let h_icon = HICON(h_icon.0);

        // 配置托盘图标数据
        let mut tip = [0; 128];
        let tip_s = to_wstr("Rust 系统托盘示例");
        tip[..tip_s.len()].copy_from_slice(tip_s.as_ref());
        let mut nid = NOTIFYICONDATAW {
            cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as u32,
            hWnd: hwnd,
            uID: 1,
            uFlags: NIF_MESSAGE | NIF_ICON | NIF_TIP,
            uCallbackMessage: WM_USER_TRAYICON,
            hIcon: h_icon,
            szTip: tip,
            ..Default::default()
        };


        // 添加托盘图标
        Shell_NotifyIconW(NIM_ADD, &mut nid);

        // 消息循环
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }

        // 退出时删除托盘图标
        Shell_NotifyIconW(NIM_DELETE, &mut nid);
    }
}

extern "system" fn wnd_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    unsafe {
        match msg {
            WM_COMMAND => {
                let wm_id = LOWORD(wparam.0 as u32) as u16;
                if wm_id == ID_TRAY_EXIT {
                    // 触发退出
                    DestroyWindow(hwnd);
                }
                LRESULT(0)
            }
            WM_USER_TRAYICON => {
                match lparam.0 as u32 {
                    WM_LBUTTONUP => {
                        MessageBoxW(Option::from(hwnd), w!("左键点击托盘图标"), w!("提示"), MB_OK);
                    }
                    WM_RBUTTONUP => {
                        show_tray_menu(hwnd)
                        // MessageBoxW(Option::from(hwnd), w!("右键点击托盘图标"), w!("提示"), MB_OK);
                    }
                    _ => {}
                }
                LRESULT(0)
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(hwnd, msg, wparam, lparam),
        }
    }
}

const ID_TRAY_SUB_OPTION1: u32 = 3;
const ID_TRAY_SUB_OPTION2: u32 = 4;

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


unsafe fn show_tray_menu(hwnd: HWND) {
    let h_menu = CreatePopupMenu().unwrap();

    // AppendMenuW(h_menu, MF_STRING, ID_TRAY_EXIT as usize, w!("退出"));
    let h_sub_menu = CreatePopupMenu().unwrap();
    AppendMenuW(h_sub_menu, MF_STRING, ID_TRAY_SUB_OPTION1 as usize, w!("子菜单1"));
    AppendMenuW(h_sub_menu, MF_STRING, ID_TRAY_SUB_OPTION2 as usize, w!("子菜单2"));

    // 在主菜单中添加带子菜单的项
    AppendMenuW(h_menu, MF_POPUP, h_sub_menu.0 as usize, w!("设置"));

    // 添加普通菜单项
    AppendMenuW(h_menu, MF_STRING, ID_TRAY_EXIT as usize, w!("退出"));

    let icon_path = to_wstr("C:\\Users\\xl\\Downloads\\aknxx-37a47-001.ico");
    let h_icon = LoadImageW(
        None,
        PCWSTR(icon_path.as_ptr()),
        IMAGE_ICON,
        32,
        32,
        LR_LOADFROMFILE,
    ).unwrap();
    // let h_icon = LoadIconW(None, IDI_APPLICATION).unwrap();
    // let h_icon=LoadIconW()
    let h_icon = HICON(h_icon.0);
    let h_bitmap = icon_to_bitmap(h_icon,16,16); // 需要把 HICON 转成 HBITMAP

    let mut mii = MENUITEMINFOW::default();
    mii.cbSize = std::mem::size_of::<MENUITEMINFOW>() as u32;
    mii.fMask = MIIM_BITMAP;
    mii.hbmpItem = h_bitmap; // HBITMAP 或 HBMMENU_CALLBACK

    SetMenuItemInfoW(h_menu, ID_TRAY_EXIT as u32, false, &mii);

    // 获取鼠标位置
    let mut pt = POINT::default();
    GetCursorPos(&mut pt);

    // 必须先把窗口设为前台，否则菜单可能不会自动消失
    // SetForegroundWindow(hwnd);

    // 显示菜单（右键菜单）
    TrackPopupMenu(
        h_menu,
        TPM_RIGHTBUTTON,
        pt.x,
        pt.y,
        Some(0),
        hwnd,
        None,
    );

    DestroyMenu(h_menu);
}

fn LOWORD(l: u32) -> u16 {
    (l & 0xffff) as u16
}
fn HIWORD(l: u32) -> u16 {
    ((l >> 16) & 0xffff) as u16
}