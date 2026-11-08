//! Widget definitions for ags.

/// Widget type
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetType {
    /// Label widget
    Label,
    /// Button widget
    Button,
    /// Image widget
    Image,
    /// Progress bar widget
    ProgressBar,
    /// Slider widget
    Slider,
    /// Entry widget
    Entry,
    /// Box widget
    Box,
    /// Grid widget
    Grid,
    /// Stack widget
    Stack,
    /// Overlay widget
    Overlay,
}

/// Widget property
#[derive(Debug, Clone, PartialEq)]
pub enum WidgetProperty {
    /// Text content
    Text(String),
    /// Image source
    ImageSource(String),
    /// Visibility
    Visible(bool),
    /// Enabled
    Enabled(bool),
    /// Width
    Width(u32),
    /// Height
    Height(u32),
    /// Margin
    Margin(u32),
    /// Padding
    Padding(u32),
    /// CSS class
    CssClass(String),
    /// CSS ID
    CssId(String),
    /// Custom property
    Custom(String, String),
}

/// Widget
#[derive(Debug, Clone)]
pub struct Widget {
    /// Widget name
    pub name: String,
    /// Widget type
    pub widget_type: WidgetType,
    /// Widget properties
    pub properties: Vec<WidgetProperty>,
    /// Child widgets
    pub children: Vec<Widget>,
    /// Parent widget name
    pub parent: Option<String>,
}

impl Widget {
    /// Create new widget
    pub fn new(name: String, widget_type: WidgetType) -> Self {
        Self {
            name,
            widget_type,
            properties: vec![],
            children: vec![],
            parent: None,
        }
    }
    
    /// Set property
    pub fn set_property(&mut self, property: WidgetProperty) {
        // Remove existing property of same type
        self.properties.retain(|p| std::mem::discriminant(p) != std::mem::discriminant(&property));
        self.properties.push(property);
    }
    
    /// Get property
    pub fn get_property(&self, property_type: &WidgetProperty) -> Option<&WidgetProperty> {
        let target_discriminant = std::mem::discriminant(property_type);
        self.properties.iter()
            .find(|p| std::mem::discriminant(*p) == target_discriminant)
    }
    
    /// Add child widget
    pub fn add_child(&mut self, child: Widget) {
        self.children.push(child);
    }
    
    /// Remove child widget
    pub fn remove_child(&mut self, child_name: &str) -> bool {
        if let Some(pos) = self.children.iter().position(|c| c.name == child_name) {
            self.children.remove(pos);
            return true;
        }
        false
    }
    
    /// Get child widget
    pub fn get_child(&self, child_name: &str) -> Option<&Widget> {
        self.children.iter().find(|c| c.name == child_name)
    }
    
    /// Set parent
    pub fn set_parent(&mut self, parent_name: String) {
        self.parent = Some(parent_name);
    }
    
    /// Get text content
    pub fn get_text(&self) -> Option<&str> {
        self.properties.iter()
            .find_map(|p| {
                if let WidgetProperty::Text(text) = p {
                    Some(text.as_str())
                } else {
                    None
                }
            })
    }
    
    /// Set text content
    pub fn set_text(&mut self, text: String) {
        self.set_property(WidgetProperty::Text(text));
    }
    
    /// Check if visible
    pub fn is_visible(&self) -> bool {
        self.properties.iter()
            .find_map(|p| {
                if let WidgetProperty::Visible(visible) = p {
                    Some(*visible)
                } else {
                    None
                }
            })
            .unwrap_or(true)
    }
    
    /// Show widget
    pub fn show(&mut self) {
        self.set_property(WidgetProperty::Visible(true));
    }
    
    /// Hide widget
    pub fn hide(&mut self) {
        self.set_property(WidgetProperty::Visible(false));
    }
}
