//! SigmaOS-ags — comprehensive Aylur's GTK Shell component.
//!
//! Provides GTK-based desktop shell functionality for Quickshell-style
//! desktop environments, including bar configuration, widget management,
//! and CSS styling.

pub mod bar;
pub mod widget;
pub mod css;
pub mod config;
pub mod manager;
pub mod options;

pub use bar::{Bar, BarPosition, BarModule, ModuleType};
pub use widget::{Widget, WidgetType, WidgetProperty};
pub use css::{CssStyle, CssRule, CssSelector};
pub use config::{ShellConfig, ShellSettings};
pub use manager::{ShellManager, ShellManagerOptions};
pub use options::{Mode, Options};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bar_position() {
        let positions = vec![
            BarPosition::Top,
            BarPosition::Bottom,
            BarPosition::Left,
            BarPosition::Right,
        ];
        assert_eq!(positions.len(), 4);
    }
    
    #[test]
    fn test_module_type() {
        let types = vec![
            ModuleType::Clock,
            ModuleType::Battery,
            ModuleType::Network,
            ModuleType::Bluetooth,
            ModuleType::Volume,
            ModuleType::Brightness,
            ModuleType::CPU,
            ModuleType::Memory,
            ModuleType::Disk,
            ModuleType::Temperature,
            ModuleType::Notification,
            ModuleType::Workspaces,
            ModuleType::WindowList,
            ModuleType::Tray,
            ModuleType::Custom,
        ];
        assert_eq!(types.len(), 15);
    }
    
    #[test]
    fn test_widget_type() {
        let types = vec![
            WidgetType::Label,
            WidgetType::Button,
            WidgetType::Image,
            WidgetType::ProgressBar,
            WidgetType::Slider,
            WidgetType::Entry,
            WidgetType::Box,
            WidgetType::Grid,
            WidgetType::Stack,
            WidgetType::Overlay,
        ];
        assert_eq!(types.len(), 10);
    }
    
    #[test]
    fn test_bar_creation() {
        let bar = Bar::new("top".into(), BarPosition::Top);
        assert_eq!(bar.name, "top");
        assert_eq!(bar.position, BarPosition::Top);
        assert!(bar.modules.is_empty());
    }
    
    #[test]
    fn test_add_module_to_bar() {
        let mut bar = Bar::new("top".into(), BarPosition::Top);
        let module = BarModule::new("clock".into(), ModuleType::Clock);
        
        bar.add_module(module);
        assert_eq!(bar.modules.len(), 1);
        assert_eq!(bar.modules[0].name, "clock");
    }
    
    #[test]
    fn test_widget_creation() {
        let widget = Widget::new("clock".into(), WidgetType::Label);
        assert_eq!(widget.name, "clock");
        assert_eq!(widget.widget_type, WidgetType::Label);
    }
    
    #[test]
    fn test_css_rule_creation() {
        let rule = CssRule::new("#clock".into(), vec![
            ("color".into(), "white".into()),
            ("font-size".into(), "16px".into()),
        ]);
        
        assert_eq!(rule.selector, "#clock");
        assert_eq!(rule.properties.len(), 2);
    }
    
    #[test]
    fn test_shell_manager_creation() {
        let mut manager = ShellManager::new();
        assert!(manager.list_bars().is_empty());
    }
    
    #[test]
    fn test_create_bar() {
        let mut manager = ShellManager::new();
        let bar_id = manager.create_bar("top".into(), BarPosition::Top);
        
        assert!(manager.get_bar(&bar_id).is_some());
        assert_eq!(manager.list_bars().len(), 1);
    }
    
    #[test]
    fn test_add_module_to_bar_manager() {
        let mut manager = ShellManager::new();
        let bar_id = manager.create_bar("top".into(), BarPosition::Top);
        
        let module = BarModule::new("clock".into(), ModuleType::Clock);
        assert!(manager.add_module_to_bar(&bar_id, module));
        
        let bar = manager.get_bar(&bar_id).unwrap();
        assert_eq!(bar.modules.len(), 1);
    }
    
    #[test]
    fn test_load_css() {
        let mut manager = ShellManager::new();
        manager.load_css("#clock { color: white; }".into());
        
        assert!(manager.get_css().is_some());
    }
    
    #[test]
    fn test_shell_settings() {
        let mut settings = ShellSettings::default();
        settings.theme = "dark".into();
        settings.font_size = 14;
        settings.margin = 8;
        
        assert_eq!(settings.theme, "dark");
        assert_eq!(settings.font_size, 14);
        assert_eq!(settings.margin, 8);
    }
    
    #[test]
    fn test_export_import() {
        let mut manager = ShellManager::new();
        let bar_id = manager.create_bar("top".into(), BarPosition::Top);
        
        let module = BarModule::new("clock".into(), ModuleType::Clock);
        manager.add_module_to_bar(&bar_id, module);
        
        manager.load_css("#clock { color: white; }".into());
        
        let export = manager.export_data();
        
        let mut manager2 = ShellManager::new();
        manager2.import_data(export);
        
        assert_eq!(manager2.list_bars().len(), 1);
        assert!(manager2.get_bar(&bar_id).is_some());
    }
    
    #[test]
    fn test_bar_statistics() {
        let mut manager = ShellManager::new();
        
        manager.create_bar("top".into(), BarPosition::Top);
        manager.create_bar("bottom".into(), BarPosition::Bottom);
        
        let stats = manager.get_statistics();
        assert_eq!(stats.total_bars, 2);
        assert_eq!(stats.total_modules, 0);
    }
}
