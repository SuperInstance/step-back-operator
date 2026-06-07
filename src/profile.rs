//! Profile — the distribution that emerges when you step back from individual hooks.

use crate::{Network, Vertex, Edge, BettiNumbers};

#[derive(Debug, Clone)]
pub struct Profile {
    pub label: String,
    pub mean_weight: f64,
    pub density: f64,
    pub betti: BettiNumbers,
    pub cluster_count: usize,
}

impl Profile {
    pub fn from_network(label: &str, network: &Network) -> Self {
        let betti = network.compute_betti();
        let weights: Vec<f64> = network.vertices.values().map(|v| v.weight).collect();
        let mean = if weights.is_empty() { 0.0 } else { weights.iter().sum::<f64>() / weights.len() as f64 };
        
        Self {
            label: label.to_string(),
            mean_weight: mean,
            density: network.density(),
            betti: betti.clone(),
            cluster_count: betti.b0 as usize,
        }
    }

    pub fn is_healthy(&self) -> bool {
        // A healthy network has reasonable density and no too many holes
        self.density > 0.1 && self.density < 0.9 && self.betti.complexity() < 0.5
    }

    pub fn has_pattern(&self) -> bool {
        self.betti.has_holes()
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} density={:.2} clusters={} betti={}", 
            self.label, 
            if self.has_pattern() { "PATTERN" } else { "flat" },
            self.density, 
            self.cluster_count,
            self.betti
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_triangle() -> Network {
        let mut net = Network::new();
        net.add_vertex(Vertex::new("a").with_weight(1.0));
        net.add_vertex(Vertex::new("b").with_weight(0.8));
        net.add_vertex(Vertex::new("c").with_weight(0.9));
        net.add_edge(Edge::new("a", "b"));
        net.add_edge(Edge::new("b", "c"));
        net.add_edge(Edge::new("c", "a"));
        net
    }

    #[test]
    fn test_profile_triangle() {
        let net = make_triangle();
        let p = Profile::from_network("test", &net);
        assert!(p.has_pattern());
        assert_eq!(p.betti.b1, 1);
        assert!((p.mean_weight - 0.9).abs() < 1e-10);
    }

    #[test]
    fn test_profile_display() {
        let net = make_triangle();
        let p = Profile::from_network("fishing", &net);
        let s = format!("{}", p);
        assert!(s.contains("PATTERN"));
        assert!(s.contains("fishing"));
    }

    #[test]
    fn test_profile_empty() {
        let net = Network::new();
        let p = Profile::from_network("empty", &net);
        assert!(!p.has_pattern());
        assert_eq!(p.density, 0.0);
    }

    #[test]
    fn test_profile_healthy() {
        let mut net = Network::new();
        for i in 0..5 { net.add_vertex(Vertex::new(format!("v{}", i))); }
        // Add some edges for reasonable density
        net.add_edge(Edge::new("v0", "v1"));
        net.add_edge(Edge::new("v1", "v2"));
        net.add_edge(Edge::new("v2", "v3"));
        net.add_edge(Edge::new("v3", "v4"));
        let p = Profile::from_network("healthy", &net);
        assert!(p.is_healthy());
    }
}
