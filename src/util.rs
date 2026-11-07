//! ags model — Aylur's GTK Shell (Quickshell predecessor).

#[derive(Debug, Clone)]
pub struct Bar {
    pub name: String,
    pub position: String,
    pub modules: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Model {
    pub bars: Vec<Bar>,
    pub css_loaded: bool,
}

impl Default for Model {
    fn default() -> Self { Self { bars: vec![
        Bar { name: "top".into(), position: "top".into(), modules: vec!["clock".into(), "battery".into(), "network".into()] },
    ], css_loaded: false } }
}

impl Model {
    pub fn new() -> Self { Self::default() }
    pub fn list(&self) -> &[Bar] { &self.bars }
    pub fn load_css(&mut self) { self.css_loaded = true; }
    pub fn bar(&self, name: &str) -> Option<&Bar> {
        self.bars.iter().find(|b| b.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn load_css() {
        let mut a = Model::new();
        assert!(!a.css_loaded);
        a.load_css();
        assert!(a.css_loaded);
        assert_eq!(a.bar("top").unwrap().modules.len(), 3);
    }
}
