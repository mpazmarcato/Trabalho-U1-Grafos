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

    let incidence = IncidenceMatrix(vec![vec![1, 1, 0], vec![0, 1, 1]]);

    println!(
        "Número de arestas (incidência): {}",
        incidence.undirected_size()
    );
}
