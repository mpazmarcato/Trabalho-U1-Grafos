use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::{print_list, print_matrix, DfsEvent, Graph};

fn main() {
    let m1 = AdjacencyMatrix(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 0, 0],
    ]);
    let incidence = IncidenceMatrix::from_adjacency_matrix(&m1);

    println!("Incidence matrix: ");
    for row in incidence.0 {
        for col in row {
            print!("{col} ");
        }
        println!();
    }

    let m2 = AdjacencyMatrix(vec![
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
    ]);
    m2.dfs(0);

    // Test conversion between adjacency matrix and adjacency list
    let matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]);

    println!("\nOriginal adjacency matrix:");
    print_matrix(&matrix);

    let list = AdjacencyList::from_adjacency_matrix(&matrix);
    println!("\nAdjacency matrix to adjacency list:");
    print_list(&list);

    let matrix2 = AdjacencyMatrix::from_adjacency_list(&list);
    println!("\nConverted back to adjacency matrix:");
    print_matrix(&matrix2);

    println!(
        "\nDFS visit count: {}",
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

    let list = AdjacencyList(vec![
        vec![1, 2], // 0 → 1,2
        vec![0],    // 1 → 0
        vec![0],    // 2 → 0
    ]);

    println!("\nNew list: ");
    print_list(&list);

    println!("BFS visit count on start 0: {}", list.bfs(0).count());
}
