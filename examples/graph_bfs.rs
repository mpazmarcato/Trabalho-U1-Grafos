use std::io::Error;

use graphs_algorithms::{GraphIO, UndirectedGraphIO, graphs::AdjacencyList};

static PATH: &str = "examples/output/dot/";
fn main() {
    // UNDIRECTED GRAPH
    let g1 = AdjacencyList(vec![
        vec![1, 7, 2],
        vec![3, 4, 0],
        vec![3, 0],
        vec![1, 2, 4, 5],
        vec![1, 3, 5, 8],
        vec![3, 4, 6, 7],
        vec![5, 7],
        vec![0, 5, 6],
        vec![4, 9],
        vec![8],
    ]);

    // print_list(&g1);
    let _ = g1.export_undirected_bfs_to_dot(1, PATH.to_owned() + "bfs/bfs_1.dot");

    let res: Result<AdjacencyList, Error> =
        GraphIO::import_from_file("examples/data/DIGRAFO1.txt".to_owned());

    match res {
        Ok(dg) => {
            let _ = dg.export_directed_bfs_to_dot(1, PATH.to_owned() + "bfs/bfs_2.dot");
        }
        Err(_) => todo!(),
    }
}
