use graphs_algorithms::graphs::AdjacencyMatrix;

pub fn create_complete_graph(size: usize) -> AdjacencyMatrix {
    let mut matrix = vec![vec![0; size]; size];
    for (i, row) in matrix.iter_mut().enumerate() {
        for (j, val) in row.iter_mut().enumerate() {
            if i != j {
                *val = 1;
            }
        }
    }
    AdjacencyMatrix(matrix)
}
