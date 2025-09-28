use graphs_algorithms::Graph;
use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};

fn main() {
    let graph = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);

    println!("Grau de cada vértice:");
    for i in 0..graph.order() {
        println!("Vértice {}: {}", i, graph.node_degree(i));
    }

    let matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]);

    println!("Grau de cada vértice:");
    for i in 0..matrix.order() {
        println!("Vértice {}: {}", i, matrix.node_degree(i));
    }

    let adjacency_matrix = AdjacencyMatrix(vec![
        vec![0, 1, 1], // 0 conectado a 1 e 2
        vec![1, 0, 0], // 1 conectado a 0
        vec![1, 0, 0], // 2 conectado a 0
    ]);

    // Gerando a matriz de incidência a partir da matriz de adjacência
    let incidence_matrix = IncidenceMatrix::from_adjacency_matrix(&adjacency_matrix);

    println!("Grau de cada vértice (Matriz de Incidência):");
    for i in 0..matrix.order() {
        println!("Vértice {}: {}", i, incidence_matrix.node_degree(i));
    }
}
