use crate::layout::Layout;
use crate::paint::color::Color;
use crate::paint::PaintTask;
use crate::paint::popup::PaintPopup;
use crate::radius::Radius;
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::rect::Rect;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::ui::Ui;

pub struct Popup {
    id: String,
    rect: Rect,
    pub(crate) layout: Option<Layout>,
}

impl Popup {
    pub fn new() -> Popup {
        Popup {
            id: crate::gen_unique_id(),
            rect: Rect::new().with_size(100.0, 200.0),
            layout: Some(Layout::top_to_bottom()),
        }
    }

    pub fn popup_style() -> ClickStyle {
        ClickStyle {
            fill: FillStyle {
                inactive: Color::rgb(240, 240, 240),
                hovered: Color::rgb(240, 240, 240),
                clicked: Color::rgb(240, 240, 240),
            },
            border: BorderStyle {
                inactive: Border {
                    width: 1.0,
                    radius: Radius::same(5),
                    color: Color::rgba(144, 209, 255, 255),
                },
                hovered: Border {
                    width: 1.0,
                    radius: Radius::same(5),
                    color: Color::rgba(144, 209, 255, 255),
                },
                clicked: Border {
                    width: 1.0,
                    radius: Radius::same(5),
                    color: Color::rgba(144, 209, 255, 255),
                },
            },
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Popup {
        self.rect.set_size(width, height);
        self
    }

    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect.clone();
        self.layout.as_mut().unwrap().max_rect = self.rect.clone();
        self.layout.as_mut().unwrap().available_rect = self.rect.clone_add_padding(&Padding::same(5.0));
    }

    pub fn rect(&self) -> &Rect { &self.rect }

    pub fn rect_mut(&mut self)->&mut Rect { &mut self.rect }

    pub fn draw(&mut self, ui: &mut Ui) {
        // let layout = ui.current_layout.as_mut().unwrap();
        // self.rect = layout.available_rect.clone_with_size(&self.rect);
        // layout.alloc_rect(&self.rect);
        let task = PaintPopup::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::Popup(task));
    }
}

