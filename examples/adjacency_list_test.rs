use std::fs::DirEntry;

use graphs_algorithms::{Graph, UndirectedGraph, graphs::AdjacencyList};

fn print_list(list: &AdjacencyList) {
    for (i, neighbors) in list.0.iter().enumerate() {
        println!("{}: {:?}", i, neighbors);
    }
}
fn main() {
    println!("Digraph!");
    let mut directed_g = AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]);

    println!("New node");
    directed_g.add_node(3);
    print_list(&directed_g);

    println!("Edge 0 - 3");
    directed_g.add_edge(0, 3);
    print_list(&directed_g);

    println!("Edge 1 - 3");
    directed_g.add_edge(1, 3);
    print_list(&directed_g);

    println!("Edge 1 - 2");
    directed_g.add_edge(1, 2);
    print_list(&directed_g);

    println!("Edge 3 - 3");
    directed_g.add_edge(3, 3);
    print_list(&directed_g);

    println!("Undirected graph!");
    let mut undirected_g = AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]);

    println!("New node");
    undirected_g.add_node(3);
    print_list(&undirected_g);

    println!("Edge 0 - 3");
    undirected_g.add_undirected_edge(0, 3);
    print_list(&undirected_g);

    println!("Edge 1 - 3");
    undirected_g.add_undirected_edge(1, 3);
    print_list(&undirected_g);

    println!("Edge 1 - 2");
    undirected_g.add_undirected_edge(1, 2);
    print_list(&undirected_g);

    println!("Edge 3 - 3");
    undirected_g.add_undirected_edge(3, 3);
    print_list(&undirected_g);
}
