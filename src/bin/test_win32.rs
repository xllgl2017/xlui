use std::ptr::null_mut;
use windows::Win32::Foundation::{COLORREF, POINT, RECT};
use windows::Win32::Graphics::Gdi::{CreateFontW, CreatePen, CreateSolidBrush, DeleteObject, Ellipse, GetStockObject, Polygon, SelectObject, FONT_CHARSET, FONT_CLIP_PRECISION, FONT_OUTPUT_PRECISION, FONT_QUALITY, HBRUSH, HDC, HGDIOBJ, PS_SOLID, WHITE_BRUSH};
use windows::Win32::Graphics::GdiPlus::{FillModeAlternate, GdipAddPathArc, GdipAddPathLine, GdipCreateFromHDC, GdipCreatePath, GdipCreatePen1, GdipCreateSolidFill, GdipDeleteBrush, GdipDeleteGraphics, GdipDeletePath, GdipDeletePen, GdipDrawPath, GdipFillPath, GdipSetSmoothingMode, GdiplusStartup, GdiplusStartupInput, GpGraphics, GpPath, GpPen, GpSolidFill, SmoothingModeAntiAlias, UnitPixel};
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::{HINSTANCE, HWND, LPARAM, LRESULT, WPARAM},
        Graphics::Gdi::{
            BeginPaint, DrawTextW, EndPaint, SetTextColor, DT_CENTER, DT_SINGLELINE, DT_VCENTER,
            PAINTSTRUCT,
        },
        UI::WindowsAndMessaging::{
            CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, LoadCursorW, PostQuitMessage,
            RegisterClassW, TranslateMessage, CS_HREDRAW, CS_VREDRAW, CW_USEDEFAULT, IDC_ARROW, MSG, WM_DESTROY,
            WM_PAINT, WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE,
        },
    },
};

fn to_wide_null(s: &str) -> Vec<u16> {
    let mut v: Vec<u16> = s.encode_utf16().collect();
    v.push(0);
    v
}

fn paint_text(text: &str, hdc: HDC, ps: PAINTSTRUCT) {
    unsafe { SetTextColor(hdc, COLORREF(0x00_00_00)); } // 黑色
    let font_name = to_wide_null("仿宋");
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
    let mut text = to_wide_null(text);
    // DrawTextW 参数：hdc, text, -1 表示以 null 结尾, 矩形: 0,0,width,height -> 这里用 DT_SINGLELINE + center
    unsafe { DrawTextW(hdc, text.as_mut_slice(), &mut ps.rcPaint.clone(), DT_CENTER | DT_VCENTER | DT_SINGLELINE); }
    // 恢复原字体并删除我们创建的字体对象
    unsafe { SelectObject(hdc, old_font); }
    unsafe { DeleteObject(HGDIOBJ::from(hfont)); }
}

