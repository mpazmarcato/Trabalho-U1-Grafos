use graphs_algorithms::UndirectedGraph;
use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::{FromGraph, Graph, Undirected};

fn main() {
    let graph: AdjacencyList<Undirected> =
        AdjacencyList::new(&vec![vec![1], vec![0, 2], vec![1]]).unwrap();

    println!("Grau de cada vértice:");
    for i in 0..graph.order() {
        println!("Vértice {}: {}", i, graph.undirected_node_degree(i));
    }

    let adjacency_matrix: AdjacencyMatrix<Undirected> = AdjacencyMatrix::new(&vec![
        vec![0, 1, 1], // 0 conectado a 1 e 2
        vec![1, 0, 0], // 1 conectado a 0
        vec![1, 0, 0], // 2 conectado a 0
    ])
    .unwrap();

    println!("Grau de cada vértice:");
    for i in 0..adjacency_matrix.order() {
        println!(
            "Vértice {}: {}",
            i,
            adjacency_matrix.undirected_node_degree(i)
        );
    }

    let incidence: IncidenceMatrix<Undirected> =
        IncidenceMatrix::new(&vec![vec![1, 1, 0], vec![0, 1, 1]]).unwrap();

    println!("Grau de cada vértice (Matriz de Incidência):");
    for i in 0..incidence.order() {
        println!("Vértice {}: {}", i, incidence.undirected_node_degree(i));
    }
}
