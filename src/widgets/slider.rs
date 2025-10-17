use crate::frame::context::{ContextUpdate, UpdateType};
use crate::frame::App;
use crate::render::{RenderParam, Visual, VisualStyle, WidgetStyle};
use crate::response::{Callback, Response};
use crate::shape::Shape;
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::{Offset, Shadow};
use std::ops::Range;

/// ### Slider的示例用法
/// ```
/// use xlui::*;
///
/// fn slider_changed<A:App>(_:&mut A,_:&mut Ui,v:f32){
///     println!("Slider改变了:{}",v);
/// }
///
/// fn draw<A:App>(ui:&mut Ui){
///     //快速创建一个Slider
///     ui.slider(10.0,0.0..100.0)
///         //设置回调函数
///         .set_callback(slider_changed::<A>);
///     let slider=Slider::new(10.0)
///         //关联ID为my_spinbox的控件
///         .contact("my_spinbox")
///         //连接到Slider值监听函数
///         .connect(slider_changed::<A>)
///         //设置控件ID
///         .id("my_slider")
///         //设置Slider值的范围
///         .with_range(0.0..100.0);
///     ui.add(slider);
/// }
/// ```
pub struct Slider {
    pub(crate) id: String,
    value: f32,
    range: Range<f32>,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, f32)>>,
    contact_ids: Vec<String>,

    visual: Visual,
    bg_render: RenderParam,
    slider_render: RenderParam,
    slided_render: RenderParam,

    offset: f32,
    geometry: Geometry,
    state: WidgetState,
}


impl Slider {
    pub fn new(v: f32) -> Slider {
        let bg_style = VisualStyle::same(WidgetStyle {
            fill: Color::rgb(220, 220, 220),
            border: Border::same(0.0),
            radius: Radius::same(3),
            shadow: Shadow::new(),
        });
        let mut slider_style = VisualStyle::same(WidgetStyle {
            fill: Color::rgb(56, 182, 244),
            border: Border::same(1.0).color(Color::BLACK),
            radius: Radius::same(8),
            shadow: Shadow::new(),
        });
        slider_style.inactive.border.set_same(0.0);
        let slided_style = VisualStyle::same(WidgetStyle {
            fill: Color::rgb(56, 182, 244),
            border: Border::same(0.0),
            radius: Radius::same(3),
            shadow: Shadow::new(),
        });
        Slider {
            id: crate::gen_unique_id(),
            value: v,
            range: 0.0..1.0,
            callback: None,
            contact_ids: vec![],
            visual: Visual::new().with_size(130.0,16.0),
            bg_render: RenderParam::new(Shape::Rectangle).with_size(114.0, 6.0).with_style(bg_style),
            slider_render: RenderParam::new(Shape::Circle).with_style(slider_style).with_size(16.0, 16.0),
            slided_render: RenderParam::new(Shape::Rectangle).with_style(slided_style).with_size(114.0, 6.0),
            offset: 0.0,
            geometry: Geometry::new().with_context_size(130.0, 16.0),
            state: WidgetState::default(),
        }
    }

