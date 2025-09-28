use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix};
use graphs_algorithms::{DfsEvent, Graph};

fn create_adjacency_matrix(size: usize) -> AdjacencyMatrix {
    let mut matrix = vec![vec![0; size]; size];
    for i in 0..size {
        matrix[i][(i + 1) % size] = 1;
        matrix[(i + 1) % size][i] = 1;
    }
    AdjacencyMatrix(matrix)
}

fn bench_dfs_matrix(c: &mut Criterion) {
    let sizes = vec![10, 50, 100, 500];

    for size in sizes {
        let matrix = create_adjacency_matrix(size);
        c.bench_with_input(
            BenchmarkId::new("dfs_adjacency_matrix", size),
            &size,
            |b, _| {
                b.iter(|| {
                    matrix
                        .dfs(0)
                        .filter(|e| matches!(e, DfsEvent::Discover(_, _)))
                        .for_each(|_| ());
                })
            },
        );
    }
}

fn bench_dfs_list(c: &mut Criterion) {
    let sizes = vec![10, 50, 100, 500];

    for size in sizes {
        let matrix = create_adjacency_matrix(size);
        let list = AdjacencyList::from_adjacency_matrix(&matrix);
        c.bench_with_input(
            BenchmarkId::new("dfs_adjacency_list", size),
            &size,
            |b, _| {
                b.iter(|| {
                    list.dfs(0)
                        .filter(|e| matches!(e, DfsEvent::Discover(_, _)))
                        .for_each(|_| ())
                })
            },
        );
    }
}

criterion_group!(benches, bench_dfs_matrix, bench_dfs_list);
criterion_main!(benches);
