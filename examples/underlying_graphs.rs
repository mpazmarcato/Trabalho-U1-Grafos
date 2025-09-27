use graphs_algorithms::{Graph, graphs::AdjacencyMatrix};

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
    let digraph = AdjacencyMatrix(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 0, 0],
    ]);
    print_matrix(&digraph);

    let undirected_graph = digraph.underlying_graph();
    print_matrix(&undirected_graph);
}
