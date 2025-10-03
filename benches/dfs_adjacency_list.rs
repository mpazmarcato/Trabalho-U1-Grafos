mod common;

use cpp_api::AdjacencyListCpp;
use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use graphs_algorithms::Graph;
use graphs_algorithms::graphs::AdjacencyList;

use common::create_complete_graph;

fn bench_dfs_comparison(c: &mut Criterion) {
    let sizes = vec![500, 1000, 2000];

    let mut group = c.benchmark_group("dfs");

    for size in sizes {
        let rust_list = AdjacencyList::from_adjacency_matrix(&create_complete_graph(size));
        let cpp_list = AdjacencyListCpp::from_adjacency_matrix(&create_complete_graph(size));

        group.bench_with_input(BenchmarkId::new("rust", size), &size, |b, _| {
            b.iter(|| rust_list.dfs(0).for_each(|_| ()))
        });

        group.bench_with_input(BenchmarkId::new("cpp", size), &size, |b, _| {
            b.iter(|| cpp_list.dfs(0))
        });
    }

    group.finish();
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(std::time::Duration::from_secs(30));
    targets = bench_dfs_comparison
}

criterion_main!(benches);
