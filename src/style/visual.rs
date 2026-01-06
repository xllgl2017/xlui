use crate::render::RenderParam;
use crate::shape::Shape;
use crate::{Rect, Ui, WidgetStyle};

pub struct Visual {
    render: RenderParam,
    disable: bool,
    foreground: bool,
}

impl Visual {
    pub fn new() -> Visual {
        Visual {
            #[cfg(feature = "gpu")]
            render: RenderParam::new(Shape::rectangle()),
            #[cfg(not(feature = "gpu"))]
            render: RenderParam::new(Shape::Rectangle),
            disable: true,
            foreground: false,
        }
    }

    #[cfg(feature = "gpu")]
    pub fn re_init(&mut self) {
        self.render.re_init();
    }

    pub fn with_enable(mut self) -> Visual {
        self.disable = false;
        self
    }

    pub fn enable(&mut self) -> &mut Visual {
        self.disable = false;
        self
    }

    pub fn with_style(mut self, style: VisualStyle) -> Visual {
        self.render.set_style(style);
        self
    }

    pub fn set_style(&mut self, style: VisualStyle) {
        self.render.set_style(style);
    }


    pub fn draw(&mut self, ui: &mut Ui, disabled: bool, hovered: bool, pressed: bool, foreground: bool) {
        if self.disable || self.foreground != foreground { return; }
        self.render.draw(ui, disabled, hovered, pressed);
    }

    pub fn foreground(&self) -> bool {
        self.foreground
    }

    pub fn disable(&self) -> bool {
        self.disable
    }

    pub fn with_size(mut self, w: f32, h: f32) -> Visual {
        self.render.rect.set_size(w, h);
        self
    }

    pub fn rect(&self) -> &Rect { self.render.rect() }

    pub fn rect_mut(&mut self) -> &mut Rect { self.render.rect_mut() }

    pub fn with_rect(mut self, rect: Rect) -> Visual {
        self.render.rect = rect;
        self
    }

    pub fn offset_to_rect(&mut self, rect: &Rect) {
        self.render.offset_to_rect(rect)
    }

    pub fn style(&self) -> &VisualStyle { &self.render.style }

    pub fn style_mut(&mut self) -> &mut VisualStyle { self.render.style_mut() }
}


#[derive(Clone)]
pub struct VisualStyle {
    pub disabled: WidgetStyle,
    pub inactive: WidgetStyle,
    pub hovered: WidgetStyle,
    pub pressed: WidgetStyle,
}

impl VisualStyle {
    pub fn new() -> VisualStyle {
        VisualStyle {
            disabled: WidgetStyle::new(),
            inactive: WidgetStyle::new(),
            hovered: WidgetStyle::new(),
            pressed: WidgetStyle::new(),
        }
    }

    pub fn same(style: WidgetStyle) -> VisualStyle {
        let mut res = VisualStyle::new();
        res.disabled = style.clone();
        res.inactive = style.clone();
        res.hovered = style.clone();
        res.pressed = style;
        res
    }

    pub fn dyn_style(&self, disabled: bool, hovered: bool, pressed: bool) -> &WidgetStyle {
        if disabled { return &self.disabled; }
        if pressed { return &self.pressed; }
        if hovered { return &self.hovered; }
        &self.inactive
    }
}
