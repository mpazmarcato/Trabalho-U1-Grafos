use graphs_algorithms::Graph;
use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};

fn main() {
    let lista = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);
    let matriz = AdjacencyMatrix(vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]]);

    let adjacency_matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]);

    // Gerando a matriz de incidência a partir da matriz de adjacência
    let incidencia = IncidenceMatrix::from_adjacency_matrix(&adjacency_matrix);

    println!("Número de vértices (lista): {}", lista.order());
    println!("Número de vértices (matriz): {}", matriz.order());
    println!("Número de vértices (incidência): {}", incidencia.order());
}
