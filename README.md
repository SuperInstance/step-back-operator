# Step-Back Operator

[![crates.io](https://img.shields.io/crates/v/step-back-operator.svg)](https://crates.io/crates/step-back-operator)
[![docs.rs](https://docs.rs/step-back-operator/badge.svg)](https://docs.rs/step-back-operator)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

> **Topological pattern detection via the first Betti number: β₁ = E − V + C**
>
> *"The intelligence isn't in the hooks. It's in the step-back."*
> — Oracle1, The Step-Back Operator

---

## The Problem

When analyzing networks — social graphs, dependency trees, communication meshes — the most interesting patterns are often the **holes**. Not the connections, but the absences. A team where everyone reports to one manager has no loops (β₁ = 0). A team with cross-cutting communication has loops — and those loops carry information about resilience, redundancy, and hidden structure.

Traditional graph analysis focuses on what's there. The Step-Back Operator focuses on **what's not there** — the topological holes that reveal the shape of your data.

## Why This Exists

Built for the SuperInstance ecosystem of cognitive agents, the Step-Back Operator provides the mathematical foundation for detecting when an agent's knowledge graph has "gaps" — empty spaces where something could exist but doesn't. Like a fisherman who doesn't just cast into water but understands the shape of the lake bottom, this crate reveals the topology underneath your observations.

## Architecture

```
                    ┌──────────────┐
                    │   Network    │
                    │  (V, E, C)   │
                    └──────┬───────┘
                           │
                    ┌──────▼───────┐
                    │  Step-Back   │
                    │  β₁ = E-V+C  │
                    └──────┬───────┘
                           │
              ┌────────────┼────────────┐
              │            │            │
       ┌──────▼──┐  ┌──────▼──┐  ┌─────▼────┐
       │ Betti   │  │  Snap   │  │ Profile  │
       │ Numbers │  │  Log    │  │ (fishing)│
       └─────────┘  └─────────┘  └──────────┘

   Vertex ───── Edge ───── Component (Connected)
     │            │              │
     └────────────┼──────────────┘
                    │
              Network topology
              with Betti analysis
```

## Installation

```toml
[dependencies]
step-back-operator = "0.1"
```

## API Reference

### Core Function: `step_back`

The fundamental operation. Given counts of edges, vertices, and connected components, computes the first Betti number:

```rust
use step_back_operator::step_back;

// Triangle: 3 edges, 3 vertices, 1 component → β₁ = 1 (one hole)
let betti = step_back(3, 3, 1);
assert_eq!(betti, 1);

// Tree: 4 edges, 5 vertices, 1 component → β₁ = 0 (no holes)
let betti = step_back(4, 5, 1);
assert_eq!(betti, 0);

// Two triangles sharing a vertex: 6 edges, 5 vertices → β₁ = 2
let betti = step_back(6, 5, 1);
assert_eq!(betti, 2);
```

### `Vertex`

A node in the topological network:

```rust
use step_back_operator::Vertex;

let v = Vertex::new("agent-1");
```

### `Edge`

A connection between two vertices:

```rust
use step_back_operator::{Vertex, Edge};

let a = Vertex::new("a");
let b = Vertex::new("b");
let edge = Edge::new(&a, &b);
```

### `ConnectedComponent`

A group of vertices reachable from each other:

```rust
use step_back_operator::ConnectedComponent;
```

### `Network`

The full topological network with automatic component detection:

```rust
use step_back_operator::Network;

let mut net = Network::new();
net.add_vertex("a");
net.add_vertex("b");
net.add_vertex("c");
net.add_edge("a", "b");
net.add_edge("b", "c");
net.add_edge("c", "a"); // closes the loop → β₁ = 1
```

### `BettiNumbers`

Computed Betti numbers for a network:

```rust
use step_back_operator::BettiNumbers;
```

### `SnapLog`

Immutable log of topological snapshots over time.

### `Profile`

A fishing profile — describes the topological "waters" you're analyzing.

## Usage Examples

### Example 1: Detecting Communication Gaps in a Team

```rust
use step_back_operator::{step_back, Network};

let mut net = Network::new();

// Team members
for name in &["alice", "bob", "carol", "dave"] {
    net.add_vertex(name);
}

// Communication channels
net.add_edge("alice", "bob");
net.add_edge("bob", "carol");
net.add_edge("carol", "dave");
// alice and dave don't talk directly

// β₁ = E - V + C = 3 - 4 + 1 = 0 (tree structure, no loops)
// This means: if bob leaves, the team falls apart
```

### Example 2: Analyzing Resilience via Betti Numbers

```rust
use step_back_operator::step_back;

// Fragile network: tree (β₁ = 0)
let fragile = step_back(4, 5, 1);
assert_eq!(fragile, 0);

// Resilient network: mesh with loops (β₁ = 3)
let resilient = step_back(7, 5, 1);
assert_eq!(resilient, 3);
```

### Example 3: Empty Spaces as Opportunities

```rust
use step_back_operator::step_back;

// Two separate triangles: 2 holes
let betti = step_back(6, 6, 2);
assert_eq!(betti, 2);

// Each "hole" represents an empty space — 
// a communication channel that COULD exist but doesn't
```

## Mathematical Background

The first Betti number (β₁) is a topological invariant from algebraic topology:

```
β₁ = E − V + C
```

Where:
- **E** = number of edges (connections)
- **V** = number of vertices (nodes)
- **C** = number of connected components

This is derived from the **Euler-Poincaré formula**:

```
χ = V − E + F = β₀ − β₁ + β₂ − ...
```

For a connected graph, β₀ = 1, so β₁ = E − V + 1. For multiple components, generalize to β₁ = E − V + C.

**Physical intuition**: β₁ counts the independent loops in a network. A tree has no loops (β₁ = 0). Every additional edge beyond the tree creates exactly one new loop.

### Fishing Metaphor

Think of topology like fishing:
- **Vertices** are the pegs on your fishing net
- **Edges** are the strings connecting pegs
- **Holes** (β₁ > 0) are the gaps in the net where fish slip through
- **Components** are separate nets

The Step-Back Operator doesn't look at the strings — it looks at the holes. That's where the real information lives.

## Performance Characteristics

| Operation | Complexity | Notes |
|-----------|-----------|-------|
| `step_back()` | O(1) | Pure arithmetic |
| Network construction | O(V + E) | Linear in graph size |
| Component detection | O(V + E) | Union-Find |
| Betti computation | O(V + E) | Single pass |

The core `step_back` function is **O(1)** — it's three subtractions and an addition. The real cost is in building the network, which is linear.

## Comparison with Alternatives

| Feature | step-back-operator | petgraph | networkx (Python) |
|---------|-------------------|----------|-------------------|
| Betti number computation | ✅ Direct | ❌ Manual | ❌ Manual |
| Topological focus | ✅ Holes/gaps | ❌ Paths | ❌ General |
| Cognitive agent integration | ✅ Native | ❌ Generic | ❌ Generic |
| Zero-dependency core | ✅ | ❌ | ❌ |
| O(1) Betti computation | ✅ | N/A | N/A |

## License

Licensed under the [MIT License](LICENSE).

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Write tests for your changes
4. Commit with conventional commits (`feat:`, `fix:`, `docs:`)
5. Push and open a Pull Request

All contributions must pass `cargo test` and `cargo clippy`.