// 每条边的宽度
struct BorderWidths {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

// 绘制圆角矩形函数
unsafe fn add_round_rect_path(
    path: &mut GpPath,
    rect: &RECT,
    radius_tl: f32,
    radius_tr: f32,
    radius_br: f32,
    radius_bl: f32,
) {
    let x = rect.left as f32;
    let y = rect.top as f32;
    let w = (rect.right - rect.left) as f32;
    let h = (rect.bottom - rect.top) as f32;

    // top-left arc
    if radius_tl > 0.0 {
        GdipAddPathArc(path, x, y, radius_tl * 2.0, radius_tl * 2.0, 180.0, 90.0);
        // path.AddArc(x, y, radius_tl * 2.0, radius_tl * 2.0, 180.0, 90.0);
    } else {
        GdipAddPathLine(path, x, y, x, y + h);
        // path.AddLine(PointF { X: x, Y: y + 0.0 }, PointF { X: x, Y: y });
    }
    GdipAddPathLine(path, x + radius_tl, y, x + w - radius_tr, y);
    // top edge
    // path.AddLine(
    //     PointF {
    //         X: x + radius_tl,
    //         Y: y,
    //     },
    //     PointF {
    //         X: x + w - radius_tr,
    //         Y: y,
    //     },
    // );

    // top-right arc
    if radius_tr > 0.0 {
        GdipAddPathArc(path, x + w - 2.0 * radius_tr, y, radius_tr * 2.0, radius_tr * 2.0, 270.0, 90.0);
        // path.AddArc(
        //     x + w - 2.0 * radius_tr,
        //     y,
        //     radius_tr * 2.0,
        //     radius_tr * 2.0,
        //     270.0,
        //     90.0,
        // );
    }

    // right edge
    GdipAddPathLine(path, x + w, y + radius_tr, x + w, y + h - radius_br);
    // path.AddLine(
    //     PointF {
    //         X: x + w,
    //         Y: y + radius_tr,
    //     },
    //     PointF {
    //         X: x + w,
    //         Y: y + h - radius_br,
    //     },
    // );

    // bottom-right arc
    if radius_br > 0.0 {
        GdipAddPathArc(path,
                       x + w - 2.0 * radius_br,
                       y + h - 2.0 * radius_br,
                       radius_br * 2.0,
                       radius_br * 2.0,
                       0.0,
                       90.0,
        );
    }

    // bottom edge
    GdipAddPathLine(path, x + w - radius_br, y + h, x + radius_bl, y + h);

    // bottom-left arc
    if radius_bl > 0.0 {
        GdipAddPathArc(path,
                       x,
                       y + h - 2.0 * radius_bl,
                       radius_bl * 2.0,
                       radius_bl * 2.0,
                       90.0,
                       90.0,
        );
    }

    // left edge
    GdipAddPathLine(path, x, y + h - radius_bl, x, y + radius_tl);
}

unsafe fn paint_rect(hdc: HDC) {
    let mut graphics: *mut GpGraphics = null_mut();
    GdipCreateFromHDC(hdc, &mut graphics);
    GdipSetSmoothingMode(graphics, SmoothingModeAntiAlias);

    let mut pen: *mut GpPen = null_mut();
    GdipCreatePen1(0xFFFF0000, 1.0, UnitPixel, &mut pen); // 红色边框

    let mut brush: *mut GpSolidFill = null_mut();
    GdipCreateSolidFill(0xFF00FFFF, &mut brush); // 青色填充

    // 创建路径
    let mut path: *mut GpPath = null_mut();
    GdipCreatePath(FillModeAlternate, &mut path);

    // 定义圆角矩形
    let rect = RECT {
        left: 50,
        top: 50,
        right: 300,
        bottom: 150,
    };
    add_round_rect_path(&mut *path, &rect, 8.0, 2.0, 4.0, 6.0);

    // 填充 + 描边
    GdipFillPath(graphics, brush.cast(), path);
    GdipDrawPath(graphics, pen, path);

    // 清理资源
    GdipDeletePath(path);
    GdipDeletePen(pen);
    GdipDeleteBrush(brush.cast());
    GdipDeleteGraphics(graphics);
}

unsafe fn paint_circle(hdc: HDC) {
    let left = 100;
    let top = 50;
    let right = 300;
    let bottom = 250;

    // 创建填充画刷（红色）
    let hbrush = CreateSolidBrush(COLORREF(0xFFFF00));
    let old_brush = SelectObject(hdc, HGDIOBJ::from(hbrush));

    // 创建画笔用于边框（黑色）
    let hpen = CreatePen(PS_SOLID, 4, COLORREF(0xFF000FF));
    let old_pen = SelectObject(hdc, HGDIOBJ::from(hpen));

    // 绘制圆形
    Ellipse(hdc, left, top, right, bottom);

    // 恢复 GDI 对象并释放
    SelectObject(hdc, old_brush);
    DeleteObject(HGDIOBJ::from(hbrush));
    SelectObject(hdc, old_pen);
    DeleteObject(HGDIOBJ::from(hpen));
}

unsafe fn paint_triangle(hdc: HDC) {
    let points = [
        POINT { x: 150, y: 50 },
        POINT { x: 50, y: 200 },
        POINT { x: 250, y: 200 },
    ];

    // 创建红色画刷填充三角形
    let hbrush = CreateSolidBrush(COLORREF(0xFFFF00));
    let old_brush = SelectObject(hdc, HGDIOBJ::from(hbrush));

    // 创建黑色画笔用于边框
    let hpen = CreatePen(PS_SOLID, 3, COLORREF(0xFF000FF));
    let old_pen = SelectObject(hdc, HGDIOBJ::from(hpen));

    Polygon(hdc, &points);

    // 恢复 GDI 对象并释放
    SelectObject(hdc, old_brush);
    DeleteObject(HGDIOBJ::from(hbrush));
    SelectObject(hdc, old_pen);
    DeleteObject(HGDIOBJ::from(hpen));
}


unsafe extern "system" fn wndproc(hwnd: HWND, msg: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    match msg {
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps);
            // paint_rect(hdc);
            // 定义圆的矩形边界
            // 定义三角形的三个点
            paint_triangle(hdc);
            EndPaint(hwnd, &ps);
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}
static mut GDI_PLUS_TOKEN: usize = 0;
fn main() -> windows::core::Result<()> {
    unsafe {
        let mut input = GdiplusStartupInput {
            GdiplusVersion: 1,
            ..Default::default()
        };
        GdiplusStartup(&raw mut GDI_PLUS_TOKEN, &mut input, null_mut());

        let hinstance = HINSTANCE::default();

        let class_name = to_wide_null("my_window_class");
        let wc = WNDCLASSW {
            lpfnWndProc: Some(wndproc),
            hInstance: hinstance,
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hCursor: LoadCursorW(None, IDC_ARROW).unwrap(),
            style: CS_HREDRAW | CS_VREDRAW,
            hbrBackground: HBRUSH(GetStockObject(WHITE_BRUSH).0), // 系统白色背景
            ..Default::default()
        };

        let atom = RegisterClassW(&wc);
        if atom == 0 {
            panic!("RegisterClassW failed");
        }

        let window_name = to_wide_null("windows-rs GDI 文本示例");
        let hwnd = CreateWindowExW(
            Default::default(),
            PCWSTR(class_name.as_ptr()),
            PCWSTR(window_name.as_ptr()),
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            800,
            600,
            None,
            None,
            Some(hinstance),
            None,
        ).unwrap();

        let mut msg = MSG::default();
        while GetMessageW(&mut msg, Some(hwnd), 0, 0).into() {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }

    Ok(())
}
