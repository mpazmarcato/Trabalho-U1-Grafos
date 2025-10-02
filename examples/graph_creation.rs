use graphs_algorithms::{GraphIO, graphs::AdjacencyMatrix, print_matrix};

static PATH: &str = "examples/data/";
fn main() {
    let m1: AdjacencyMatrix = GraphIO::undirected_from_file(PATH.to_owned() + "GRAFO_2.txt");
    print_matrix(&m1);
}
