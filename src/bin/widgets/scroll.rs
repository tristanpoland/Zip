use druid::widget::prelude::*;
use druid::{Point, Rect, Vec2, MouseEvent, Affine};

pub struct ScrollableContent<T> {
    inner: Box<dyn Widget<T>>,
    scroll_offset: Point,
    viewport_size: Size,
    content_size: Size,
}

impl<T: Data> ScrollableContent<T> {
    pub fn new(inner: impl Widget<T> + 'static) -> Self {
        ScrollableContent {
            inner: Box::new(inner),
            scroll_offset: Point::ORIGIN,
            viewport_size: Size::ZERO,
            content_size: Size::ZERO,
        }
    }

    fn update_scroll(&mut self, delta: Vec2) {
        let max_scroll = Size::new(
            (self.content_size.width - self.viewport_size.width).max(0.0),
            (self.content_size.height - self.viewport_size.height).max(0.0),
        );

        self.scroll_offset = Point::new(
            (self.scroll_offset.x - delta.x).clamp(0.0, max_scroll.width),
            (self.scroll_offset.y - delta.y).clamp(0.0, max_scroll.height),
        );
    }
}

impl<T: Data> Widget<T> for ScrollableContent<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Wheel(mouse) => {
                self.update_scroll(mouse.wheel_delta);
                ctx.request_paint();
            }
            _ => {
                // Adjust event position for scrolling
                let transformed_event = match event {
                    Event::MouseDown(mouse) => Some(Event::MouseDown(MouseEvent {
                        pos: mouse.pos - self.scroll_offset.to_vec2(),
                        ..*mouse
                    })),
                    Event::MouseUp(mouse) => Some(Event::MouseUp(MouseEvent {
                        pos: mouse.pos - self.scroll_offset.to_vec2(),
                        ..*mouse
                    })),
                    Event::MouseMove(mouse) => Some(Event::MouseMove(MouseEvent {
                        pos: mouse.pos - self.scroll_offset.to_vec2(),
                        ..*mouse
                    })),
                    _ => None,
                };

                if let Some(transformed_event) = transformed_event {
                    self.inner.event(ctx, &transformed_event, data, env);
                } else {
                    self.inner.event(ctx, event, data, env);
                }
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.inner.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, data: &T, env: &Env) {
        self.inner.update(ctx, _old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.viewport_size = bc.max();
        self.content_size = self.inner.layout(ctx, &BoxConstraints::new(
            Size::ZERO,
            Size::new(bc.max().width, f64::INFINITY),
        ), data, env);
        self.viewport_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let clip_rect = Rect::from_origin_size(Point::ORIGIN, self.viewport_size);
        ctx.clip(clip_rect);
        
        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(-self.scroll_offset.to_vec2()));
            self.inner.paint(ctx, data, env);
        });
    }
}