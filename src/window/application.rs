use crate::frame::App;
use crate::window::event::WindowEvent;
use crate::window::wino::LoopWindow;
use crate::window::WindowId;
use std::process::exit;
use std::sync::Arc;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread::spawn;
use crate::window::ime::IME;
use crate::window::x11::ime::flag::Capabilities;

pub struct Application {
    windows: i32,
    channel: (SyncSender<(WindowId, WindowEvent)>, Receiver<(WindowId, WindowEvent)>),
    ime: Arc<IME>,
}

impl Application {
    pub fn new() -> Self {
        let ime = Arc::new(IME::new_x11("xlui ime").enable());
        ime.set_capabilities(Capabilities::PreeditText | Capabilities::Focus);
        let ii = ime.clone();
        ime.create_binding(ii);

        Application {
            windows: 0,
            channel: sync_channel(1),
            ime,
        }
    }

    pub fn create_window<A: App>(&mut self, app: A) {
        self.windows += 1;
        let sender = self.channel.0.clone();
        let mut window = pollster::block_on(async { LoopWindow::create_window(app, sender, self.ime.clone()).await });
        spawn(move || {
            window.run();
        });
    }

    pub fn run(mut self) {
        loop {
            let (_, event) = self.channel.1.recv().unwrap();
            if let WindowEvent::ReqClose = event {
                self.windows -= 1;
                if self.windows == 0 { exit(0); }
            }
        }
    }
}

