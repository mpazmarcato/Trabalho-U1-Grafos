mod common;

use std::hint::black_box;

use cpp_api::AdjacencyListCpp;
use graphs_algorithms::Graph;
use graphs_algorithms::graphs::AdjacencyList;
use iai::main;

use common::create_complete_graph;

fn setup() -> AdjacencyList {
    AdjacencyList::from_adjacency_matrix(&create_complete_graph(1000))
}
fn cpp_setup() -> AdjacencyListCpp {
    AdjacencyListCpp::from_adjacency_matrix(&create_complete_graph(1000))
}

fn single_shot_dfs() {
    setup().dfs(black_box(0)).for_each(|_| ())
}

fn single_shot_cpp_dfs() {
    cpp_setup().dfs(black_box(0))
}

main!(setup, cpp_setup, single_shot_dfs, single_shot_cpp_dfs);
