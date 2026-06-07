//! Snap log — never hide a snap. Every hook pull is a vertex.

use crate::{Network, Vertex, Edge, BettiNumbers};

#[derive(Debug, Clone)]
pub struct SnapEntry {
    pub id: String,
    pub value: bool,      // yes or no
    pub metadata: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
pub struct SnapLog {
    pub entries: Vec<SnapEntry>,
    pub network: Network,
}

impl SnapLog {
    pub fn new() -> Self {
        Self { entries: Vec::new(), network: Network::new() }
    }

    pub fn snap(&mut self, id: &str, value: bool, edges_to: &[&str]) {
        let entry = SnapEntry {
            id: id.to_string(),
            value,
            metadata: Vec::new(),
        };
        self.network.add_vertex(Vertex::new(id).with_weight(if value { 1.0 } else { 0.0 }));
        for target in edges_to {
            self.network.add_edge(Edge::new(id, target));
        }
        self.entries.push(entry);
    }

    pub fn snap_with_meta(&mut self, id: &str, value: bool, edges_to: &[&str], meta: &[(&str, &str)]) {
        self.snap(id, value, edges_to);
        if let Some(entry) = self.entries.last_mut() {
            for (k, v) in meta {
                entry.metadata.push((k.to_string(), v.to_string()));
            }
        }
    }

    pub fn yes_count(&self) -> usize {
        self.entries.iter().filter(|e| e.value).count()
    }

    pub fn no_count(&self) -> usize {
        self.entries.iter().filter(|e| !e.value).count()
    }

    pub fn step_back(&self) -> BettiNumbers {
        self.network.compute_betti()
    }
}

impl Default for SnapLog {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snap_log_basic() {
        let mut log = SnapLog::new();
        log.snap("hook-1", true, &[]);
        log.snap("hook-2", false, &["hook-1"]);
        log.snap("hook-3", true, &["hook-1", "hook-2"]);
        assert_eq!(log.entries.len(), 3);
        assert_eq!(log.yes_count(), 2);
        assert_eq!(log.no_count(), 1);
    }

    #[test]
    fn test_snap_log_step_back() {
        let mut log = SnapLog::new();
        log.snap("a", true, &[]);
        log.snap("b", true, &["a"]);
        log.snap("c", true, &["a", "b"]);
        let betti = log.step_back();
        assert_eq!(betti.b0, 1);
        assert_eq!(betti.b1, 1); // a-b-c-a forms a triangle = 1 hole
    }

    #[test]
    fn test_snap_with_metadata() {
        let mut log = SnapLog::new();
        log.snap_with_meta("hook-1", true, &[], &[("depth", "12fathoms"), ("bearing", "NE")]);
        assert_eq!(log.entries[0].metadata.len(), 2);
    }

    #[test]
    fn test_snap_log_tree() {
        let mut log = SnapLog::new();
        log.snap("root", true, &[]);
        log.snap("a", false, &["root"]);
        log.snap("b", true, &["root"]);
        let betti = log.step_back();
        assert_eq!(betti.b1, 0); // tree, no holes
    }

    #[test]
    fn test_empty_log() {
        let log = SnapLog::new();
        assert_eq!(log.step_back().b0, 0);
    }
}
