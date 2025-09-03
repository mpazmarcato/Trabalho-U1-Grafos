use crate::graphs::{AdjacencyMatrix};

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
  }
