use druid::widget::{Button, Flex, Label, TextBox, Controller};
use druid::{AppLauncher, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::widget::prelude::*;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::runtime::Runtime;
use zip::{
    network::NetworkClient,
    BrowserError
};

mod widgets;
use widgets::scroll::ScrollableContent;
use widgets::tabs::TabBar;

// Controllers
struct UrlBarController {
    runtime: tokio::runtime::Handle,
}

impl UrlBarController {
    fn new() -> Self {
        Self {
            runtime: tokio::runtime::Handle::current(),
        }
    }
}

impl<W: Widget<BrowserState>> Controller<BrowserState, W> for UrlBarController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut BrowserState, env: &Env) {
        if let Event::KeyDown(key_event) = event {
            if key_event.key == druid::keyboard_types::Key::Enter {
                let url = normalize_url(&data.url_bar);
                data.url_bar = url.clone();
                let runtime = self.runtime.clone();
                
                // Create owned copies for the async block
                let state = Arc::new(Mutex::new(data.clone()));
                runtime.spawn(async move {
                    let mut data = state.lock().await;
                    if let Err(e) = data.load_url(&url).await {
                        eprintln!("Error loading page: {}", e);
                    }
                });
                
                ctx.request_update();
            }
        }
        child.event(ctx, event, data, env);
    }
}

struct RefreshController {
    runtime: tokio::runtime::Handle,
}

impl RefreshController {
    fn new() -> Self {
        Self {
            runtime: tokio::runtime::Handle::current(),
        }
    }
}

impl<W: Widget<BrowserState>> Controller<BrowserState, W> for RefreshController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut BrowserState, env: &Env) {
        if let Event::MouseDown(_) = event {
            if let Some(tab) = data.tabs.get(data.current_tab) {
                let url = tab.url.clone();
                if !url.is_empty() {
                    let runtime = self.runtime.clone();
                    
                    // Create owned copies for the async block
                    let state = Arc::new(Mutex::new(data.clone()));
                    
                    runtime.spawn(async move {
                        let mut data = state.lock().await;
                        if let Err(e) = data.load_url(&url).await {
                            eprintln!("Error refreshing page: {}", e);
                        }
                    });
                    
                    ctx.request_update();
                }
            }
        }
        child.event(ctx, event, data, env);
    }
}

// Navigation history for each tab
#[derive(Clone, Data)]
struct NavigationHistory {
    back_stack: Arc<Vec<String>>,
    forward_stack: Arc<Vec<String>>,
    current: String,
}

impl NavigationHistory {
    fn new() -> Self {
        Self {
            back_stack: Arc::new(Vec::new()),
            forward_stack: Arc::new(Vec::new()),
            current: String::new(),
        }
    }

    fn push(&mut self, url: String) {
        if !self.current.is_empty() {
            let mut back = (*self.back_stack).clone();
            back.push(self.current.clone());
            self.back_stack = Arc::new(back);
        }
        self.current = url;
        self.forward_stack = Arc::new(Vec::new());
    }

    fn can_go_back(&self) -> bool {
        !self.back_stack.is_empty()
    }

    fn can_go_forward(&self) -> bool {
        !self.forward_stack.is_empty()
    }

    fn go_back(&mut self) -> Option<String> {
        if self.can_go_back() {
            let mut back = (*self.back_stack).clone();
            let mut forward = (*self.forward_stack).clone();
            let previous = back.pop()?;
            forward.push(self.current.clone());
            self.back_stack = Arc::new(back);
            self.forward_stack = Arc::new(forward);
            self.current = previous.clone();
            Some(previous)
        } else {
            None
        }
    }

    fn go_forward(&mut self) -> Option<String> {
        if self.can_go_forward() {
            let mut forward = (*self.forward_stack).clone();
            let mut back = (*self.back_stack).clone();
            let next = forward.pop()?;
            back.push(self.current.clone());
            self.forward_stack = Arc::new(forward);
            self.back_stack = Arc::new(back);
            self.current = next.clone();
            Some(next)
        } else {
            None
        }
    }
}

#[derive(Clone, Data, Lens)]
struct Tab {
    url: String,
    content: String,
    title: String,
    id: usize,
    #[data(ignore)]
    history: NavigationHistory,
}

impl Tab {
    fn new(id: usize) -> Self {
        Self {
            url: String::new(),
            content: String::new(),
            title: String::from("New Tab"),
            id,
            history: NavigationHistory::new(),
        }
    }
}

#[derive(Clone, Data, Lens)]
struct BrowserState {
    tabs: Arc<Vec<Tab>>,
    current_tab: usize,
    url_bar: String,
    #[data(ignore)]
    network_client: Arc<NetworkClient>,
    next_tab_id: usize,
    #[data(ignore)]
    html_parser: Arc<HTMLParser>,
    #[data(ignore)]
    css_parser: Arc<CSSParser>,
}

impl BrowserState {
    fn new() -> Self {
        Self {
            tabs: Arc::new(vec![Tab::new(0)]),
            current_tab: 0,
            url_bar: String::new(),
            network_client: Arc::new(NetworkClient::new()),
            next_tab_id: 1,
            html_parser: Arc::new(HTMLParser::new()),
            css_parser: Arc::new(CSSParser::new()),
        }
    }

