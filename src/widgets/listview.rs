use std::any::Any;
use crate::layout::scroll_area::ScrollArea;
use crate::layout::{HorizontalLayout, LayoutKind};
use crate::map::Map;
use crate::radius::Radius;
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::ui::Ui;
use crate::widgets::item::ItemWidget;
use crate::widgets::Widget;
use std::fmt::Display;
use std::sync::{Arc, RwLock};
use crate::frame::App;

pub struct ListView<T> {
    id: String,
    data: Vec<T>,
    items: Map<usize>,
    previous: Option<String>,
    current: Arc<RwLock<Option<String>>>,
    callback: Option<Box<dyn FnMut(&mut dyn Any, &mut Ui, &T)>>,
    rect: Rect,
}

impl<T: Display + 'static> ListView<T> {
    pub fn new(data: Vec<T>) -> Self {
        ListView {
            id: crate::gen_unique_id(),
            data,
            items: Map::new(),
            previous: None,
            rect: Rect::new(),
            current: Arc::new(RwLock::new(None)),
            callback: None,
        }
    }


    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.rect.set_size(w, h);
        self
    }


    fn item_widget(&self, ui: &mut Ui, datum: &T) -> String {
        let style = ClickStyle {
            fill: FillStyle {
                inactive: Color::TRANSPARENT,
                hovered: Color::rgba(153, 193, 241, 220),
                clicked: Color::rgba(153, 193, 241, 220),
            },
            border: BorderStyle {
                inactive: Border::new(1.0).radius(Radius::same(3)).color(Color::rgb(190, 190, 190)),
                hovered: Border::new(0.0).radius(Radius::same(3)),
                clicked: Border::new(0.0).radius(Radius::same(3)),
            },
        };
        let rect = ui.available_rect();
        let current = self.current.clone();
        let item = ItemWidget::new(LayoutKind::Horizontal(HorizontalLayout::new()))
            .with_size(rect.width(), 38.0).with_style(style).parent(self.current.clone())
            .connect(move |item_id| {
                current.write().unwrap().replace(item_id.to_string());
                println!("item clicked");
            });
        let item_id = item.id.clone();
        item.show(ui, |ui| {
            ui.image("logo.jpg", (30.0, 30.0));
            ui.vertical(|ui| {
                ui.label(datum.to_string());
                ui.horizontal(|ui| {
                    ui.label("00:00");
                    ui.label("200");
                    ui.label("HTTP/1.1");
                    ui.label("10 KB");
                    ui.label("10 KB");
                });
            });
        });
        item_id
    }

    pub fn current(&self) -> Option<&T> {
        let current = self.current.read().unwrap();
        let current = current.as_ref()?;
        let current_index = self.items[current];
        Some(&self.data[current_index])
    }

    pub fn remove(&mut self, index: usize) {
        self.data.remove(index);
    }

    pub fn push(&mut self, datum: T) {
        self.data.push(datum);
    }

    pub fn set_callback<A: App>(&mut self, f: impl FnMut(&mut A, &mut Ui, &T) + 'static) {
        self.callback = Some(Callback::create_list(f));
    }
}

impl<T: Display + 'static> Widget for ListView<T> {
    fn draw(&mut self, ui: &mut Ui) -> Response {
        self.rect = ui.available_rect().clone_with_size(&self.rect);
        let mut area = ScrollArea::new();
        area.set_rect(self.rect.clone());
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::TRANSPARENT;
        fill_style.fill.hovered = Color::TRANSPARENT;
        fill_style.fill.clicked = Color::TRANSPARENT;
        fill_style.border.inactive = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        area.set_style(fill_style);
        area.show(ui, |ui| {
            for (row, datum) in self.data.iter().enumerate() {
                let id = self.item_widget(ui, datum);
                self.items.insert(id, row);
            }
        });
        Response {
            id: self.id.clone(),
            rect: self.rect.clone(),
        }
    }

    fn update(&mut self, _ui: &mut Ui) {}

    fn redraw(&mut self, ui: &mut Ui) {
        let current = self.current.read().unwrap();
        if current.as_ref() != self.previous.as_ref() {
            self.previous = current.clone();
            if let Some(ref mut callback) = self.callback {
                let app = ui.app.take().unwrap();
                let current = self.current.read().unwrap();
                let index = self.items[current.as_ref().unwrap()];
                let data = &self.data[index];
                callback(*app, ui, data);
                ui.app = Some(app);
            }
        }
    }
}