//! A Rust crate for representing and manipulating graphs using various data structures.
//!
//! This crate provides traits and implementations for directed and undirected graphs,
//! supporting adjacency lists, adjacency matrices, and incidence matrices. It also
//! includes I/O traits for importing and exporting graphs, as well as utility functions
//! for printing and inspecting graph structures.
//!
//! # Modules
//! - `graph`: Core graph traits and BFS/DFS events.
//! - `adjacency_list`: Implementation of graphs using adjacency lists.
//! - `adjacency_matrix`: Implementation of graphs using adjacency matrices.
//! - `incidence_matrix`: Implementation of graphs using incidence matrices.
//! - `graph_io`: Traits for importing/exporting graphs.
//! - `utils`: Helper functions for printing things on the shell.
mod adjacency_list;
mod adjacency_matrix;
mod graph;
mod graph_io;
mod incidence_matrix;
pub mod utils;

pub use graph::BfsEvent;
pub use graph::DfsEvent;
pub use graph::Edge;
pub use graph::Graph;
pub use graph::UndirectedGraph;
pub use graph_io::GraphIO;
pub use graph_io::UndirectedGraphIO;
pub use utils::print_list;
pub use utils::print_matrix;

pub mod graphs {
    pub use crate::{
        adjacency_list::AdjacencyList, adjacency_matrix::AdjacencyMatrix,
        incidence_matrix::IncidenceMatrix,
    };
}
