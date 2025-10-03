use graphs_algorithms::{
    GraphIO, UndirectedGraphIO,
    graphs::{AdjacencyList, AdjacencyMatrix},
};

static PATH: &str = "examples/output/";

fn main() {
    // UNDIRECTED GRAPHS
    let g1: AdjacencyMatrix = AdjacencyMatrix(vec![
        vec![0, 1, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
        vec![0, 0, 1, 0],
    ]);
    let _ = g1.write_undirected_graph(PATH.to_owned() + "dot/graph_undirected1.dot");

    let g2 = AdjacencyMatrix(vec![
        vec![0, 1, 0, 0, 1],
        vec![1, 0, 1, 0, 0],
        vec![0, 1, 0, 1, 1],
        vec![0, 0, 1, 0, 0],
        vec![1, 0, 1, 0, 0],
    ]);
    let _ = g2.write_undirected_graph(PATH.to_owned() + "dot/graph_undirected2.dot");

    let g3 = AdjacencyList(vec![
        vec![1, 2],
        vec![0, 2],
        vec![0, 1, 3, 4],
        vec![2, 4],
        vec![2],
    ]);
    let _ = g3.write_undirected_graph(PATH.to_owned() + "dot/graph_undirected3.dot");

    let g4 = AdjacencyMatrix(vec![
        vec![0, 1, 1, 0, 1, 0],
        vec![1, 0, 1, 1, 0, 1],
        vec![1, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 1],
        vec![1, 0, 1, 1, 0, 1],
        vec![0, 1, 0, 1, 1, 0],
    ]);
    let _ = g4.write_undirected_graph(PATH.to_owned() + "dot/graph_undirected4.dot");

    let g5 = AdjacencyMatrix(vec![
        vec![0, 1, 1, 1, 1, 0],
        vec![1, 0, 1, 1, 1, 1],
        vec![1, 1, 0, 1, 1, 1],
        vec![1, 1, 1, 0, 1, 1],
        vec![1, 1, 1, 1, 0, 1],
        vec![0, 1, 1, 1, 1, 0],
    ]);
    let _ = g5.write_undirected_graph(PATH.to_owned() + "dot/graph_undirected5.dot");

    // DIGRAPHS

    let dg1 = AdjacencyList(vec![vec![1], vec![2], vec![3], vec![]]);
    let _ = dg1.write_graph(PATH.to_owned() + "dot/graph_directed1.dot");

    let dg2 = AdjacencyList(vec![vec![1, 2], vec![], vec![3], vec![]]);
    let _ = dg2.write_graph(PATH.to_owned() + "dot/graph_directed2.dot");

    let dg3 = AdjacencyMatrix(vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
    ]);
    let _ = dg3.write_graph(PATH.to_owned() + "dot/graph_directed3.dot");

    let dg4 = AdjacencyMatrix(vec![
        vec![0, 1, 1, 0, 1],
        vec![0, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 1, 0, 0, 0],
    ]);
    let _ = dg4.write_graph(PATH.to_owned() + "dot/graph_directed4.dot");

    let dg5 = AdjacencyList(vec![
        vec![1, 3],
        vec![4],
        vec![0, 4],
        vec![4],
        vec![0],
        vec![2, 6],
        vec![7],
        vec![6],
    ]);

    let _ = dg5.write_graph(PATH.to_owned() + "dot/graph_directed5.dot");
}
