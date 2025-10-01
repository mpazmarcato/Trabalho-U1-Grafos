use crate::graphs::{AdjacencyList, AdjacencyMatrix};

#[derive(Debug, Clone)]
pub struct IncidenceMatrix(pub Vec<Vec<i32>>);

impl IncidenceMatrix {
    pub fn from_adjacency_matrix(matrix: &AdjacencyMatrix) -> Self {
        let n = matrix.0.len();

        let mut edges: Vec<(usize, usize)> = Vec::new();
        for i in 0..n {
            for j in 0..n {
                if matrix.0[i][j] == 1 {
                    edges.push((i, j));
                }
            }
        }

        let m = edges.len();
        let mut inc: Vec<Vec<i32>> = vec![vec![0; n]; m];

        for (i, &(l, c)) in edges.iter().enumerate() {
            inc[i][l] = -1;
            inc[i][c] = 1;
        }

        IncidenceMatrix(inc)
    }

    pub fn from_adjacency_list_digraph(_list: &AdjacencyList) -> Self {
        let mut inc: Vec<Vec<i32>> = Vec::new();

        for (c_out, v) in _list.0.iter().enumerate() {
            if !v.is_empty() {
                for c_in in v.iter() {
                    let mut edge: Vec<i32> = vec![0; _list.order()];

                    edge[c_out] = -1;
                    edge[*c_in] = 1;

                    inc.push(edge);
                }
            }
        }

        IncidenceMatrix(inc)
    }

    pub fn node_degree(&self, vertex: usize) -> usize {
        if self.0.is_empty() || vertex >= self.0[0].len() {
            return 0;
        }

        self.0.iter().filter(|row| row[vertex] != 0).count() / 2
    }

    pub fn order(&self) -> usize {
        if self.0.is_empty() {
            0
        } else {
            self.0[0].len()
        }
    }

    pub fn size(&self) -> usize {
        self.0.len() / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_degree_incidence_matrix() {
        // Graph: 0 ── 1
        //        │
        //        2
        let matrix = AdjacencyMatrix(vec![vec![0, 1, 1], vec![1, 0, 0], vec![1, 0, 0]]);

        let incidence = IncidenceMatrix::from_adjacency_matrix(&matrix);

        assert_eq!(incidence.node_degree(0), 2);
        assert_eq!(incidence.node_degree(1), 1);
        assert_eq!(incidence.node_degree(2), 1);
    }

    #[test]
    fn test_order_incidence_matrix() {
        // Graph: 0 ── 1 ── 2
        let adj = AdjacencyMatrix(vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]]);

        let inc = IncidenceMatrix::from_adjacency_matrix(&adj);

        assert_eq!(inc.order(), 3);
    }

    #[test]
    fn test_size_incidence_matrix() {
        // Graph: 0 ── 1 ── 2
        let adj = AdjacencyMatrix(vec![vec![0, 1, 0], vec![1, 0, 1], vec![0, 1, 0]]);

        let inc = IncidenceMatrix::from_adjacency_matrix(&adj);

        assert_eq!(inc.size(), 2);
    }

    #[test]
    fn from_adjacency_list_digraph() {
        // Graph:
        //     0      -> 1
        //       \    /   \
        //        -> 3     -> 2
        //       /
        //      4
        let adj_list = AdjacencyList(vec![vec![3], vec![2], vec![], vec![1], vec![3]]);
        let answer: Vec<Vec<i32>> = vec![
            vec![-1, 0, 0, 1, 0],
            vec![0, -1, 1, 0, 0],
            vec![0, 1, 0, -1, 0],
            vec![0, 0, 0, 1, -1],
        ];

        let inc = IncidenceMatrix::from_adjacency_list_digraph(&adj_list);

        assert_eq!(inc.0, answer);
    }

        assert_eq!(inc.0, answer);
    }
}
