//! Vertices — the events themselves. Each hook pull. Each snap.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Vertex {
    pub id: String,
    pub labels: HashMap<String, String>,
    pub weight: f64,
}

impl Vertex {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into(), labels: HashMap::new(), weight: 1.0 }
    }

    pub fn with_label(mut self, key: &str, val: &str) -> Self {
        self.labels.insert(key.to_string(), val.to_string());
        self
    }

    pub fn with_weight(mut self, w: f64) -> Self {
        self.weight = w;
        self
    }

    pub fn label(&self, key: &str) -> Option<&str> {
        self.labels.get(key).map(|s| s.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertex_new() {
        let v = Vertex::new("hook-1");
        assert_eq!(v.id, "hook-1");
        assert_eq!(v.weight, 1.0);
    }

    #[test]
    fn test_vertex_labels() {
        let v = Vertex::new("hook-1").with_label("depth", "12fathoms");
        assert_eq!(v.label("depth"), Some("12fathoms"));
        assert_eq!(v.label("temp"), None);
    }

    #[test]
    fn test_vertex_weight() {
        let v = Vertex::new("heavy").with_weight(3.5);
        assert!((v.weight - 3.5).abs() < 1e-10);
    }
}
