use crate::error::UiResult;
use crate::window::win32::{CREATE_CHILD, REQ_UPDATE, RE_INIT};
use crate::window::UserEvent;
use raw_window_handle::{DisplayHandle, RawDisplayHandle, RawWindowHandle, WindowHandle, WindowsDisplayHandle};
use std::num::NonZeroIsize;
use windows::Win32::Foundation::{HWND, LPARAM, POINT, WPARAM};
use windows::Win32::UI::Input::Ime::{ImmGetContext, ImmReleaseContext, ImmSetCompositionWindow, CFS_POINT, COMPOSITIONFORM};
use windows::Win32::UI::WindowsAndMessaging::{DestroyWindow, GetWindowLongPtrW, PostMessageW, ShowWindow, GWLP_HINSTANCE, SW_HIDE, SW_SHOW, WM_PAINT};

pub struct Win32WindowHandle {
    pub(crate) hwnd: HWND,
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
            UserEvent::ReInit => RE_INIT
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
        unsafe { PostMessageW(Option::from(self.hwnd), WM_PAINT, WPARAM(0), LPARAM(0))?; }
        Ok(())
    }

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

    pub fn display_handle(&self) -> DisplayHandle<'_> {
        let win32_display_handle = WindowsDisplayHandle::new();
        let raw_display_handle = RawDisplayHandle::Windows(win32_display_handle);
        unsafe { DisplayHandle::borrow_raw(raw_display_handle) }
    }
}

unsafe impl Sync for Win32WindowHandle {}

unsafe impl Send for Win32WindowHandle {}

impl Drop for Win32WindowHandle {
    fn drop(&mut self) {
        unsafe { DestroyWindow(self.hwnd).unwrap(); }
    }
}