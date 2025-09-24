use graphs_algorithms::{Graph, graphs::AdjacencyList};

fn print_list(list: &AdjacencyList) {
    for (i, neighbors) in list.0.iter().enumerate() {
        println!("{}: {:?}", i, neighbors);
    }
}
fn main() {
    let mut l1 = AdjacencyList(vec![vec![1, 2], vec![0], vec![0]]);

    println!("New node");
    l1.add_node(3);
    print_list(&l1);

    println!("Edge 0 - 3");
    l1.add_edge(0, 3);
    print_list(&l1);

    println!("Edge 1 - 3");
    l1.add_edge(1, 3);
    print_list(&l1);

    println!("Edge 1 - 2");
    l1.add_edge(1, 2);
    print_list(&l1);
}
