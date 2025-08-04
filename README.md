![logo](img/logo/logo_96.png)
# xlui:

### 示例
```rust
fn main() {
    let attr = WindowAttribute {
        inner_size: (800, 600).into(),
        ..Default::default()
    };
    let mut app = Application::new().with_attrs(attr);
    app.run(XlUiApp::new());
}

struct XlUiApp {
    label: Label,
    count: i32,
}

impl XlUiApp {
    pub fn new() -> Self {
        Self {
            label: Label::new("hello".to_string()).width(100.0),
            count: 0,
        }
    }

    pub fn add(&mut self, uim: &mut UiM) {
        self.count += 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(uim);
    }

    pub fn reduce(&mut self, uim: &mut UiM) {
        self.count -= 1;
        self.label.set_text(format!("count: {}", self.count));
        self.label.update(uim);
    }

}

impl App for XlUiApp {
    fn draw(&mut self, ui: &mut Ui) {
        self.label.draw(ui);
        ui.horizontal(|ui| {
            Button::new("+".to_string()).width(30.0).height(30.0).connect(Self::add).draw(ui);
            Button::new("-".to_string()).width(30.0).height(30.0).connect(Self::reduce).draw(ui);
        });
    }
}
```