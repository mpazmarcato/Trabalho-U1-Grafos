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
fn single_shot_bfs() {
    setup().bfs(black_box(0)).for_each(|_| ())
}

#[library_benchmark]
fn single_shot_cpp_bfs() {
    cpp_setup().bfs(black_box(0))
}

library_benchmark_group! {
    name = bench_bfs_group;
    benchmarks = single_shot_bfs, single_shot_cpp_bfs
}

main!(library_benchmark_groups = bench_bfs_group);
