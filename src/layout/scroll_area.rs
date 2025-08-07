use crate::layout::Layout;
use crate::paint::PaintTask;
use crate::paint::scroll_area::PaintScrollArea;
use crate::response::button::ButtonResponse;
use crate::response::DrawnEvent;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::widgets::scroll::bar::ScrollBar;

pub struct ScrollArea {
    id: String,
    pub(crate) rect: Rect,
    pub(crate) layouts: Vec<Layout>,
    pub(crate) padding: Padding,
    current_layout: Option<Layout>,
    pub(crate) v_bar: ScrollBar,
}

impl ScrollArea {
    pub fn new() -> ScrollArea {
        ScrollArea {
            id: crate::gen_unique_id(),
            rect: Rect::new(),
            layouts: vec![],
            padding: Padding::same(10.0),
            current_layout: Some(Layout::top_to_bottom()),
            v_bar: ScrollBar::new(),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.rect.set_width(width);
        self.rect.set_height(height);
        self.v_bar.rect.set_height(height);
        self
    }

    pub fn show(mut self, ui: &mut Ui, callback: impl Fn(&mut Ui)) {
        ui.current_scrollable = true;
        let mut previous_layout = ui.current_layout.take().unwrap();
        self.rect = previous_layout.available_rect.clone_with_size(&self.rect);
        // self.rect.set_width(300.0);
        // self.rect.set_height(400.0);
        let v_bar_width = self.v_bar.rect.width();
        self.v_bar.rect = self.rect.clone();
        self.v_bar.rect.x.min = self.v_bar.rect.x.max - v_bar_width; //outer


        let mut current_layout = self.current_layout.take().unwrap();
        current_layout.available_rect = self.rect.clone_add_padding(&self.padding);
        current_layout.max_rect = self.rect.clone_add_padding(&self.padding);
        ui.current_layout.replace(current_layout);
        ui.current_scrollable = true;
        callback(ui);
        let current_layout = ui.current_layout.take().unwrap();
        previous_layout.alloc_rect(&self.rect); //分配大小
        // previous_layout.alloc_layout(&current_layout);
        ui.current_layout.replace(previous_layout);
        ui.current_scrollable = false;
        self.layouts.insert(0, current_layout);
        self.layouts.append(&mut ui.scroll_layouts);
        self.draw(ui);
    }


    fn draw(self, ui: &mut Ui) {
        let id = self.id.clone();
        ui.response.insert(self.id.clone(), ButtonResponse::new(self.rect.clone()).event(DrawnEvent::Click));
        let task = PaintScrollArea::new(self, ui);
        ui.add_paint_task(id, PaintTask::ScrollArea(task));
    }
}