class BrowserTab {
    constructor(url = 'https://example.com') {
        this.url = url;
        this.history = [url];
        this.currentIndex = 0;
        this.createElements();
    }

    createElements() {
        // Create tab element
        this.tabElement = document.createElement('div');
        this.tabElement.className = 'tab';
        this.tabElement.innerHTML = `
            <span class="tab-title">New Tab</span>
            <span class="tab-close">×</span>
        `;

        // Create content container
        this.contentElement = document.createElement('div');
        this.contentElement.className = 'tab-content';

        // Create iframe
        this.iframe = document.createElement('iframe');
        this.iframe.className = 'web-content';
        this.contentElement.appendChild(this.iframe);

        // Handle iframe load events
        this.iframe.onload = () => {
            try {
                const title = this.iframe.contentDocument?.title;
                if (title) {
                    this.tabElement.querySelector('.tab-title').textContent = title;
                }
            } catch (e) {
                // Handle cross-origin restrictions
                this.tabElement.querySelector('.tab-title').textContent = new URL(this.url).hostname;
            }
        };
    }

    async navigate(url) {
        // Handle search queries
        if (!url.includes('.') || !url.includes('/')) {
            url = `https://www.google.com/search?q=${encodeURIComponent(url)}`;
        } else if (!url.startsWith('http')) {
            url = 'https://' + url;
        }

        this.url = url;
        this.iframe.src = url;

        if (this.currentIndex < this.history.length - 1) {
            this.history.splice(this.currentIndex + 1);
        }
        this.history.push(url);
        this.currentIndex++;
        this.updateNavButtons();
        
        return true;
    }

    canGoBack() {
        return this.currentIndex > 0;
    }

    canGoForward() {
        return this.currentIndex < this.history.length - 1;
    }

    goBack() {
        if (this.canGoBack()) {
            this.currentIndex--;
            this.navigate(this.history[this.currentIndex]);
        }
    }

    goForward() {
        if (this.canGoForward()) {
            this.currentIndex++;
            this.navigate(this.history[this.currentIndex]);
        }
    }

    reload() {
        this.iframe.src = this.url;
    }

    updateNavButtons() {
        document.querySelector('.nav-button.back').disabled = !this.canGoBack();
        document.querySelector('.nav-button.forward').disabled = !this.canGoForward();
    }
}

class Browser {
    constructor() {
        this.tabs = [];
        this.activeTab = null;
        this.isVerticalMode = false;
        this.setupEventListeners();
        
        // Create the grip element if it doesn't exist
        if (!document.querySelector('.vertical-tabs-grip')) {
            const grip = document.createElement('div');
            grip.className = 'vertical-tabs-grip';
            grip.innerHTML = `
                <div class="grip-dots">
                    <span></span>
                    <span></span>
                    <span></span>
                </div>
            `;
            document.querySelector('.main-container').prepend(grip);
        }
        
        this.createTab();
        
        // Improve overflow detection with ResizeObserver
        const resizeObserver = new ResizeObserver(() => {
            const tabBar = document.querySelector('.tab-bar');
            const tabBarWidth = tabBar.offsetWidth - 40; // Account for padding and new tab button
            let totalTabsWidth = 0;
            
            // Calculate total width of all tabs
            tabBar.querySelectorAll('.tab').forEach(element => {
                totalTabsWidth += element.offsetWidth + 4; // Add gap
            });
            
            if (totalTabsWidth > tabBarWidth && !this.isVerticalMode) {
                this.isVerticalMode = true;
                this.updateTabLayout();
            }
        });
        resizeObserver.observe(document.querySelector('.tab-bar'));
    }

