use graphs_algorithms::{
    Graph,
    graphs::{AdjacencyList, AdjacencyMatrix},
    print_list, print_matrix,
};

fn main() {
    let digraph = AdjacencyMatrix(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 0, 0],
    ]);
    println!("Original digraph: ");
    print_matrix(&digraph);

    println!("Now, the underlying graph: ");
    let undirected_graph = digraph.underlying_graph();
    print_matrix(&undirected_graph);

    println!("Original digraph!");
    let digraph2 = AdjacencyList(vec![vec![3], vec![], vec![1], vec![1, 4], vec![5], vec![2]]);
    print_list(&digraph2);

    println!("Now, the underlying graph: ");
    let undirected_graph2 = digraph2.underlying_graph();
    print_list(&undirected_graph2);
}
