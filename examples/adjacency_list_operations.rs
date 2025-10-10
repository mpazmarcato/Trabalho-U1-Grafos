use graphs_algorithms::{
    Directed, Direction, Graph, Undirected, UndirectedGraph, graphs::AdjacencyList, print_list,
};

fn digraph_add() {
    println!("Digraph!");
    let mut directed_g: AdjacencyList<Directed> =
        AdjacencyList::new(&vec![vec![1, 2], vec![0], vec![0]]).unwrap();

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
}

fn digraph_delete() {
    println!("Digraph!");

    let mut directed_g: AdjacencyList<Directed> = AdjacencyList::new(&vec![
        vec![3],
        vec![],
        vec![1],
        vec![1, 4],
        vec![5],
        vec![2],
    ])
    .unwrap();

    print_list(&directed_g);

    println!("Delete edge 3 -> 1");
    directed_g.remove_edge(3, 1);
    print_list(&directed_g);

    println!("Delete Node 2");
    directed_g.remove_node(2);
    print_list(&directed_g);
}

fn undirected_graph_add() {
    println!("Undirected graph!");
    let mut undirected_g: AdjacencyList<Undirected> =
        AdjacencyList::new(&vec![vec![1, 2], vec![0], vec![0]]).unwrap();

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

fn undirected_graph_delete() {
    println!("Undirected graph!");

    let mut undirected_g: AdjacencyList<Undirected> = AdjacencyList::new(&vec![
        vec![1, 2],
        vec![0, 2],
        vec![0, 1, 3, 4],
        vec![2],
        vec![2],
    ])
    .unwrap();

    print_list(&undirected_g);

    println!("Delete edge 2 -> 1");
    undirected_g.remove_undirected_edge(2, 1);
    print_list(&undirected_g);

    println!("Delete Node 2");
    undirected_g.remove_node(2);
    print_list(&undirected_g);
}

fn main() {
    undirected_graph_delete();
}
