use std::ffi::c_void;
use std::ptr;
use std::ptr::NonNull;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;

use raw_window_handle::{DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle, RawWindowHandle, WindowHandle, XlibDisplayHandle, XlibWindowHandle};
use wgpu::{Device, Queue, StoreOp, Surface};
use x11::xlib;

#[link(name = "X11")]
unsafe extern {}

struct X11Window {
    display: *mut xlib::Display,
    window: xlib::Window,
    screen: i32,
    wm_delete_atom: xlib::Atom,
}

impl X11Window {
    fn new(width: u32, height: u32, title: &str) -> Result<Self, String> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err("Cannot open X display".into());
            }
            let screen = xlib::XDefaultScreen(display);
            let root = xlib::XRootWindow(display, screen);

            let black = xlib::XBlackPixel(display, screen);
            let white = xlib::XWhitePixel(display, screen);

            let window = xlib::XCreateSimpleWindow(
                display,
                root,
                0,
                0,
                width,
                height,
                1,
                black,
                white,
            );

            // Set window title
            let c_title = std::ffi::CString::new(title).unwrap();
            xlib::XStoreName(display, window, c_title.as_ptr());

            // Select events: expose, key, mouse, structure notify (resize), pointer motion
            let events = xlib::ExposureMask
                | xlib::KeyPressMask
                | xlib::KeyReleaseMask
                | xlib::ButtonPressMask
                | xlib::ButtonReleaseMask
                | xlib::PointerMotionMask
                | xlib::StructureNotifyMask;
            xlib::XSelectInput(display, window, events as i64);

            // WM_DELETE_WINDOW
            // let wm_protocols = xlib::XInternAtom(display, b"WM_PROTOCOLS\0".as_ptr() as *const i8, 0);
            let wm_delete = xlib::XInternAtom(display, b"WM_DELETE_WINDOW\0".as_ptr() as *const i8, 0);
            xlib::XSetWMProtocols(display, window, &wm_delete as *const xlib::Atom as *mut xlib::Atom, 1);

            xlib::XMapWindow(display, window);
            xlib::XFlush(display);

            Ok(Self {
                display,
                window,
                screen,
                wm_delete_atom: wm_delete,
            })
        }
    }


    fn request_redraw(&self) {
        unsafe {
            xlib::XClearArea(self.display, self.window, 0, 0, 0, 0, xlib::True);
            xlib::XFlush(self.display);
        }
    }
}

impl Drop for X11Window {
    fn drop(&mut self) {
        unsafe {
            xlib::XDestroyWindow(self.display, self.window);
            xlib::XCloseDisplay(self.display);
        }
    }
}

impl HasWindowHandle for X11Window {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        let xlib_window_handle = XlibWindowHandle::new(self.window);
        let raw_window_handle = RawWindowHandle::Xlib(xlib_window_handle);
        unsafe { Ok(WindowHandle::borrow_raw(raw_window_handle)) }
    }
}

impl HasDisplayHandle for X11Window {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        let display = NonNull::new(self.display as *mut c_void);
        let x11_display_handle = XlibDisplayHandle::new(display, self.screen);
        let raw_display_handle = RawDisplayHandle::Xlib(x11_display_handle);
        unsafe { Ok(DisplayHandle::borrow_raw(raw_display_handle)) }
    }
}


unsafe impl Send for X11Window {}
unsafe impl Sync for X11Window {}


