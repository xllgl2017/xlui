use std::ffi::c_ulong;

#[derive(Clone, Debug)]
pub enum Key {
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
            65421 => Key::Enter,
            32 => Key::Space,
            65367 => Key::End,
            65360 => Key::Home,
            65361 => Key::LeftArrow,
            65362 => Key::UpArrow,
            65363 => Key::RightArrow,
            65364 => Key::DownArrow,
            65535 => Key::Delete,
            65436 => Key::Char('1'),
            65433 => Key::Char('2'),
            65435 => Key::Char('3'),
            65430 => Key::Char('4'),
            65437 => Key::Char('5'),
            65432 => Key::Char('6'),
            _ => {
                println!("key-{}", l);
                Key::Char(char::from(l as u8))
            }
        }
    }
}