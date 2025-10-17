use crate::response::Response;
use crate::ui::Ui;
use crate::widgets::space::Space;
use std::any::{Any, TypeId};
use std::ops::{BitAnd, BitOr, BitOrAssign, Deref, DerefMut};
use crate::size::Geometry;
use crate::UpdateType;

pub mod label;
pub mod button;
pub mod image;
pub mod textedit;
pub mod scroll;
pub mod spinbox;
pub mod slider;
pub mod checkbox;
pub mod radio;
pub mod select;
pub mod rectangle;
pub mod item;
pub mod listview;
pub mod space;
pub mod processbar;
pub mod triangle;
pub mod circle;
pub mod table;
pub mod combo;

pub mod tab;

pub trait Widget: Any {
    ///后续更新调用
    fn update(&mut self, ui: &mut Ui) -> Response<'_>;
    ///控件的几何信息：位置、大小、最小大小、最大大小、间隔、对齐
    fn geometry(&mut self) -> &mut Geometry;
    ///控件状态信息: 焦点、按下、滑动、改变、禁用
    fn state(&mut self) -> &mut WidgetState;
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


#[derive(Default)]
pub struct WidgetState {
    focused: bool,
    hovered: bool,
    pressed: bool,
    changed: bool,
    pub(crate) disabled: bool,
    // selected: bool,
}

impl WidgetState {
    ///更新控件滑动状态，返回值通知是否重绘
    pub fn on_hovered(&mut self, hovered: bool) -> bool {
        self.changed = self.hovered != hovered;
        self.hovered = hovered;
        self.changed && !self.disabled
    }

    ///更新控件按下状态，返回值通知是否重绘
    pub fn on_pressed(&mut self, pressed: bool) -> bool {
        self.changed = self.pressed != pressed || self.focused != pressed;
        self.pressed = pressed;
        self.focused = pressed;
        self.changed && !self.disabled
    }

    ///更新控件点击时状态，返回值通知是否重绘
    pub fn on_clicked(&mut self, clicked: bool) -> bool {
        self.changed = clicked;
        self.pressed = false;
        self.focused = false;
        self.changed && !self.disabled
    }

    ///移除控件按下、焦点状态
    pub fn on_release(&mut self) -> bool {
        self.changed = self.pressed || self.focused;
        self.pressed = false;
        self.focused = false;
        self.changed && !self.disabled
    }

    ///控件按下的滑动移动
    pub fn hovered_moving(&self) -> bool {
        self.pressed && self.focused
    }

    ///disable-为是否启用样式，默认不启用
    pub fn handle_event(&mut self, ui: &mut Ui, geometry: &Geometry, disable: bool) {
        match ui.update_type {
            UpdateType::MouseMove => {
                let hovered = ui.device.device_input.hovered_at(&geometry.padding_rect());
                if self.on_pressed(hovered) && !disable {
                    ui.context.window.request_redraw();
                };
            }
            UpdateType::MousePress => {
                let pressed = ui.device.device_input.pressed_at(&geometry.padding_rect());
                if self.on_pressed(pressed) && !disable {
                    ui.context.window.request_redraw();
                };
            }
            UpdateType::MouseRelease => if self.on_release() && !disable { ui.context.window.request_redraw(); },
            _ => {}
        }
    }
}