fn redraw(surface: &Surface, device: &Device, queue: &Queue) -> Result<(), Box<dyn std::error::Error>> {
    let frame = surface.get_current_texture()?;
    let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("frame-encoder"),
    });

    // Animated clear color
    // let t = start.elapsed().as_secs_f32();
    let r = 0.5;
    let g = 0.5;
    let b = 0.5;

    let ops = wgpu::Operations {
        load: wgpu::LoadOp::Clear(wgpu::Color {
            r: r as f64,
            g: g as f64,
            b: b as f64,
            a: 1.0,
        }),
        store: StoreOp::Store,
    };

    {
        let _rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("clear-pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        // place draw calls here
    }

    queue.submit(Some(encoder.finish()));
    frame.present();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // window initial size
    // let mut width = 800u32;
    // let mut height = 600u32;

    // create X11 window
    let xwin = Arc::new(X11Window::new(800, 600, "x11-wgpu-highfps")?);

    // create wgpu instance and surface
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
    let surface = instance.create_surface(xwin.clone())?;

    // pick adapter compatible with surface
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    })).unwrap();
    //     .ok_or("No suitable GPU adapter found")?;

    // request device + queue
    let (device, queue) = pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: Some("device"),
            required_features: Default::default(),
            required_limits: Default::default(),
            memory_hints: Default::default(),
            trace: Default::default(),
        },
    ))?;
    let caps = surface.get_capabilities(&adapter);
    let format = caps.formats[0];
    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: 800,
        height: 600,
        present_mode: wgpu::PresentMode::AutoVsync,
        desired_maximum_frame_latency: 2,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: vec![],
    };
    surface.configure(&device, &config);

    redraw(&surface, &device, &queue).unwrap();
    unsafe {
        let mut event: xlib::XEvent = std::mem::zeroed();
        // while xlib::XPending(xwin.display) > 0 {}

        loop {
            sleep(Duration::from_millis(5));
            xlib::XNextEvent(xwin.display, &mut event);
            let typ = event.get_type();
            match typ {
                xlib::Expose => {
                    println!("{}", "redraw");
                    let _ = redraw(&surface, &device, &queue);
                    // Redraw requested; we just continue to render each frame anyway
                }
                xlib::ConfigureNotify => {
                    // Window resized
                    let xcfg: xlib::XConfigureEvent = event.configure;
                    let new_w = xcfg.width as u32;
                    let new_h = xcfg.height as u32;
                    if new_w == 0 || new_h == 0 {
                        // ignore weird zero sizes
                    } else if new_w != config.width || new_h != config.height {
                        config.width = new_w;
                        config.height = new_h;
                        // width = new_w;
                        // height = new_h;
                        // reconfigure surface to new size
                        surface.configure(&device, &config);
                        xwin.request_redraw();
                        // redraw(&surface, &device, &queue)?;
                        // eprintln!("Resized to {}x{}", new_w, new_h);
                    }
                }
                xlib::ClientMessage => {
                    // Check for WM_DELETE_WINDOW
                    let xclient: xlib::XClientMessageEvent = event.client_message;
                    if xclient.data.get_long(0) as xlib::Atom == xwin.wm_delete_atom {
                        break;
                    }
                }
                xlib::KeyPress => {
                    // Map key to keysym
                    let xkey: xlib::XKeyEvent = event.key;
                    let ks = xlib::XLookupKeysym(&xkey as *const xlib::XKeyEvent as *mut _, 0);
                    // XK_Escape constant from x11 crate keysym
                    // if ks == x11::keysym::XK_Escape {
                    //     running = false;
                    // } else {
                    //     // print pressed key code/keysym for debug
                    //     eprintln!("KeyPress: keycode={} keysym={}", xkey.keycode, ks);
                    // }
                }
                xlib::ButtonRelease => {
                    let xb: xlib::XButtonEvent = event.button;
                    eprintln!("Mouse Release {} at ({}, {})", xb.button, xb.x, xb.y);
                }
                xlib::ButtonPress => {
                    let xb: xlib::XButtonEvent = event.button;
                    eprintln!("Mouse Press {} at ({}, {})", xb.button, xb.x, xb.y);
                }
                xlib::MotionNotify => {
                    let xm: xlib::XMotionEvent = event.motion;
                    // you may want to throttle printing in real app
                    eprintln!("Mouse move: ({}, {})", xm.x, xm.y);
                }
                _ => {}
            }
        }
    }

    Ok(())
}
