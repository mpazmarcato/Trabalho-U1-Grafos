use std::io::Error;

use graphs_algorithms::{GraphIO, UndirectedGraphIO, graphs::AdjacencyList, print_list};
use graphs_algorithms::utils::print_tip;

static PATH: &str = "examples/dot/";
fn main() {
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

    println!("BFS on following undirected graph: ");
    print_list(&g1);

    let path1 = PATH.to_owned() + "bfs/bfs_1.dot";
    let _ = g1.export_undirected_bfs_to_dot(1, path1.to_owned());
    println!("Undirected graph was exported to dot file on path {}! ", path1);

    let res: Result<AdjacencyList, Error> =
        GraphIO::import_from_file("examples/data/DIGRAFO1.txt".to_owned());

    match res {
        Ok(dg) => {
            println!("BFS on following digraph: ");
            print_list(&g1);

            let path2 = PATH.to_owned() + "bfs/bfs_2.dot";
            let _ = dg.export_directed_bfs_to_dot(1, path2.to_owned());
            println!("Digraph was exported to dot file on path {}! ", path2);
        }
        Err(_) => println!("oops.. shouldn't fall here"),
    }

    print_tip();
}
