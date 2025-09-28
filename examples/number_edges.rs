use graphs_algorithms::UndirectedGraph;
use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};

fn main() {
    let lista = AdjacencyList(vec![vec![1], vec![0, 2], vec![1]]);
    println!("Número de arestas (lista): {}", lista.undirected_size());

    let matriz = AdjacencyMatrix(vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]]);
    println!(
        "Número de arestas (matriz de adjacências): {}",
        matriz.undirected_size()
    );

    let adjacency_matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]);

    // Gerando a matriz de incidência a partir da matriz de adjacência
    let incidencia = IncidenceMatrix::from_adjacency_matrix(&adjacency_matrix);

    println!("Número de arestas (incidência): {}", incidencia.size());
}
