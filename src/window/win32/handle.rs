use crate::error::UiResult;
use crate::render::image::{load_win32_image_raw, ImageSource};
use crate::text::cchar::{CChar, LineChar};
use crate::window::win32::clipboard::Win32Clipboard;
use crate::window::win32::{until, CREATE_CHILD, REQ_UPDATE, RE_INIT, USER_UPDATE};
use crate::window::UserEvent;
use crate::{Border, Color, Radius, Rect, RichText, Size};
#[cfg(feature = "gpu")]
use raw_window_handle::{DisplayHandle, RawDisplayHandle, RawWindowHandle, WindowHandle, WindowsDisplayHandle};
#[cfg(feature = "gpu")]
use std::num::NonZeroIsize;
use std::ptr::null_mut;
use std::sync::RwLock;
use windows::core::PCWSTR;
use windows::Win32::Foundation::{COLORREF, HWND, LPARAM, POINT, WPARAM};
use windows::Win32::Graphics::Gdi::{BitBlt, CreateCompatibleDC, CreateDIBSection, CreateFontW, DeleteDC, DeleteObject, DrawTextW, GetCharWidth32W, InvalidateRect, SelectObject, SetBkMode, SetTextColor, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, DT_LEFT, DT_SINGLELINE, DT_TOP, FONT_CHARSET, FONT_CLIP_PRECISION, FONT_OUTPUT_PRECISION, FONT_QUALITY, HBITMAP, HDC, HFONT, HGDIOBJ, SRCCOPY, TRANSPARENT};
use windows::Win32::Graphics::GdiPlus::{FillModeAlternate, GdipAddPathArc, GdipAddPathLine, GdipCreateFromHDC, GdipCreatePath, GdipCreatePen1, GdipCreateSolidFill, GdipDeleteBrush, GdipDeleteGraphics, GdipDeletePath, GdipDeletePen, GdipDrawEllipse, GdipDrawPath, GdipDrawPolygon, GdipFillEllipse, GdipFillPath, GdipFillPolygon, GdipSetSmoothingMode, GpGraphics, GpPath, GpPen, GpSolidFill, PointF, SmoothingModeAntiAlias, SmoothingModeAntiAlias8x8, SmoothingModeHighQuality, UnitPixel};
use windows::Win32::Graphics::Imaging::{GUID_WICPixelFormat32bppPBGRA, WICBitmapDitherTypeNone, WICBitmapInterpolationModeFant, WICBitmapPaletteTypeCustom};
use windows::Win32::UI::Input::Ime::{ImmGetContext, ImmReleaseContext, ImmSetCompositionWindow, CFS_POINT, COMPOSITIONFORM};
use windows::Win32::UI::WindowsAndMessaging::{DestroyWindow, GetWindowLongPtrW, PostMessageW, ShowWindow, GWLP_HINSTANCE, SW_HIDE, SW_SHOW, WM_PAINT};

pub struct Win32WindowHandle {
    pub(crate) hwnd: HWND,
    pub(crate) clipboard: Win32Clipboard,
    pub(crate) size: RwLock<Size>,
}
impl Win32WindowHandle {
    pub fn set_ime_position(&self, x: f32, y: f32, _: f32) -> UiResult<()> {
        let himc = unsafe { ImmGetContext(self.hwnd) };
        let mut cf = COMPOSITIONFORM::default();
        cf.dwStyle = CFS_POINT;
        cf.ptCurrentPos = POINT { x: x as i32, y: y as i32 };
        unsafe { ImmSetCompositionWindow(himc, &cf).ok()? }
        unsafe { ImmReleaseContext(self.hwnd, himc).ok()? };
        Ok(())
    }

    pub fn send_update(&self, event: UserEvent) {
        let event = match event {
            UserEvent::ReqUpdate => REQ_UPDATE,
            UserEvent::CreateChild => CREATE_CHILD,
            UserEvent::ReInit => RE_INIT,
            UserEvent::UserUpdate => USER_UPDATE,
        };
        unsafe { PostMessageW(Some(self.hwnd), event, WPARAM(0), LPARAM(0)).unwrap() }
    }


