#[derive(Debug)]
pub(crate) struct CChar {
    pub(crate) cchar: char,
    pub(crate) width: f32,
}

impl CChar {
    pub fn new(cchar: char, width: f32) -> CChar {
        CChar { cchar, width }
    }
}

#[derive(Default, Debug)]
pub struct LineChar {
    pub(crate) chars: Vec<CChar>,
    pub(crate) auto_wrap: bool,
    pub(crate) width: f32,
}

impl LineChar {
    pub fn new() -> LineChar {
        LineChar {
            chars: vec![],
            auto_wrap: true,
            width: 0.0,
        }
    }

    pub(crate) fn push(&mut self, cchar: CChar) {
        self.width += cchar.width;
        self.chars.push(cchar);
    }

    pub fn raw_text(&self) -> String {
        let mut res: String = self.chars.iter().map(|x| x.cchar.to_string()).collect();
        if !self.auto_wrap { res += "\n"; }
        res
    }

    pub fn get_width_in_char(&self, index: usize) -> f32 {
        let mut width = 0.0;
        // let index = if index >= self.chars.len() { return self.width } else { index };
        self.chars[..index].iter().for_each(|x| width += x.width);
        width
    }

    pub fn len(&self) -> usize { self.chars.len() }
}
