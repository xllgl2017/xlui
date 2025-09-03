use dbus::arg::{Arg, ArgType, Get, Iter, PropMap, Variant};
use std::borrow::Cow;
use dbus::Signature;

pub mod bus;
pub mod context;
pub mod flag;
pub mod signal;

#[derive(Debug, Clone)]
pub struct Text<'a> {
    string: Cow<'a, str>,
}

impl<'s> Text<'s> {
    pub fn chars(&self) -> Vec<char> {
        self.string.chars().collect()
    }
}

impl<'a> Arg for Text<'a> {
    const ARG_TYPE: ArgType = ArgType::Variant;

    fn signature() -> Signature<'static> {
        Signature::from("v\u{0}")
    }
}

impl<'a> Get<'a> for Text<'static> {
    fn get(i: &mut Iter<'a>) -> Option<Self> {
        let mut text_var: Variant<Iter<'a>> = i.get()?;
        let text_struct: (&'a str, PropMap, &'a str, Variant<Iter<'a>>) = text_var.0.get()?;
        Some(Text {
            string: Cow::Owned(text_struct.2.to_owned()),
        })
    }
}