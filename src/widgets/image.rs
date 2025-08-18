//! ### Image的示例用法
//!```
//! use xlui::ui::Ui;
//! use xlui::widgets::image::Image;
//! use xlui::widgets::Widget;
//!
//! fn draw(ui:&mut Ui){
//!    let mut image=Image::new("logo.png")
//!        //设置控件大小
//!        .with_size(30.0,30.0)
//!        //设置控件ID
//!        .with_id("my_image");
//!    //快速创建图片
//!    ui.add(image);
//!
//! }
//!
//! fn update(ui:&mut Ui){
//!    //注意这里不应该在每次调用update的时候都更新图片。建议给一个状态，更新状态是否修改图片
//!    //修改图片
//!    ui.set_image_handle("logo_2.png");
//!    //获取Image的可变引用
//!    let image:&mut Image=ui.get_widget("my_image").unwrap();
//!    //修改图片
//!    image.set_image("logo_2.png");
//! }
//! ```

use crate::size::rect::Rect;
use crate::size::SizeMode;
use crate::ui::Ui;
use crate::vertex::ImageVertex;
use crate::widgets::Widget;
use wgpu::util::DeviceExt;
use crate::frame::context::UpdateType;
use crate::response::Response;

pub struct Image {
    pub(crate) id: String,
    source: String,
    pub(crate) rect: Rect,
    size_mode: SizeMode,

    vertices: Vec<ImageVertex>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    changed: bool,
}

impl Image {
    pub fn new(fp: &str) -> Self {
        Image {
            id: crate::gen_unique_id(),
            source: fp.to_string(),
            rect: Rect::new(),
            size_mode: SizeMode::Fix,
            vertices: vec![],
            vertex_buffer: None,
            index_buffer: None,
            changed: false,
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

    pub fn set_size(&mut self, width: f32, height: f32) {
        self.rect.set_size(width, height);
        self.size_mode = SizeMode::Fix;
    }

    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn set_image(&mut self, source: impl ToString) {
        self.source = source.to_string();
    }

    fn init(&mut self, ui: &mut Ui) {
        self.rect = ui.layout().available_rect().clone_with_size(&self.rect);
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        let size = ui.context.render.image.insert_image(&ui.device, self.source.to_string(), self.source.as_str());
        self.reset_size(size);
        let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
        self.vertices = vec![
            ImageVertex::new_coord(self.rect.left_top(), [0.0, 0.0], &ui.context.size),
            ImageVertex::new_coord(self.rect.left_bottom(), [0.0, 1.0], &ui.context.size),
            ImageVertex::new_coord(self.rect.right_bottom(), [1.0, 1.0], &ui.context.size),
            ImageVertex::new_coord(self.rect.right_top(), [1.0, 0.0], &ui.context.size)
        ];
        let vertex_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(self.vertices.as_slice()),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        self.vertex_buffer = Some(vertex_buffer);
        let index_buffer = ui.device.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });
        self.index_buffer = Some(index_buffer);
    }

    fn update_buffer(&mut self, ui: &mut Ui) {
        if !self.changed && !ui.context.resize { return; }
        self.changed = false;
        for (index, v) in self.vertices.iter_mut().enumerate() {
            match index {
                0 => v.position = self.rect.left_top(),
                1 => v.position = self.rect.left_bottom(),
                2 => v.position = self.rect.right_bottom(),
                3 => v.position = self.rect.right_top(),
                _ => {}
            }
            v.screen_size = ui.context.size.as_gamma_size();
        }
        ui.device.queue.write_buffer(
            self.vertex_buffer.as_ref().unwrap(), 0,
            bytemuck::cast_slice(self.vertices.as_slice()));
    }
}

impl Widget for Image {
    fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.image.render(
            &self.source.to_string(),
            self.vertex_buffer.as_ref().unwrap(),
            self.index_buffer.as_ref().unwrap(),
            pass,
        );
    }

    fn update(&mut self, ui: &mut Ui) -> Response {
        match ui.update_type {
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            UpdateType::Offset(ref o) => {
                if !ui.can_offset { return Response::new(&self.id, &self.rect); }
                self.rect.offset(o);
                self.changed = true;
            }
            _ => {}
        }
        Response::new(&self.id, &self.rect)
    }
}