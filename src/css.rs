//! CSS styling for ags.

use std::collections::HashMap;

/// CSS selector
#[derive(Debug, Clone, PartialEq)]
pub enum CssSelector {
    /// ID selector
    Id(String),
    /// Class selector
    Class(String),
    /// Element selector
    Element(String),
    /// Universal selector
    Universal,
}

/// CSS rule
#[derive(Debug, Clone)]
pub struct CssRule {
    /// CSS selector
    pub selector: String,
    /// CSS properties
    pub properties: Vec<(String, String)>,
    /// CSS priority
    pub priority: u32,
}

impl CssRule {
    /// Create new CSS rule
    pub fn new(selector: String, properties: Vec<(String, String)>) -> Self {
        Self {
            selector,
            properties,
            priority: 0,
        }
    }
    
    /// Set property
    pub fn set_property(&mut self, key: String, value: String) {
        if let Some(pos) = self.properties.iter().position(|(k, _)| k == &key) {
            self.properties[pos] = (key, value);
        } else {
            self.properties.push((key, value));
        }
    }
    
    /// Get property
    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
    
    /// Set priority
    pub fn set_priority(&mut self, priority: u32) {
        self.priority = priority;
    }
}

/// CSS style
#[derive(Debug, Clone)]
pub struct CssStyle {
    /// CSS rules
    pub rules: Vec<CssRule>,
    /// CSS variables
    pub variables: HashMap<String, String>,
    /// Theme name
    pub theme: String,
}

impl Default for CssStyle {
    fn default() -> Self {
        Self {
            rules: vec![],
            variables: HashMap::new(),
            theme: "default".into(),
        }
    }
}

impl CssStyle {
    /// Create new CSS style
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add CSS rule
    pub fn add_rule(&mut self, rule: CssRule) {
        self.rules.push(rule);
    }
    
    /// Remove CSS rule
    pub fn remove_rule(&mut self, selector: &str) -> bool {
        if let Some(pos) = self.rules.iter().position(|r| r.selector == selector) {
            self.rules.remove(pos);
            return true;
        }
        false
    }
    
    /// Get CSS rule
    pub fn get_rule(&self, selector: &str) -> Option<&CssRule> {
        self.rules.iter().find(|r| r.selector == selector)
    }
    
    /// Get all rules
    pub fn get_all_rules(&self) -> &[CssRule] {
        &self.rules
    }
    
    /// Set CSS variable
    pub fn set_variable(&mut self, name: String, value: String) {
        self.variables.insert(name, value);
    }
    
    /// Get CSS variable
    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.variables.get(name)
    }
    
    /// Set theme
    pub fn set_theme(&mut self, theme: String) {
        self.theme = theme;
    }
    
    /// Get theme
    pub fn get_theme(&self) -> &str {
        &self.theme
    }
    
    /// Parse CSS string
    pub fn parse_css(&mut self, css: &str) {
        // Simple CSS parser - in real implementation would use proper CSS parser
        for line in css.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("/*") || line.starts_with("//") {
                continue;
            }
            
            // Parse selector and properties
            if let Some(brace_pos) = line.find('{') {
                let selector = line[..brace_pos].trim().to_string();
                let properties_str = &line[brace_pos + 1..];
                
                if let Some(end_brace) = properties_str.find('}') {
                    let properties_str = &properties_str[..end_brace];
                    let mut properties = vec![];
                    
                    for prop in properties_str.split(';') {
                        if let Some(colon_pos) = prop.find(':') {
                            let key = prop[..colon_pos].trim().to_string();
                            let value = prop[colon_pos + 1..].trim().to_string();
                            if !key.is_empty() && !value.is_empty() {
                                properties.push((key, value));
                            }
                        }
                    }
                    
                    if !properties.is_empty() {
                        let rule = CssRule::new(selector, properties);
                        self.add_rule(rule);
                    }
                }
            }
        }
    }
    
    /// Export CSS
    pub fn export_css(&self) -> String {
        let mut css = String::new();
        
        // Export variables
        for (name, value) in &self.variables {
            css.push_str(&format!("--{}: {};\n", name, value));
        }
        
        // Export rules
        for rule in &self.rules {
            css.push_str(&format!("{} {{\n", rule.selector));
            for (key, value) in &rule.properties {
                css.push_str(&format!("  {}: {};\n", key, value));
            }
            css.push_str("}\n\n");
        }
        
        css
    }
}
