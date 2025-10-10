use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::{DfsEvent, Directed, FromGraph, Graph, Undirected};

fn main() {
    let m1: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 0, 0],
    ])
    .unwrap();
    let incidence = IncidenceMatrix::from_graph(&m1);

    println!("Incidence matrix: ");
    for row in incidence.data() {
        for col in row {
            print!("{col} ");
        }
        println!();
    }

    let m2: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&vec![
        vec![0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0],
        vec![1, 0, 1, 1, 0, 0, 0, 1, 0, 0, 1],
        vec![1, 1, 0, 1, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 1, 1, 0, 0, 0, 1, 1, 1, 0, 0],
        vec![0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 1],
        vec![0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1],
        vec![0, 1, 0, 0, 0, 0, 1, 0, 1, 1, 0],
    ])
    .unwrap();
    m2.dfs(0);

    // Test conversion between adjacency matrix and adjacency list
    let matrix: AdjacencyMatrix<Undirected> =
        AdjacencyMatrix::new(&vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]).unwrap();

    println!("\nOriginal adjacency matrix:");
    for row in &matrix.data() {
        println!("{:?}", row);
    }

    let list = AdjacencyList::from_graph(&matrix);
    println!("\nAdjacency matrix to adjacency list:");
    for (i, neighbors) in list.data().iter().enumerate() {
        println!("{}: {:?}", i, neighbors);
    }

    let matrix2 = AdjacencyMatrix::from_graph(&list);
    println!("\nConverted back to adjacency matrix:");
    for row in &matrix2.data() {
        println!("{:?}", row);
    }

    println!(
        "DFS visit count: {}",
        matrix
            .dfs(0)
            .filter(|e| matches!(e, DfsEvent::Discover(_, _)))
            .count()
    );
    println!(
        "BFS visit count: {}",
        matrix
            .dfs(0)
            .filter(|e| matches!(e, DfsEvent::Discover(_, _)))
            .count()
    );

    let list: AdjacencyList<Undirected> = AdjacencyList::new(&vec![
        vec![1, 2], // 0 → 1,2
        vec![0],    // 1 → 0
        vec![0],    // 2 → 0
    ])
    .unwrap();

    println!("Visitados no BFS: {}", list.bfs(0).count());
}
