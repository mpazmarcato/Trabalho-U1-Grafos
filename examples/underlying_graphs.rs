use graphs_algorithms::{
    Graph,
    graphs::{AdjacencyList, AdjacencyMatrix},
};

fn print_matrix(m: &AdjacencyMatrix) {
    println!("Current matrix: ");
    for row in &m.0 {
        print!("[ ");
        for col in row {
            print!("{col} ");
        }
        print!("]");
        println!();
    }
}

fn print_list(list: &AdjacencyList) {
    for (i, neighbors) in list.0.iter().enumerate() {
        println!("{}: {:?}", i, neighbors);
    }
}

fn main() {
    let digraph = AdjacencyMatrix(vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 0, 0],
    ]);
    print_matrix(&digraph);

    let undirected_graph = digraph.underlying_graph();
    print_matrix(&undirected_graph);

    println!("Original graph!");
    let digraph2 = AdjacencyList(vec![vec![3], vec![], vec![1], vec![1, 4], vec![5], vec![2]]);
    print_list(&digraph2);

    println!("Underlying graph!");
    let undirected_graph2 = digraph2.underlying_graph();
    print_list(&undirected_graph2);
}
