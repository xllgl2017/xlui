use std::mem;
use crate::frame::context::UpdateType;
use crate::frame::App;
use crate::layout::{Layout, LayoutItem, LayoutKind};
use crate::response::Callback;
use crate::size::border::Border;
use crate::size::radius::Radius;
use crate::style::color::Color;
use crate::style::{BorderStyle, ClickStyle, FillStyle};
use crate::ui::Ui;
use crate::widgets::item::ItemWidget;
use crate::{HorizontalLayout, Label, Padding, ScrollWidget};
use std::ops::Range;
use std::sync::{Arc, RwLock};
use crate::layout::recycle::RecycleLayout;
use crate::widgets::WidgetKind;

pub enum ListUpdate<T> {
    Push(T),
    Remove,
}

/// # ListView的是使用示例
/// ```
/// use xlui::*;
///
/// struct XlUi{
///     list_view: ListView<i32>
/// }
///
/// impl XlUi{
///     pub fn new()->XlUi{
///         //这里的data可以是任意类型
///         let data=vec![1,2,3,4];
///         let mut list_view=ListView::new(data)
///             //设置控件大小
///             .with_size(100.0,100.0);
///         //设置Item的Widget
///         list_view.set_item_widget(|ui,datum|{
///             ui.label(format!("Item-{}",datum))
///         });
///         XlUi{
///             list_view
///         }
///     }
///
///
///     fn item_changed(&mut self,ui:&mut Ui){
///         //获取当前已选择的Item
///         if let Some(datum) = self.list_view.current() {
///             println!("list: {:?}", self.list_view.current());
///         }
///         //添加一条Item
///         self.list_view.push(0);
///         //删除一条Item
///         self.list_view.remove(0);
///         //获取当前已选择Item的索引
///         self.list_view.current_index();
///     }
/// }
///
/// impl App for XlUi{
///     fn draw(&mut self, ui: &mut Ui) {
///         //设置Item改变回调函数
///         self.list_view.set_callback(Self::item_changed);
///         //监听滚动，并更新Item数据显示
///         self.list_view.on_scrolling(|datum,layout|{
///             let item:&mut Label=layout.get_widget(&"list_item".to_string()).unwrap();
///             item.set_text(datum);
///         });
///         self.list_view.show(ui);
///     }
///
///     fn update(&mut self, ui: &mut Ui) {
///         //这里需要调update，否则push、remove不起作用
///         self.list_view.update(ui);
///     }
///
///
/// }
/// ```

pub struct ListView<T> {
    lid: String,
    data: Vec<T>,
    current: Arc<RwLock<Option<String>>>,
    callback: Arc<Option<Box<dyn Fn(&mut Box<dyn App>, &mut Ui)>>>,
    onscroll: Box<dyn Fn(&T, &mut LayoutKind)>,
    dyn_item_widget: Box<dyn Fn(&mut Ui, &T)>,
    updates: Vec<ListUpdate<T>>,
    width: f32,
    height: f32,
    previous_display: Range<usize>,
    item_height: f32,
    hovered: Option<usize>,
    selected: Option<usize>,
}

