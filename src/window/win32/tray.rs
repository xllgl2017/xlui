use windows::Win32::UI::WindowsAndMessaging::WM_USER;

pub struct Tray {
    pub(crate) icon: Option<String>,
    pub(crate) menus: Vec<TrayMenu>,
    pub(crate) hovered_text: String,
}

impl Tray {
    pub fn new() -> Tray {
        Tray {
            icon: None,
            menus: vec![],
            hovered_text: "".to_string(),
        }
    }

    pub fn icon(mut self, icon: impl ToString) -> Tray {
        self.icon = Some(icon.to_string());
        self
    }

    pub fn hovered_text(mut self, text: impl ToString) -> Tray {
        self.hovered_text = text.to_string();
        self
    }
    pub fn add_menu(&mut self, label: impl ToString, icon: Option<String>) {
        self.menus.push(TrayMenu {
            label: label.to_string(),
            icon,
            event: WM_USER as usize + 2 + self.menus.len(),
        })
    }
}

pub(crate) struct TrayMenu {
    pub(crate) label: String,
    pub(crate) icon: Option<String>,
    pub(crate) event: usize,
}