use graphs_algorithms::Graph;
use graphs_algorithms::UndirectedGraph;
use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};

fn main() {
    let graph = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);

    println!("Grau de cada vértice:");
    for i in 0..graph.order() {
        println!("Vértice {}: {}", i, graph.undirected_node_degree(i));
    }

    let matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]);

    println!("Grau de cada vértice:");
    for i in 0..matrix.order() {
        println!("Vértice {}: {}", i, matrix.undirected_node_degree(i));
    }

    let incidence = IncidenceMatrix(vec![vec![1, 1, 0], vec![0, 1, 1]]);

    println!("Grau de cada vértice (Matriz de Incidência):");
    for i in 0..incidence.order() {
        println!("Vértice {}: {}", i, incidence.undirected_node_degree(i));
    }
}
