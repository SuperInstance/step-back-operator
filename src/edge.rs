//! Edges — connections between events. The relation across all of them.

#[derive(Debug, Clone)]
pub struct Edge {
    pub source: String,
    pub target: String,
    pub weight: f64,
    pub label: String,
}

impl Edge {
    pub fn new(source: &str, target: &str) -> Self {
        Self { source: source.to_string(), target: target.to_string(), weight: 1.0, label: String::new() }
    }

    pub fn with_weight(mut self, w: f64) -> Self {
        self.weight = w;
        self
    }

    pub fn with_label(mut self, label: &str) -> Self {
        self.label = label.to_string();
        self
    }

    pub fn connects(&self, a: &str, b: &str) -> bool {
        (self.source == a && self.target == b) || (self.source == b && self.target == a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_new() {
        let e = Edge::new("a", "b");
        assert_eq!(e.source, "a");
        assert_eq!(e.target, "b");
        assert_eq!(e.weight, 1.0);
    }

    #[test]
    fn test_edge_connects() {
        let e = Edge::new("a", "b");
        assert!(e.connects("a", "b"));
        assert!(e.connects("b", "a"));
        assert!(!e.connects("a", "c"));
    }

    #[test]
    fn test_edge_weight() {
        let e = Edge::new("x", "y").with_weight(0.7);
        assert!((e.weight - 0.7).abs() < 1e-10);
    }
}
