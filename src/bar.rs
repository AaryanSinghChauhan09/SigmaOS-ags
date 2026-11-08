//! Bar configuration for ags.

/// Bar position
#[derive(Debug, Clone, PartialEq)]
pub enum BarPosition {
    /// Top bar
    Top,
    /// Bottom bar
    Bottom,
    /// Left bar
    Left,
    /// Right bar
    Right,
}

/// Module type
#[derive(Debug, Clone, PartialEq)]
pub enum ModuleType {
    /// Clock module
    Clock,
    /// Battery module
    Battery,
    /// Network module
    Network,
    /// Bluetooth module
    Bluetooth,
    /// Volume module
    Volume,
    /// Brightness module
    Brightness,
    /// CPU module
    CPU,
    /// Memory module
    Memory,
    /// Disk module
    Disk,
    /// Temperature module
    Temperature,
    /// Notification module
    Notification,
    /// Workspaces module
    Workspaces,
    /// Window list module
    WindowList,
    /// Tray module
    Tray,
    /// Custom module
    Custom,
}

/// Bar module
#[derive(Debug, Clone)]
pub struct BarModule {
    /// Module name
    pub name: String,
    /// Module type
    pub module_type: ModuleType,
    /// Module configuration
    pub config: std::collections::HashMap<String, String>,
    /// Module enabled
    pub enabled: bool,
    /// Module priority
    pub priority: u32,
}

impl BarModule {
    /// Create new bar module
    pub fn new(name: String, module_type: ModuleType) -> Self {
        Self {
            name,
            module_type,
            config: std::collections::HashMap::new(),
            enabled: true,
            priority: 0,
        }
    }
    
    /// Set configuration value
    pub fn set_config(&mut self, key: String, value: String) {
        self.config.insert(key, value);
    }
    
    /// Get configuration value
    pub fn get_config(&self, key: &str) -> Option<&String> {
        self.config.get(key)
    }
    
    /// Enable module
    pub fn enable(&mut self) {
        self.enabled = true;
    }
    
    /// Disable module
    pub fn disable(&mut self) {
        self.enabled = false;
    }
    
    /// Set priority
    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }
}

/// Bar
#[derive(Debug, Clone)]
pub struct Bar {
    /// Bar name
    pub name: String,
    /// Bar position
    pub position: BarPosition,
    /// Bar modules
    pub modules: Vec<BarModule>,
    /// Bar height
    pub height: u32,
    /// Bar margin
    pub margin: u32,
    /// Bar visible
    pub visible: bool,
}

impl Bar {
    /// Create new bar
    pub fn new(name: String, position: BarPosition) -> Self {
        Self {
            name,
            position,
            modules: vec![],
            height: 32,
            margin: 8,
            visible: true,
        }
    }
    
    /// Add module to bar
    pub fn add_module(&mut self, module: BarModule) {
        self.modules.push(module);
        self.modules.sort_by_key(|m| m.priority);
    }
    
    /// Remove module from bar
    pub fn remove_module(&mut self, module_name: &str) -> bool {
        if let Some(pos) = self.modules.iter().position(|m| m.name == module_name) {
            self.modules.remove(pos);
            return true;
        }
        false
    }
    
    /// Get module
    pub fn get_module(&self, module_name: &str) -> Option<&BarModule> {
        self.modules.iter().find(|m| m.name == module_name)
    }
    
    /// Get module mutably
    pub fn get_module_mut(&mut self, module_name: &str) -> Option<&mut BarModule> {
        self.modules.iter_mut().find(|m| m.name == module_name)
    }
    
    /// Set bar height
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }
    
    /// Set bar margin
    pub fn set_margin(&mut self, margin: u32) {
        self.margin = margin;
    }
    
    /// Show bar
    pub fn show(&mut self) {
        self.visible = true;
    }
    
    /// Hide bar
    pub fn hide(&mut self) {
        self.visible = false;
    }
}
