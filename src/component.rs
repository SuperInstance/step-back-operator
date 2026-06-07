//! Connected components — clusters of related events.

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct ConnectedComponent {
    pub vertices: HashSet<String>,
    pub id: usize,
}

impl ConnectedComponent {
    pub fn new(id: usize) -> Self {
        Self { vertices: HashSet::new(), id }
    }

    pub fn with_vertices(mut self, verts: &[&str]) -> Self {
        for v in verts {
            self.vertices.insert(v.to_string());
        }
        self
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn contains(&self, vertex: &str) -> bool {
        self.vertices.contains(vertex)
    }
}

/// Find connected components using union-find
pub fn find_components(
    vertex_ids: &[String],
    edges: &[(String, String)],
) -> Vec<ConnectedComponent> {
    let mut parent: HashMap<&str, &str> = HashMap::new();
    for v in vertex_ids {
        parent.insert(v, v);
    }

    fn find<'a>(parent: &HashMap<&'a str, &'a str>, x: &'a str) -> &'a str {
        let mut root = x;
        while parent.get(root).copied() != Some(root) {
            root = parent[root];
        }
        root
    }

    for (a, b) in edges {
        if let (Some(ra), Some(rb)) = (find(&parent, a).into(), find(&parent, b).into()) {
            let ra = find(&parent, a);
            let rb = find(&parent, b);
            if ra != rb {
                parent.insert(ra, rb);
            }
        }
    }

    let mut groups: HashMap<String, Vec<String>> = HashMap::new();
    for v in vertex_ids {
        let root = find(&parent, v.as_str()).to_string();
        groups.entry(root).or_default().push(v.clone());
    }

    groups.into_values().enumerate().map(|(i, verts)| {
        let mut comp = ConnectedComponent::new(i);
        for v in verts {
            comp.vertices.insert(v);
        }
        comp
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_new() {
        let c = ConnectedComponent::new(0).with_vertices(&["a", "b", "c"]);
        assert_eq!(c.size(), 3);
        assert!(c.contains("a"));
        assert!(!c.contains("d"));
    }

    #[test]
    fn test_find_components_disconnected() {
        let verts = vec!["a".into(), "b".into(), "c".into(), "d".into()];
        let edges = vec![("a".into(), "b".into()), ("c".into(), "d".into())];
        let comps = find_components(&verts, &edges);
        assert_eq!(comps.len(), 2);
    }

    #[test]
    fn test_find_components_connected() {
        let verts = vec!["a".into(), "b".into(), "c".into()];
        let edges = vec![("a".into(), "b".into()), ("b".into(), "c".into())];
        let comps = find_components(&verts, &edges);
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].size(), 3);
    }

    #[test]
    fn test_find_components_empty() {
        let comps = find_components(&[], &[]);
        assert_eq!(comps.len(), 0);
    }
}
