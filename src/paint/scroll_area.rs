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

pub struct PaintScrollArea {
    layouts: Vec<Layout>,
    fill: PaintRectangle,
    pub(crate) rect: Rect,
    scroll: PaintScrollBar,
    focused: bool,
    scrolling: bool,
    context_rect: Rect,
}

impl PaintScrollArea {
    pub fn new(scroll_area: ScrollArea, ui: &mut Ui) -> Self {
        println!("{} {}", scroll_area.layouts[0].height, scroll_area.layouts[0].max_rect.height());


        let mut fill_rect = scroll_area.rect.clone();
        fill_rect.x.max = fill_rect.x.max - scroll_area.v_bar.rect.width() - 2.0;
        let mut fill = PaintRectangle::new(ui, fill_rect);
        let mut fill_style = ui.style.widget.click.clone();
        fill_style.border.inactive = Border::new(1).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.hovered = Border::new(1).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill_style.border.clicked = Border::new(1).color(Color::rgba(144, 209, 255, 255)).radius(Radius::same(2));
        fill.set_style(fill_style);
        fill.prepare(&ui.device, false, false);
        let layout = &scroll_area.layouts[0];
        let scroll = PaintScrollBar::new(ui, &scroll_area.v_bar.rect, layout.height + scroll_area.padding.vertical());
        PaintScrollArea {
            fill,
            rect: scroll_area.rect,
            context_rect: layout.rect(),
            layouts: scroll_area.layouts,
            scroll,
            focused: false,
            scrolling: false,

        }
    }

    pub fn draw(&mut self, device: &Device, context: &mut Context, render_pass: &mut wgpu::RenderPass) {
        self.fill.render(&context.render, render_pass);
        self.scroll.render(&context.render, render_pass);
        let mut clip = &self.context_rect;
        render_pass.set_scissor_rect(clip.x.min as u32, clip.y.min as u32, clip.width() as u32, clip.height() as u32);
        for layout in self.layouts.iter_mut() {
            layout.draw(device, context, render_pass);
        }
        render_pass.set_scissor_rect(0, 0, context.size.width, context.size.height);
    }

    pub fn mouse_move(&mut self, device: &Device, context: &mut Context) -> Vec<(String, Rect)> {
        // self.scroll.mouse_move(device, context);
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.fill.param.rect.has_position(x, y);
        let mut updates = vec![];
        if (has_pos || self.scrolling) && self.focused && device.device_input.mouse.pressed { //处于滚动中
            self.scrolling = device.device_input.mouse.pressed;
            let oy = device.device_input.mouse.offset_y();
            self.scroll.offset_y(device, -oy);
            for layout in self.layouts.iter_mut() {
                updates.append(&mut layout.offset(device, 0.0, -self.scroll.offset_y));
                if self.scroll.offset_y != 0.0 { context.window.request_redraw(); }
            }
        } else {
            self.scroll.mouse_move(device, context);
            for layout in self.layouts.iter_mut() {
                if self.scroll.offset_y != 0.0 { updates.append(&mut layout.offset(device, 0.0, -self.scroll.offset_y)); }
                // updates.append(&mut layout.mouse_move(device, context));
            }
        }


        // if self.fill.param.rect.has_position(x, y) {}
        //
        //
        // if self.fill.param.rect.has_position(x, y) && self.focused && device.device_input.mouse.pressed {} else {
        //     for layout in self.layouts.iter_mut() {
        //         if self.scroll.offset_y != 0.0 {
        //             updates.append(&mut layout.offset(device, 0.0, -self.scroll.offset_y));
        //             context.window.request_redraw();
        //         }
        //         layout.mouse_move(device, context);
        //     }
        // }
        updates
    }

    pub fn mouse_down(&mut self, device: &Device, context: &mut Context) {
        let (x, y) = device.device_input.mouse.lastest();
        self.focused = self.fill.param.rect.has_position(x, y);
        self.scrolling = false;
        println!("scroll down {}", self.focused);
        self.scroll.mouse_down(device);
        if self.focused { //处于视图内部
            for layout in self.layouts.iter_mut() {
                layout.mouse_down(device, context);
            }
        }
    }

    pub fn delta_input(&mut self, device: &Device, context: &Context) -> Vec<(String, Rect)> {
        let (x, y) = device.device_input.mouse.lastest();
        let has_pos = self.rect.has_position(x, y);
        if !has_pos { return vec![]; }
        self.scroll.offset_y(device, -device.device_input.mouse.delta_y() * 10.0);
        let mut updates = vec![];
        for layout in self.layouts.iter_mut() {
            updates.append(&mut layout.offset(device, 0.0, -self.scroll.offset_y))
        }
        context.window.request_redraw();
        updates
    }
}