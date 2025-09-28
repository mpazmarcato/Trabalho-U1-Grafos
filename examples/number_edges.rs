use graphs_algorithms::graphs::{AdjacencyMatrix, IncidenceMatrix};

fn main() {
    let adjacency_matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]);

    // Gerando a matriz de incidência a partir da matriz de adjacência
    let incidencia = IncidenceMatrix::from_adjacency_matrix(&adjacency_matrix);

    println!("Número de vértices (incidência): {}", incidencia.size());
}
