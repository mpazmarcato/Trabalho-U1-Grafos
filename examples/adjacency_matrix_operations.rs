use graphs_algorithms::{Graph, UndirectedGraph, graphs::AdjacencyMatrix, print_matrix};

fn digraph_create_and_add() {
    println!("Digraph!");
    let mut directed_m = AdjacencyMatrix(vec![vec![0, 1, 1], vec![0, 0, 0], vec![0, 1, 0]]);

    print_matrix(&directed_m);
    directed_m.add_node(3);
    print_matrix(&directed_m);
    println!("Edge 1 -> 3");
    directed_m.add_edge(1, 3);
    print_matrix(&directed_m);
    println!("Edge 2 -> 3");
    directed_m.add_edge(2, 3);
    print_matrix(&directed_m);
    println!("Edge 3 -> 0");
    directed_m.add_edge(3, 0);
    print_matrix(&directed_m);
    println!("Edge 0 -> 0");
    directed_m.add_edge(0, 0);
    print_matrix(&directed_m);
}

fn undirected_graph_create_and_add() {
    println!("Undirected graph!");
    let mut undirected_m = AdjacencyMatrix(vec![
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0],
        vec![0, 1, 0, 1, 0, 0],
        vec![1, 1, 1, 0, 1, 1],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 1, 0],
    ]);
    print_matrix(&undirected_m);
    println!("Edge 0 - 5");
    undirected_m.add_undirected_edge(0, 5);
    print_matrix(&undirected_m);
}

fn digraph_delete() {
    let mut m = AdjacencyMatrix(vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 1],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 1, 0, 0],
    ]);
    print_matrix(&m);
    println!("Delete edge 3 -> 2");
    m.remove_edge(3, 2);
    print_matrix(&m);

    println!("Delete node 2");
    m.remove_node(2);
    print_matrix(&m);
}

fn undirected_graph_delete() {
    println!("Undirected graph!");
    let mut undirected_m = AdjacencyMatrix(vec![
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0],
        vec![0, 1, 0, 1, 0, 0],
        vec![1, 1, 1, 0, 1, 1],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 1, 0],
    ]);
    print_matrix(&undirected_m);

    println!("Delete edge 3 - 4 ");
    undirected_m.remove_undirected_edge(3, 4);
    print_matrix(&undirected_m);

    println!("Delete node 3");
    undirected_m.remove_node(3);
    print_matrix(&undirected_m);
}

fn main() {
    undirected_graph_delete();
}
