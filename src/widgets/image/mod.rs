use crate::paint::image::PaintImage;
use crate::paint::PaintTask;
use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::ui::{Ui, UiM};
use crate::widgets::Widget;
use crate::Pos;
use image::GenericImageView;

pub struct Image {
    pub(crate) id: String,
    pub(crate) source: &'static str,
    pub(crate) rect: Rect,
    size_mode: SizeMode,
}

impl Image {
    pub fn new(fp: &'static str) -> Self {
        Image {
            id: crate::gen_unique_id(),
            source: fp,
            rect: Rect {
                x: Pos {
                    min: 0.0,
                    max: 0.0,
                },
                y: Pos {
                    min: 300.0,
                    max: 400.0,
                },
            },
            size_mode: SizeMode::Fix,
        }
    }

    fn reset_size(&mut self, (width, height): (u32, u32)) {
        match self.size_mode {
            SizeMode::Auto => self.rect.set_size(width as f32, height as f32),
            SizeMode::FixWidth => {
                let scale = self.rect.height() / height as f32;
                self.rect.set_width(scale * width as f32)
            }
            SizeMode::FixHeight => {
                let scale = self.rect.width() / width as f32;
                self.rect.set_height(scale * height as f32);
            }
            _ => {}
        }
    }


    pub fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }

    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.rect.set_width(width);
        self.rect.set_height(height);
        self.size_mode = SizeMode::Fix;
        self
    }
}

impl Widget for Image {
    fn draw(&mut self, ui: &mut Ui) {
        let layout = ui.current_layout.as_mut().unwrap();
        self.rect = layout.available_rect.clone_with_size(&self.rect);
        let size = ui.ui_manage.context.render.image.insert_image(&ui.device, self.source.to_string(), self.source);
        self.reset_size(size);
        println!("image {:?}", self.rect);
        layout.alloc_rect(&self.rect);
        let task = PaintImage::new(ui, self);
        ui.add_paint_task(self.id.clone(), PaintTask::Image(task));
    }

    fn update(&mut self, uim: &mut UiM) {
        todo!() //替换图片
    }
}