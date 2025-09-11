use crate::ui::Ui;
use std::any::Any;
use std::ops::DerefMut;
use crate::frame::App;
use crate::InnerWindow;
use crate::widgets::WidgetSize;
use crate::widgets::button::Button;

pub struct Callback;

impl Callback {
    pub(crate) fn create_click<A: 'static>(mut f: impl FnMut(&mut A, &mut Button, &mut Ui) + 'static) -> Box<dyn FnMut(&mut Box<dyn App>, &mut Button, &mut Ui)> {
        Box::new(move |box_app, btn, uim| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, btn, uim);
        })
    }

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

    pub(crate) fn create_check<A: 'static>(f: fn(&mut A, &mut Ui, bool)) -> Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, bool)> {
        Box::new(move |box_app, uim, value| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    pub(crate) fn create_spinbox<A: 'static, T: 'static>(f: fn(&mut A, &mut Ui, T)) -> Box<dyn FnMut(&mut Box<dyn App>, &mut Ui, T)> {
        Box::new(move |box_app, uim, value| {
            let app = box_app.deref_mut() as &mut dyn Any;
            let t = app.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

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
    pub(crate) size: WidgetSize,
}

impl<'a> Response<'a> {
    pub fn new(id: &'a String, size: WidgetSize) -> Self {
        Response {
            id,
            size,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}