    pub fn id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, f32)) -> Self {
        self.callback = Some(Callback::create_slider(f));
        self
    }

    pub fn with_range(mut self, range: Range<f32>) -> Self {
        self.range = range;
        self
    }

    pub fn set_callback<A: App>(&mut self, f: fn(&mut A, &mut Ui, f32)) {
        self.callback = Some(Callback::create_slider(f));
    }

    ///控件关联，id为其他控件的id
    pub fn contact(mut self, id: impl ToString) -> Self {
        self.contact_ids.push(id.to_string());
        self
    }

    fn init(&mut self) {
        //分配大小
        self.re_init();
    }

    fn re_init(&mut self) {
        //已滑动背景
        let scale = self.value / (self.range.end - self.range.start);
        let width = self.slided_render.rect().width() * scale;
        self.slided_render.rect_mut().set_width(width);
        //滑块
        self.slider_render.rect_mut().set_width(self.geometry.context_height());
        self.offset = self.value * self.bg_render.rect().width() / (self.range.end - self.range.start);
        self.slider_render.rect_mut().offset_x(&Offset::new().with_x(self.offset));
        self.state.changed = true;
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if let Some(v) = ui.context.updates.remove(&self.id) {
            v.update_f32(&mut self.value);
            ui.widget_changed |= WidgetChange::Value;
        }
        if self.state.changed { ui.widget_changed |= WidgetChange::Value; }
        self.state.changed = false;
        if ui.widget_changed.contains(WidgetChange::Position) {
            self.geometry.offset_to_rect(&ui.draw_rect);
            self.visual.offset_to_rect(&ui.draw_rect);
            let mut fill_rect = ui.draw_rect.clone();
            fill_rect.contract(8.0, 5.0);
            self.bg_render.offset_to_rect(&fill_rect);
            self.slided_render.offset_to_rect(&fill_rect);
            self.slider_render.offset_to_rect(&ui.draw_rect);
            self.slider_render.rect_mut().offset(&Offset::new().with_x(self.offset));
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            if self.value >= self.range.end {
                self.value = self.range.end;
            } else if self.value <= self.range.start {
                self.value = self.range.start;
            }
            let scale = self.value / (self.range.end - self.range.start);
            self.slided_render.rect_mut().set_width(self.bg_render.rect().width() * scale);
            *self.slider_render.rect_mut() = self.geometry.context_rect();
            self.slider_render.rect_mut().set_width(self.geometry.context_height());
            let offset = self.value * self.bg_render.rect().width() / (self.range.end - self.range.start);
            self.offset = self.slider_render.rect_mut().offset_x_limit(offset, self.geometry.context_rect().dx());
            self.slider_render.rect_mut().offset(&Offset::new().with_x(self.offset));
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.visual.draw(ui, self.state.disabled, false, false, false);
        self.bg_render.draw(ui, false, false, false);
        self.slided_render.draw(ui, false, false, false);
        self.slider_render.draw(ui, self.state.disabled, self.state.hovered || self.state.focused, self.state.pressed);
        self.visual.draw(ui, self.state.disabled, false, false, true);
    }
}

impl Widget for Slider {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(),
            #[cfg(feature = "gpu")]
            UpdateType::ReInit => {
                self.fill_render.re_init();
                self.slided_render.re_init();
                self.slider_render.re_init();
            }
            UpdateType::MouseMove => { //滑动
                if self.state.hovered_moving() {
                    let ox = ui.device.device_input.mouse.lastest.relative.x - self.bg_render.rect().dx().min;
                    let mut cl = ox / self.bg_render.rect().width();
                    if cl >= 1.0 {
                        self.offset = self.bg_render.rect().width();
                        cl = 1.0;
                    } else if cl <= 0.0 {
                        self.offset = 0.0;
                        cl = 0.0;
                    }
                    let cv = (self.range.end - self.range.start) * cl;
                    self.value = self.range.start + cv;
                    self.state.changed = true;
                    if let Some(ref mut callback) = self.callback {
                        let app = ui.app.take().unwrap();
                        callback(app, ui, self.value);
                        ui.app.replace(app);
                    }
                    ui.send_updates(&self.contact_ids, ContextUpdate::F32(self.value));
                    ui.update_type = UpdateType::None;
                    ui.context.window.request_redraw();
                    return Response::new(&self.id, WidgetSize::same(self.geometry.margin_width(), self.geometry.margin_height()));
                }
                let hovered = ui.device.device_input.hovered_at(self.slider_render.rect());
                if self.state.on_hovered(hovered) { ui.context.window.request_redraw(); };
            }
            UpdateType::MousePress => {
                let pressed = ui.device.device_input.pressed_at(self.slider_render.rect());
                if self.state.on_pressed(pressed) { ui.context.window.request_redraw() }
            }
            UpdateType::MouseRelease => if self.state.on_release() { ui.context.window.request_redraw(); },
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.margin_width(), self.geometry.margin_height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }

    fn state(&mut self) -> &mut WidgetState {
        &mut self.state
    }
}