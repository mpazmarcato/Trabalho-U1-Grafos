use core::panic;

use graphs_algorithms::graphs::AdjacencyList;

fn main() {
    let _undirected_graph = AdjacencyList(vec![
        vec![1, 7, 2],
        vec![3, 4, 0],
        vec![0],
        vec![1, 4, 5],
        vec![1, 3, 5],
        vec![3, 4, 6],
        vec![5],
        vec![0, 8],
        vec![7, 9],
        vec![8],
    ]);
    fn _m(i: usize) -> char {
        match i {
            0 => 's',
            1 => 'a',
            2 => 'b',
            3 => 'c',
            4 => 'd',
            5 => 'e',
            6 => 'f',
            7 => 'g',
            8 => 'h',
            9 => 'i',
            _ => panic!(),
        }
    }
}
