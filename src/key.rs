use std::ffi::c_ulong;

#[derive(Clone, Debug)]
pub(crate) enum Key {
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
}

impl Key {
    pub fn from_c_ulong(l: c_ulong) -> Key {
        match l {
            65288 => Key::Backspace,
            65293 => Key::Enter,
            32 => Key::Space,
            65367 => Key::End,
            65360 => Key::Home,
            65361 => Key::LeftArrow,
            65362 => Key::UpArrow,
            65363 => Key::RightArrow,
            65364 => Key::DownArrow,
            65535 => Key::Delete,
            _ => Key::Char(char::from(l as u8)),
        }
    }
}