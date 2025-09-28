use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::Graph; 

fn main() {
    let graph = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);

    println!("Grau de cada vértice:");
    for i in 0..graph.order() {
        println!("Vértice {}: {}", i, graph.node_degree(i));
    }

    let matrix = AdjacencyMatrix(vec![
        vec![0, 1, 1],
        vec![1, 0, 1],
        vec![1, 1, 0],
    ]);

    println!("Grau de cada vértice:");
    for i in 0..matrix.order() {
        println!("Vértice {}: {}", i, matrix.node_degree(i));
    }
}
