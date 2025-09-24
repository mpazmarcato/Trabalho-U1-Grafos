use graphs_algorithms::{Graph, graphs::AdjacencyMatrix};

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
fn main() {
    let mut m1 = AdjacencyMatrix(vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]]);

    print_matrix(&m1);
    m1.add_node(3);
    print_matrix(&m1);
    println!("Edge 1 - 3");
    m1.add_edge(1, 3);
    print_matrix(&m1);
    println!("Edge 2 - 3");
    m1.add_edge(2, 3);
    print_matrix(&m1);
    println!("Edge 3 - 0");
    m1.add_edge(3, 0);
    print_matrix(&m1);
    println!("Edge 0 - 0");
    m1.add_edge(0, 0);
    print_matrix(&m1);

    let mut m2 = AdjacencyMatrix(vec![
        vec![0, 0, 0, 1, 0, 0],
        vec![0, 0, 1, 1, 0, 0],
        vec![0, 1, 0, 1, 0, 0],
        vec![1, 1, 1, 0, 1, 1],
        vec![0, 0, 0, 1, 0, 1],
        vec![0, 0, 0, 1, 1, 0],
    ]);
    print_matrix(&m2);
    println!("Edge 0 - 5");
    m2.add_edge(0, 5);
    print_matrix(&m2);
}
