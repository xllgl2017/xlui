use crate::frame::App;
use crate::map::Map;
use crate::window::event::WindowEvent;
use crate::window::wino::{EventLoopHandle, LoopWindow};
use crate::window::{Window, WindowId};
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use x11::xlib::XInitThreads;

pub struct X11Application {
    windows: Map<Window>,
    channel: (SyncSender<(WindowId, WindowEvent)>, Receiver<(WindowId, WindowEvent)>),
}

impl X11Application {
    pub fn new() -> Self {
        X11Application {
            windows: Map::new(),
            channel: sync_channel(1),
        }
    }

    // pub fn create_new_window<A: App>(&mut self, app: A) {
    //     // let attr = app.window_attributes();
    //     // let x11_window = X11Window::new(attr.inner_size, &attr.title, self.channel.0.clone()).unwrap();
    //     // let x11_window = Arc::new(LoopWindow::Xlib(x11_window));
    //     // let wid = x11_window.id();
    //     // let window = pollster::block_on(async { Window::new_x11(x11_window.clone(), Box::new(app), attr, self.channel.0.clone()).await }).unwrap();
    //     // spawn(move || {
    //     //     x11_window.run();
    //     // });
    //     // self.windows.insert(wid.to_string(), window);
    //     // self.channel.0.send((wid, WindowEvent::Redraw)).unwrap();
    // }

    pub fn run<A: App>(&self, app: A) {
        unsafe {
            if XInitThreads() == 0 {
                panic!("XInitThreads failed");
            }
        }
        let mut window = pollster::block_on(async { LoopWindow::create_window(app, self.channel.0.clone()).await });
        window.run();
    }

    // pub fn run(mut self) {
    //
    //     // let window = &mut self.windows[0];
    //     // window.render();
    //     // let x11_window = window.app_ctx.context.window.clone();
    //     // let device=&mut window.app_ctx.device;
    //     // x11_window.run();
    //     // loop {
    //     //     // sleep(Duration::from_millis(100));
    //     //     let (wid, event) = self.channel.1.recv().unwrap();
    //     //     self.event(wid, event);
    //     // }
    // }
}

// impl EventLoopHandle for X11Application {
//     fn event(&mut self, id: WindowId, event: WindowEvent) {
//         println!("{:?}", event);
//         let window = self.windows.get_mut(id).unwrap();
//         match event {
//             WindowEvent::KeyPress => {}
//             WindowEvent::KeyRelease => {}
//             WindowEvent::MouseMove(_) => {}
//             WindowEvent::MouseWheel => {}
//             WindowEvent::MousePress(_) => {}
//             WindowEvent::MouseRelease(_) => {}
//             WindowEvent::Redraw => window.render(),
//             WindowEvent::Reinit => {}
//             WindowEvent::Resize(size) => {
//                 window.app_ctx.device.surface_config.width = size.width;
//                 window.app_ctx.device.surface_config.height = size.height;
//                 let device = &window.app_ctx.device.device;
//                 let config = &window.app_ctx.device.surface_config;
//                 println!("3333333333333333");
//                 window.app_ctx.device.surface.configure(device, config);
//                 println!("444444444444444");
//                 // window.resize(size);
//                 // window.render();
//             }
//             WindowEvent::ReqClose => {
//                 self.windows.remove(&id.to_string());
//                 if self.windows.len() == 0 { exit(0); }
//             }
//             WindowEvent::Update(_) => {}
//         }
//     }
// }