use crate::error::{UiError, UiResult};
use crate::window::x11::ime::context::Context;
use dbus::blocking::Connection;
use dbus::channel::Channel;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::time::Duration;
use std::{env, fs};

pub const DEST: &'static str = "org.freedesktop.IBus";
pub const DBUS_PATH: &'static str = "/org/freedesktop/IBus";

pub struct Bus {
    conn: Arc<Connection>,
    ctx: Context,

}

impl Bus {
    pub fn new(name: &str) -> UiResult<Bus> {
        let addr = Bus::get_address()?;
        let mut channel = Channel::open_private(&addr)?;
        channel.register()?;
        let conn = Arc::new(Connection::from(channel));
        let ctx = Context::init(conn.clone(), name)?;
        Ok(Bus {
            conn,
            ctx,
        })
    }

    pub fn process(&self, timeout: Duration) -> UiResult<bool> {
        let processed = self.conn.process(timeout)?;
        Ok(processed)
    }

    fn get_machine_id() -> UiResult<String> {
        let mid = "/etc/machine-id";
        let mid2 = "/var/lib/dbus/machine-id";
        let machine_id = fs::read_to_string(mid).or_else(|_| fs::read_to_string(mid2))?;
        Ok(machine_id.trim().to_string())
    }


    fn get_address() -> UiResult<String> {
        if let Ok(addr) = env::var("IBUS_ADDRESS") { return Ok(addr); }
        let display = env::var("DISPLAY").unwrap_or(":0.0".to_string());
        let mut displays = display.split(":");
        let mut host = displays.next().ok_or("获取显示主机错误")?.to_string();
        if host == "" { host = "unix".to_string(); }
        let num = displays.next().ok_or("获取显示编号错误")?.split(".").next().ok_or("获取显示编号1错误")?;
        let config_home = env::var("XDG_CONFIG_HOME").or_else(|_| {
            let mut home = env::var("HOME").or(Err("无法获取config路径"))?;
            home += "/.config";
            Ok::<String, UiError>(home)
        })?;
        let machine_id = Bus::get_machine_id()?;
        let addr_fp = format!("{}/ibus/bus/{}-{}-{}", config_home, machine_id, host, num);
        let addr_file = File::open(&addr_fp)?;
        let reader = BufReader::new(addr_file);
        let prefix = "IBUS_ADDRESS=";
        for line in reader.lines() {
            let line = line?;
            match line.trim_start().strip_prefix(prefix) {
                None => continue,
                Some(addr) => return Ok(addr.to_string())
            }
        }
        Err(format!("找不到Dbus addr: {}", prefix).into())
    }

    pub fn ctx(&self) -> &Context {
        &self.ctx
    }

}
