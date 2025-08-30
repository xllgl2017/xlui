use std::mem;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::map::Map;
use crate::window::event::WindowEvent;
use crate::window::wino::{EventLoopHandle, LoopWindow};

pub struct Application {
    windows: Map<LoopWindow>,
    channel: (Sender<WindowEvent>, Receiver<WindowEvent>),
}

impl Application {
    pub fn new() -> Self {
        Application {
            windows: Map::new(),
            channel: channel(),
        }
    }

    pub fn run(mut self) {

    }
}

impl EventLoopHandle for Application {
    fn event(&mut self, id: &String, event: WindowEvent) {
        let window = self.windows.get_mut(id).unwrap();
        match event {
            WindowEvent::Redraw => {}
            WindowEvent::Resize(size) => {}
            WindowEvent::MouseMove(pos) => {}
            WindowEvent::MouseWheel => {}
            WindowEvent::MousePress(pos) => {}
            WindowEvent::MouseRelease(pos) => {}
            WindowEvent::KeyPress => {}
            WindowEvent::KeyRelease => {}
            _ => {}
        }
    }
}