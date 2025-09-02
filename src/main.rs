mod graphs {
    pub mod adjacency_list;
}

use crate::graphs::adjacency_list;

fn main() {
    let matrix = vec![
        vec![0, 1, 0],
        vec![1, 1, 0],
        vec![0, 1, 0],
    ];

    let list = adjacency_list::list_matrix(&matrix);

    println!("Adjacency list: ");
    for (i, neighbours) in list.iter().enumerate() {
        println!("{i} -> {neighbours:?}")
    }
}