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
    pub fn add_menu(&mut self, label: impl ToString, icon: Option<String>) -> &mut TrayMenu {
        self.menus.push(TrayMenu {
            id: crate::unique_id_u32(),
            label: label.to_string(),
            icon,
            callback: Box::new(|| {}),
            children: vec![],
        });
        self.menus.last_mut().unwrap()
    }
}

pub struct TrayMenu {
    pub(crate) id: u32,
    pub(crate) label: String,
    pub(crate) icon: Option<String>,
    pub(crate) callback: Box<dyn Fn()>,
    pub(crate) children: Vec<TrayMenu>,
}

impl TrayMenu {
    pub fn set_callback(&mut self, callback: impl Fn() + 'static) {
        self.callback = Box::new(callback);
    }

    pub fn add_child(&mut self, label: impl ToString, icon: Option<String>) -> &mut TrayMenu {
        self.children.push(TrayMenu {
            id: crate::unique_id_u32(),
            label: label.to_string(),
            icon,
            callback: Box::new(|| {}),
            children: vec![],
        });
        self.children.last_mut().unwrap()
    }

    pub fn set_icon(&mut self, icon: impl ToString) {
        self.icon=Some(icon.to_string());
    }
}