//! Betti numbers — the shape of the distribution.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BettiNumbers {
    pub b0: isize, // connected components
    pub b1: isize, // loops/holes
    pub vertex_count: usize,
    pub edge_count: usize,
}

impl BettiNumbers {
    pub fn is_tree(&self) -> bool {
        self.b1 == 0 && self.b0 == 1
    }

    pub fn has_holes(&self) -> bool {
        self.b1 > 0
    }

    pub fn complexity(&self) -> f64 {
        if self.vertex_count == 0 { return 0.0; }
        self.b1 as f64 / self.vertex_count as f64
    }
}

impl std::fmt::Display for BettiNumbers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "β₀={} β₁={} (V={} E={})", self.b0, self.b1, self.vertex_count, self.edge_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let b = BettiNumbers { b0: 1, b1: 0, vertex_count: 5, edge_count: 4 };
        assert!(b.is_tree());
        assert!(!b.has_holes());
    }

    #[test]
    fn test_cycle() {
        let b = BettiNumbers { b0: 1, b1: 1, vertex_count: 3, edge_count: 3 };
        assert!(!b.is_tree());
        assert!(b.has_holes());
    }

    #[test]
    fn test_complexity() {
        let b = BettiNumbers { b0: 1, b1: 5, vertex_count: 10, edge_count: 14 };
        assert!((b.complexity() - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_display() {
        let b = BettiNumbers { b0: 1, b1: 2, vertex_count: 5, edge_count: 6 };
        assert_eq!(format!("{}", b), "β₀=1 β₁=2 (V=5 E=6)");
    }
}
