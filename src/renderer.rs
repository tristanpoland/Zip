// src/renderer.rs
use crate::layout::layout::{LayoutBox, Dimensions};
use druid::{
    widget::prelude::*,
    Widget,
    Color,
    Rect,
    Point,
};

pub struct RenderWidget {
    layout: Option<LayoutBox>,
}

impl RenderWidget {
    pub fn new() -> Self {
        RenderWidget { layout: None }
    }

    pub fn set_layout(&mut self, layout: LayoutBox) {
        self.layout = Some(layout);
    }

    fn render_layout_box(&self, layout_box: &LayoutBox, ctx: &mut PaintCtx) {
        // Draw background
        let rect = Rect::from_origin_size(
            Point::new(
                layout_box.dimensions.content.x as f64,
                layout_box.dimensions.content.y as f64,
            ),
            (
                layout_box.dimensions.content.width as f64,
                layout_box.dimensions.content.height as f64,
            ),
        );
        ctx.fill(rect, &Color::WHITE);

        // Draw border
        ctx.stroke(rect, &Color::BLACK, 1.0);

        // Recursively render children
        for child in &layout_box.children {
            self.render_layout_box(child, ctx);
        }
    }
}

impl Widget<String> for RenderWidget {
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut String, _env: &Env) {}

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &String, _env: &Env) {}

    fn update(&mut self, _ctx: &mut UpdateCtx, _old_data: &String, _data: &String, _env: &Env) {}

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &String, _env: &Env) -> Size {
        bc.max()
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &String, _env: &Env) {
        if let Some(layout) = &self.layout {
            self.render_layout_box(layout, ctx);
        }
    }
}