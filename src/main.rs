mod graphs;

use graphs::AdjacencyList;
use graphs::AdjacencyMatrix;

fn main() {
    let matrix = AdjacencyMatrix(vec![vec![0, 1, 0], vec![1, 1, 0], vec![0, 1, 0]]);

    let list = AdjacencyList::from_adjacency_matrix(&matrix);

    println!("Adjacency list: ");
    for (i, neighbours) in list.0.iter().enumerate() {
        println!("{i} -> {neighbours:?}")
    }
}
