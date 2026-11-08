//! Shell manager for ags.

use std::collections::HashMap;
use crate::bar::{Bar, BarPosition, BarModule};
use crate::css::CssStyle;
use crate::config::ShellSettings;
use crate::widget::Widget;

/// Shell manager options
#[derive(Debug, Clone)]
pub struct ShellManagerOptions {
    /// Auto-start shell
    pub auto_start: bool,
    /// Enable CSS hot reload
    pub css_hot_reload: bool,
    /// Enable widget hot reload
    pub widget_hot_reload: bool,
    /// Config file path
    pub config_path: String,
}

impl Default for ShellManagerOptions {
    fn default() -> Self {
        Self {
            auto_start: true,
            css_hot_reload: true,
            widget_hot_reload: true,
            config_path: "/etc/ags/config.json".into(),
        }
    }
}

/// Shell statistics
#[derive(Debug, Clone)]
pub struct ShellStatistics {
    /// Total bars
    pub total_bars: usize,
    /// Total modules
    pub total_modules: usize,
    /// Total widgets
    pub total_widgets: usize,
    /// Total CSS rules
    pub total_css_rules: usize,
    /// Shell running
    pub shell_running: bool,
    /// Last reload timestamp
    pub last_reload: u64,
}

/// Shell manager
#[derive(Debug, Clone)]
pub struct ShellManager {
    /// Bars
    bars: HashMap<String, Bar>,
    /// Widgets
    widgets: HashMap<String, Widget>,
    /// CSS style
    css: Option<CssStyle>,
    /// Shell settings
    settings: ShellSettings,
    /// Options
    options: ShellManagerOptions,
    /// Statistics
    statistics: ShellStatistics,
    /// Shell running
    shell_running: bool,
}

impl Default for ShellManager {
    fn default() -> Self {
        Self {
            bars: HashMap::new(),
            widgets: HashMap::new(),
            css: None,
            settings: ShellSettings::default(),
            options: ShellManagerOptions::default(),
            statistics: ShellStatistics {
                total_bars: 0,
                total_modules: 0,
                total_widgets: 0,
                total_css_rules: 0,
                shell_running: false,
                last_reload: 0,
            },
            shell_running: false,
        }
    }
}

impl ShellManager {
    /// Create new shell manager
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create shell manager with options
    pub fn with_options(options: ShellManagerOptions) -> Self {
        let mut manager = Self::default();
        manager.options = options;
        manager
    }
    
    /// Initialize shell
    pub fn initialize(&mut self) {
        // Create default bar
        let default_bar = Bar::new("default".into(), BarPosition::Top);
        self.bars.insert("default".into(), default_bar);
        
        // Load default CSS
        let mut css = CssStyle::new();
        css.set_theme("default".into());
        css.set_variable("bg-color".into(), "#1a1a1a".into());
        css.set_variable("fg-color".into(), "#ffffff".into());
        self.css = Some(css);
        
        self.update_statistics();
        
        if self.options.auto_start {
            self.start_shell();
        }
    }
    
    /// Start shell
    pub fn start_shell(&mut self) -> bool {
        if !self.shell_running {
            self.shell_running = true;
            self.statistics.shell_running = true;
            return true;
        }
        false
    }
    
    /// Stop shell
    pub fn stop_shell(&mut self) -> bool {
        if self.shell_running {
            self.shell_running = false;
            self.statistics.shell_running = false;
            return true;
        }
        false
    }
    
    /// Create bar
    pub fn create_bar(&mut self, name: String, position: BarPosition) -> String {
        let id = format!("bar-{}", self.bars.len() + 1);
        let mut bar = Bar::new(name.clone(), position);
        bar.name = name;
        
        self.bars.insert(id.clone(), bar);
        self.update_statistics();
        id
    }
    
    /// Get bar
    pub fn get_bar(&self, bar_id: &str) -> Option<&Bar> {
        self.bars.get(bar_id)
    }
    
    /// Get bar mutably
    pub fn get_bar_mut(&mut self, bar_id: &str) -> Option<&mut Bar> {
        self.bars.get_mut(bar_id)
    }
    
    /// Get all bars
    pub fn list_bars(&self) -> Vec<&Bar> {
        self.bars.values().collect()
    }
    
