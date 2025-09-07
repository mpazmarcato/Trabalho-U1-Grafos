mod graphs;

use graphs::AdjacencyList;
use graphs::AdjacencyMatrix;

use crate::graphs::IncidenceMatrix;

fn main() {
    let m1 = AdjacencyMatrix(vec![vec![0, 1, 0], vec![1, 1, 0], vec![0, 1, 0]]);

    let list = AdjacencyList::from_adjacency_matrix(&m1);

    println!("Adjacency list: ");
    for (i, neighbours) in list.0.iter().enumerate() {
        println!("{i} -> {neighbours:?}")
    }

    let m2 = AdjacencyMatrix(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 0, 0],
    ]);
    let incidence = IncidenceMatrix::from_adjacency_matrix(&m2);

    println!("Incidence matrix: ");
    for (i, row) in incidence.0.iter().enumerate() {
        for (j, &col) in row.iter().enumerate() {
            print!("{col} ");
        }
        println!();
    }

    let m3 = AdjacencyMatrix(vec![
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

    AdjacencyMatrix::dfs(m3.0);
}
