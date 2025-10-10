use graphs_algorithms::{
    Directed, Graph, GraphIO, Undirected, UndirectedGraphIO, graphs::AdjacencyList, print_list,
};

static PATH: &str = "examples/dot/";

fn main() {
    let graph1: AdjacencyList<Undirected> = AdjacencyList::new(&vec![
        vec![1, 2],
        vec![2, 3, 7, 10],
        vec![0, 1, 4, 3],
        vec![2, 6, 8, 7],
        vec![2, 5],
        vec![4],
        vec![3, 9, 10],
        vec![1, 3],
        vec![3, 10],
        vec![6, 10],
        vec![1, 6, 9, 8],
    ])
    .unwrap();

    println!("[UNDIRECTED] Graph 1: ");
    print_list(&graph1);

    let result1 = graph1.export_undirected_to_dot(PATH.to_owned() + "graph1.dot");
    match result1 {
        Ok(_) => println!("Created .dot file for the graph above!"),
        Err(e) => println!("{e}"),
    }

    let result2 = graph1.export_undirected_bfs_to_dot(6, PATH.to_owned() + "bfs/bfs_graph1.dot");
    match result2 {
        Ok(_) => println!("Created BFS tree with 6 as root for the graph above!"),
        Err(e) => println!("{e}"),
    }

    let result3 = graph1.export_undirected_dfs_to_dot(6, PATH.to_owned() + "dfs/dfs_graph1.dot");
    match result3 {
        Ok(_) => println!("Created DFS tree with 6 as root for the graph above!"),
        Err(e) => println!("{e}"),
    }

    // DIGRAPHS
    let graph2: AdjacencyList<Directed> = AdjacencyList::new(&vec![
        vec![1, 3],
        vec![4],
        vec![4, 0],
        vec![4],
        vec![0],
        vec![2, 6],
        vec![7],
        vec![8],
    ])
    .unwrap();

    println!("[DIRECTED] Graph 2: ");
    print_list(&graph2);

    let result4 = graph2.export_to_dot(PATH.to_owned() + "graph2.dot");
    match result4 {
        Ok(_) => println!("Created .dot file for the graph above!"),
        Err(e) => println!("{e}"),
    }

    let result5 = graph2.export_directed_bfs_to_dot(5, PATH.to_owned() + "bfs/bfs_graph2.dot");
    match result5 {
        Ok(_) => println!("Created BFS tree with 5 as root for the graph above!"),
        Err(e) => println!("{e}"),
    }

    let result6 = graph2.export_directed_dfs_to_dot(5, PATH.to_owned() + "dfs/dfs_graph2.dot");
    match result6 {
        Ok(_) => println!("Created DFS tree with 5 as root for the graph above!"),
        Err(e) => println!("{e}"),
    }
}
