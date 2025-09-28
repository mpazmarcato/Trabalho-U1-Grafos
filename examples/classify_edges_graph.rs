use graphs_algorithms::Edge;
use graphs_algorithms::Graph;
use graphs_algorithms::graphs::AdjacencyList;

fn main() {
    let digraph = AdjacencyList(vec![
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
    fn m(i: usize) -> char {
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
    for e in digraph.classify_edges(0) {
        match e {
            Edge::Tree(v, u) => println!("Tree: {} -> {}", m(v), m(u)),
            Edge::Back(v, u) | Edge::ParentBack(v, u) => println!("Back: {} -> {}", m(v), m(u)),
            Edge::Foward(v, u) => println!("Foward: {} -> {}", m(v), m(u)),
            Edge::Cross(v, u) => println!("Cross: {} -> {}", m(v), m(u)),
        }
    }
}
