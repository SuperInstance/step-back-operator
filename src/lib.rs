//! # Step-Back Operator
//!
//! Topological pattern detection via the first Betti number: β₁ = E - V + C
//!
//! "The intelligence isn't in the hooks. It's in the step-back."
//! — Oracle1, The Step-Back Operator

mod vertex;
mod edge;
mod component;
mod network;
mod betti;
mod snap_log;
mod profile;

pub use vertex::Vertex;
pub use edge::Edge;
pub use component::ConnectedComponent;
pub use network::Network;
pub use betti::BettiNumbers;
pub use snap_log::SnapLog;
pub use profile::Profile;

/// Compute the first Betti number: β₁ = E - V + C
/// The number of "holes" — empty spaces where something could exist but doesn't.
pub fn step_back(edges: usize, vertices: usize, components: usize) -> isize {
    (edges as isize) - (vertices as isize) + (components as isize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_back_triangle() {
        // Triangle: 3 edges, 3 vertices, 1 component → β₁ = 3 - 3 + 1 = 1
        assert_eq!(step_back(3, 3, 1), 1);
    }

    #[test]
    fn test_step_back_tree() {
        // Tree: 4 edges, 5 vertices, 1 component → β₁ = 4 - 5 + 1 = 0
        assert_eq!(step_back(4, 5, 1), 0);
    }

    #[test]
    fn test_step_back_two_triangles() {
        // Two triangles sharing a vertex: 6 edges, 5 vertices, 1 component → β₁ = 2
        assert_eq!(step_back(6, 5, 1), 2);
    }

    #[test]
    fn test_step_back_disconnected() {
        // Two separate triangles: 6 edges, 6 vertices, 2 components → β₁ = 2
        assert_eq!(step_back(6, 6, 2), 2);
    }

    #[test]
    fn test_step_back_empty() {
        assert_eq!(step_back(0, 0, 0), 0);
    }

    #[test]
    fn test_step_back_single_vertex() {
        assert_eq!(step_back(0, 1, 1), 0);
    }
}
