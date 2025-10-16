use crate::frame::context::{ContextUpdate, UpdateType};
use crate::frame::App;
use crate::render::circle::param::CircleParam;
use crate::render::rectangle::param::RectParam;
use crate::render::{RenderKind, RenderParam};
use crate::response::{Callback, Response};
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::size::rect::Rect;
use crate::size::Geometry;
use crate::style::color::Color;
use crate::style::ClickStyle;
use crate::ui::Ui;
use crate::widgets::{Widget, WidgetChange, WidgetSize, WidgetState};
use crate::Offset;
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

    fill_render: RenderParam,
    slider_render: RenderParam,
    slided_render: RenderParam,

    offset: f32,
    geometry: Geometry,
    state: WidgetState,
}


impl Slider {
    pub fn new(v: f32) -> Slider {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(220, 220, 220);
        fill_style.fill.hovered = Color::rgb(220, 220, 220);
        fill_style.fill.clicked = Color::rgb(220, 220, 220);
        fill_style.border.inactive = Border::same(0.0).radius(Radius::same(3));
        fill_style.border.hovered = Border::same(0.0).radius(Radius::same(3));
        fill_style.border.clicked = Border::same(0.0).radius(Radius::same(3));

        let mut slider_style = ClickStyle::new();
        slider_style.fill.inactive = Color::rgb(56, 182, 244);
        slider_style.fill.hovered = Color::rgb(56, 182, 244);
        slider_style.fill.clicked = Color::rgb(56, 182, 244);
        slider_style.border.inactive = Border::same(0.0).color(Color::BLACK).radius(Radius::same(8));
        slider_style.border.hovered = Border::same(1.0).color(Color::BLACK).radius(Radius::same(8));
        slider_style.border.clicked = Border::same(1.0).color(Color::BLACK).radius(Radius::same(8));

        let mut slided_style = ClickStyle::new();
        slided_style.fill.inactive = Color::rgb(56, 182, 244);
        slided_style.fill.hovered = Color::rgb(56, 182, 244);
        slided_style.fill.clicked = Color::rgb(56, 182, 244);
        slided_style.border.inactive = Border::same(0.0).radius(Radius::same(3));
        slided_style.border.hovered = Border::same(0.0).radius(Radius::same(3));
        slided_style.border.clicked = Border::same(0.0).radius(Radius::same(3));
        let fill_param = RectParam::new().with_size(114.0, 6.0).with_style(fill_style);
        let slider_param = CircleParam::new(Rect::new().with_size(16.0, 16.0), slider_style);
        let slided_param = RectParam::new().with_size(114.0, 6.0).with_style(slided_style);
        Slider {
            id: crate::gen_unique_id(),
            value: v,
            range: 0.0..1.0,
            callback: None,
            contact_ids: vec![],
            fill_render: RenderParam::new(RenderKind::Rectangle(fill_param)),
            slider_render: RenderParam::new(RenderKind::Circle(slider_param)),
            slided_render: RenderParam::new(RenderKind::Rectangle(slided_param)),
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

    fn init(&mut self, ui: &mut Ui) {
        //分配大小
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        #[cfg(feature = "gpu")]
        //背景
        self.fill_render.init(ui, false, false);
        //已滑动背景
        let scale = self.value / (self.range.end - self.range.start);
        let width = self.slided_render.rect().width() * scale;
        self.slided_render.rect_mut().set_width(width);
        #[cfg(feature = "gpu")]
        self.slided_render.init(ui, false, false);
        //滑块
        self.slider_render.rect_mut().set_width(self.geometry.context_height());
        self.offset = self.value * self.fill_render.rect().width() / (self.range.end - self.range.start);
        self.slider_render.rect_mut().offset_x(&Offset::new().with_x(self.offset));
        #[cfg(feature = "gpu")]
        self.slider_render.init(ui, false, false);
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
            let mut fill_rect = ui.draw_rect.clone();
            fill_rect.contract(8.0, 5.0);
            self.fill_render.offset_to_rect(&fill_rect);
            self.slided_render.offset_to_rect(&fill_rect);
            let mut slider_rect = ui.draw_rect.clone();
            slider_rect.offset_x(&Offset::new().with_x(self.offset));

            slider_rect.set_height(ui.draw_rect.height());
            self.slider_render.offset_to_rect(&slider_rect);
        }

        if ui.widget_changed.contains(WidgetChange::Value) {
            if self.value >= self.range.end {
                self.value = self.range.end;
            } else if self.value <= self.range.start {
                self.value = self.range.start;
            }
            let scale = self.value / (self.range.end - self.range.start);
            self.slided_render.rect_mut().set_width(self.fill_render.rect().width() * scale);
            *self.slider_render.rect_mut() = self.geometry.context_rect();
            self.slider_render.rect_mut().set_width(self.geometry.context_height());
            let offset = self.value * self.fill_render.rect().width() / (self.range.end - self.range.start);
            self.offset = self.slider_render.rect_mut().offset_x_limit(offset, self.geometry.context_rect().dx());
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.fill_render.draw(ui, false, false);
        self.slided_render.draw(ui, false, false);
        self.slider_render.draw(ui, self.state.hovered || self.state.focused, self.state.pressed);
    }
}

impl Widget for Slider {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init | UpdateType::ReInit => self.init(ui),
            UpdateType::MouseMove => { //滑动
                if self.state.hovered_moving() {
                    let ox = ui.device.device_input.mouse.lastest.relative.x - self.fill_render.rect().dx().min;
                    let mut cl = ox / self.fill_render.rect().width();
                    if cl >= 1.0 {
                        self.offset = self.fill_render.rect().width();
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