// src/widgets/tabs.rs
use druid::widget::prelude::*;
use druid::{Color, RenderContext, FontDescriptor, FontFamily, Rect, Point, TextLayout};

// Assuming BrowserState is defined in a module named `browser`
use super::super::BrowserState;

pub struct TabBar {
    tabs_bounds: Vec<Rect>,
}

impl TabBar {
    pub fn new() -> Self {
        TabBar {
            tabs_bounds: Vec::new(),
        }
    }
}

impl Widget<BrowserState> for TabBar {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut BrowserState, _env: &Env) {
        if let Event::MouseDown(mouse) = event {
            for (i, bound) in self.tabs_bounds.iter().enumerate() {
                if bound.contains(mouse.pos) {
                    data.current_tab = i;
                    if let Some(tab) = data.tabs.get(i) {
                        data.url_bar = tab.url.clone();
                    }
                    ctx.request_paint();
                    break;
                }
            }
        }
    }

    fn lifecycle(&mut self, _ctx: &mut LifeCycleCtx, _event: &LifeCycle, _data: &BrowserState, _env: &Env) {}

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &BrowserState, _data: &BrowserState, _env: &Env) {
        ctx.request_paint();
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &BrowserState, _env: &Env) -> Size {
        let tab_height = 30.0;
        let tab_width = 150.0;
        
        self.tabs_bounds.clear();
        for i in 0..data.tabs.len() {
            let bound = Rect::from_origin_size(
                Point::new(i as f64 * tab_width, 0.0),
                Size::new(tab_width, tab_height),
            );
            self.tabs_bounds.push(bound);
        }

        Size::new(bc.max().width, tab_height)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &BrowserState, _env: &Env) {
        let font = FontDescriptor::new(FontFamily::SYSTEM_UI).with_size(14.0);
        
        for (i, bound) in self.tabs_bounds.iter().enumerate() {
            let is_current = i == data.current_tab;
            let background_color = if is_current {
                Color::WHITE
            } else {
                Color::grey8(240)
            };
            
            // Draw tab background
            ctx.fill(bound, &background_color);
            ctx.stroke(bound, &Color::grey8(200), 1.0);
            
            // Draw tab title
            if let Some(tab) = data.tabs.get(i) {
                let title = if tab.title.is_empty() {
                    "New Tab"
                } else {
                    &tab.title
                };
                
                let mut text_layout = TextLayout::new();
                text_layout.set_text(title.to_string());
                text_layout.set_font(font.clone());
                text_layout.set_text_color(Color::BLACK);
                text_layout.rebuild_if_needed(ctx.text(), _env);
                if let Some(layout) = text_layout.layout() {
                    ctx.draw_text(layout, bound.inset(-5.0).center());
                }
            }
        }
    }
}   