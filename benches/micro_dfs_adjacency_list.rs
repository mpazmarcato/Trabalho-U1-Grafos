mod common;

use std::hint::black_box;

use cpp_api::AdjacencyListCpp;
use gungraun::{library_benchmark, library_benchmark_group, main};

use graphs_algorithms::Graph;
use graphs_algorithms::graphs::AdjacencyList;

use common::create_complete_graph;

fn setup() -> AdjacencyList {
    AdjacencyList::from_adjacency_matrix(&create_complete_graph(1000))
}

fn cpp_setup() -> AdjacencyListCpp {
    AdjacencyListCpp::from_adjacency_matrix(&create_complete_graph(1000))
}

#[library_benchmark]
fn single_shot_dfs() {
    setup().dfs(black_box(0)).for_each(|_| ())
}

#[library_benchmark]
fn single_shot_cpp_dfs() {
    cpp_setup().dfs(black_box(0))
}

library_benchmark_group! {
    name = bench_dfs_group;
    benchmarks = single_shot_dfs, single_shot_cpp_dfs
}

main!(library_benchmark_groups = bench_dfs_group);
