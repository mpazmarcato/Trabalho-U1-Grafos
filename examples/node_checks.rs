use graphs_algorithms::UndirectedGraph;
use graphs_algorithms::graphs::{AdjacencyList, AdjacencyMatrix, IncidenceMatrix};
use graphs_algorithms::utils::print_incidence_matrix;
use graphs_algorithms::{Graph, print_list, print_matrix};

fn main() {
    let dg1 = AdjacencyList(vec![vec![1, 3], vec![2], vec![1], vec![2, 4], vec![]]);

    println!("Current digraph on adjacency list:");
    print_list(&dg1);

    println!("Graph order: {} ", dg1.order());
    println!("Graph size: {} ", dg1.size());
    println!("Degree calculation!");
    for i in 0..dg1.order() {
        let degree = dg1.node_degrees(i);
        println!(
            "Vertex {} has internal degree {} and external degree {}",
            i, degree.0, degree.1
        );
    }

    let ug1 = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 1], vec![1, 1, 0]]);
    println!("Current undirected graph on adjacency matrix:");
    print_matrix(&ug1);

    println!("Graph order: {} ", ug1.order());
    println!("Graph size: {} ", ug1.undirected_size());
    println!("Degree calculation!");
    for i in 0..ug1.order() {
        println!("Vertex {} has degree {}", i, ug1.undirected_node_degree(i));
    }

    let ug2 = IncidenceMatrix(vec![vec![1, 1, 0], vec![0, 1, 1]]);

    println!("Current undirected graph on incidence matrix: ");
    print_incidence_matrix(&ug2);

    println!("Graph order: {} ", ug2.order());
    println!("Graph size: {} ", ug2.undirected_size());
    println!("Degree calculation!");
    for i in 0..ug2.order() {
        println!("Vertex {} has degree {}", i, ug2.undirected_node_degree(i));
    }
}
