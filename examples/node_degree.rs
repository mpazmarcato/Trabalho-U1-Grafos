use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::Graph; 

fn main() {
    // Grafo simples: 0 ── 1 ── 2
    let graph = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);

    println!("Grau de cada vértice:");
    for i in 0..graph.order() {
        println!("Vértice {}: {}", i, graph.node_degree(i));
    }
}
