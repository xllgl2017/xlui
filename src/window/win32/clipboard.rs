use crate::error::UiResult;
use crate::window::win32::until;
use crate::window::ClipboardData;
use windows::Win32::Foundation::{HANDLE, HGLOBAL};
use windows::Win32::System::DataExchange::*;
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalUnlock, GMEM_MOVEABLE};
use windows::Win32::System::Ole::*;

pub struct Win32Clipboard;

impl Win32Clipboard {
    pub fn get_clipboard_data(&self) -> UiResult<ClipboardData> {
        unsafe { OpenClipboard(None)? };
        let clipboard_data = unsafe { GetClipboardData(CF_UNICODETEXT.0 as u32)? };
        if clipboard_data.is_invalid() { return Err("获取粘贴板数据失败".into()); }
        let ptr = unsafe { GlobalLock(HGLOBAL(clipboard_data.0)) };
        if ptr.is_null() { return Err("获取粘贴板失败".into()); }
        // let size = unsafe { GlobalSize(HGLOBAL(clipboard_data.0)) };
        let text = {
            let mut len = 0;
            unsafe {
                while *((ptr as *const u16).add(len)) != 0 {
                    len += 1;
                }
            }
            let slice = unsafe { std::slice::from_raw_parts(ptr as *const u16, len) };
            String::from_utf16_lossy(slice)
        };
        unsafe { GlobalUnlock(HGLOBAL(clipboard_data.0))?; }
        unsafe { CloseClipboard()? };


        Ok(ClipboardData::Text(text))
    }

    pub fn set_clipboard_data(&self, data: ClipboardData) -> UiResult<()> {
        unsafe { OpenClipboard(None)? };
        unsafe { EmptyClipboard()?; }
        match data {
            ClipboardData::Unsupported => {}
            ClipboardData::Text(data) => {
                let data = until::to_wstr(data.as_str());
                let size = data.len() * size_of::<u16>();
                let h_mem = unsafe { GlobalAlloc(GMEM_MOVEABLE, size)? };
                let ptr = unsafe { GlobalLock(h_mem) } as *mut u8;
                unsafe { std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, ptr, size) };
                unsafe { GlobalUnlock(h_mem)? };
                unsafe { SetClipboardData(CF_UNICODETEXT.0 as u32, Some(HANDLE(h_mem.0)))?; }
                unsafe { CloseClipboard()? };
            }
            ClipboardData::Image(_) => {}
            ClipboardData::Url(_) => {}
        }
        Ok(())
    }
}