use crate::ui::Ui;
use std::any::Any;
use std::ops::DerefMut;
use crate::frame::App;
use crate::size::rect::Rect;
use crate::widgets::button::Button;
use crate::window::inner::InnerWindow;

pub struct Callback {
    // click: Option<Box<dyn FnMut(&mut dyn Any, &mut Context)>>,
    // slider: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, f32)>>,
    // checkbox: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, bool)>>,
    // spinbox: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, i32)>>,
    // textedit: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, &str)>>,
    // combo_item: Option<Box<dyn FnMut(&mut PaintComboBox, usize)>>,
}

impl Callback {
    // pub fn new() -> Self {
    //     Callback {
    //         click: None,
    //         slider: None,
    //         checkbox: None,
    //         spinbox: None,
    //         textedit: None,
    //         combo_item: None,
    //     }
    // }
    // pub fn set_click<A: 'static>(&mut self, f: fn(&mut A, &mut UiM)) {
    //     self.click = Some(Self::create_click(f));
    // }

    pub(crate) fn create_click<A: 'static>(mut f: impl FnMut(&mut A, &mut Button, &mut Ui) + 'static) -> Box<dyn FnMut(&mut Box<dyn App>, &mut Button, &mut Ui)> {
        Box::new(move |box_app, btn, uim| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, btn, uim);
        })
    }

    // pub fn click(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.click = f;
    //     res
    // }

    pub(crate) fn create_slider<A: 'static>(f: fn(&mut A, &mut Ui, f32)) -> Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, f32)> {
        Box::new(move |box_app, uim, value| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    pub(crate) fn create_inner_close<A: 'static>(mut f: impl FnMut(&mut A, InnerWindow, &mut Ui) + 'static) -> Box<dyn FnMut(&mut Box<dyn App>, InnerWindow, &mut Ui)> {
        Box::new(move |box_app, window, ui| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, window, ui)
        })

    }

    // pub fn slider(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, f32)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.slider = f;
    //     res
    // }

    pub(crate) fn create_check<A: 'static>(f: fn(&mut A, &mut Ui, bool)) -> Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, bool)> {
        Box::new(move |box_app, uim, value| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    // pub fn check(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, bool)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.checkbox = f;
    //     res
    // }

    pub(crate) fn create_spinbox<A: 'static, T: 'static>(f: fn(&mut A, &mut Ui, T)) -> Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, T)> {
        Box::new(move |box_app, uim, value| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    // pub fn spinbox(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, i32)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.spinbox = f;
    //     res
    // }

    pub(crate) fn create_textedit<A: 'static>(f: fn(&mut A, &mut Ui, String)) -> Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, String)> {
        Box::new(move |box_app, uim, value| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    // pub fn textedit(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, &str)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.textedit = f;
    //     res
    // }

    pub(crate) fn create_combobox<A: 'static, T: 'static>(f: fn(&mut A, &mut Ui, &T)) -> Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, &T)> {
        Box::new(move |box_app, uim, value| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }
    // pub fn combo_item(f: Option<Box<dyn FnMut(&mut PaintComboBox, usize)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.combo_item = f;
    //     res
    // }

    // pub(crate) fn create_item<W: 'static>(f: fn(&mut W, &String)) -> Box<dyn FnMut(&mut dyn Any, &String)> {
    //     Box::new(move |target, id| {
    //         let t = target.downcast_mut::<W>().unwrap();
    //         f(t, id)
    //     })
    // }
    //
    // pub(crate) fn create_t<A: 'static, T: 'static>(r: T, f: fn(&mut A, &mut Context, &T)) -> Box<dyn FnMut(&mut dyn Any, &mut Context)> {
    //     Box::new(move |target, uim| {
    //         let t = target.downcast_mut::<A>().unwrap();
    //         f(t, uim, &r)
    //     })
    // }

    pub(crate) fn create_list<A: 'static>(f: impl Fn(&mut A, &mut Ui) + 'static) -> Box<dyn Fn(&mut Box<dyn App>, &mut Ui)> {
        Box::new(move |box_app, uim| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, uim);
        })
    }
}

pub struct Response<'a> {
    pub id: &'a String,
    pub rect: &'a Rect,
}

impl<'a> Response<'a> {
    pub fn new(id: &'a String, rect: &'a Rect) -> Self {
        Response { id, rect }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }
}