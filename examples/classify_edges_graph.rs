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
    let to_ch = |i| match i {
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
    };
    let print_edge = |e| match e {
        Edge::Tree(v, u) => println!("Tree: {} -> {}", to_ch(v), to_ch(u)),
        Edge::Back(v, u) | Edge::ParentBack(v, u) => {
            println!("Back: {} -> {}", to_ch(v), to_ch(u))
        }
        Edge::Forward(v, u) => println!("Forward: {} -> {}", to_ch(v), to_ch(u)),
        Edge::Cross(v, u) => println!("Cross: {} -> {}", to_ch(v), to_ch(u)),
    };
    let mut iter = digraph.classify_edges(0);
    for e in &mut iter {
        print_edge(e);
    }
    iter.new_start(8);
    for e in &mut iter {
        print_edge(e)
    }
}
