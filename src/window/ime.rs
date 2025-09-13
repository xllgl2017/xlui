#[cfg(all(target_os = "linux", not(feature = "winit")))]
use crate::error::UiResult;
#[cfg(all(target_os = "linux", not(feature = "winit")))]
use crate::window::x11::ime::bus::Bus;
#[cfg(all(target_os = "linux", not(feature = "winit")))]
use crate::window::x11::ime::flag::Modifiers;
#[cfg(all(target_os = "linux", not(feature = "winit")))]
use crate::window::x11::ime::signal::{CommitText, UpdatePreeditText};
#[cfg(not(feature = "winit"))]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(all(target_os = "linux", not(feature = "winit")))]
use std::sync::Arc;
use std::sync::RwLock;
#[cfg(all(target_os = "linux", not(feature = "winit")))]
use std::time::Duration;

#[cfg(all(target_os = "linux", not(feature = "winit")))]
pub enum IMEKind {
    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    X11(Bus),
    #[cfg(feature = "winit")]
    Winit,
}

#[derive(Clone, Debug)]
pub enum IMEData {
    Preedit(Vec<char>),
    Commit(Vec<char>),
}

pub struct IME {
    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    kind: IMEKind,
    #[cfg(not(feature = "winit"))]
    working: AtomicBool,
    #[cfg(not(feature = "winit"))]
    chars: RwLock<Vec<char>>,
    #[cfg(not(feature = "winit"))]
    commited: AtomicBool,
    requested: RwLock<Vec<bool>>,
}

#[cfg(all(target_os = "linux", not(feature = "winit")))]
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
            working: AtomicBool::new(false),
            chars: RwLock::new(Vec::new()),
            commited: AtomicBool::new(false),
            requested: RwLock::new(Vec::new()),
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

    pub(crate) fn post_key(&self, keysym: u32, code: u32, modifiers: Modifiers) -> UiResult<bool> {
        match self.kind {
            IMEKind::X11(ref bus) => bus.ctx().process_key_event(keysym, code, modifiers),
        }
    }
}


impl IME {
    #[cfg(feature = "winit")]
    pub fn new_winit() -> IME {
        IME {
            requested: RwLock::new(Vec::new()),
        }
    }

    #[cfg(all(not(feature = "winit"), target_os = "windows"))]
    pub fn new_win32() -> IME {
        IME {
            working: AtomicBool::new(false),
            chars: RwLock::new(Vec::new()),
            commited: AtomicBool::new(false),
            requested: RwLock::new(Vec::new()),
        }
    }

    pub fn request_ime(&self, i: bool) {
        self.requested.write().unwrap().push(i);
    }

    #[cfg(not(feature = "winit"))]
    pub fn ime_draw(&self, new_chars: Vec<char>) {
        let mut chars = self.chars.write().unwrap();
        *chars = new_chars;
    }
    #[cfg(not(feature = "winit"))]
    pub fn ime_commit(&self, commit: Vec<char>) {
        self.working.store(false, Ordering::SeqCst);
        self.commited.store(true, Ordering::SeqCst);
        self.ime_draw(commit);
    }
    #[cfg(not(feature = "winit"))]
    pub fn ime_done(&self) -> Vec<char> {
        self.commited.store(false, Ordering::SeqCst);
        let mut chars = self.chars.write().unwrap();
        let res = chars.clone();
        chars.clear();
        res
    }
    #[cfg(not(feature = "winit"))]
    pub fn chars(&self) -> Vec<char> {
        let chars = self.chars.read().unwrap();
        let res = chars.clone();
        res
    }

    // pub fn len(&self) -> usize {
    //     self.chars.read().unwrap().len()
    // }
    #[cfg(not(feature = "winit"))]
    pub fn is_working(&self) -> bool {
        self.working.load(Ordering::SeqCst)
    }
    #[cfg(not(feature = "winit"))]
    pub fn is_commited(&self) -> bool {
        self.commited.load(Ordering::SeqCst)
    }

    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    pub(crate) fn update_working(&self) {
        let requested = self.requested.write().unwrap();
        let req = requested.iter().find(|x| **x == true);
        self.working.store(req.is_some(), Ordering::SeqCst);
    }
    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    pub(crate) fn update(&self) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            IMEKind::X11(ref bus) => { bus.process(Duration::from_secs(0)).unwrap(); }
            #[cfg(feature = "winit")]
            IMEKind::Winit => {}
        }
    }
    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    pub(crate) fn focus_in(&self) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            IMEKind::X11(ref bus) => { bus.ctx().focus_in().unwrap(); }
            #[cfg(feature = "winit")]
            IMEKind::Winit => {}
        }
    }
    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    pub(crate) fn focus_out(&self) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            IMEKind::X11(ref bus) => { bus.ctx().focus_out().unwrap(); }
            #[cfg(feature = "winit")]
            IMEKind::Winit => {}
        }
    }

    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    pub(crate) fn set_capabilities(&self, capabilities: u32) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            IMEKind::X11(ref bus) => { bus.ctx().set_capabilities(capabilities).unwrap(); }
            #[cfg(feature = "winit")]
            IMEKind::Winit => {}
        }
    }

    #[cfg(all(target_os = "linux", not(feature = "winit")))]
    pub(crate) fn set_cursor_position(&self, x: i32, y: i32) {
        match self.kind {
            #[cfg(all(target_os = "linux", not(feature = "winit")))]
            IMEKind::X11(ref bus) => { bus.ctx().set_cursor_location(x, y, 1, 1).unwrap(); }
            #[cfg(feature = "winit")]
            IMEKind::Winit => {}
        }
    }
}

unsafe impl Send for IME {}
unsafe impl Sync for IME {}