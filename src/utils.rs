use crate::graphs::{AdjacencyList, AdjacencyMatrix};

pub fn print_list(list: &AdjacencyList) {
    for (i, neighbors) in list.0.iter().enumerate() {
        println!("{}: {:?}", i, neighbors);
    }
}

pub fn print_matrix(m: &AdjacencyMatrix) {
    for row in &m.0 {
        print!("[ ");
        for col in row {
            print!("{col} ");
        }
        print!("]");
        println!();
    }
}
