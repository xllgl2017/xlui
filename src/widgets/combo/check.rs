use crate::key::Key;
use crate::render::triangle::param::TriangleParam;
use crate::render::RenderParam;
#[cfg(feature = "gpu")]
use crate::render::WrcRender;
use crate::response::{Callback, Response};
use crate::size::Geometry;
use crate::widgets::{WidgetChange, WidgetSize};
use crate::{Align, App, Border, BorderStyle, CheckBox, ClickStyle, Color, FillStyle, Offset, Padding, Popup, Radius, Rect, TextEdit, Ui, UpdateType, Widget};
use std::fmt::Display;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

/// ### CheckComboBox的示例用法
///```
/// use xlui::*;
///
/// fn combo_changed<A:App>(_:&mut A,_:&mut Ui,t:&i32){
///    println!("ComboBox的Item改变了：{}",t);
/// }
///
/// fn draw<A:App>(ui:&mut Ui){
///    //这里的data可以是任意实现了Display的类型
///    let data=vec![1,2,3,4];
///    let combo=CheckComboBox::new(data)
///        //设置打开的弹窗布局的高度
///        .with_popup_height(150.0)
///        //连接到Item改变的监听函数
///        .connect(combo_changed::<A>);
///    ui.add(combo);
/// }
/// ```

pub struct CheckComboBox<T> {
    pub(crate) id: String,
    popup_id: String,
    edit: TextEdit,
    data: Vec<T>,
    popup_rect: Rect,
    callback: Option<Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, &T)>>,
    allow_render: RenderParam<TriangleParam>,

    selected: Arc<RwLock<Vec<String>>>,
    changed: Arc<AtomicBool>,
}

