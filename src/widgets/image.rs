use crate::frame::context::UpdateType;
use crate::render::image::ImageSource;
use crate::response::Response;
use crate::size::Geometry;
use crate::ui::Ui;
use crate::vertex::ImageVertex;
use crate::widgets::{Widget, WidgetChange, WidgetSize};
use crate::Size;
use wgpu::util::DeviceExt;

/// ### Image的示例用法
///```
/// use xlui::*;
///
/// fn draw(ui:&mut Ui){
///    let mut image=Image::new("logo.png")
///        //设置控件大小
///        .with_size(30.0,30.0)
///        //设置控件ID
///        .with_id("my_image");
///    //快速创建图片
///    ui.add(image);
///
/// }
///
/// fn update(ui:&mut Ui){
///    //注意这里不应该在每次调用update的时候都更新图片。建议给一个状态，更新状态是否修改图片
///    //获取Image的可变引用
///    let image:&mut Image=ui.get_widget("my_image").unwrap();
///    //修改图片
///    image.set_image("logo_2.png");
/// }
/// ```
pub struct Image {
    id: String,
    source: ImageSource,
    geometry: Geometry,

    vertices: Vec<ImageVertex>,
    vertex_buffer: Option<wgpu::Buffer>,
    index_buffer: Option<wgpu::Buffer>,
    changed: bool,
}

impl Image {
    pub fn new(source: impl Into<ImageSource>) -> Self {
        Image {
            id: crate::gen_unique_id(),
            source: source.into(),
            geometry: Geometry::new(),
            vertices: vec![],
            vertex_buffer: None,
            index_buffer: None,
            changed: false,
        }
    }

    fn reset_size(&mut self, size: Size) {
        self.geometry.set_size(size.width, size.height);
    }


    pub fn with_size(mut self, width: f32, height: f32) -> Self {
        self.geometry.set_fix_size(width,height);
        self
    }


    pub fn with_id(mut self, id: impl ToString) -> Self {
        self.id = id.to_string();
        self
    }

    pub fn set_image(&mut self, source: impl Into<ImageSource>) {
        self.source = source.into();
        self.changed = true;
    }

    fn init(&mut self, ui: &mut Ui) {
        self.re_init(ui);
    }

    fn re_init(&mut self, ui: &mut Ui) {
        let size = ui.context.render.image.insert_image(&ui.device, &self.source);
        self.reset_size(size);
        let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];
        let rect = self.geometry.rect();
        self.vertices = vec![
            ImageVertex::new_coord(rect.left_top(), [0.0, 0.0], Size::from(&ui.device.surface_config)),
            ImageVertex::new_coord(rect.left_bottom(), [0.0, 1.0], Size::from(&ui.device.surface_config)),
            ImageVertex::new_coord(rect.right_bottom(), [1.0, 1.0], Size::from(&ui.device.surface_config)),
            ImageVertex::new_coord(rect.right_top(), [1.0, 0.0], Size::from(&ui.device.surface_config))
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
        if self.changed { ui.widget_changed |= WidgetChange::Value; }
        self.changed = false;
        if !ui.widget_changed.unchanged() {
            self.geometry.offset_to_rect(&ui.draw_rect);
            let rect = self.geometry.rect();
            for (index, v) in self.vertices.iter_mut().enumerate() {
                match index {
                    0 => v.position = rect.left_top(),
                    1 => v.position = rect.left_bottom(),
                    2 => v.position = rect.right_bottom(),
                    3 => v.position = rect.right_top(),
                    _ => {}
                }
                v.screen_size = [ui.device.surface_config.width as f32, ui.device.surface_config.height as f32];
            }
            ui.device.queue.write_buffer(
                self.vertex_buffer.as_ref().unwrap(), 0,
                bytemuck::cast_slice(self.vertices.as_slice()));
            ui.context.render.image.insert_image(&ui.device, &self.source);
        }
    }
    pub(crate) fn redraw(&mut self, ui: &mut Ui) {
        self.update_buffer(ui);
        let pass = ui.pass.as_mut().unwrap();
        ui.context.render.image.render(
            &self.source.uri(),
            self.vertex_buffer.as_ref().unwrap(),
            self.index_buffer.as_ref().unwrap(),
            pass,
        );
    }
}

impl Widget for Image {
    fn update(&mut self, ui: &mut Ui) -> Response<'_> {
        match ui.update_type {
            UpdateType::Draw => self.redraw(ui),
            UpdateType::Init => self.init(ui),
            UpdateType::ReInit => self.re_init(ui),
            _ => {}
        }
        Response::new(&self.id, WidgetSize::same(self.geometry.width(), self.geometry.height()))
    }

    fn geometry(&mut self) -> &mut Geometry {
        &mut self.geometry
    }
}