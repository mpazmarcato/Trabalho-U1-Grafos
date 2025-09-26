mod graph;

mod adjacency_list;
mod adjacency_matrix;
mod incidence_matrix;

pub use graph::Edge;
pub use graph::Graph;
pub use graph::UndirectedGraph;

pub mod graphs {
    pub use crate::{
        adjacency_list::AdjacencyList, adjacency_matrix::AdjacencyMatrix,
        incidence_matrix::IncidenceMatrix,
    };
}
