use crate::frame::context::Context;
use std::any::Any;

pub enum DrawnEvent {
    None,
    Hover,
    Click,
}

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

    pub(crate) fn create_click<A: 'static>(mut f: impl FnMut(&mut A, &mut Context) + 'static) -> Box<dyn FnMut(&mut dyn Any, &mut Context)> {
        Box::new(move |target, uim| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim);
        })
    }

    // pub fn click(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.click = f;
    //     res
    // }

    pub(crate) fn create_slider<A: 'static>(f: fn(&mut A, &mut Context, f32)) -> Box<dyn FnMut(&mut dyn Any, &mut Context, f32)> {
        Box::new(move |target, uim, value| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    // pub fn slider(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, f32)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.slider = f;
    //     res
    // }

    pub(crate) fn create_check<A: 'static>(f: fn(&mut A, &mut Context, bool)) -> Box<dyn FnMut(&mut dyn Any, &mut Context, bool)> {
        Box::new(move |target, uim, value| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    // pub fn check(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, bool)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.checkbox = f;
    //     res
    // }

    pub(crate) fn create_spinbox<A: 'static>(f: fn(&mut A, &mut Context, i32)) -> Box<dyn FnMut(&mut dyn Any, &mut Context, i32)> {
        Box::new(move |target, uim, value| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    // pub fn spinbox(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, i32)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.spinbox = f;
    //     res
    // }

    pub(crate) fn create_textedit<A: 'static>(f: fn(&mut A, &mut Context, String)) -> Box<dyn FnMut(&mut dyn Any, &mut Context, String)> {
        Box::new(move |target, uim, value| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }

    // pub fn textedit(f: Option<Box<dyn FnMut(&mut dyn Any, &mut Context, &str)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.textedit = f;
    //     res
    // }

    pub(crate) fn create_combobox<A: 'static>(f: fn(&mut A, &mut Context, usize)) -> Box<dyn FnMut(&mut dyn Any, &mut Context, usize)> {
        Box::new(move |target, uim, value| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim, value)
        })
    }
    // pub fn combo_item(f: Option<Box<dyn FnMut(&mut PaintComboBox, usize)>>) -> Self {
    //     let mut res = Callback::new();
    //     res.combo_item = f;
    //     res
    // }

    pub(crate) fn create_item<A: 'static>(r: usize, f: fn(&mut A, &mut Context, usize)) -> Box<dyn FnMut(&mut dyn Any, &mut Context)> {
        Box::new(move |target, uim| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim, r)
        })
    }

    pub(crate) fn create_t<A: 'static, T: 'static>(r: T, f: fn(&mut A, &mut Context, &T)) -> Box<dyn FnMut(&mut dyn Any, &mut Context)> {
        Box::new(move |target, uim| {
            let t = target.downcast_mut::<A>().unwrap();
            f(t, uim, &r)
        })
    }
}