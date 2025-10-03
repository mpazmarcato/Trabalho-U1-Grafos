use graphs_algorithms::Graph;
use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix};

fn main() {
    let lista = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);
    let matriz = AdjacencyMatrix(vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]]);

    println!("Número de vértices (lista): {}", lista.order());
    println!("Número de vértices (matriz): {}", matriz.order());
}