impl<T: Display + 'static> CheckComboBox<T> {
    pub fn new(data: Vec<T>) -> Self {
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::rgb(230, 230, 230);
        fill_style.border.inactive = Border::same(1.0).radius(Radius::same(3)).color(Color::rgba(144, 209, 255, 255));
        let mut allow_style = ClickStyle::new();
        allow_style.fill = FillStyle::same(Color::BLACK);
        CheckComboBox {
            id: crate::gen_unique_id(),
            popup_id: "".to_string(),
            edit: TextEdit::single_edit("").with_width(100.0),
            data,
            popup_rect: Rect::new().with_size(100.0, 150.0),
            callback: None,
            selected: Arc::new(RwLock::new(vec![])),

            changed: Arc::new(AtomicBool::new(false)),
            allow_render: RenderParam::new(TriangleParam::new((0.0, 0.0).into(), (10.0, 0.0).into(), (5.0, 8.0).into(), allow_style)),
        }
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.edit.buffer().geometry.set_fix_size(width, height);
        self
    }

    // 设置popup的高度
    pub fn with_popup_height(mut self, height: f32) -> Self {
        self.popup_rect.set_height(height);
        self
    }

    fn add_item(&self, ui: &mut Ui, item: &T) {
        let value = item.to_string();
        let current = self.selected.clone();
        let state = self.changed.clone();
        let item = CheckBox::new(false, &value).with_width(self.popup_rect.width() - 10.0);
        let mut item = item.connect_inner(move || {
            let mut current = current.write().unwrap();
            match current.iter().position(|x| x == &value) {
                None => current.push(value.clone()),
                Some(index) => { current.remove(index); }
            }
            state.store(true, Ordering::SeqCst);
        });
        item.geometry_mut().an(Align::LeftCenter).pd(Padding::same(3.0));
        item.style_mut().param.set_style(ClickStyle {
            fill: FillStyle {
                inactive: Color::TRANSPARENT,
                hovered: Color::rgba(153, 193, 241, 220),
                clicked: Color::rgba(153, 193, 241, 220),
            },
            border: BorderStyle {
                inactive: Border::same(0.0),
                hovered: Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2)),
                clicked: Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2)),
            },
        });
        ui.add(item);
    }

    fn add_items(&self, ui: &mut Ui) {
        for datum in self.data.iter() {
            self.add_item(ui, datum);
        }
    }

    pub fn connect<A: 'static>(mut self, f: fn(&mut A, &mut Ui, &T)) -> Self {
        self.callback = Some(Callback::create_combobox(f));
        self
    }

    fn init(&mut self, ui: &mut Ui) {
        //下拉框布局
        let popup = Popup::new(ui, self.popup_rect.width(), self.popup_rect.height());
        self.popup_id = popup.id.clone();
        popup.show(ui, |ui| self.add_items(ui));
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        #[cfg(feature = "gpu")]
        //背景
        self.allow_render.init_triangle(ui, false, false);
        //文本
        self.edit.update(ui);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if self.changed.load(Ordering::SeqCst) {
            let selected = self.selected.read().unwrap();
            let value = selected.join("; ");
            self.edit.update_text(ui, value);
            self.changed.store(false, Ordering::SeqCst);
        }
        // self.edit.update_text(ui, "sdfsdf".to_string());
        // let select = self.selected.read().unwrap();
        // if *select != self.previous_select {
        //     self.previous_select = select.clone();
        //     if let Some(ref select) = self.previous_select {
        //         self.edit.update_text(ui, select.clone());
        //         if let Some(ref mut callback) = self.callback {
        //             let app = ui.app.take().unwrap();
        //             let t = self.data.iter().find(|x| &x.to_string() == select).unwrap();
        //             callback(app, ui, t);
        //             ui.app.replace(app);
        //             ui.context.window.request_redraw();
        //         }
        //         self.changed = true;
        //     }
        //
        //     let popup = &mut ui.popups.as_mut().unwrap()[&self.popup_id];
        //     popup.request_state(false);
        // }

        if ui.widget_changed.contains(WidgetChange::Position) {
            self.popup_rect.offset_to_rect(&ui.draw_rect);
            self.popup_rect.offset_y(&Offset::new().covered().with_y(self.edit.buffer().geometry.height() + 5.0));
            ui.popups.as_mut().unwrap()[&self.popup_id].set_rect(self.popup_rect.clone());
            let mut allow_rect = ui.draw_rect.clone();
            allow_rect.set_x_min(self.edit.buffer().geometry.right() - 15.0);
            allow_rect.add_min_y(5.0);
            self.allow_render.param.offset_to_rect(&allow_rect);
            #[cfg(feature = "gpu")]
            self.allow_render.update(ui, false, false);
        }
    }

    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        self.edit.update(ui);
        #[cfg(feature = "gpu")]
        let pass = ui.pass.as_mut().unwrap();
        #[cfg(feature = "gpu")]
        ui.context.render.triangle.render(&self.allow_render, pass);
    }

    //初始化时设置当前item，默认为None
    // pub fn with_current_index(mut self, index: usize) -> Self {
    //     let current = self.data[index].to_string();
    //     self.edit.update_text().set_text(current.clone());
    //     *self.selected.write().unwrap() = Some(current);
    //     self
    // }
}


impl<T: Display + 'static> Widget for CheckComboBox<T> {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::MouseRelease => {
                let clicked = ui.device.device_input.mouse.clicked.load(Ordering::SeqCst);
                self.edit.update(ui);
                if clicked && self.edit.buffer().geometry.rect().has_position(ui.device.device_input.mouse.lastest.relative) {
                    let popup = &mut ui.popups.as_mut().unwrap()[&self.popup_id];
                    popup.request_state(true);
                    ui.update_type = UpdateType::None;
                    ui.context.window.request_redraw();
                }
            }
            UpdateType::KeyRelease(ref key) => {
                match key {
                    Key::Backspace => {}
                    _ => {}
                }
            }
            _ => { self.edit.update(ui); }
        }
        Response::new(&self.id, WidgetSize::same(self.edit.buffer().geometry.width(), self.edit.buffer().geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.edit.buffer().geometry
    }
}