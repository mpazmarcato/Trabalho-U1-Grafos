use graphs_algorithms::{
    Graph, UndirectedGraph,
    graphs::{AdjacencyList, AdjacencyMatrix},
};

fn main() {
    // UNDIRECTED GRAPHS
    let g1 = AdjacencyMatrix(vec![
        vec![0, 1, 0, 0],
        vec![1, 0, 1, 0],
        vec![0, 1, 0, 1],
        vec![0, 0, 1, 0],
    ]);
    g1.write_undirected_graph("./output/graph_undirected1.dot".to_owned());

    let g2 = AdjacencyMatrix(vec![
        vec![0, 1, 0, 0, 1],
        vec![1, 0, 0, 0, 0],
        vec![0, 1, 0, 1, 1],
        vec![0, 0, 1, 0, 0],
        vec![1, 0, 1, 0, 0],
    ]);
    g2.write_undirected_graph("./output/graph_undirected2.dot".to_owned());

    let g3 = AdjacencyList(vec![vec![1], vec![0, 2], vec![1, 3, 4], vec![2], vec![2]]);
    g3.write_undirected_graph("./output/graph_undirected3.dot".to_owned());

    let g4 = AdjacencyMatrix(vec![
        vec![0, 1, 1, 0, 1, 0],
        vec![1, 0, 1, 1, 0, 1],
        vec![1, 1, 0, 1, 1, 0],
        vec![0, 1, 1, 0, 1, 1],
        vec![1, 0, 1, 1, 0, 1],
        vec![0, 1, 0, 1, 1, 0],
    ]);
    g4.write_undirected_graph("./output/graph_undirected4.dot".to_owned());

    let g5 = AdjacencyMatrix(vec![
        vec![0, 1, 1, 1, 1, 0],
        vec![1, 0, 1, 1, 1, 1],
        vec![1, 1, 0, 1, 1, 1],
        vec![1, 1, 1, 0, 1, 1],
        vec![1, 1, 1, 1, 0, 1],
        vec![0, 1, 1, 1, 1, 0],
    ]);
    g5.write_undirected_graph("./output/graph_undirected5.dot".to_owned());

    // DIGRAPHS

    let dg1 = AdjacencyList(vec![vec![1], vec![2], vec![3], vec![]]);
    dg1.write_graph("./output/graph_directed1.dot".to_owned());

    let dg2 = AdjacencyList(vec![vec![1, 2], vec![], vec![3], vec![]]);
    dg2.write_graph("./output/graph_directed2.dot".to_owned());

    let dg3 = AdjacencyMatrix(vec![
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 1, 1],
        vec![0, 0, 0, 0, 0],
        vec![1, 0, 0, 0, 0],
    ]);
    dg3.write_graph("./output/graph_directed3.dot".to_owned());

    let dg4 = AdjacencyMatrix(vec![
        vec![0, 1, 1, 0, 1],
        vec![0, 0, 1, 1, 0],
        vec![0, 0, 0, 1, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 1, 0, 0, 0],
    ]);
    dg4.write_graph("./output/graph_directed4.dot".to_owned());

    let dg5 = AdjacencyMatrix(vec![
        vec![0, 1, 1, 1, 0, 1],
        vec![0, 0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 1, 1],
        vec![1, 0, 0, 0, 1, 1],
        vec![1, 1, 0, 0, 0, 1],
        vec![1, 1, 1, 0, 0, 0],
    ]);
    dg5.write_graph("./output/graph_directed5.dot".to_owned());
}
