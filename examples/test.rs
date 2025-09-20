use graphs_algorithms::graphs::{AdjacencyMatrix, IncidenceMatrix};

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
    m2.dfs();
}
