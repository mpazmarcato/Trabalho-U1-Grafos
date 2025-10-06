use std::io::Error;

use graphs_algorithms::{UndirectedGraphIO, graphs::AdjacencyMatrix, print_matrix};

static PATH: &str = "examples/data/";
fn main() {
    let res: Result<AdjacencyMatrix, Error> =
        UndirectedGraphIO::import_undirected_from_file(PATH.to_owned() + "GRAFO_2.txt");

    let m1 = res.unwrap();
    print_matrix(&m1);

    let res2: Result<AdjacencyMatrix, Error> =
        UndirectedGraphIO::import_undirected_from_file(PATH.to_owned() + "GRAFO_0.txt");

    match res2 {
        Ok(data) => println!("{:?}", data),
        Err(e) => println!("{e}"),
    }
}