    pub fn set_visible(&self, visible: bool) -> UiResult<()> {
        match visible {
            true => unsafe { ShowWindow(self.hwnd, SW_SHOW).ok()?; },
            false => unsafe { ShowWindow(self.hwnd, SW_HIDE).ok()?; },
        }
        Ok(())
    }

    pub fn request_redraw(&self) -> UiResult<()> {
        #[cfg(not(feature = "wgpu"))]
        unsafe { InvalidateRect(Option::from(self.hwnd), None, true).unwrap() };
        unsafe { PostMessageW(Option::from(self.hwnd), WM_PAINT, WPARAM(0), LPARAM(0))?; }
        Ok(())
    }

    #[cfg(feature = "gpu")]
    pub fn window_handle(&self) -> WindowHandle<'_> {
        let hwnd_nz = NonZeroIsize::new(self.hwnd.0 as isize).unwrap();
        let mut win32_window_handle = raw_window_handle::Win32WindowHandle::new(hwnd_nz);
        let hinst = unsafe { GetWindowLongPtrW(self.hwnd, GWLP_HINSTANCE) };
        if let Some(nz) = NonZeroIsize::new(hinst) {
            win32_window_handle.hinstance = Some(nz);
        }

        let raw_window_handle = RawWindowHandle::Win32(win32_window_handle);
        unsafe { WindowHandle::borrow_raw(raw_window_handle) }
    }

    #[cfg(feature = "gpu")]
    pub fn display_handle(&self) -> DisplayHandle<'_> {
        let win32_display_handle = WindowsDisplayHandle::new();
        let raw_display_handle = RawDisplayHandle::Windows(win32_display_handle);
        unsafe { DisplayHandle::borrow_raw(raw_display_handle) }
    }

    #[cfg(not(feature = "gpu"))]
    pub fn paint_text(&self, hdc: HDC, text: &RichText, rect: Rect) {
        unsafe {
            SetBkMode(hdc, TRANSPARENT);

            SetTextColor(hdc, COLORREF(text.color.as_rgb_u32())); //字体颜色
            let hfont = self.create_font(text.height as i32, text.family.as_ref().unwrap());
            // 选择字体进入 HDC
            let old_font = SelectObject(hdc, HGDIOBJ::from(hfont));
            let mut text = until::to_wstr(&text.text);
            // DrawTextW 参数：hdc, text, -1 表示以 null 结尾, 矩形: 0,0,width,height -> 这里用 DT_SINGLELINE + center
            DrawTextW(hdc, text.as_mut_slice(), &mut rect.as_win32_rect(), DT_SINGLELINE | DT_TOP | DT_LEFT);
            // 恢复原字体并删除我们创建的字体对象
            SelectObject(hdc, old_font);
            DeleteObject(HGDIOBJ::from(hfont));
        }
    }

    fn add_round_rect_path(path: &mut GpPath, rect: &Rect, radius: &Radius) {
        unsafe {
            let x = rect.dx().min;
            let y = rect.dy().min;
            let w = rect.width();
            let h = rect.height();

            // top-left arc
            if radius.left_top > 0 {
                GdipAddPathArc(path, x, y, radius.left_top as f32 * 2.0, radius.left_top as f32 * 2.0, 180.0, 90.0);
            } else {
                GdipAddPathLine(path, x, y, x, y);
            }
            // top edge
            GdipAddPathLine(path, x + radius.left_top as f32, y, x + w - radius.right_top as f32, y);
            // top-right arc
            if radius.right_top as f32 > 0.0 {
                GdipAddPathArc(path, x + w - 2.0 * radius.right_top as f32, y, radius.right_top as f32 * 2.0, radius.right_top as f32 * 2.0, 270.0, 90.0);
            }

            // right edge
            GdipAddPathLine(path, x + w, y + radius.right_top as f32, x + w, y + h - radius.right_bottom as f32);
            // bottom-right arc
            if radius.right_bottom as f32 > 0.0 {
                GdipAddPathArc(path, x + w - 2.0 * radius.right_bottom as f32, y + h - 2.0 * radius.right_bottom as f32, radius.right_bottom as f32 * 2.0,
                               radius.right_bottom as f32 * 2.0, 0.0, 90.0);
            }

            // bottom edge
            GdipAddPathLine(path, x + w - radius.right_bottom as f32, y + h, x + radius.left_bottom as f32, y + h);

            // bottom-left arc
            if radius.left_bottom as f32 > 0.0 {
                GdipAddPathArc(path, x, y + h - 2.0 * radius.left_bottom as f32, radius.left_bottom as f32 * 2.0,
                               radius.left_bottom as f32 * 2.0, 90.0, 90.0);
            }

            // left edge
            GdipAddPathLine(path, x, y + h - radius.left_bottom as f32, x, y + radius.left_top as f32);
        }
    }

    pub fn paint_rect(&self, hdc: HDC, fill: &Color, border: &Border, rect: &Rect) {
        unsafe {
            let mut graphics: *mut GpGraphics = null_mut();
            GdipCreateFromHDC(hdc, &mut graphics);
            GdipSetSmoothingMode(graphics, SmoothingModeAntiAlias8x8);

            let mut pen: *mut GpPen = null_mut();
            GdipCreatePen1(border.color.as_rgba_u32(), border.top_width, UnitPixel, &mut pen); // 红色边框

            let mut brush: *mut GpSolidFill = null_mut();
            GdipCreateSolidFill(fill.as_rgba_u32(), &mut brush); // 青色填充

            // 创建路径
            let mut path: *mut GpPath = null_mut();
            GdipCreatePath(FillModeAlternate, &mut path);

            Self::add_round_rect_path(&mut *path, rect, &border.radius);

            // 填充 + 描边
            GdipFillPath(graphics, brush.cast(), path);
            if border.top_width != 0.0 {
                // GdipSetSmoothingMode(graphics, SmoothingModeNone);
                GdipDrawPath(graphics, pen, path);
            }

            // 清理资源
            GdipDeletePath(path);
            GdipDeletePen(pen);
            GdipDeleteBrush(brush.cast());
            GdipDeleteGraphics(graphics);
        }
    }

    pub fn paint_circle(&self, hdc: HDC, rect: &Rect, fill: &Color, border: &Border) {
        unsafe {
            // 创建 Graphics 对象
            let mut graphics: *mut GpGraphics = null_mut();
            GdipCreateFromHDC(hdc, &mut graphics);

            GdipSetSmoothingMode(graphics, SmoothingModeAntiAlias);

            // 创建填充刷（支持 alpha）
            let fill_color = (fill.a as u32) << 24 | (fill.r as u32) << 16 | (fill.g as u32) << 8 | (fill.b as u32);
            let mut brush: *mut GpSolidFill = null_mut();
            GdipCreateSolidFill(fill.as_rgba_u32(), &mut brush);

            // 绘制圆形（支持透明度）
            GdipFillEllipse(graphics, brush.cast(), rect.dx().min, rect.dy().min, rect.width(), rect.height());

            // 边框
            if border.width() > 0.0 {
                let mut pen: *mut GpPen = null_mut();
                GdipCreatePen1(border.color.as_rgba_u32(), border.width(), UnitPixel, &mut pen);
                GdipDrawEllipse(graphics, pen, rect.dx().min, rect.dy().min, rect.width(), rect.height());
                GdipDeletePen(pen);
            }


            // 清理
            GdipDeleteBrush(brush as _);
            GdipDeleteGraphics(graphics);
        }
    }

    pub fn size(&self) -> Size { self.size.read().unwrap().clone() }

    fn create_font(&self, height: i32, family: &str) -> HFONT {
        let font_name = until::to_wstr(family);
        // 创建字体
        let hfont = unsafe {
            CreateFontW(
                height,                 // 字体高度（像素）
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
        hfont
    }

    pub fn measure_char_widths(&self, text: &RichText) -> Vec<LineChar> {
        unsafe {
            // 创建内存 DC（不依赖窗口）
            let hdc = CreateCompatibleDC(None);
            let hfont = self.create_font(text.height as i32, text.family.as_ref().unwrap());
            let old_font = SelectObject(hdc, HGDIOBJ::from(hfont));
            let lines = text.text.replace("\r\n", "\n");;
            let mut res = vec![];
            for line in lines.split("\n") {
                let mut wtext = until::to_wstr(line);
                wtext.remove(wtext.len() - 1); //把\0删除
                let mut line = LineChar::new();
                // 逐字符测量
                for &ch in &wtext {
                    let mut w = 0i32;
                    GetCharWidth32W(hdc, ch as u32, ch as u32, &mut w);
                    line.push(CChar::new(char::from_u32(ch as u32).unwrap_or(' '), w as f32));
                }
                res.push(line);
            }
            // 转 UTF-16


            // 清理
            SelectObject(hdc, old_font);
            DeleteObject(HGDIOBJ::from(hfont));
            DeleteDC(hdc);

            res
        }
    }

    pub fn paint_image(&self, hdc: HDC, source: &ImageSource, rect: Rect) {
        unsafe {
            let (factory, frame) = load_win32_image_raw(&source).unwrap();
            let scaler = factory.CreateBitmapScaler().unwrap();
            scaler.Initialize(&frame, rect.width() as u32, rect.height() as u32, WICBitmapInterpolationModeFant);

            // 转换为 32bpp BGRA
            let mut format_converter = factory.CreateFormatConverter().unwrap();

            format_converter.Initialize(
                &scaler,
                &GUID_WICPixelFormat32bppPBGRA,
                WICBitmapDitherTypeNone,
                None,
                0.0,
                WICBitmapPaletteTypeCustom,
            ).unwrap();
            let mut width = 0;
            let mut height = 0;
            unsafe { format_converter.GetSize(&mut width, &mut height).unwrap(); }

            let mut hbitmap: HBITMAP = HBITMAP::default();
            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width as i32,
                    biHeight: -(height as i32), // top-down
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0,
                    ..Default::default()
                },
                ..Default::default()
            };
            let mut bits: *mut std::ffi::c_void = null_mut();
            hbitmap = CreateDIBSection(Some(hdc), &bmi, DIB_RGB_COLORS, &mut bits, None, 0).unwrap();
            let stride = (width as f32 * 4.0) as usize;
            let buf_size = stride * height as usize;
            let buffer_slice = std::slice::from_raw_parts_mut(bits as *mut u8, buf_size);


            // 将 WIC 图像写入 HBITMAP
            format_converter.CopyPixels(null_mut(), stride as u32, buffer_slice).unwrap();

            // 绘制到窗口
            let hdc_mem = CreateCompatibleDC(Option::from(hdc));
            let old_bmp = SelectObject(hdc_mem, HGDIOBJ::from(hbitmap));
            BitBlt(hdc, rect.dx().min as i32, rect.dy().min as i32, width as i32, height as i32, Option::from(hdc_mem), 0, 0, SRCCOPY);
            SelectObject(hdc_mem, old_bmp);
            DeleteDC(hdc_mem);
            DeleteObject(HGDIOBJ::from(hbitmap));
        }
    }

    pub fn paint_triangle(&self, hdc: HDC, points: [PointF; 3], fill: &Color, border: &Border) {
        unsafe {
            // 创建 Graphics 对象
            let mut graphics: *mut GpGraphics = null_mut();
            GdipCreateFromHDC(hdc, &mut graphics);
            // 启用抗锯齿
            GdipSetSmoothingMode(graphics, SmoothingModeAntiAlias);

            // === 填充 ===
            let mut brush: *mut GpSolidFill = null_mut();
            GdipCreateSolidFill(fill.as_rgba_u32(), &mut brush);
            GdipFillPolygon(graphics, brush.cast(), points.as_ptr(), 3, FillModeAlternate);
            GdipDeleteBrush(brush.cast());

            // === 边框 ===
            if border.width() > 0.0 {
                let mut pen: *mut GpPen = null_mut();
                GdipCreatePen1(border.color.as_rgba_u32(), border.width(), UnitPixel, &mut pen);
                GdipDrawPolygon(graphics, pen, points.as_ptr(), 3);
                GdipDeletePen(pen);
            }
        }
    }
}

unsafe impl Sync for Win32WindowHandle {}

unsafe impl Send for Win32WindowHandle {}

impl Drop for Win32WindowHandle {
    fn drop(&mut self) {
        unsafe { DestroyWindow(self.hwnd).unwrap(); }
    }
}