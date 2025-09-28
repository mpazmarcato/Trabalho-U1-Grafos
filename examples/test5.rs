use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::Graph; 

fn main() {
    // Criando um grafo: 2 ── 0 ── 1
    let graph = AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]);

    println!("Grau de cada vértice (lista de adjacência):");
    for i in 0..graph.order() {
        println!("Vértice {}: {}", i, graph.node_degree(&i));
    }

    let matrix = AdjacencyMatrix(vec![
        vec![0, 1, 1], // 0 conectado a 1 e 2
        vec![1, 0, 0], // 1 conectado a 0
        vec![1, 0, 0], // 2 conectado a 0
    ]);

    println!("Grau de cada vértice (matriz de adjacência):");
    for i in 0..matrix.0.len() {
        println!("Vértice {}: {}", i, matrix.node_degree(i));
    }

//    let matrix = AdjacencyMatrix(vec![
//         vec![0, 1, 1], // 0 conectado a 1 e 2
//         vec![1, 0, 0], // 1 conectado a 0
//         vec![1, 0, 0], // 2 conectado a 0
//     ]);

//     let incidence = IncidenceMatrix::from_adjacency_matrix(&matrix);

//     println!("Grau de cada vértice (matriz de incidência ajustado):");
//     for i in 0..incidence.0[0].len() {
//         // Divide por 2 para grafos não direcionados
//         let degree = incidence.node_degree(i) / 2;
//         println!("Vértice {}: {}", i, degree);
//     }
}
