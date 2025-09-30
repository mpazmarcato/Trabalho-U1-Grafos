use graphs_algorithms::{Graph, GraphIO, graphs::AdjacencyMatrix, print_matrix};

fn main() {
    let m1: AdjacencyMatrix = GraphIO::from_file("DIGRAFO1.txt".to_owned());
    print_matrix(&m1);
}
