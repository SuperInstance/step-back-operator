//! Network — the complete graph of events, edges, and components.

use crate::{Vertex, Edge, BettiNumbers, step_back};
use crate::component::find_components;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct Network {
    pub vertices: HashMap<String, Vertex>,
    pub edges: Vec<Edge>,
}

impl Network {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_vertex(&mut self, v: Vertex) {
        self.vertices.insert(v.id.clone(), v);
    }

    pub fn add_edge(&mut self, e: Edge) {
        self.edges.push(e);
    }

    pub fn compute_betti(&self) -> BettiNumbers {
        let vertex_ids: Vec<String> = self.vertices.keys().cloned().collect();
        let edge_pairs: Vec<(String, String)> = self.edges.iter()
            .map(|e| (e.source.clone(), e.target.clone()))
            .collect();
        let components = find_components(&vertex_ids, &edge_pairs);
        
        let v = self.vertices.len();
        let e = self.edges.len();
        let c = components.len();
        let b1 = step_back(e, v, c);

        BettiNumbers {
            b0: c as isize,  // connected components
            b1,              // holes/loops
            vertex_count: v,
            edge_count: e,
        }
    }

    pub fn density(&self) -> f64 {
        let n = self.vertices.len() as f64;
        if n <= 1.0 { return 0.0; }
        let max_edges = n * (n - 1.0) / 2.0;
        self.edges.len() as f64 / max_edges
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_network() {
        let net = Network::new();
        let betti = net.compute_betti();
        assert_eq!(betti.b0, 0);
        assert_eq!(betti.b1, 0);
    }

    #[test]
    fn test_single_vertex() {
        let mut net = Network::new();
        net.add_vertex(Vertex::new("a"));
        let betti = net.compute_betti();
        assert_eq!(betti.b0, 1);
        assert_eq!(betti.b1, 0);
    }

    #[test]
    fn test_triangle() {
        let mut net = Network::new();
        net.add_vertex(Vertex::new("a"));
        net.add_vertex(Vertex::new("b"));
        net.add_vertex(Vertex::new("c"));
        net.add_edge(Edge::new("a", "b"));
        net.add_edge(Edge::new("b", "c"));
        net.add_edge(Edge::new("c", "a"));
        let betti = net.compute_betti();
        assert_eq!(betti.b0, 1);
        assert_eq!(betti.b1, 1); // one hole
    }

    #[test]
    fn test_tree_no_holes() {
        let mut net = Network::new();
        net.add_vertex(Vertex::new("root"));
        net.add_vertex(Vertex::new("a"));
        net.add_vertex(Vertex::new("b"));
        net.add_vertex(Vertex::new("c"));
        net.add_edge(Edge::new("root", "a"));
        net.add_edge(Edge::new("root", "b"));
        net.add_edge(Edge::new("root", "c"));
        let betti = net.compute_betti();
        assert_eq!(betti.b0, 1);
        assert_eq!(betti.b1, 0); // tree has no holes
    }

    #[test]
    fn test_density() {
        let mut net = Network::new();
        for i in 0..4 { net.add_vertex(Vertex::new(format!("v{}", i))); }
        net.add_edge(Edge::new("v0", "v1"));
        net.add_edge(Edge::new("v1", "v2"));
        // 2 edges out of 6 max = density 1/3
        assert!((net.density() - 2.0/6.0).abs() < 1e-10);
    }
}
