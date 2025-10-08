use crate::error::UiResult;
use crate::key::Key;
use crate::window::event::WindowEvent;
use crate::window::ime::IMEData;
use crate::window::win32::{Win32Window, CREATE_CHILD, REQ_UPDATE, RE_INIT, TRAY_ICON};
use crate::window::wino::EventLoopHandle;
use crate::{Color, Pos, Rect, RichTextExt, Size};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, LRESULT, RECT, WPARAM};
use windows::Win32::Graphics::Gdi::{BeginPaint, BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, CreateFontW, CreateSolidBrush, DeleteDC, DeleteObject, DrawTextW, EndPaint, FillRect, GetDC, InvalidateRect, ReleaseDC, SelectObject, SetTextColor, ValidateRect, DT_CENTER, DT_SINGLELINE, DT_VCENTER, FONT_CHARSET, FONT_CLIP_PRECISION, FONT_OUTPUT_PRECISION, FONT_QUALITY, HBITMAP, HDC, HGDIOBJ, PAINTSTRUCT, SRCCOPY};
use windows::Win32::UI::Input::Ime::{ImmGetCompositionStringW, ImmGetContext, ImmReleaseContext, GCS_COMPSTR, GCS_RESULTSTR};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY, VK_BACK, VK_CONTROL, VK_DELETE, VK_DOWN, VK_END, VK_HOME, VK_LEFT, VK_RETURN, VK_RIGHT, VK_UP};
use windows::Win32::UI::WindowsAndMessaging::*;

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

fn paint_text(text: &str, hdc: HDC, ps: PAINTSTRUCT) {
    unsafe { SetTextColor(hdc, COLORREF(0x00_00_00)); } // 黑色
    let font_name = to_wstr("仿宋");
    let hfont = unsafe {
        CreateFontW(
            32,                 // 字体高度（像素）
            0,                  // 宽度（0 = 自动）
            0,                  // 角度
            0,                  // 基线角度
            500,                // 粗细（FW_BOLD = 700）
            0,                  // 斜体 (1 = TRUE)
            0,                  // 下划线
            0,                  // 删除线
            FONT_CHARSET(0),                  // 字体集 (DEFAULT_CHARSET)
            FONT_OUTPUT_PRECISION(0),                  // 输出精度
            FONT_CLIP_PRECISION(0),                  // 剪辑精度
            FONT_QUALITY(0),                  // 输出质量
            0,                  // 字体 pitch & family
            PCWSTR(font_name.as_ptr()), // 字体名称
        )
    };
    // 选择字体进入 HDC
    let old_font = unsafe { SelectObject(hdc, HGDIOBJ::from(hfont)) };
    let mut text = to_wstr(text);
    // DrawTextW 参数：hdc, text, -1 表示以 null 结尾, 矩形: 0,0,width,height -> 这里用 DT_SINGLELINE + center
    unsafe { DrawTextW(hdc, text.as_mut_slice(), &mut ps.rcPaint.clone(), DT_CENTER | DT_VCENTER | DT_SINGLELINE); }
    // 恢复原字体并删除我们创建的字体对象
    unsafe { SelectObject(hdc, old_font); }
    unsafe { DeleteObject(HGDIOBJ::from(hfont)); }
}