impl<T: 'static> ListView<T> {
    pub fn new(data: Vec<T>) -> Self {
        ListView {
            lid: "".to_string(),
            data,
            current: Arc::new(RwLock::new(None)),
            callback: Arc::new(None),
            onscroll: Box::new(|_, _| {}),
            dyn_item_widget: Box::new(|ui, _| { ui.add(Label::new("ListItem").with_id("list_item")); }),
            updates: vec![],
            width: 100.0,
            height: 150.0,
            previous_display: 0..0,
            item_height: 38.0,
            hovered: None,
            selected: None,
        }
    }


    pub fn with_size(mut self, w: f32, h: f32) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    pub fn with_item_height(mut self, h: f32) -> Self {
        self.item_height = h;
        self
    }

    pub fn set_item_widget(&mut self, item_widget: impl Fn(&mut Ui, &T) + 'static) {
        self.dyn_item_widget = Box::new(item_widget);
    }


    fn item_widget(&self, ui: &mut Ui, datum: &T, index: usize) -> LayoutItem {
        let previous_update = ui.update_type.clone();
        ui.update_type = UpdateType::Init;
        let style = ClickStyle {
            fill: FillStyle {
                inactive: Color::TRANSPARENT,
                hovered: Color::rgba(153, 193, 241, 220),
                clicked: Color::rgba(153, 193, 241, 220),
            },
            border: BorderStyle {
                inactive: Border::same(1.0).radius(Radius::same(3)).color(Color::rgb(190, 190, 190)),
                hovered: Border::same(0.0).radius(Radius::same(3)),
                clicked: Border::same(0.0).radius(Radius::same(3)),
            },
        };
        let current = self.current.clone();
        let callback = self.callback.clone();
        let item_layout = HorizontalLayout::left_to_right().with_size(self.width - 18.0, self.item_height)
            .with_padding(Padding::same(2.0));
        let mut item = ItemWidget::new(LayoutKind::new(item_layout), index.to_string()).with_style(style)
            .parent(self.current.clone()).connect(move |item_id, ui| {
            current.write().unwrap().replace(item_id.to_string());
            if let Some(callback) = callback.as_ref() {
                let app = ui.app.take().unwrap();
                callback(app, ui);
                ui.app = Some(app);
            }
            println!("item clicked");
        });
        item.show(ui, |ui| (self.dyn_item_widget)(ui, &datum));
        let item = WidgetKind::new(ui, item);
        ui.update_type = previous_update;
        LayoutItem::Widget(item)
    }

    pub fn current_index(&self) -> Option<usize> {
        let current = self.current.read().unwrap();
        let current = current.as_ref()?.parse::<usize>().unwrap();
        Some(current)
    }

    pub fn current(&self) -> Option<&T> {
        let index = self.selected?;
        self.data.get(index)
    }

    fn _remove(&mut self, ui: &mut Ui) {
        let area: &mut ScrollWidget = ui.layout().get_widget(&self.lid).unwrap();
        let recycle: &mut RecycleLayout = area.layout.as_mut().unwrap().as_mut_().unwrap();
        recycle.remove_item();
        recycle.items_mut().iter_mut().for_each(|x| {
            let item: &mut ItemWidget = x.widget_mut().unwrap();
            item.restore_status(false, 0.to_string());
        });
        let h = recycle.size().rh;
        area.reset_context_height(h);
        self.previous_display = 0..0;
        self.current.write().unwrap().take();
        self.hovered.take();
        self.selected.take();
        if let UpdateType::Draw = ui.update_type {} else { ui.context.window.request_redraw(); }

        // let mut layout = ui.layout.take().expect("应在App::update中调用");
        // let area = layout.get_layout(&self.lid).expect("找不到ListView");
        // area.remove_widget(ui, &wid);
        // if let LayoutKind::ScrollArea(area) = area {
        //     area.reset_context_height();
        // }
        // ui.layout = Some(layout);
    }

    pub fn remove(&mut self, index: usize) -> T {
        let datum = self.data.remove(index);
        self.updates.push(ListUpdate::Remove);
        // let (wid, t) = self.items.remove_map_by_index(index);
        // let mut current = self.current.write().unwrap();
        // if current.as_ref() == Some(&wid) { *current = None; }
        // self.updates.push(ListUpdate::Remove(wid));
        datum
    }

    fn _push(&mut self, datum: T, ui: &mut Ui) {
        let area: &mut ScrollWidget = ui.layout().get_widget(&self.lid).unwrap();
        let recycle: &mut RecycleLayout = area.layout.as_mut().unwrap().as_mut_().unwrap();
        if recycle.items().len() < recycle.draw_count() {
            let item = self.item_widget(ui, &datum, self.data.len());
            let area: &mut ScrollWidget = ui.layout().get_widget(&self.lid).unwrap();
            let recycle: &mut RecycleLayout = area.layout.as_mut().unwrap().as_mut_().unwrap();
            recycle.add_item(item);
            let h = recycle.size().rh;
            area.reset_context_height(h);
        } else {
            let area: &mut ScrollWidget = ui.layout().get_widget(&self.lid).unwrap();
            let recycle: &mut RecycleLayout = area.layout.as_mut().unwrap().as_mut_().unwrap();
            recycle.add_item_empty();
            let h = recycle.size().rh;
            area.reset_context_height(h);
        }
        self.data.push(datum);
        if let UpdateType::Draw = ui.update_type {} else { ui.context.window.request_redraw(); }
        // let mut layout = ui.layout.take().expect("应在App::update中调用");
        // let area = layout.get_layout(&self.lid).expect("找不到ListView");
        // if let LayoutKind::ScrollArea(area) = area {
        //     ui.layout = Some(LayoutKind::Vertical(area.layout.take().unwrap()));
        //     ui.update_type = UpdateType::Init;
        //     let wid = self.item_widget(ui, &datum);
        //     if let LayoutKind::Vertical(layout) = ui.layout.take().unwrap() {
        //         area.layout = Some(layout);
        //     }
        //     ui.update_type = UpdateType::None;
        //     area.reset_context_height();
        //     self.items.insert(wid, datum);
        // }
        // ui.layout = Some(layout);
    }

    pub fn push(&mut self, datum: T) {
        self.updates.push(ListUpdate::Push(datum));
    }

    pub fn set_callback<A: App>(&mut self, f: impl Fn(&mut A, &mut Ui) + 'static) {
        self.callback = Arc::new(Some(Callback::create_list(f)));
    }

    pub fn on_scrolling(&mut self, func: impl Fn(&T, &mut LayoutKind) + 'static) {
        self.onscroll = Box::new(func);
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let layout = RecycleLayout::new().with_item_height(self.item_height);
        let mut area = ScrollWidget::vertical().with_layout(layout).with_size(self.width, self.height);
        self.lid = area.id.clone();
        let mut fill_style = ClickStyle::new();
        fill_style.fill.inactive = Color::TRANSPARENT;
        fill_style.fill.hovered = Color::TRANSPARENT;
        fill_style.fill.clicked = Color::TRANSPARENT;
        fill_style.border.inactive = Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::same(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        area.set_style(fill_style);
        area.show(ui, |ui| {
            let recycle: &mut RecycleLayout = ui.layout().as_mut_().unwrap();
            let draw_count = recycle.draw_count();
            for i in 0..self.data.len() {
                if i <= draw_count {
                    let item = self.item_widget(ui, &self.data[i], i);
                    ui.layout().add_item(item);
                } else {
                    let recycle: &mut RecycleLayout = ui.layout().as_mut_().unwrap();
                    recycle.add_item_empty();
                }
            }
        });
    }

    fn update_view(&mut self, ui: &mut Ui) {
        for update in mem::take(&mut self.updates) {
            match update {
                ListUpdate::Push(datum) => self._push(datum, ui),
                ListUpdate::Remove => self._remove(ui),
            }
        }
    }

    pub fn update(&mut self, ui: &mut Ui) {
        self.update_view(ui);
        match ui.update_type {
            UpdateType::Draw => {
                let area: &mut ScrollWidget = ui.layout().get_widget(&self.lid).unwrap();
                let recycle_layout: &mut RecycleLayout = area.layout.as_mut().unwrap().as_mut_().unwrap();
                let display = recycle_layout.display_range();
                if display == &self.previous_display { return; }
                let mut start = self.previous_display.start;
                recycle_layout.items_mut().iter_mut().for_each(|x| {
                    let item: &mut ItemWidget = x.widget_mut().unwrap();
                    if item.state().hovered { self.hovered = Some(start); }
                    // if selected { self.selected = Some(start); }
                    start += 1;
                });
                let display = recycle_layout.display_range();
                let mut start = display.start;
                for item in recycle_layout.items_mut().iter_mut() {
                    let item: &mut ItemWidget = item.widget_mut().unwrap();
                    item.restore_status(self.hovered == Some(start),  start.to_string());
                    (self.onscroll)(&self.data[start], item.layout());
                    start += 1;
                    if start == self.data.len() { break; }
                }
                self.previous_display = recycle_layout.display_range().clone();
            }
            _ => {}
        }
        // for update in mem::take(&mut self.updates) {
        //     match update {
        //         ListUpdate::Push(datum) => self._push(datum, ui),
        //         ListUpdate::Remove(wid) => self._remove(wid, ui)
        //     }
        // }
    }
}