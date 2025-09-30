use graphs_algorithms::{GraphIO, graphs::AdjacencyMatrix, print_matrix};

static PATH: &str = "examples/data/";
fn main() {
    let m1: AdjacencyMatrix = GraphIO::from_file(PATH.to_owned() + "DIGRAFO1.txt");
    print_matrix(&m1);
}
