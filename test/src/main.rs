use eframe::{egui, NativeOptions};
use webview::*;
use std::sync::{Arc, Mutex};

struct Tab {
    url: String,
    title: String,
    web_view: Option<Arc<Mutex<WebView>>>,
    loading: bool,
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            url: "about:blank".to_string(),
            title: "New Tab".to_string(),
            web_view: None,
            loading: false,
        }
    }
}

struct BrowserApp {
    tabs: Vec<Tab>,
    active_tab: usize,
    url_input: String,
    drag_offset: Option<egui::Pos2>,
    is_fullscreen: bool,
    window_handle: Option<*mut std::ffi::c_void>,
}

impl Default for BrowserApp {
    fn default() -> Self {
        let mut app = Self {
            tabs: vec![],
            active_tab: 0,
            url_input: String::new(),
            drag_offset: None,
            is_fullscreen: false,
            window_handle: None,
        };
        app.new_tab();
        app
    }
}

impl BrowserApp {
    fn new_tab(&mut self) {
        self.tabs.push(Tab::default());
        self.active_tab = self.tabs.len() - 1;
        
        // Create the webview for this tab if we have a window handle
        if let Some(handle) = self.window_handle {
            if let Some(tab) = self.tabs.last_mut() {
                let html = Content::Html(r#"
                    <html>
                        <head><title>New Tab</title></head>
                        <body><h1>New Tab</h1></body>
                    </html>
                "#.to_string());

                let webview = WebView::new(
                    "Browser",
                    html,
                    800,
                    600,
                    true,
                    true
                ).unwrap();
                
                tab.web_view = Some(Arc::new(Mutex::new(webview)));
            }
        }
    }

    fn close_tab(&mut self, index: usize) {
        self.tabs.remove(index);
        if self.tabs.is_empty() {
            self.new_tab();
        } else if self.active_tab >= self.tabs.len() {
            self.active_tab = self.tabs.len() - 1;
        }
    }

    fn navigate(&mut self, url: &str) {
        if let Some(tab) = self.tabs.get_mut(self.active_tab) {
            if let Some(web_view) = &tab.web_view {
                tab.loading = true;
                tab.url = url.to_string();
                
                if let Ok(view) = web_view.lock() {
                    let content = Content::Url(url.to_string());
                    let _ = WebView::new("Browser", content, 800, 600, true, true);
                }
            } else {
                self.new_tab();
                self.navigate(url);
            }
        }
    }

    fn set_window_handle(&mut self, handle: *mut std::ffi::c_void) {
        self.window_handle = Some(handle);
        // Recreate tabs with the new window handle
        let urls: Vec<String> = self.tabs.iter().map(|tab| tab.url.clone()).collect();
        self.tabs.clear();
        for url in urls {
            self.new_tab();
            if !url.is_empty() && url != "about:blank" {
                self.navigate(&url);
            }
        }
    }
}

impl eframe::App for BrowserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Set window handle if we haven't already
        if self.window_handle.is_none() {
            #[cfg(target_os = "windows")]
            unsafe {
                use winapi::um::winuser::GetActiveWindow;
                let hwnd = GetActiveWindow();
                if !hwnd.is_null() {
                    self.set_window_handle(hwnd as *mut _);
                }
            }
        }

        // Custom title bar
        egui::TopBottomPanel::top("title_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("⨯").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
                if ui.button("□").clicked() {
                    self.is_fullscreen = !self.is_fullscreen;
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(self.is_fullscreen));
                }
                if ui.button("−").clicked() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Minimized(true));
                }
                
                ui.label("Simple Browser");
                
                // Make the title bar draggable
                let response = ui.allocate_response(ui.available_size(), egui::Sense::drag());
                if response.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
            });
        });

        // Tab bar
        egui::TopBottomPanel::top("tab_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut tab_to_close = None;
                
                for (index, tab) in self.tabs.iter().enumerate() {
                    let selected = index == self.active_tab;
                    if ui.selectable_label(selected, &tab.title).clicked() {
                        self.active_tab = index;
                    }
                    if ui.small_button("×").clicked() {
                        tab_to_close = Some(index);
                    }
                }
                
                if ui.button("+").clicked() {
                    self.new_tab();
                }
                
                if let Some(index) = tab_to_close {
                    self.close_tab(index);
                }
            });
        });

        // Navigation bar
        egui::TopBottomPanel::top("nav_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("←").clicked() {
                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                        if let Some(web_view) = &tab.web_view {
                            if let Ok(view) = web_view.lock() {
                                view.eval("history.back();");
                            }
                        }
                    }
                }
                
                if ui.button("→").clicked() {
                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                        if let Some(web_view) = &tab.web_view {
                            if let Ok(view) = web_view.lock() {
                                view.eval("history.forward();");
                            }
                        }
                    }
                }
                
                if ui.button("⟳").clicked() {
                    if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                        if let Some(web_view) = &tab.web_view {
                            if let Ok(view) = web_view.lock() {
                                view.eval("location.reload();");
                            }
                        }
                    }
                }
                
                let mut url = self.url_input.clone();
                if ui.text_edit_singleline(&mut url).lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    self.navigate(&url);
                }
                self.url_input = url;
            });
        });

        // Main content area - keep space for the webview
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                if tab.loading {
                    ui.spinner();
                }
                ui.allocate_space(ui.available_size());

                // Update webview size
                if let Some(web_view) = &tab.web_view {
                    if let Ok(view) = web_view.lock() {
                        let rect = ctx.available_rect();
                        let size = rect.size();
                        let _ = WebView::new(
                            "Browser",
                            Content::Html(String::new()),
                            size.x as i32,
                            size.y as i32,
                            true,
                            true
                        );
                    }
                }
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_decorations(false)
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    
    eframe::run_native(
        "Simple Browser",
        native_options,
        Box::new(|_cc| Box::new(BrowserApp::default())),
    )
}