    fn new_tab(&mut self) {
        let tab = Tab::new(self.next_tab_id);
        self.next_tab_id += 1;
        
        let mut tabs = (*self.tabs).clone();
        tabs.push(tab);
        self.tabs = Arc::new(tabs);
        self.current_tab = self.tabs.len() - 1;
    }

    fn close_tab(&mut self, index: usize) {
        if self.tabs.len() > 1 {
            let mut tabs = (*self.tabs).clone();
            tabs.remove(index);
            self.tabs = Arc::new(tabs);
            if self.current_tab >= self.tabs.len() {
                self.current_tab = self.tabs.len() - 1;
            }
        }
    }

    async fn load_url(&mut self, url: &str) -> Result<(), BrowserError> {
        let content = self.network_client.fetch(url).await?;
        let dom = self.html_parser.parse(&content)?;
        
        let mut tabs = (*self.tabs).clone();
        if let Some(tab) = tabs.get_mut(self.current_tab) {
            tab.history.push(url.to_string());
            tab.url = url.to_string();
            tab.content = content.clone();
            tab.dom = Some(dom);
            tab.title = extract_title(&content).unwrap_or_else(|| "Untitled".to_string());
        }
        self.tabs = Arc::new(tabs);
        
        Ok(())
    }

    fn navigate_back(&mut self) -> Option<String> {
        let mut tabs = (*self.tabs).clone();
        if let Some(tab) = tabs.get_mut(self.current_tab) {
            let url = tab.history.go_back();
            if let Some(ref u) = url {
                self.url_bar = u.clone();
            }
            url
        } else {
            None
        }
    }

    fn navigate_forward(&mut self) -> Option<String> {
        let mut tabs = (*self.tabs).clone();
        if let Some(tab) = tabs.get_mut(self.current_tab) {
            let url = tab.history.go_forward();
            if let Some(ref u) = url {
                self.url_bar = u.clone();
            }
            url
        } else {
            None
        }
    }
}

fn normalize_url(input: &str) -> String {
    if !input.starts_with("http://") && !input.starts_with("https://") {
        format!("https://{}", input)
    } else {
        input.to_string()
    }
}

fn extract_title(html: &str) -> Option<String> {
    let title_start = html.find("<title>")?;
    let title_end = html.find("</title>")?;
    Some(html[title_start + 7..title_end].trim().to_string())
}

fn build_ui() -> impl Widget<BrowserState> {
    let url_bar = TextBox::new()
        .lens(BrowserState::url_bar)
        .expand_width()
        .controller(UrlBarController::new())
        .padding(5.0);

    let button_style = |btn: Button<BrowserState>| {
        btn.fix_size(32.0, 32.0)
           .padding(4.0)
    };

    let new_tab_button = button_style(Button::new("+")
        .on_click(|_ctx, data: &mut BrowserState, _env| {
            data.new_tab();
        }));

    let back_button = button_style(Button::new("←")
        .on_click(|ctx, data: &mut BrowserState, _env| {
            if let Some(url) = data.navigate_back() {
                let runtime = tokio::runtime::Handle::current();
                let state = Arc::new(Mutex::new(data.clone()));
                runtime.spawn(async move {
                    let mut data = state.lock().await;
                    if let Err(e) = data.load_url(&url).await {
                        eprintln!("Error navigating back: {}", e);
                    }
                });
                ctx.request_update();
            }
        }));

    let forward_button = button_style(Button::new("→")
        .on_click(|ctx, data: &mut BrowserState, _env| {
            if let Some(url) = data.navigate_forward() {
                let runtime = tokio::runtime::Handle::current();
                let state = Arc::new(Mutex::new(data.clone()));
                runtime.spawn(async move {
                    let mut data = state.lock().await;
                    if let Err(e) = data.load_url(&url).await {
                        eprintln!("Error navigating forward: {}", e);
                    }
                });
                ctx.request_update();
            }
        }));

    let refresh_button = button_style(Button::new("🔄")
        .controller(RefreshController::new()));

    let content_area = ScrollableContent::new(
        Label::dynamic(|data: &BrowserState, _env: &_| {
            data.tabs.get(data.current_tab)
                .map(|tab| tab.content.clone())
                .unwrap_or_default()
        })
        .with_text_size(14.0)
        .expand()
        .padding(10.0)
    );

    Flex::column()
        .with_child(TabBar::new().fix_height(36.0))
        .with_child(
            Flex::row()
                .with_child(back_button)
                .with_spacer(2.0)
                .with_child(forward_button)
                .with_spacer(2.0)
                .with_child(refresh_button)
                .with_spacer(8.0)
                .with_flex_child(url_bar, 1.0)
                .with_spacer(8.0)
                .with_child(new_tab_button)
                .padding(10.0),
        )
        .with_flex_child(content_area, 1.0)
}

fn main() -> Result<(), PlatformError> {
    // Initialize logging
    env_logger::init();

    // Create and configure Tokio runtime
    let runtime = Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = runtime.enter();

    // Create the main window
    let main_window = WindowDesc::new(build_ui())
        .title("Rust Browser")
        .window_size((1024.0, 768.0));

    // Create the initial state
    let initial_state = BrowserState::new();

    // Launch the application
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
}