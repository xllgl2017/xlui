//! #ListView的泛类可以是任意类型
//! ```
//! use xlui::frame::App;
//! use xlui::ui::Ui;//!
//!
//! use xlui::widgets::listview::ListView;
//!
//! struct XlUi{
//!     list_view: ListView<i32>
//! }
//!
//! impl XlUi{
//!     fn item_changed(&mut self,ui:&mut Ui){
//!         if let Some(datum) = self.list_view.current() {
//!             println!("list: {}", self.list_view.current());
//!         }
//!     }
//! }
//!
//! impl App for XlUi{
//!     fn draw(&mut self, ui: &mut Ui) {
//!         self.list_view.set_callback(Self::item_changed);
//!         self.list_view.show(ui,|ui,datum|ui.label(datum.to_string()));
//!     }
//!
//!     fn update(&mut self, ui: &mut Ui) {
//!
//!     }
//!
//!     fn redraw(&mut self, ui: &mut Ui) {
//!
//!     }
//!
//! }
//! ```
//!
//!
//!

use crate::frame::App;
use crate::layout::scroll_area::ScrollArea;
use crate::layout::{HorizontalLayout, LayoutKind};
use crate::map::Map;
use crate::radius::Radius;
use crate::response::Callback;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::ui::Ui;
use crate::widgets::item::ItemWidget;
use std::any::Any;
use std::sync::{Arc, RwLock};

pub enum ListUpdate {
    Push(usize),
    Remove(usize),
}

pub struct ListView<T> {
    lid: String,
    data: Vec<T>,
    items: Map<usize>,
    current: Arc<RwLock<Option<String>>>,
    callback: Arc<Option<Box<dyn Fn(&mut dyn Any, &mut Ui)>>>,
    dyn_item_widget: Box<dyn Fn(&mut Ui, &T)>,
    rect: Rect,
    updates: Vec<ListUpdate>,
}

impl<T: 'static> ListView<T> {
    pub fn new(data: Vec<T>) -> Self {
        ListView {
            lid: "".to_string(),
            data,
            items: Map::new(),
            rect: Rect::new(),
            current: Arc::new(RwLock::new(None)),
            callback: Arc::new(None),
            dyn_item_widget: Box::new(|ui, _| ui.label("ListItem")),
            updates: vec![],
        }
    }


    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.rect.set_size(w, h);
        self
    }

    pub fn set_item_widget(&mut self, item_widget: impl Fn(&mut Ui, &T) + 'static) {
        self.dyn_item_widget = Box::new(item_widget);
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
        let callback = self.callback.clone();
        let item = ItemWidget::new(LayoutKind::Horizontal(HorizontalLayout::new()))
            .with_size(rect.width(), 38.0).with_style(style).parent(self.current.clone())
            .connect(move |item_id, ui| {
                current.write().unwrap().replace(item_id.to_string());
                if let Some(callback) = callback.as_ref() {
                    let app = ui.app.take().unwrap();
                    callback(*app, ui);
                    ui.app = Some(app);
                }
                println!("item clicked");
            });
        let item_id = item.id.clone();
        item.show(ui, |ui| (self.dyn_item_widget)(ui, &datum));
        item_id
    }

    pub fn current_index(&self) -> Option<usize> {
        let current = self.current.read().unwrap();
        let current = current.as_ref()?;
        Some(self.items[current])
    }

    pub fn current(&self) -> Option<&T> {
        let current = self.current.read().unwrap();
        let current = current.as_ref()?;
        let current_index = self.items[current];
        Some(&self.data[current_index])
    }

    fn _remove(&mut self, index: usize, ui: &mut Ui) {
        let (wid, _) = self.items.remove_map_by_index(index);
        let area = ui.get_layout(&self.lid).expect("找不到ListView");
        area.remove_widget(&wid);
    }

    pub fn remove(&mut self, index: usize) -> T {
        let res = self.data.remove(index);
        self.updates.push(ListUpdate::Remove(index));
        res
    }

    fn _push(&mut self, index: usize, ui: &mut Ui) {
        let mut layout = ui.layout.take().unwrap();
        let area = layout.get_layout(&self.lid).expect("找不到ListView");
        if let LayoutKind::ScrollArea(area) = area {
            ui.layout = Some(LayoutKind::Vertical(area.layout.take().unwrap()));
            let wid = self.item_widget(ui, &self.data[index]);
            if let LayoutKind::Vertical(layout) = ui.layout.take().unwrap() {
                area.layout = Some(layout);
            }
            area.reset_context_height();
            self.items.insert(wid, self.data.len() - 1);
        }
        ui.layout = Some(layout);
    }

    pub fn push(&mut self, datum: T) {
        self.updates.push(ListUpdate::Push(self.data.len()));
        self.data.push(datum);
    }

    pub fn set_callback<A: App>(&mut self, f: impl Fn(&mut A, &mut Ui) + 'static) {
        self.callback = Arc::new(Some(Callback::create_list(f)));
    }

    pub fn show(&mut self, ui: &mut Ui) {
        self.rect = ui.available_rect().clone_with_size(&self.rect);
        let mut area = ScrollArea::new();
        self.lid = area.id.clone();
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
        ui.layout().alloc_rect(&self.rect);
    }

    pub fn update(&mut self, ui: &mut Ui) {
        for update in std::mem::take(&mut self.updates) {
            match update {
                ListUpdate::Push(i) => self._push(i, ui),
                ListUpdate::Remove(i) => self._remove(i, ui)
            }
        }
    }
}