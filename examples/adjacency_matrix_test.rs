use graphs_algorithms::{Graph, UndirectedGraph, graphs::AdjacencyMatrix};

fn print_matrix(m: &AdjacencyMatrix) {
    println!("Current matrix: ");
    for row in &m.0 {
        print!("[ ");
        for col in row {
            print!("{col} ");
        }
        print!("]");
        println!();
    }
}
fn main() {
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
