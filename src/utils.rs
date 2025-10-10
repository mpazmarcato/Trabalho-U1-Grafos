use crate::{
    Direction, Graph,
    graphs::{AdjacencyList, AdjacencyMatrix},
};

pub fn print_list<D: Direction>(list: &AdjacencyList<D>) {
    for (i, neighbors) in list.data().iter().enumerate() {
        println!("{}: {:?}", i, neighbors);
    }
}

pub fn print_matrix<D: Direction>(m: &AdjacencyMatrix<D>) {
    for row in &m.data() {
        print!("[ ");
        for col in row {
            print!("{col} ");
        }
        print!("]");
        println!();
    }
}
