use graphs_algorithms::{GraphIO, UndirectedGraphIO, graphs::AdjacencyList, print_list};

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
    let _ = g1.undirected_write_bfs_tree(1, PATH.to_owned() + "bfs/bfs_1.dot");

    let dg1: AdjacencyList = GraphIO::from_file("examples/data/DIGRAFO1.txt".to_owned());
    // print_list(&dg1);

    let _ = dg1.write_bfs_tree(1, PATH.to_owned() + "bfs/bfs_2.dot");
}
