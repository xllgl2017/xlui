use crate::response::Response;
use crate::size::SizeMode;
use crate::widgets::WidgetSize;
use crate::{Button, LayoutKind, RichText, Ui, UpdateType, VerticalLayout, Widget};

pub struct TabItem {
    label: Button,
    layout: LayoutKind,
}

pub struct TabWidget {
    id: String,
    current: Option<usize>,
    items: Vec<TabItem>,
    size_mode: SizeMode,
}

impl TabWidget {
    pub fn new() -> TabWidget {
        TabWidget {
            id: crate::gen_unique_id(),
            current: None,
            items: vec![],
            size_mode: SizeMode::Auto,
        }
    }
    pub fn add_tab(&mut self, ui: &mut Ui, name: impl Into<RichText>, context: impl FnOnce(&mut Ui)) {
        self.current = Some(self.items.len());
        let ut = ui.update_type.clone();
        ui.update_type = UpdateType::Init;
        let mut label = Button::new(name).width(50.0).height(25.0);
        label.update(ui);
        let current_layout = VerticalLayout::top_to_bottom();
        let previous_layout = ui.layout.replace(LayoutKind::new(current_layout));
        context(ui);
        let current_layout = ui.layout.take().unwrap();
        ui.layout = previous_layout;
        let item = TabItem {
            label,
            layout: current_layout,
        };
        self.items.push(item);
        ui.update_type = ut;
    }
}


impl Widget for TabWidget {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        let mut context_rect = ui.draw_rect.clone();
        let mut tab_text_rect = ui.draw_rect.clone();
        for (index, item) in self.items.iter_mut().enumerate() {
            ui.draw_rect = tab_text_rect.clone();
            let ut = ui.update_type.clone();
            let resp = item.label.update(ui);
            ui.draw_rect.set_size(resp.size.dw, resp.size.dh);
            println!("{:?} {:?}", ui.draw_rect, ui.device.device_input.mouse.lastest.relative);
            if let UpdateType::MouseRelease = ut && ui.draw_rect.has_position(ui.device.device_input.mouse.lastest.relative) {
                self.current = Some(index);
            }
            tab_text_rect.add_min_x(resp.size.dw);
        }
        println!("{:?}", self.current);
        context_rect.add_min_y(25.0);
        if let Some(current) = self.current {
            ui.draw_rect = context_rect;
            self.items[current].layout.update(ui);
        }
        let (w, h) = self.size_mode.size(100.0, 100.0);
        Response::new(&self.id, WidgetSize::same(w, h))
    }
}