    setupEventListeners() {
        // Navigation buttons
        document.querySelector('.nav-button.back').onclick = () => this.activeTab?.goBack();
        document.querySelector('.nav-button.forward').onclick = () => this.activeTab?.goForward();
        document.querySelector('.nav-button.reload').onclick = () => this.activeTab?.reload();

        // Address bar
        const addressBar = document.querySelector('.address-bar input');
        addressBar.onkeydown = (e) => {
            if (e.key === 'Enter') {
                this.activeTab?.navigate(addressBar.value);
            }
        };

        // New tab button
        document.querySelector('.new-tab').onclick = () => this.createTab();

        // Window controls
        document.querySelector('.window-button.close').onclick = () => window.close();
        document.querySelector('.window-button.minimize').onclick = () => window.minimize();
        document.querySelector('.window-button.maximize').onclick = () => {
            if (window.isMaximized?.()) {
                window.unmaximize();
            } else {
                window.maximize();
            }
        };

        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            if (e.ctrlKey || e.metaKey) {
                switch(e.key) {
                    case 't':
                        e.preventDefault();
                        this.createTab();
                        break;
                    case 'w':
                        e.preventDefault();
                        if (this.activeTab) {
                            this.closeTab(this.activeTab);
                        }
                        break;
                    case 'r':
                        e.preventDefault();
                        this.activeTab?.reload();
                        break;
                    case 'l':
                        e.preventDefault();
                        addressBar.select();
                        break;
                }
            }
        });

        // Add vertical tabs toggle button listener
        const toggleButton = document.querySelector('.vertical-tabs-toggle');
        toggleButton.onclick = () => this.toggleVerticalTabs();
    }

    toggleVerticalTabs() {
        this.isVerticalMode = !this.isVerticalMode;
        this.updateTabLayout();
    }

    checkTabOverflow() {
        const tabBar = document.querySelector('.tab-bar');
        const tabBarWidth = tabBar.offsetWidth - 40;
        let totalTabsWidth = 0;
        
        // Calculate total width of all tabs
        tabBar.querySelectorAll('.tab').forEach(element => {
            totalTabsWidth += element.offsetWidth + 4;
        });
        
        // Update vertical mode if tabs overflow
        if (totalTabsWidth > tabBarWidth) {
            this.isVerticalMode = true;
            this.updateTabLayout();
            
            // Show the vertical tabs
            const verticalTabs = document.querySelector('.vertical-tabs');
            verticalTabs.classList.add('show');
        }
    }

    updateTabLayout() {
        const tabBar = document.querySelector('.tab-bar');
        const verticalTabs = document.querySelector('.vertical-tabs');
        const verticalTabsGrip = document.querySelector('.vertical-tabs-grip');
        
        // Update UI elements
        tabBar.classList.toggle('vertical-mode', this.isVerticalMode);
        verticalTabsGrip.classList.toggle('show', this.isVerticalMode);
        verticalTabs.classList.toggle('show', this.isVerticalMode);
        
        if (this.isVerticalMode) {
            // Clear existing content
            verticalTabs.innerHTML = '';
            
            // Create tabs container
            const tabsContainer = document.createElement('div');
            tabsContainer.style.cssText = `
                display: flex;
                flex-direction: column;
                flex: 1;
                padding: 8px;
                gap: 4px;
                overflow-y: auto;
            `;
            
            // Add tabs
            this.tabs.forEach(tab => {
                const verticalTab = tab.tabElement.cloneNode(true);
                verticalTab.onclick = () => this.setActiveTab(tab);
                verticalTab.querySelector('.tab-close').onclick = (e) => {
                    e.stopPropagation();
                    this.closeTab(tab);
                };
                tabsContainer.appendChild(verticalTab);
            });
            
            // Add new tab button at the bottom
            const newTabBtn = document.createElement('button');
            newTabBtn.className = 'new-tab';
            newTabBtn.textContent = '+ New Tab';
            newTabBtn.style.marginTop = 'auto'; // Push to bottom
            newTabBtn.onclick = () => this.createTab();
            
            verticalTabs.appendChild(tabsContainer);
            verticalTabs.appendChild(newTabBtn);
        }
    }

    createTab(url = 'https://example.com') {
        const tab = new BrowserTab(url);
        this.tabs.push(tab);

        // Add tab button before the new tab button
        const tabBar = document.querySelector('.tab-bar');
        tabBar.insertBefore(tab.tabElement, document.querySelector('.new-tab'));

        // Add content view to the container
        document.getElementById('content-container').appendChild(tab.contentElement);

        // Setup tab event listeners
        tab.tabElement.onclick = () => this.setActiveTab(tab);
        tab.tabElement.querySelector('.tab-close').onclick = (e) => {
            e.stopPropagation();
            this.closeTab(tab);
        };

        this.setActiveTab(tab);
        tab.navigate(url);
        
        // Force check for overflow after a short delay to ensure DOM updates
        setTimeout(() => this.checkTabOverflow(), 0);
        return tab;
    }

    setActiveTab(tab) {
        if (this.activeTab) {
            this.activeTab.tabElement.classList.remove('active');
            this.activeTab.contentElement.classList.remove('active');
        }

        this.activeTab = tab;
        tab.tabElement.classList.add('active');
        tab.contentElement.classList.add('active');
        document.querySelector('.address-bar input').value = tab.url;
        tab.updateNavButtons();
    }

    closeTab(tab) {
        const index = this.tabs.indexOf(tab);
        if (index === -1) return;

        tab.tabElement.remove();
        tab.contentElement.remove();
        this.tabs.splice(index, 1);

        if (this.tabs.length === 0) {
            this.createTab();
        } else if (tab === this.activeTab) {
            this.setActiveTab(this.tabs[Math.max(0, index - 1)]);
        }
        this.checkTabOverflow(); // Check for overflow after closing tab
    }
}

// Initialize the browser
const browser = new Browser();