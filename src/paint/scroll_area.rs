use crate::frame::context::Context;
use crate::layout::scroll_area::ScrollArea;
use crate::layout::Layout;
use crate::paint::color::Color;
use crate::paint::rectangle::PaintRectangle;
use crate::paint::scroll_bar::PaintScrollBar;
use crate::radius::Radius;
use crate::size::border::Border;
use crate::size::rect::Rect;
use crate::ui::Ui;
use crate::Device;
use crate::response::Response;

pub struct PaintScrollArea {
    layout: Layout,
    fill: PaintRectangle,
    pub(crate) rect: Rect,
    scroll: PaintScrollBar,
    focused: bool,
    scrolling: bool,
    context_rect: Rect,
}

impl PaintScrollArea {
    pub fn new(mut scroll_area: ScrollArea, ui: &mut Ui) -> Self {
        // println!("{} {}", scroll_area.layout.height, scroll_area.layout.max_rect.height());

        let mut fill_rect = scroll_area.rect.clone();
        fill_rect.x.max = fill_rect.x.max - scroll_area.v_bar.rect.width() - 2.0;
        let mut fill = PaintRectangle::new(ui, fill_rect);
        let mut fill_style = ui.style.widget.click.clone();
        fill_style.border.inactive = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1.0).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill.set_style(fill_style);
        fill.prepare(&ui.device, false, false);

        let layout = scroll_area.layout.take().unwrap();
        let scroll = PaintScrollBar::new(ui, &scroll_area.v_bar.rect, layout.height + scroll_area.padding.vertical() + layout.item_space);
        PaintScrollArea {
            fill,
            rect: scroll_area.rect,
            context_rect: layout.max_rect.clone(),
            layout,
            scroll,
            focused: false,
            scrolling: false,

        }
    }

    pub fn draw(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        self.fill.render(&context.render, render_pass);
        self.scroll.render(&context.render, render_pass);
        let clip = &self.context_rect;
        render_pass.set_scissor_rect(clip.x.min as u32, clip.y.min as u32, clip.width() as u32, clip.height() as u32);
        self.layout.draw(device, context, render_pass);
        render_pass.set_scissor_rect(0, 0, context.size.width, context.size.height);
    }

    pub fn mouse_move(&mut self, device: &Device, context: &mut Context, resp: &mut Response) -> Vec<(String, Rect)> {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.fill.param.rect.has_position(x, y);
        let res = if (has_pos || self.scrolling) && self.focused && device.device_input.mouse.pressed { //处于滚动中
            self.scrolling = device.device_input.mouse.pressed;
            let oy = device.device_input.mouse.offset_y();
            self.scroll.offset_y(device, -oy, true);
            self.layout.offset(device, 0.0, -self.scroll.offset_y)
        } else {
            self.scroll.mouse_move(device, context);
            if self.scroll.offset_y == 0.0 {
                self.layout.mouse_move(device, context, resp);
                vec![]
            } else {
                self.layout.offset(device, 0.0, -self.scroll.offset_y)
            }
        };
        if self.scroll.offset_y != 0.0 { context.window.request_redraw(); }
        res
    }

    pub fn mouse_down(&mut self, device: &Device, context: &mut Context, resp: &mut Response) {
        let (x, y) = device.device_input.mouse.lastest();
        self.focused = self.fill.param.rect.has_position(x, y);
        self.scrolling = false;
        self.scroll.mouse_down(device);
        if self.focused { //处于视图内部
            self.layout.mouse_down(device, context, resp);
            // for layout in self.layouts.iter_mut() {
            //     layout.mouse_down(device, context, resp);
            // }
        }
    }

    pub fn delta_input(&mut self, device: &Device, context: &Context) -> Vec<(String, Rect)> {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.rect.has_position(x, y);
        if !has_pos { return vec![]; }
        self.scroll.offset_y(device, -device.device_input.mouse.delta_y() * 10.0, true);
        if self.scroll.offset_y == 0.0 { return vec![]; }
        let mut updates = self.layout.offset(device, 0.0, -self.scroll.offset_y);
        // for layout in self.layouts.iter_mut() {
        //     updates.append(&mut layout.offset(device, 0.0, -self.scroll.offset_y))
        // }
        context.window.request_redraw();
        updates
    }
}