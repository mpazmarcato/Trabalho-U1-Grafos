use crate::graphs::{AdjacencyList, AdjacencyMatrix};
use crate::incidence_matrix::IncidenceMatrix;

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

pub fn print_incidence_matrix(m: &IncidenceMatrix) {
    for row in &m.0 {
        print!("[ ");
        for col in row {
            print!("{col} ");
        }
        print!("]");
        println!();
    }
}

pub fn print_tip() {
    println!("[TIP]: you should try 'make png' to see these graphs!");
}
