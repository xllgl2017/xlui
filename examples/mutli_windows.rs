use xlui::*;

fn main() {
    XlUi::new().run().unwrap_or_else(|e| println!("{}", e.to_string()));
}

struct Window {
    title: String,
}

impl Window {
    fn new(title: impl ToString) -> Window {
        Window {
            title: title.to_string(),
        }
    }
}

impl App for Window {
    fn draw(&mut self, ui: &mut Ui) {
        ui.label(self.title.clone().size(18.0));
    }

    fn window_attributes(&self) -> WindowAttribute {
        WindowAttribute {
            inner_size: (400, 300).into(),
            ..Default::default()
        }
    }
}


struct XlUi {}

impl XlUi {
    pub fn new() -> XlUi {
        XlUi {}
    }

    fn on_rect_close(&mut self, window: InnerWindow, _: &mut Ui) {
        //关闭后获取窗口所有权
        let _window: Window = window.to_();
    }


    fn open_inner_window(&mut self, _: &mut Button, ui: &mut Ui) {
        ui.create_inner_window(Window::new("InnerWindow")).on_close(Self::on_rect_close);
    }

    fn open_child_window(&mut self, _: &mut Button, ui: &mut Ui) {
        ui.create_window(Window::new("ChildWindow"));
    }

    // fn open_test_layout(&mut self, _: &mut Button, ui: &mut Ui) {
    //     ui.create_inner_window(TestLayout {});
    // }
}

impl App for XlUi {
    fn draw(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.button("内部窗口").set_callback(Self::open_inner_window);
            ui.button("子窗口").set_callback(Self::open_child_window);
        });
    }
}