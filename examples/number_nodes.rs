use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::{FromGraph, Graph, Undirected, UndirectedGraph};

fn main() {
    let lista: AdjacencyList<Undirected> =
        AdjacencyList::new(&vec![vec![1], vec![0, 2], vec![1]]).unwrap();
    let matriz = AdjacencyMatrix::new(&vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]]).unwrap();

    let adjacency_matrix: AdjacencyMatrix<Undirected> =
        AdjacencyMatrix::new(&vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]).unwrap();

    // Gerando a matriz de incidência a partir da matriz de adjacência
    let incidencia = IncidenceMatrix::from_graph(&adjacency_matrix);

    println!("Número de vértices (lista): {}", lista.undirected_order());
    println!("Número de vértices (matriz): {}", matriz.undirected_order());
    println!("Número de vértices (incidência): {}", incidencia.order());
}
