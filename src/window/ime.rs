use std::mem;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;
use dbus::blocking::Connection;
use dbus::Message;
use crate::window::x11::ime::bus::Bus;
use crate::window::x11::ime::flag::Modifiers;
use crate::window::x11::ime::signal::{CommitText, UpdatePreeditText};

pub enum IMEKind {
    X11(Bus)
}

pub struct IME {
    kind: IMEKind,
    available: bool,
    working: AtomicBool,
    chars: RwLock<Vec<char>>,
    commited: AtomicBool,
}


impl IME {
    fn preedit_text(text: UpdatePreeditText, ime: &Arc<IME>) -> bool {
        println!("preedit_text-{:?}", text);
        ime.ime_draw(text.text.chars());
        true
    }

    fn commit(text: CommitText, ime: &Arc<IME>) -> bool {
        println!("commit-{:?}", text);
        ime.ime_commit(text.text.chars());
        true
    }


    pub fn new_x11(name: &str) -> Self {
        IME {
            kind: IMEKind::X11(Bus::new(name).unwrap()),
            available: false,
            working: AtomicBool::new(false),
            chars: RwLock::new(Vec::new()),
            commited: AtomicBool::new(false),
        }
    }

    pub(crate) fn create_binding(&self, ime: Arc<IME>) {
        match self.kind {
            IMEKind::X11(ref bus) => {
                let i = ime.clone();
                bus.ctx().on_update_preedit_text(move |a, _, _| Self::preedit_text(a, &i)).unwrap();
                bus.ctx().on_commit_text(move |a, _, _| Self::commit(a, &ime)).unwrap();
            }
        }
    }


    pub fn enable(mut self) -> Self {
        self.available = true;
        self
    }

    pub fn disable(mut self) -> Self {
        self.available = false;
        self
    }

    pub fn ime_start(&self) {
        self.working.store(true, Ordering::SeqCst);
    }

    pub fn ime_draw(&self, new_chars: Vec<char>) {
        let mut chars = self.chars.write().unwrap();
        *chars = new_chars;
    }

    pub fn ime_commit(&self, commit: Vec<char>) {
        self.working.store(false, Ordering::SeqCst);
        self.commited.store(true, Ordering::SeqCst);
        self.ime_draw(commit);
    }

    pub fn ime_done(&self) -> Vec<char> {
        self.commited.store(false, Ordering::SeqCst);
        let mut chars = self.chars.write().unwrap();
        let chars = mem::take(&mut *chars);
        chars
    }

    pub fn chars(&self) -> Vec<char> {
        let chars = self.chars.read().unwrap();
        chars.clone()
    }

    pub fn is_available(&self) -> bool {
        self.available
    }

    pub fn is_working(&self) -> bool {
        self.working.load(Ordering::SeqCst)
    }

    pub fn is_commited(&self) -> bool {
        self.commited.load(Ordering::SeqCst)
    }

    pub(crate) fn post_key(&self, keysym: u32, code: u32, modifiers: Modifiers) {
        match self.kind {
            IMEKind::X11(ref bus) => {
                bus.ctx().process_key_event(keysym, code, modifiers).unwrap();
            }
        }
    }

    pub(crate) fn update(&self) {
        match self.kind {
            IMEKind::X11(ref bus) => { bus.process(Duration::from_secs(0)).unwrap(); }
        }
    }

    pub(crate) fn focus_in(&self) {
        match self.kind {
            IMEKind::X11(ref bus) => { bus.ctx().focus_in().unwrap(); }
        }
    }

    pub(crate) fn focus_out(&self) {
        match self.kind {
            IMEKind::X11(ref bus) => { bus.ctx().focus_out().unwrap(); }
        }
    }

    pub(crate) fn set_capabilities(&self, capabilities: u32) {
        match self.kind {
            IMEKind::X11(ref bus) => { bus.ctx().set_capabilities(capabilities).unwrap(); }
        }
    }

    pub(crate) fn set_cursor_position(&self, x: i32, y: i32) {
        match self.kind {
            IMEKind::X11(ref bus) => { bus.ctx().set_cursor_location(x, y, 1, 1).unwrap(); }
        }
    }
}

unsafe impl Send for IME {}
unsafe impl Sync for IME {}