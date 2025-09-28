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

    pub fn from_adjacency_list(_list: &AdjacencyList) -> Self {
        todo!()
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
        let adj = AdjacencyMatrix(vec![
            vec![0, 1, 0],
            vec![1, 0, 1],
            vec![0, 1, 0],
        ]);

        let inc = IncidenceMatrix::from_adjacency_matrix(&adj);

        assert_eq!(inc.order(), 3); // número de vértices = 3
    }
}
