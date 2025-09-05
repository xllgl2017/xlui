use dbus::arg::{ReadAll, TypeMismatchError};
use dbus::message::SignalArgs;
use crate::window::x11::ime::context::INTERFACE_NAME;
use crate::window::x11::ime::Text;

#[derive(Debug)]
pub struct UpdatePreeditText {
    pub text: Text<'static>,
}
impl ReadAll for UpdatePreeditText {
    fn read(i: &mut dbus::arg::Iter) -> Result<Self, TypeMismatchError> {
        let text: Text = i.read()?;
        Ok(UpdatePreeditText {
            text,
        })
    }
}
impl SignalArgs for UpdatePreeditText {
    const NAME: &'static str = "UpdatePreeditText";
    const INTERFACE: &'static str = INTERFACE_NAME;
}

#[derive(Debug)]
pub struct CommitText {
    pub text: Text<'static>,
}
impl ReadAll for CommitText {
    fn read(i: &mut dbus::arg::Iter) -> Result<Self, TypeMismatchError> {
        let text: Text = i.read()?;
        Ok(CommitText { text })
    }
}
impl SignalArgs for CommitText {
    const NAME: &'static str = "CommitText";
    const INTERFACE: &'static str = INTERFACE_NAME;
}