    /// Delete bar
    pub fn delete_bar(&mut self, bar_id: &str) -> bool {
        if self.bars.remove(bar_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Add module to bar
    pub fn add_module_to_bar(&mut self, bar_id: &str, module: BarModule) -> bool {
        if let Some(bar) = self.bars.get_mut(bar_id) {
            bar.add_module(module);
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Remove module from bar
    pub fn remove_module_from_bar(&mut self, bar_id: &str, module_name: &str) -> bool {
        if let Some(bar) = self.bars.get_mut(bar_id) {
            if bar.remove_module(module_name) {
                self.update_statistics();
                return true;
            }
        }
        false
    }
    
    /// Create widget
    pub fn create_widget(&mut self, name: String, widget_type: crate::widget::WidgetType) -> String {
        let id = format!("widget-{}", self.widgets.len() + 1);
        let widget = Widget::new(name, widget_type);
        
        self.widgets.insert(id.clone(), widget);
        self.update_statistics();
        id
    }
    
    /// Get widget
    pub fn get_widget(&self, widget_id: &str) -> Option<&Widget> {
        self.widgets.get(widget_id)
    }
    
    /// Get widget mutably
    pub fn get_widget_mut(&mut self, widget_id: &str) -> Option<&mut Widget> {
        self.widgets.get_mut(widget_id)
    }
    
    /// Get all widgets
    pub fn list_widgets(&self) -> Vec<&Widget> {
        self.widgets.values().collect()
    }
    
    /// Delete widget
    pub fn delete_widget(&mut self, widget_id: &str) -> bool {
        if self.widgets.remove(widget_id).is_some() {
            self.update_statistics();
            return true;
        }
        false
    }
    
    /// Load CSS
    pub fn load_css(&mut self, css_string: String) -> bool {
        let mut css = CssStyle::new();
        css.parse_css(&css_string);
        self.css = Some(css);
        self.update_statistics();
        true
    }
    
    /// Get CSS
    pub fn get_css(&self) -> Option<&CssStyle> {
        self.css.as_ref()
    }
    
    /// Get CSS mutably
    pub fn get_css_mut(&mut self) -> Option<&mut CssStyle> {
        self.css.as_mut()
    }
    
    /// Set shell settings
    pub fn set_settings(&mut self, settings: ShellSettings) {
        self.settings = settings;
        self.update_statistics();
    }
    
    /// Get shell settings
    pub fn get_settings(&self) -> &ShellSettings {
        &self.settings
    }
    
    /// Update statistics
    fn update_statistics(&mut self) {
        self.statistics.total_bars = self.bars.len();
        self.statistics.total_modules = self.bars.values()
            .map(|b| b.modules.len())
            .sum();
        self.statistics.total_widgets = self.widgets.len();
        self.statistics.total_css_rules = self.css.as_ref()
            .map(|c| c.rules.len())
            .unwrap_or(0);
    }
    
    /// Get statistics
    pub fn get_statistics(&self) -> &ShellStatistics {
        &self.statistics
    }
    
    /// Export shell data
    pub fn export_data(&self) -> ShellExport {
        ShellExport {
            bars: self.bars.values().cloned().collect(),
            widgets: self.widgets.values().cloned().collect(),
            css: self.css.clone(),
            settings: self.settings.clone(),
        }
    }
    
    /// Import shell data
    pub fn import_data(&mut self, export: ShellExport) {
        self.bars.clear();
        for bar in export.bars {
            self.bars.insert(format!("bar-{}", self.bars.len() + 1), bar);
        }
        
        self.widgets.clear();
        for widget in export.widgets {
            self.widgets.insert(format!("widget-{}", self.widgets.len() + 1), widget);
        }
        
        self.css = export.css;
        self.settings = export.settings;
        self.update_statistics();
    }
    
    /// Reload shell
    pub fn reload_shell(&mut self) -> bool {
        self.statistics.last_reload = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.update_statistics();
        true
    }
}

/// Shell export data
#[derive(Debug, Clone)]
pub struct ShellExport {
    /// Bars
    pub bars: Vec<Bar>,
    /// Widgets
    pub widgets: Vec<Widget>,
    /// CSS style
    pub css: Option<CssStyle>,
    /// Shell settings
    pub settings: ShellSettings,
}
