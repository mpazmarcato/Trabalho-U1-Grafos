use core::panic;

use graphs_algorithms::{print_list, Edge, UndirectedGraphIO};
use graphs_algorithms::UndirectedGraph;
use graphs_algorithms::graphs::AdjacencyList;
use graphs_algorithms::utils::print_tip;

fn main() {
    let undirected_graph = AdjacencyList(vec![
        vec![1, 7, 2],
        vec![3, 4, 0],
        vec![0],
        vec![1, 4, 5],
        vec![1, 3, 5],
        vec![3, 4, 6],
        vec![5],
        vec![0, 8],
        vec![7, 9],
        vec![8],
    ]);

    println!("Graph: ");
    print_list(&undirected_graph);

    for e in undirected_graph.classify_undirected_edges(0) {
        match e {
            Edge::Tree(v, u) => println!("Tree: {} -> {}", v, u),
            Edge::Back(v, u) => println!("Back: {} -> {}", v, u),
            _ => panic!("should not get here"),
        }
    }

    let path = "examples/dot/classify_edges/undirected.dot";
    let _ = undirected_graph.export_undirected_dfs_to_dot(0, path.to_string());

    println!("Graph was exported to dot file on path {}! ", path);

    print_tip();
}