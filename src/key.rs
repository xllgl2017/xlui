use std::os::raw::c_uint;

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
    pub fn from_c_ulong(l: c_uint) -> Key {
        println!("key-{}", l);
        match l {


            10 => Key::Char('1'),
            11 => Key::Char('2'),
            12 => Key::Char('3'),
            13 => Key::Char('4'),
            14 => Key::Char('5'),
            15 => Key::Char('6'),
            16 => Key::Char('7'),
            17 => Key::Char('8'),
            18 => Key::Char('9'),
            19 => Key::Char('0'),
            20 => Key::Char('-'),
            21 => Key::Char('='),

            22 => Key::Backspace,
            24 => Key::Char('q'),
            25 => Key::Char('w'),
            26 => Key::Char('e'),
            27 => Key::Char('r'),
            28 => Key::Char('t'),
            29 => Key::Char('y'),
            30 => Key::Char('u'),
            31 => Key::Char('i'),
            32 => Key::Char('o'),
            33 => Key::Char('p'),
            34 => Key::Char('['),
            35 => Key::Char(']'),
            36 => Key::Enter,
            38 => Key::Char('a'),
            39 => Key::Char('s'),
            40 => Key::Char('d'),
            41 => Key::Char('f'),
            42 => Key::Char('g'),
            43 => Key::Char('h'),
            44 => Key::Char('j'),
            45 => Key::Char('k'),
            46 => Key::Char('l'),
            47 => Key::Char(';'),
            48 => Key::Char('\''),

            52 => Key::Char('z'),
            53 => Key::Char('x'),
            54 => Key::Char('c'),
            55 => Key::Char('v'),
            56 => Key::Char('b'),
            57 => Key::Char('n'),
            58 => Key::Char('m'),
            59 => Key::Char(','),
            60 => Key::Char('.'),
            61 => Key::Char('/'),
            63 => Key::Char('*'),
            65 => Key::Space,

            79 => Key::Char('7'),
            80 => Key::Char('8'),
            81 => Key::Char('9'),
            82 => Key::Char('-'),
            83 => Key::Char('4'),
            84 => Key::Char('5'),
            85 => Key::Char('6'),
            86 => Key::Char('+'),
            87 => Key::Char('1'),
            88 => Key::Char('2'),
            89 => Key::Char('3'),
            91 => Key::Char('.'),
            104 => Key::Enter,
            106 => Key::Char('/'),
            119 => Key::Delete,
            110 => Key::Home,
            111 => Key::UpArrow,
            113 => Key::LeftArrow,
            114 => Key::RightArrow,
            115 => Key::End,
            116 => Key::DownArrow,
            _ => {
                Key::Char(char::from(l as u8))
            }
        }
    }
}