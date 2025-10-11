mod graph;

mod adjacency_list;
mod adjacency_matrix;
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
