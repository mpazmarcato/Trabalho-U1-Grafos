use crate::graphs::AdjacencyMatrix;

// FIXME: ideally the struct field should be private.
#[derive(Debug, Clone)]
pub struct AdjacencyList(pub Vec<Vec<usize>>);

impl AdjacencyList {
    pub fn from_adjacency_matrix(matrix: &AdjacencyMatrix) -> Self {
        let mut adjacency_list = vec![Vec::new(); matrix.0.len()];
        for (i, row) in matrix.0.iter().enumerate() {
            adjacency_list[i].extend(row.iter().enumerate().filter_map(|(j, &val)| {
                if val != 0 {
                    Some(j)
                } else {
                    None
                }
            }));
        }
        AdjacencyList(adjacency_list)
    }
}

// TODO: Implement the Graph trait
// impl Graph for AdjacencyList {}
