use std::ptr::null_mut;
use std::sync::mpsc::{channel, Sender, Receiver};
use windows::core::w;
use windows::Win32::Foundation::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::UI::Input::Ime::*;

#[derive(Debug)]
enum WindowEvent {
    ImeCommit(String),
    ImeComposition(String),
    Close,
    None,
}

unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_CLOSE => {
            PostQuitMessage(0);
            return LRESULT(0);
        }

        // 输入法组合消息
        WM_IME_COMPOSITION => {
            let himc = ImmGetContext(hwnd);
            if !himc.is_invalid() {
                let lparam_flags = lparam.0 as u32;

                // 获取最终确认的字符串
                if (lparam_flags & GCS_RESULTSTR.0) != 0 {
                    let size = ImmGetCompositionStringW(himc, GCS_RESULTSTR, None, 0);
                    if size > 0 {
                        let len = (size / 2) as usize;
                        let mut buf: Vec<u16> = vec![0; len];
                        ImmGetCompositionStringW(
                            himc,
                            GCS_RESULTSTR,
                            Some(buf.as_mut_ptr() as *mut _),
                            size as u32,
                        );
                        let s = String::from_utf16_lossy(&buf);
                        println!("mouse result: {}", s);

                        // 发送事件到 run 循环
                        let tx = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Sender<WindowEvent>;
                        if !tx.is_null() {
                            (*tx).send(WindowEvent::ImeCommit(s)).ok();
                        }
                    }
                }

                // 获取正在输入的字符串
                if (lparam_flags & GCS_COMPSTR.0) != 0 {
                    let size = ImmGetCompositionStringW(himc, GCS_COMPSTR, None, 0);
                    if size > 0 {
                        let len = (size / 2) as usize;
                        let mut buf: Vec<u16> = vec![0; len];
                        ImmGetCompositionStringW(
                            himc,
                            GCS_COMPSTR,
                            Some(buf.as_mut_ptr() as *mut _),
                            size as u32,
                        );
                        let s = String::from_utf16_lossy(&buf);

                        let tx = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Sender<WindowEvent>;
                        if !tx.is_null() {
                            (*tx).send(WindowEvent::ImeComposition(s)).ok();
                        }
                    }
                }

                ImmReleaseContext(hwnd, himc).ok();
            }
            return LRESULT(0); // 已处理
        }

        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

fn main() {
    unsafe {
        // 创建消息通道
        let (tx, rx): (Sender<WindowEvent>, Receiver<WindowEvent>) = channel();

        // 注册窗口类
        let hinstance = GetModuleHandleW(None).unwrap();
        let class_name = w!("MyWindowClass");

        let wc = WNDCLASSW {
            lpfnWndProc: Some(wndproc),
            hInstance: HINSTANCE::from(hinstance),
            lpszClassName: class_name,
            ..Default::default()
        };
        RegisterClassW(&wc);

        // 创建窗口
        let hwnd = CreateWindowExW(
            Default::default(),
            class_name,
            w!("IME Test"),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            Some(HINSTANCE::from(hinstance)),
            None,
        ).unwrap();

        // 把 tx 存在 GWLP_USERDATA，WndProc 里可以取出来
        SetWindowLongPtrW(hwnd, GWLP_USERDATA, &tx as *const _ as isize);

        // ✅ run 循环
        loop {
            let mut msg = MSG::default();
            while PeekMessageW(&mut msg, None, 0, 0, PM_REMOVE).into() {
                if msg.message == WM_QUIT {
                    return;
                }
                match msg.message {

                    _ => {
                        TranslateMessage(&msg);
                        DispatchMessageW(&msg);
                    }
                }

            }

            // 处理从 WndProc 传来的事件
            while let Ok(event) = rx.try_recv() {
                match event {
                    WindowEvent::ImeCommit(s) => {
                        println!("IME Commit: {}", s);
                    }
                    WindowEvent::ImeComposition(s) => {
                        println!("IME Composition: {}", s);
                    }
                    WindowEvent::Close => return,
                    WindowEvent::None => {}
                }
            }
        }
    }
}
