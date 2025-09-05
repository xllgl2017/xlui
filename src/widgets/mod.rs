use crate::response::Response;
use crate::ui::Ui;
use std::any::{Any, TypeId};
use std::ops::{BitAnd, BitOr, BitOrAssign, Deref, DerefMut};
use crate::widgets::space::Space;

pub mod label;
pub mod button;
pub mod image;
pub mod textedit;
pub mod scroll;
pub mod spinbox;
pub mod slider;
pub mod checkbox;
pub mod radio;
pub mod combobox;
pub mod select;
pub mod rectangle;
pub(crate) mod item;
pub mod listview;
pub mod space;
pub mod processbar;
pub mod triangle;
pub mod circle;
pub mod table;
pub mod column;
pub mod cell;

pub type UiDraw = Box<dyn Fn(&mut Ui)>;
pub mod tab;

pub trait Widget: Any {
    // #[deprecated="use Widget::update"]
    // fn redraw(&mut self, ui: &mut Ui); //绘制调用
    fn update(&mut self, ui: &mut Ui) -> Response<'_>; //后续更新调用
    // #[allow(unused_attributes)]
    // fn store(&mut self, datum: &dyn Any) {}
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
            width: resp.size.dw,
            height: resp.size.dh,
            widget: Box::new(widget),
            change: WidgetChange::None,
        }
    }

    // pub fn offset(&mut self, o: &Offset, pr: &Rect) -> bool {
    //     self.rect.offset(o);
    //     !self.rect.out_of_rect(pr)
    // }
    pub fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        ui.widget_changed = WidgetChange::Position;
        let resp = self.widget.update(ui);
        if resp.size.dw != self.width || resp.size.dh != self.height {
            self.id = resp.id.to_string();
            self.width = resp.size.dw;
            self.height = resp.size.dh;
            self.change = WidgetChange::Position;
        } else {
            self.change = WidgetChange::None;
        }
        resp
    }

    #[deprecated = "use update"]
    pub fn redraw(&mut self, ui: &mut Ui) {
        self.widget.update(ui);
    }

    // pub fn change_position(&mut self, x: f32, y: f32) {
    //     self.rect.set_x_min(x);
    //     self.rect.set_x_max(y);
    //     self.change = WidgetChange::Position;
    // }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn as_mut_<T: Widget>(&mut self) -> Option<&mut T> {
        let widget = self.widget.deref_mut() as &mut dyn Any;
        widget.downcast_mut::<T>()
    }

    pub fn as_<T: Widget>(&self) -> Option<&T> {
        let widget = self.widget.deref() as &dyn Any;
        widget.downcast_ref::<T>()
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn is_space(&self) -> bool {
        let widget = self.widget.deref() as &dyn Any;
        widget.type_id() == TypeId::of::<Space>()
    }
}

#[derive(PartialEq, Clone)]
pub struct WidgetSize {
    pub(crate) dw: f32, //绘制宽度
    pub(crate) dh: f32, //绘制高度
    pub(crate) rw: f32, //真实宽度
    pub(crate) rh: f32, //真实高度
}

impl WidgetSize {
    pub fn same(w: f32, h: f32) -> WidgetSize {
        WidgetSize {
            dw: w,
            dh: h,
            rw: w,
            rh: h,
        }
    }
}