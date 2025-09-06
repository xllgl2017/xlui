use crate::error::UiResult;
use raw_window_handle::{DisplayHandle, RawDisplayHandle, RawWindowHandle, WindowHandle, WindowsDisplayHandle};
use std::num::NonZeroIsize;
use windows::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use windows::Win32::UI::Input::Ime::{ImmGetContext, ImmReleaseContext, ImmSetCompositionWindow, CFS_POINT, COMPOSITIONFORM, HIMC};
use windows::Win32::UI::WindowsAndMessaging::{CloseWindow, GetWindowLongPtrW, PostMessageW, ShowWindow, GWLP_HINSTANCE, SW_HIDE, SW_SHOW, WM_PAINT};

pub struct Win32WindowHandle {
    pub(crate) hwnd: HWND,
    // pub(crate) ime: HIMC,
}
impl Win32WindowHandle {
    // pub fn request_ime(&self) -> UiResult<()> {
    //
    //     let mut ime = self.ime.write()?;
    //     *ime = Some(himc);
    //     Ok(())
    // }

    pub fn set_ime_position(&self, x: f32, y: f32) -> UiResult<()> {
        let himc = unsafe { ImmGetContext(self.hwnd) };
        let comp_form = COMPOSITIONFORM {
            dwStyle: CFS_POINT,
            ptCurrentPos: POINT { x: x as i32, y: y as i32 },
            rcArea: Default::default(),
        };
        unsafe { ImmSetCompositionWindow(himc, &comp_form).ok()?; }
        unsafe { ImmReleaseContext(self.hwnd, himc).ok()? };
        Ok(())
    }


    pub fn set_visible(&self, visible: bool) -> UiResult<()> {
        match visible {
            true => unsafe { ShowWindow(self.hwnd, SW_SHOW).ok()?; },
            false => unsafe { ShowWindow(self.hwnd, SW_HIDE).ok()?; },
        }
        Ok(())
    }

    pub fn request_redraw(&self) -> UiResult<()> {
        unsafe { PostMessageW(Option::from(self.hwnd), WM_PAINT, WPARAM(0), LPARAM(0))?; }
        Ok(())
    }

    pub fn window_handle(&self) -> WindowHandle {
        let hwnd_nz = NonZeroIsize::new(self.hwnd.0 as isize).unwrap();
        let mut win32_window_handle = raw_window_handle::Win32WindowHandle::new(hwnd_nz);
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

unsafe impl Sync for Win32WindowHandle {}

unsafe impl Send for Win32WindowHandle {}

impl Drop for Win32WindowHandle {
    fn drop(&mut self) {
        unsafe { CloseWindow(self.hwnd).unwrap(); }
    }
}