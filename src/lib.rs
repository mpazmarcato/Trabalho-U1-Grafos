mod graph;

mod adjacency_list;
mod adjacency_matrix;
mod incidence_matrix;

pub use graph::DfsEvent;
pub use graph::Directed;
pub use graph::Direction;
pub use graph::Edge;
pub use graph::FromGraph;
pub use graph::Graph;
pub use graph::Undirected;
pub use graph::UndirectedGraph;

pub mod graphs {
    pub use crate::{
        adjacency_list::AdjacencyList, adjacency_matrix::AdjacencyMatrix,
        incidence_matrix::IncidenceMatrix,
    };
}
