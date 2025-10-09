use graphs_algorithms::{
    Directed, Direction, Graph,
    graphs::{AdjacencyList, AdjacencyMatrix},
};

fn print_matrix<D: Direction>(m: &AdjacencyMatrix<D>) {
    println!("Current matrix: ");
    for row in &m.data() {
        print!("[ ");
        for col in row {
            print!("{col} ");
        }
        print!("]");
        println!();
    }
}

fn print_list<D: Direction>(list: &AdjacencyList<D>) {
    for (i, neighbors) in list.data().iter().enumerate() {
        println!("{}: {:?}", i, neighbors);
    }
}

fn main() {
    let digraph: AdjacencyMatrix<Directed> = AdjacencyMatrix::new(&vec![
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0, 0],
        vec![0, 1, 0, 0, 0, 0],
        vec![1, 1, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 0, 0],
    ])
    .unwrap();
    print_matrix(&digraph);

    let undirected_graph = digraph.underlying_graph();
    print_matrix(&undirected_graph);

    println!("Original graph!");
    let digraph2: AdjacencyList<Directed> = AdjacencyList::new(&vec![
        vec![3],
        vec![],
        vec![1],
        vec![1, 4],
        vec![5],
        vec![2],
    ])
    .unwrap();
    print_list(&digraph2);

    println!("Underlying graph!");
    let undirected_graph2 = digraph2.underlying_graph();
    print_list(&undirected_graph2);
}
