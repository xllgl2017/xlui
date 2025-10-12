use std::os::raw::c_uint;

#[derive(Clone, Debug, Default)]
pub enum Key {
    #[default]
    Unknown,
    Backspace,
    Enter,
    Space,
    Home,
    End,
    Delete,
    Char(char),
    LeftArrow,
    RightArrow,
    UpArrow,
    DownArrow,
    LShift,
    RShift,
    CapsLock,
    CtrlC,
    CtrlV,
    CtrlX,
    CtrlA,
}

impl Key {
    pub fn from_c_ulong(keycode: c_uint, buffer: &[i8]) -> Key {
        println!("key-{}", keycode);
        match keycode {
            22 => Key::Backspace,
            36 => Key::Enter,
            50 => Key::LShift,
            62 => Key::RShift,
            65 => Key::Space,
            66 => Key::CapsLock,
            104 => Key::Enter,
            119 => Key::Delete,
            110 => Key::Home,
            111 => Key::UpArrow,
            113 => Key::LeftArrow,
            114 => Key::RightArrow,
            115 => Key::End,
            116 => Key::DownArrow,
            _ => {
                if buffer.len() == 0 { return Key::Unknown; }
                let slice = buffer.iter().map(|x| *x as u8).collect::<Vec<_>>();
                let char = String::from_utf8(slice).unwrap();
                Key::Char(char.chars().next().unwrap())
            }
        }
    }
}