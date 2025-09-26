use graphs_algorithms::graphs::AdjacencyList;

fn main() {
    let _digraph = AdjacencyList(vec![
        vec![1, 2, 7],
        vec![3],
        vec![],
        vec![2, 4],
        vec![1, 5],
        vec![3],
        vec![5, 7],
        vec![6],
        vec![4, 9],
        vec![],
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
    todo!()
}
