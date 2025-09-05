use crate::error::UiResult;
use dbus::blocking::{Connection, Proxy};
use dbus::channel::Token;
use dbus::{Message, Path};
use std::sync::Arc;
use std::time::Duration;
use crate::window::x11::ime::bus::{DBUS_PATH, DEST};
use crate::window::x11::ime::flag::Modifiers;
use crate::window::x11::ime::signal::{CommitText, UpdatePreeditText};

pub const INTERFACE_NAME: &'static str = "org.freedesktop.IBus.InputContext";


pub struct Context {
    conn: Arc<Connection>,
    obj_path: Path<'static>,
}

impl Context {
    pub fn init(conn: Arc<Connection>, name: &str) -> UiResult<Context> {
        let proxy = conn.with_proxy(DEST, DBUS_PATH, Duration::from_secs(1));
        let (obj_path, ): (Path,) = proxy.method_call(DEST, "CreateInputContext", (name,))?;
        Ok(Context {
            conn,
            obj_path,
        })
    }

    pub fn set_capabilities(&self, caps: u32) -> UiResult<()> {
        let proxy = self.conn.with_proxy(DEST, &self.obj_path, Duration::from_secs(1));
        let () = proxy.method_call(INTERFACE_NAME, "SetCapabilities", (caps,))?;
        Ok(())
    }

    pub fn set_cursor_location(&self, x: i32, y: i32, w: i32, h: i32) -> UiResult<()> {
        let proxy = self.conn.with_proxy(DEST, &self.obj_path, Duration::from_secs(1));
        let () = proxy.method_call(INTERFACE_NAME, "SetCursorLocation", (x, y, w, h))?;
        Ok(())
    }

    pub fn focus_in(&self) -> UiResult<()> {
        let proxy = self.conn.with_proxy(DEST, &self.obj_path, Duration::from_secs(1));
        let () = proxy.method_call(INTERFACE_NAME, "FocusIn", ())?;
        Ok(())
    }

    pub fn focus_out(&self) -> UiResult<()> {
        let proxy = self.conn.with_proxy(DEST, &self.obj_path, Duration::from_secs(1));
        let () = proxy.method_call(INTERFACE_NAME, "FocusOut", ())?;
        Ok(())
    }

    pub fn on_commit_text<F>(&self, mut callback: F) -> UiResult<Token>
    where
        F: FnMut(CommitText, &Connection, &Message) -> bool + Send + 'static,
    {
        let token = self.with_proxy(|p| {
            p.match_signal(move |a: CommitText, b: &Connection, c: &Message| {
                (callback)(a, b, c)
            })
        })?;
        Ok(token)
    }

    pub fn on_update_preedit_text<F>(&self, mut callback: F) -> UiResult<Token>
    where
        F: FnMut(UpdatePreeditText, &Connection, &Message) -> bool + Send + 'static,
    {
        let token = self.with_proxy(|p| {
            p.match_signal(move |a: UpdatePreeditText, b: &Connection, c: &Message| {
                callback(a, b, c)
            })
        })?;
        Ok(token)
    }

    pub fn process_key_event(&self, sym: u32, code: u32, modifiers: Modifiers) -> UiResult<bool> {
        let proxy = self.conn.with_proxy(DEST, &self.obj_path, Duration::from_secs(1));
        let key_args = (sym, code, modifiers as u32);
        let (handled, ): (bool,) = proxy.method_call(INTERFACE_NAME, "ProcessKeyEvent", key_args)?;
        Ok(handled)
    }

    pub fn with_proxy<R, F: FnOnce(Proxy<&Connection>) -> R>(&self, f: F) -> R {
        let proxy = self.conn.with_proxy(DEST, &self.obj_path, Duration::from_secs(1));
        f(proxy)
    }
}