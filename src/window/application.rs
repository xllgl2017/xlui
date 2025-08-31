use crate::frame::App;
use crate::window::event::WindowEvent;
use crate::window::wino::{EventLoopHandle, LoopWindow};
use crate::window::WindowId;
use std::process::exit;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread::spawn;

pub struct Application {
    windows: i32,
    channel: (SyncSender<(WindowId, WindowEvent)>, Receiver<(WindowId, WindowEvent)>),
}

impl Application {
    pub fn new() -> Self {
        Application {
            windows: 0,
            channel: sync_channel(1),
        }
    }

    pub fn create_window<A: App>(&mut self, app: A) {
        self.windows += 1;
        let sender = self.channel.0.clone();
        let mut window = pollster::block_on(async { LoopWindow::create_window(app, sender).await });
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