pub unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    let app = match unsafe { (GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut Win32Window).as_mut() } {
        None => return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }, //无法定位application,不做任何处理
        Some(app) => app,
    };
    let window = match app.get_window_mut_by_hand(hwnd) {
        None => return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) }, //无法定位当前的window,不做任何处理
        Some(window) => window,
    };
    match msg {
        WM_CLOSE => {
            println!("req quit-{:?}", hwnd);
            app.close_window(hwnd);
        }
        WM_SIZE => {
            println!("resize");
            let width = loword(lparam.0 as u32) as f32;
            let height = hiword(lparam.0 as u32) as f32;
            println!("resize-{}-{}", width, height);
            window.handle_event(WindowEvent::Resize(Size { width, height }));
        }
        TRAY_ICON => {
            match lparam.0 as u32 {
                WM_RBUTTONUP => app.show_tray_menu().unwrap(),
                _ => {}
            }
        }
        WM_COMMAND => {
            if let Some(ref tray) = app.tray {
                let menu = tray.menus.iter().find(|x| x.id == wparam.0 as u32);
                if let Some(menu) = menu { (menu.callback)() }
            }
        }
        // WM_ERASEBKGND => LRESULT(1), // 不擦背景
        WM_IME_STARTCOMPOSITION | WM_IME_ENDCOMPOSITION | WM_IME_COMPOSITION => {
            // let himc = window.win32().himc.read().unwrap();
            let himc = unsafe { ImmGetContext(window.handle().win32().hwnd) };
            println!("ime-----{}", lparam.0);
            if lparam.0 == 7168 || lparam.0 == 2048 {
                let size = unsafe { ImmGetCompositionStringW(himc, GCS_RESULTSTR, None, 0) };
                if size > 0 {
                    let len = size as usize / 2;
                    let mut buf: Vec<u16> = vec![0; len];
                    unsafe { ImmGetCompositionStringW(himc, GCS_RESULTSTR, Some(buf.as_mut_ptr() as *mut _), size as u32) };
                    let s = String::from_utf16_lossy(&buf);
                    window.handle().ime().ime_commit(s.chars().collect());
                    println!("ime2: {}", s);
                    unsafe { ImmReleaseContext(window.handle().win32().hwnd, himc).unwrap() };
                    window.handle_event(WindowEvent::IME(IMEData::Commit(window.handle().ime.ime_done())));
                }
            }
            if lparam.0 == 440 {
                let size = unsafe { ImmGetCompositionStringW(himc, GCS_COMPSTR, None, 0) };
                if size > 0 {
                    let len = (size as usize) / 2;
                    let mut buf: Vec<u16> = vec![0; len];
                    unsafe { ImmGetCompositionStringW(himc, GCS_COMPSTR, Some(buf.as_mut_ptr() as *mut _), size as u32) };
                    let s = String::from_utf16_lossy(&buf);
                    println!("ime1: {}", s);
                    window.handle().ime().ime_draw(s.chars().collect());
                    unsafe { ImmReleaseContext(window.handle().win32().hwnd, himc).unwrap() };
                    window.handle_event(WindowEvent::IME(IMEData::Preedit(window.handle().ime.chars())));
                }
            }
        }
        WM_PAINT => {
            println!("paint");
            if !window.app_ctx.redraw_thread.is_finished() || crate::time_ms() - window.app_ctx.previous_time <= 10 { return LRESULT(0); }
            #[cfg(not(feature = "gpu"))]
            {
                let mut ps = PAINTSTRUCT::default();
                let hdc = BeginPaint(hwnd, &mut ps);
                let mut rect = RECT::default();
                GetClientRect(hwnd, &mut rect);
                // 创建兼容的内存 DC 和位图
                let mem_dc = CreateCompatibleDC(Option::from(hdc));
                let mem_bmp = CreateCompatibleBitmap(hdc, rect.right - rect.left, rect.bottom - rect.top);
                SelectObject(mem_dc, HGDIOBJ::from(mem_bmp));

                // ✅ 填充背景颜色
                let brush = CreateSolidBrush(COLORREF(Color::rgb(240, 240, 240).as_rgb_u32())); // 白色背景
                FillRect(mem_dc, &rect, brush);
                DeleteObject(HGDIOBJ::from(brush));
                window.handle_event(WindowEvent::Redraw(ps, mem_dc));
                BitBlt(
                    hdc,
                    0, 0,
                    rect.right - rect.left,
                    rect.bottom - rect.top,
                    Option::from(mem_dc),
                    0, 0,
                    SRCCOPY,
                );
                DeleteObject(HGDIOBJ::from(mem_bmp));
                DeleteDC(mem_dc);
                EndPaint(hwnd, &ps).unwrap();
            }
            #[cfg(feature = "gpu")]
            window.handle_event(WindowEvent::Redraw)
        }
        WM_KEYDOWN => {
            let ctrl_pressed = (unsafe { GetKeyState(VK_CONTROL.0 as i32) } as u16 & 0x8000) != 0;
            if ctrl_pressed && wparam.0 == 'C' as usize {
                window.handle_event(WindowEvent::KeyPress(Key::CtrlC));
            } else if ctrl_pressed && wparam.0 == 'V' as usize {
                window.handle_event(WindowEvent::KeyPress(Key::CtrlV));
            } else if ctrl_pressed && wparam.0 == 'A' as usize {
                window.handle_event(WindowEvent::KeyPress(Key::CtrlA));
            } else if ctrl_pressed && wparam.0 == 'X' as usize {
                window.handle_event(WindowEvent::KeyPress(Key::CtrlX));
            } else {
                match VIRTUAL_KEY(wparam.0 as u16) {
                    VK_HOME => window.handle_event(WindowEvent::KeyPress(Key::Home)),
                    VK_END => window.handle_event(WindowEvent::KeyPress(Key::End)),
                    VK_RETURN => window.handle_event(WindowEvent::KeyPress(Key::Enter)),
                    VK_LEFT => window.handle_event(WindowEvent::KeyPress(Key::LeftArrow)),
                    VK_UP => window.handle_event(WindowEvent::KeyPress(Key::UpArrow)),
                    VK_DOWN => window.handle_event(WindowEvent::KeyPress(Key::DownArrow)),
                    VK_RIGHT => window.handle_event(WindowEvent::KeyPress(Key::RightArrow)),
                    VK_DELETE => window.handle_event(WindowEvent::KeyPress(Key::Delete)),
                    VK_BACK => window.handle_event(WindowEvent::KeyPress(Key::Backspace)),
                    _ => {}
                }
            }
        }
        WM_KEYUP => {
            match VIRTUAL_KEY(wparam.0 as u16) {
                VK_HOME => window.handle_event(WindowEvent::KeyRelease(Key::Home)),
                VK_END => window.handle_event(WindowEvent::KeyRelease(Key::End)),
                VK_RETURN => window.handle_event(WindowEvent::KeyRelease(Key::Enter)),
                VK_LEFT => window.handle_event(WindowEvent::KeyRelease(Key::LeftArrow)),
                VK_UP => window.handle_event(WindowEvent::KeyRelease(Key::UpArrow)),
                VK_DOWN => window.handle_event(WindowEvent::KeyRelease(Key::DownArrow)),
                VK_RIGHT => window.handle_event(WindowEvent::KeyRelease(Key::RightArrow)),
                VK_DELETE => window.handle_event(WindowEvent::KeyRelease(Key::Delete)),
                VK_BACK => window.handle_event(WindowEvent::KeyRelease(Key::Backspace)),
                _ => {}
            }
        }
        WM_CHAR => {
            if let Some(r) = char::from_u32(wparam.0 as u32) && !r.is_control() {
                println!("Char input: {:?}", r);
                match r {
                    '\r' => window.handle_event(WindowEvent::None),
                    _ => window.handle_event(WindowEvent::KeyRelease(Key::Char(r)))
                }
            }
        }
        WM_LBUTTONDOWN => {
            let x = get_x_lparam(lparam) as f32;
            let y = get_y_lparam(lparam) as f32;
            window.handle_event(WindowEvent::MousePress(Pos { x, y }));
        }
        WM_LBUTTONUP => {
            let x = get_x_lparam(lparam) as f32;
            let y = get_y_lparam(lparam) as f32;
            window.handle_event(WindowEvent::MouseRelease(Pos { x, y }))
        }
        WM_MOUSEMOVE => {
            let x = get_x_lparam(lparam) as f32;
            let y = get_y_lparam(lparam) as f32;
            window.handle_event(WindowEvent::MouseMove((x, y).into()))
        }
        WM_MOUSEWHEEL => {
            let delta = ((wparam.0 >> 16) & 0xFFFF) as i16;
            window.handle_event(WindowEvent::MouseWheel(delta as f32))
        }
        REQ_UPDATE => window.handle_event(WindowEvent::ReqUpdate),
        CREATE_CHILD => {
            if let Some(user_app) = window.app_ctx.context.new_window.take() {
                let handle = window.handle().clone();
                app.create_child_window(&handle, user_app).unwrap();
            }
        }
        WM_ERASEBKGND => return LRESULT(1),
        RE_INIT => {
            println!("re_init");
            window.handle_event(WindowEvent::ReInit)
        }
        _ => return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
    }

    LRESULT(0)
}