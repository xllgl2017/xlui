use crate::response::Response;
use crate::size::rect::Rect;
use crate::ui::Ui;
use std::any::Any;
use std::ops::{BitAnd, BitOr, BitOrAssign, DerefMut};
use crate::Offset;

pub mod label;
// pub mod button;
// pub mod image;
// pub mod textedit;
pub mod scroll;
// pub mod spinbox;
// pub mod slider;
// pub mod checkbox;
// pub mod radio;
// pub mod combobox;
// pub mod select;
// pub mod rectangle;
// pub(crate) mod item;
// pub mod listview;
// pub mod processbar;
// pub mod triangle;
// pub mod circle;

pub trait Widget: Any {
    fn redraw(&mut self, ui: &mut Ui); //绘制调用
    fn update(&mut self, ui: &mut Ui) -> Response<'_>; //后续更新调用
}


#[derive(Copy, Clone, PartialEq)]
pub enum WidgetChange {
    None = 0,
    Position = 1 << 0, //位置、大小改变
    Value = 1 << 1, //值改变
    PositionAndValue = (1 << 0) | (1 << 1),
}

impl WidgetChange {
    pub fn unchanged(&self) -> bool {
        match self {
            WidgetChange::None => true,
            _ => false
        }
    }

    pub fn contains(self, other: WidgetChange) -> bool {
        (self & other) == other
    }

    fn from_bit(v: u32) -> WidgetChange {
        match v {
            0 => WidgetChange::None,
            1 => WidgetChange::Position,
            2 => WidgetChange::Value,
            3 => WidgetChange::PositionAndValue,
            _ => WidgetChange::None
        }
    }
}

impl BitOr for WidgetChange {
    type Output = WidgetChange;

    fn bitor(self, rhs: Self) -> Self::Output {
        let res = self as u32 | rhs as u32;
        WidgetChange::from_bit(res)
    }
}

impl BitOrAssign for WidgetChange {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

impl BitAnd for WidgetChange {
    type Output = WidgetChange;
    fn bitand(self, rhs: Self) -> Self::Output {
        let res = self as u32 & rhs as u32;
        WidgetChange::from_bit(res)
    }
}

pub struct WidgetKind {
    widget: Box<dyn Widget>,
    id: String,
    width: f32,
    height: f32,
    change: WidgetChange,
}

impl WidgetKind {
    pub fn new(ui: &mut Ui, mut widget: impl Widget) -> Self {
        let resp = widget.update(ui);
        WidgetKind {
            id: resp.id.to_string(),
            width: resp.width,
            height: resp.height,
            widget: Box::new(widget),
            change: WidgetChange::None,
        }
    }

    // pub fn offset(&mut self, o: &Offset, pr: &Rect) -> bool {
    //     self.rect.offset(o);
    //     !self.rect.out_of_rect(pr)
    // }
    pub fn update(&mut self, ui: &mut Ui) -> Response {
        ui.widget_changed = WidgetChange::Position;
        let resp = self.widget.update(ui);
        if resp.width != self.width || resp.height != self.height {
            self.id = resp.id.to_string();
            self.width = resp.width;
            self.height = resp.height;
            self.change = WidgetChange::Position;
        } else {
            self.change = WidgetChange::None;
        }
        resp
    }

    pub fn redraw(&mut self, ui: &mut Ui) {
        self.widget.redraw(ui);
    }

    // pub fn change_position(&mut self, x: f32, y: f32) {
    //     self.rect.set_x_min(x);
    //     self.rect.set_x_max(y);
    //     self.change = WidgetChange::Position;
    // }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn as_mut_<T: Widget>(&mut self) -> &mut T {
        let widget = self.widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<T>().unwrap()
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}