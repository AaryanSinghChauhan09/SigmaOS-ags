//! Configuration for ags.

/// Shell settings
#[derive(Debug, Clone)]
pub struct ShellSettings {
    /// Theme name
    pub theme: String,
    /// Font size
    pub font_size: u32,
    /// Margin
    pub margin: u32,
    /// Padding
    pub padding: u32,
    /// Bar height
    pub bar_height: u32,
    /// Bar margin
    pub bar_margin: u32,
    /// Animation duration
    pub animation_duration: u32,
    /// Enable animations
    pub enable_animations: bool,
    /// Auto-hide bars
    pub auto_hide_bars: bool,
    /// Show notifications
    pub show_notifications: bool,
    /// Notification position
    pub notification_position: String,
}

impl Default for ShellSettings {
    fn default() -> Self {
        Self {
            theme: "default".into(),
            font_size: 12,
            margin: 8,
            padding: 4,
            bar_height: 32,
            bar_margin: 8,
            animation_duration: 200,
            enable_animations: true,
            auto_hide_bars: false,
            show_notifications: true,
            notification_position: "top-right".into(),
        }
    }
}

/// Shell configuration
#[derive(Debug, Clone)]
pub struct ShellConfig {
    /// Shell settings
    pub settings: ShellSettings,
    /// Bar configurations
    pub bars: std::collections::HashMap<String, crate::bar::Bar>,
    /// CSS styles
    pub css: Option<crate::css::CssStyle>,
    /// Plugins
    pub plugins: Vec<String>,
    /// Config file path
    pub config_path: String,
}

impl Default for ShellConfig {
    fn default() -> Self {
        Self {
            settings: ShellSettings::default(),
            bars: std::collections::HashMap::new(),
            css: None,
            plugins: vec![],
            config_path: "/etc/ags/config.json".into(),
        }
    }
}
