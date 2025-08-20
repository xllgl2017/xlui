use crate::frame::context::UpdateType;
use crate::layout::scroll_area::ScrollArea;
use crate::layout::Layout;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderParam, WrcRender};
use crate::size::border::Border;
use crate::size::padding::Padding;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle, Shadow};
use crate::ui::Ui;

pub struct Popup {
    pub(crate) id: String,
    scroll_area: ScrollArea,
    fill_render: RenderParam<RectParam>,
    pub(crate) open: bool,
}

impl Popup {
    pub fn new(ui: &mut Ui, rect: Rect) -> Popup {
        let shadow = Shadow {
            offset: [5.0, 8.0],
            spread: 10.0,
            color: Color::rgba(0, 0, 0, 30),
        };

        let fill_param = RectParam::new(rect.clone(), Popup::popup_style())
            .with_shadow(shadow);
        let mut fill_render = RenderParam::new(fill_param);
        fill_render.init_rectangle(ui, false, false);
        let mut area = ScrollArea::new().padding(Padding::same(5.0));
        area.set_rect(rect);
        Popup {
            id: crate::gen_unique_id(),
            scroll_area: area,
            fill_render,
            open: false,
        }
    }

    pub fn show(mut self, ui: &mut Ui, context: impl FnMut(&mut Ui)) {
        self.scroll_area.draw(ui, context);
        ui.popups.as_mut().unwrap().insert(self.id.clone(), self);
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
}

impl Layout for Popup {
    fn update(&mut self, ui: &mut Ui) {
        match ui.update_type {
            UpdateType::Init | UpdateType::ReInit => self.scroll_area.update(ui),
            _ => if self.open { self.scroll_area.update(ui); }
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        if !self.open { return; }
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.rectangle.render(&self.fill_render, pass);
        self.scroll_area.redraw(ui);
    }
}
